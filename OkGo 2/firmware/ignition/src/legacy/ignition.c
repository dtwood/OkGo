#include <stdint.h>
#include <stdbool.h>

#include <libopencm3/stm32/gpio.h>
#include <libopencm3/stm32/rcc.h>

#include "rfm95w.h"
#include "utils.h"
#include "adc.h"

#include "ignition.h"
#include "ignition_pins.h"
#include "ignition_radio.h"

const uint8_t RADIO_POWER_DBM = 10; /* Radio tx power in dBm */
const uint32_t PACKET_DROP_DELAY = 5000; /* Drop delay in ms */

/* Internal functions */
void ignition_init(ignition_state *state, ignition_radio_state *radio_state);

/* Do beeping */
void do_beep(ignition_state *state);

void ignition_init(ignition_state *state, ignition_radio_state *radio_state)
{
    /* Initialise local state variables */
    state->armed = false;
    state->fire_ch1 = false;
    state->fire_ch2 = false;
    state->fire_ch3 = false;
    state->fire_ch4 = false;
    state->centre_frf = FRF_868;
    state->beep_start = 0;
    state->beep_volume = 2;

    /* Setup crystal oscillator and systick */
    rcc_clock_setup_in_hsi_out_48mhz();
    systick_init();

    /* Clock GPIOs, set pin modes */
    ignition_pins_init();

    /* Initialise radio and local state variables, read stored config*/
    ignition_radio_init(radio_state);
    rfm_setfreq(state->centre_frf);
    rfm_setpower(RADIO_POWER_DBM);

    /* ADC Setup: Clock periph, run init. Pins done in ignition_pins */
    rcc_periph_clock_enable(RCC_ADC);
    adc_init();
}
