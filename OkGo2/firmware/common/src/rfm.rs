use f0;
use stm32f0xx;

extern "C" {
    /// Set the RFM95W centre frequency using an FRF register value
    pub fn rfm_setfreq(frf: u32);

    /// Transmit a packet length len stored in buf, optional PA_BOOST to 100mW TX
    pub fn rfm_transmit(buf: *const u8, len: u8);

    /// Retrieve a packet, length len, into buf
    pub fn rfm_receive(buf: *mut u8, len: u8);

    /// Put module into receive mode then return
    pub fn rfm_receive_async(len: u8);

    /// Check if a packet has been received and is waiting to be retrieved
    pub fn rfm_packet_waiting() -> bool;

    /// Attempt to retrieve a packet received in async mode.  Return success
    pub fn rfm_packet_retrieve(buf: *mut u8, len: u8) -> bool;

    /// Set transmit power to a dBm value from 2 to +17dBm
    pub fn rfm_setpower(power: u8);

    /// Retrieve RSSI/SNR of last packet received
    pub fn rfm_getrssi() -> u8;
}

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
        unsafe {
            rfm_setfreq(freq as u32);
            rfm_setpower(power);
        }
    }

    fn init_helper(&self) {
        //     uint8_t RegOpMode, RegModemConfig1, RegModemConfig2;

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
        ::utils::delay_ms(10);

        // Check we're in sleep mode
        _rfm_setmode(Mode::Sleep);
        let mut reg_op_mode: u8 = _rfm_readreg(Registers::OpMode);

        // Activate LoRa!
        reg_op_mode |= Mode::LongRange as u8;
        _rfm_writereg(Registers::OpMode, reg_op_mode);

        let reg_modem_config_1 =
            // Set bandwidth to 125kHz -> 0111
            RFM_Bw2 | RFM_Bw1 | RFM_Bw0 |
            // Set coding rate to 4/8 -> 100
            RFM_CodingRate2 |
            // Implicit header mode
            RFM_ImplicitHeaderModeOn;
        _rfm_writereg(RFM_RegModemConfig1, RegModemConfig1);


        RegModemConfig2 =
            // Set SF9 = 256 chips/symbol
            RFM_SpreadingFactor3 |
            // Enable CRCs:
            RFM_RxPayloadCrcOn;
        /* Write config: */
        _rfm_writereg(RFM_RegModemConfig2, RegModemConfig2);
    }
}
