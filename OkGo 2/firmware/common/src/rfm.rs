extern "C" {
    /// Initialise the RFM95W.
    pub fn rfm_initialise(spi_periph: u32, nss_port: u32, nss_pin: u32);

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
