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
