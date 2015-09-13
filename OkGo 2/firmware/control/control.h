#ifndef CONTROL_H
#define CONTROL_H

/* Constants */

/* Channel statuses */
#define CH_STATUS_OK 0
#define CH_STATUS_CONT_ERR 1
#define CH_STATUS_FIRE 2

/* ADC channels */
#define ADC_BATT 0


/* Structs */
typedef struct
{
	bool armed;
	uint32_t centre_frf;
	uint8_t ch1_status, ch2_status, ch3_status, ch4_status;
} control_state;

#endif