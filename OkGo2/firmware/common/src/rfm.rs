use f0;
use stm32f0xx;
use radio_reg::*;
use utils::delay_us;

/// These frequency register values are found using:
/// Freq = FRF * 32,000,000 / 2^19
/// FRF = Freq / 32,000,000 * 2^19
#[repr(u32)]
pub enum Frf {
    /// http://stakeholders.ofcom.org.uk/binaries/spectrum/spectrum-policy-area/spectrum-management/research-guidelines-tech-info/interface-requirements/IR_2030-june2014.pdf
    /// Limit 25mW = 14dBm ERP, no channel bw limit
    /// Either limit duty cycle to 1% or implement Directive 1999/5/EC or equiv.
    /// Freq = 865,913,993 Hz -> FRF = 14187134.8613 */
    Frf868 = 14187135,
    /// http://www.digikey.com/en/articles/techzone/2011/may/unlicensed-915-mhz-band-fits-many-applications-and-allows-higher-transmit-power
    /// Limit 4W = 36dBm, unsure of bw limit
    /// DSSS required but not FHSS
    /// Freq = 925,892,009 Hz -> FRF = 15169814.6755
    Frf915 = 15169815,
}

pub struct Radio {
    nss: f0::output::Output,
    spi: f0::spi::Spi,
}

impl Radio {
    pub const fn new(nss: f0::output::Output, spi: f0::spi::Spi) -> Radio {
        Radio { nss: nss, spi: spi }
    }

    pub fn init(&self, rcc: &stm32f0xx::RCC, spi: &stm32f0xx::SPI1, freq: Frf, power: u8) {
        // Clock SPI1 peripheral and setup GPIOs appropriately:
        // NSS, SCK, MOSI, RESET are outputs,
        // MISO is input.
        // SPI setup is done in rfm95w.c
        rcc.apb2enr.write(|w| w.spi1en().set_bit());

        // Make sure NSS doesn't blip when we enable it:
        self.nss.clear();
        self.nss.setup();
        self.spi.setup_spi1(spi);

        // Run RFM95W initialization
        self.init_helper();
        self.setfreq(freq);
        self.setpower(power);
    }

    fn init_helper(&self) {
        // uint8_t RegOpMode, RegModemConfig1, RegModemConfig2;

        // Initialise SPI peripheral
        // self.spi.reset()
        // self.spi.disable_crc();
        // self.spi.init_master(
        //     spi::BaudRate::FpclkDiv64, // Slightly under 1MHz
        //     spi::Cpol::ClkTo0WhenIdle, // ???
        //     spi::Cpha::ClkTransition1,
        //     spi::Crcl::Bit8, // DFF/CRC length
        //     spi::BitOrder::MsbFirst // MSB first
        // );

        // Manual NSS handling:
        // self.spi.enable_software_slave_management();
        // self.spi.set_nss_high();
        self.nss.set();

        // self.spi.set_data_size(spi::DataSize::Bit8)
        // self.spi.set_fifo_reception_threshold_8bit(); // 8-bit rx-length

        // self.spi.enable()


        // Wait for chip to warm up
        delay_us(10_000);

        // Check we're in sleep mode
        self.setmode(Mode::Sleep);
        let mut reg_op_mode = OpMode::from_bits(self.readreg(Register::OpMode)).unwrap();

        // Activate LoRa!
        reg_op_mode |= LongRangeMode;
        self.writereg(Register::OpMode, reg_op_mode.bits());

        let reg_modem_config_1 =
            // Set bandwidth to 125kHz -> 0111
            Bw2 | Bw1 | Bw0 |
            // Set coding rate to 4/8 -> 100
            CodingRate2 |
            // Implicit header mode
            ImplicitHeaderModeOn;
        self.writereg(Register::ModemConfig1, reg_modem_config_1.bits());


        let reg_modem_config_2 =
            // Set SF9 = 256 chips/symbol
            SpreadingFactor3 |
            // Enable CRCs:
            RxPayloadCrcOn;
        self.writereg(Register::ModemConfig2, reg_modem_config_2.bits());
    }

