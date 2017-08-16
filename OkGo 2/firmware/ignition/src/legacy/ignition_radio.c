#include <stdbool.h>
#include <stdint.h>
#include <string.h>

#include <libopencm3/stm32/gpio.h>
#include <libopencm3/stm32/rcc.h>
#include <libopencm3/stm32/spi.h>

#include "ignition.h"
#include "ignition_pins.h"
#include "ignition_radio.h"
#include "rfm95w.h"
#include "adc.h"
#include "hmac.h"
#include "key.h"

/*********** Internal functions ******************/
/* Convert raw ADC value to continuity ohms */
uint8_t adc_to_ohms(uint16_t raw);

/************* Exported functions *****************/
/* Setup the SPI peripheral and call the RGM95W initialization procedure.
 * Also initialise all the state variables to sensible defaults */
void ignition_radio_init(ignition_radio_state *radio_state)
{
    /* Clock SPI1 peripheral and setup GPIOs appropriately:
     * NSS, SCK, MOSI, RESET are outputs,
     * MISO is input.
     * SPI setup is done in rfm95w.c */
    rcc_periph_clock_enable(RCC_SPI1);

    /* Make sure NSS doesn't blip when we enable it: */
    gpio_set(RFM_NSS_PORT, RFM_NSS);
    gpio_mode_setup(RFM_NSS_PORT, GPIO_MODE_OUTPUT, GPIO_PUPD_NONE, RFM_NSS);
    gpio_mode_setup(RFM_SCK_PORT, GPIO_MODE_AF, GPIO_PUPD_NONE, RFM_SCK);
    gpio_mode_setup(RFM_MOSI_PORT, GPIO_MODE_AF, GPIO_PUPD_NONE, RFM_MOSI);
    gpio_mode_setup(RFM_MISO_PORT, GPIO_MODE_AF, GPIO_PUPD_NONE, RFM_MISO);
    gpio_set_af(RFM_SCK_PORT, GPIO_AF0, RFM_SCK);
    gpio_set_af(RFM_MOSI_PORT, GPIO_AF0, RFM_MOSI);
    gpio_set_af(RFM_MISO_PORT, GPIO_AF0, RFM_MISO);

    /* Run RFM95W initialization */
    rfm_initialise(SPI1, RFM_NSS_PORT, RFM_NSS);

    radio_state->valid_rx = false;
    radio_state->lost_link = true;
}

/* Initiate packet reception and block until a packet is received */
void ignition_radio_receive_blocking(ignition_radio_state *radio_state)
{
    uint8_t rx_buf[11];

    rfm_receive(rx_buf, 11);
    ignition_radio_parse_packet(radio_state, rx_buf, 11);
    radio_state->packet_rssi = rfm_getrssi();
}

/* Retrieve and parse a packet received in async receive */
void ignition_radio_receive_async(ignition_radio_state *radio_state)
{
    uint8_t rx_buf[11];

    if(rfm_packet_retrieve(rx_buf, 11))
        ignition_radio_parse_packet(radio_state, rx_buf, 11);
    else
        radio_state->valid_rx = false;
    radio_state->packet_rssi = rfm_getrssi();
}

/* Parse a received radio packet and fill in the received packet datastore */
void ignition_radio_parse_packet(ignition_radio_state *radio_state,
                                 uint8_t *buf, uint8_t len)
{
    uint8_t hmac[10];

    if(len != 11)
    {
        /* Invalid packet length! */
        radio_state->valid_rx = false;
        return;
    }

    radio_state->command = buf[0];

    hmac_md5_80(buf, 1, key, key_len, hmac);

    if(memcmp(hmac, buf + 1, 10) == 0)
        radio_state->valid_rx = true; /* Good HMAC */
    else
        radio_state->valid_rx = false; /* Invalid HMAC */

}
