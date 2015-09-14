#ifndef IGNITION_H
#define IGNITION_H

/* Structs */
typedef struct
{
	bool armed;
	bool fire_ch1, fire_ch2, fire_ch3, fire_ch4;
	uint32_t centre_frf;
} ignition_state;

#endif