    /// Write the byte of data to the address
    fn writereg(&self, register: Register, data: u8) {
        self.nss.clear();
        self.spi.write(register as u8 | (1 << 7));
        self.spi.write(data);
        self.nss.set();
    }

    /// Read a byte of data from the address
    fn readreg(&self, register: Register) -> u8 {
        self.nss.clear();
        self.spi.write(register as u8 & !(1 << 7));
        let data = self.spi.transfer(0x00);
        self.nss.set();
        data
    }

    /// Bulk write to a register from a buffer
    fn bulkwrite(&self, register: Register, data: &[u8]) {
        self.nss.clear();
        self.spi.write(register as u8 | (1 << 7));
        for byte in data {
            self.spi.write(*byte);
        }
        self.nss.set();
    }

    /// Bulk read from a register to a buffer
    fn bulkread(&self, register: Register, data: &mut [u8]) {
        self.nss.clear();
        self.spi.write(register as u8 & !(1 << 7));
        for byte in data {
            *byte = self.spi.transfer(0x00);
        }
        self.nss.set();
    }

    fn setmode(&self, mode: Mode) {
        // We read the old value, set the mode bits as appropriate then rewrite the
        // register, so we don't stomp all over the other register bits
        let old_mode = self.readreg(Register::OpMode) & 0b11111000;
        self.writereg(Register::OpMode, old_mode | mode as u8);

        // Wait for new mode to take effect
        while self.readreg(Register::OpMode) & 0b00000111 != mode as u8 {}
    }

    /// Set the RFM95W centre frequency using an FRF register value
    pub fn setfreq(&self, frf: Frf) {
        let frf = frf as u32;

        // Check the radio is sleeping to set the frequency/
        self.setmode(Mode::Sleep);

        // Write 24 bits of frequency
        self.writereg(Register::FrfMsb, (frf >> 16) as u8);
        self.writereg(Register::FrfMid, (frf >> 8) as u8);
        self.writereg(Register::FrfLsb, frf as u8);

        // Wake up the radio, spin up the synthesizers!
        self.setmode(Mode::Standby)
    }

    /// Set transmit power to a dBm value from 2 to +17dBm
    pub fn setpower(&self, power: u8) {
        let mut power = power;
        let mut pa_config: u8 = 0x00;

        // Force boost mode for the HopeRF module, restricts power range to
        // 2 - 17dBm (without using extra boost to 20dBm)
        if (power < 2) || (power > 17) {
            power = 2; // 2dBm sensible default
        }

        // Select boost PA
        pa_config |= PaSelect.bits();

        /* Actual Power = OutputPower + 2dBm, so set OutputPower=power-2 */
        pa_config |= power - 2;

        self.writereg(Register::PaConfig, pa_config);
    }


    /// Transmit a packet length len stored in buf, optional PA_BOOST to 100mW TX
    pub fn transmit(&self, data: &[u8]) {
        // Check we're in stand-by
        self.setmode(Mode::Standby);

        // Set packet length
        self.writereg(Register::PayloadLength, data.len() as u8);

        // Move to the beginning of the TX FIFO
        self.writereg(
            Register::FifoAddrPtr,
            self.readreg(Register::FifoTxBaseAddr),
        );

        // Fill the FIFO
        self.bulkwrite(Register::Fifo, data);

        // Request TX mode to initiate send
        self.setmode(Mode::Tx);

        // TODO: For now, block on sending
        while self.readreg(Register::OpMode) & 0b00000111 == Mode::Tx as u8 {}

        // Clear txdone interrupt flag
        self.writereg(Register::OpMode, TxDone.bits()); // Really?
    }

    /// Retrieve a received packet, into buf
    pub fn receive(&self, data: &mut [u8]) {
        // Check we're in stand-by
        self.setmode(Mode::Standby);

        // Set packet length
        self.writereg(Register::PayloadLength, data.len() as u8);

        let mut valid_received = false;
        while !valid_received {
            // Set Fifo to beginning of RX buffer
            self.writereg(
                Register::FifoAddrPtr,
                self.readreg(Register::FifoRxBaseAddr),
            );

            // Initiate receive using mode change
            self.setmode(Mode::RxSingle);

            // Block until receipt or timeout
            let mut irq_flags = IrqFlags::all();
            while !irq_flags.intersects(RxDone | RxTimeout) {
                irq_flags = IrqFlags::from_bits(self.readreg(Register::IrqFlags)).unwrap();
            }

            // Received if not timeout and CRC done and length correct
            valid_received = self.readreg(Register::RxNbBytes) == data.len() as u8 &&
                irq_flags.intersects(RxDone) &&
                !irq_flags.intersects(PayloadCrcError);

            // Clear RxDone, RxTimeout, and CRC fail interrupts
            self.writereg(
                Register::IrqFlags,
                (RxDone | PayloadCrcError | RxTimeout).bits(),
            );
        }

        /* Move FIFO pointer to beginning of last packet received */
        self.writereg(
            Register::FifoAddrPtr,
            self.readreg(Register::FifoRxCurrentAddr),
        );

        /* Read packet out of FIFO into our buffer */
        self.bulkread(Register::Fifo, data);
    }

    /// Put module into receive mode then return
    pub fn receive_async(&self, len: u8) {
        // Check we're in stand-by
        self.setmode(Mode::Standby);

        // Set packet length
        self.writereg(Register::PayloadLength, len);

        // Clear possible interrupts
        self.writereg(
            Register::IrqFlags,
            (RxDone | PayloadCrcError | RxTimeout).bits(),
        );

        // Set Fifo to beginning of RX buffer
        self.writereg(
            Register::FifoAddrPtr,
            self.readreg(Register::FifoRxBaseAddr),
        );

        // Initiate receive using mode change
        self.setmode(Mode::RxContinuous);
    }

    /// Attempt to retrieve a packet received in async mode.  Return success */
    pub fn packet_retrieve(&self, data: &mut [u8]) -> bool {
        let irq_flags = IrqFlags::from_bits(self.readreg(Register::IrqFlags)).unwrap();
        let rx_len = self.readreg(Register::RxNbBytes);

        if irq_flags.intersects(RxDone) && !irq_flags.intersects(PayloadCrcError) &&
            rx_len as usize == data.len()
        {
            // Good receive.
            // Move FIFO pointer to beginning of last packet received
            self.writereg(
                Register::FifoAddrPtr,
                self.readreg(Register::FifoRxCurrentAddr),
            );

            // Read packet out of FIFO into our buffer
            self.bulkread(Register::Fifo, data);

            // Clear RxDone, RxTimeout, and CRC fail interrupts
            self.writereg(
                Register::IrqFlags,
                (RxDone | PayloadCrcError | RxTimeout).bits(),
            );

            return true;
        } else {
            // Bad receive
            // Clear RxDone, RxTimeout, and CRC fail interrupts
            self.writereg(
                Register::IrqFlags,
                (RxDone | PayloadCrcError | RxTimeout).bits(),
            );

            return false;
        }
    }

    /// Check if a packet has been received and is waiting to be retrieved
    pub fn packet_waiting(&self) -> bool {
        IrqFlags::from_bits(self.readreg(Register::IrqFlags))
            .unwrap()
            .intersects(RxDone)
    }

    /// Retrieve RSSI/SNR of last packet received
    pub fn getrssi(&self) -> u8 {
        const DISPLAY_SNR: bool = true;

        if DISPLAY_SNR {
            let mut signed_val = self.readreg(Register::PktSnrValue) as i16;
            // Shift it up by 128 to make all vals positive
            signed_val += 128;
            // Scale 0-255 to 0-99
            (signed_val as u16 * 99 / 255) as u8
        } else {
            // Scale 0-155 (highest observed) to 0-99%
            let rssi = self.readreg(Register::PktRssiValue) as u16 * 99 / 155;
            if rssi > 99 {
                99
            } else {
                rssi as u8
            }
        }
    }
}
