EESchema Schematic File Version 2
LIBS:ignition-rescue
LIBS:power
LIBS:device
LIBS:transistors
LIBS:conn
LIBS:linear
LIBS:regul
LIBS:74xx
LIBS:cmos4000
LIBS:adc-dac
LIBS:memory
LIBS:xilinx
LIBS:microcontrollers
LIBS:dsp
LIBS:microchip
LIBS:analog_switches
LIBS:motorola
LIBS:texas
LIBS:intel
LIBS:audio
LIBS:interface
LIBS:digital-audio
LIBS:philips
LIBS:display
LIBS:cypress
LIBS:siliconi
LIBS:opto
LIBS:atmel
LIBS:contrib
LIBS:valves
LIBS:rfm95w
LIBS:swd
LIBS:relay_spst
LIBS:stm32f071cbt6
LIBS:tvs_small
LIBS:testpoint
LIBS:sma
LIBS:led_r
LIBS:lipo_3s
LIBS:spst_small
LIBS:part
LIBS:push_illum
LIBS:push_illum_straight
LIBS:ignition-cache
EELAYER 25 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 2
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L PWR_FLAG #FLG01
U 1 1 5562BA6E
P 6425 850
F 0 "#FLG01" H 6425 945 50  0001 C CNN
F 1 "PWR_FLAG" H 6425 1030 50  0000 C CNN
F 2 "" H 6425 850 60  0000 C CNN
F 3 "" H 6425 850 60  0000 C CNN
	1    6425 850 
	1    0    0    -1  
$EndComp
$Comp
L PWR_FLAG #FLG02
U 1 1 5562BA86
P 6425 1050
F 0 "#FLG02" H 6425 1145 50  0001 C CNN
F 1 "PWR_FLAG" H 6425 1230 50  0000 C CNN
F 2 "" H 6425 1050 60  0000 C CNN
F 3 "" H 6425 1050 60  0000 C CNN
	1    6425 1050
	-1   0    0    1   
$EndComp
$Comp
L R R101
U 1 1 5562BB53
P 7150 1100
F 0 "R101" V 7230 1100 50  0000 C CNN
F 1 "10k" V 7150 1100 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 7080 1100 30  0001 C CNN
F 3 "" H 7150 1100 30  0000 C CNN
F 4 "9237755" H 7150 1100 60  0001 C CNN "Farnell"
	1    7150 1100
	1    0    0    -1  
$EndComp
$Comp
L R R102
U 1 1 5562BB96
P 7150 1500
F 0 "R102" V 7230 1500 50  0000 C CNN
F 1 "3k3" V 7150 1500 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 7080 1500 30  0001 C CNN
F 3 "" H 7150 1500 30  0000 C CNN
F 4 "9237682" H 7150 1500 60  0001 C CNN "Farnell"
	1    7150 1500
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR03
U 1 1 5562BBD7
P 7150 1700
F 0 "#PWR03" H 7150 1450 50  0001 C CNN
F 1 "GND" H 7150 1550 50  0000 C CNN
F 2 "" H 7150 1700 60  0000 C CNN
F 3 "" H 7150 1700 60  0000 C CNN
	1    7150 1700
	1    0    0    -1  
$EndComp
$Comp
L APE8865N-33-HF-3 U107
U 1 1 5562BCEC
P 6000 6450
F 0 "U107" H 5700 6700 40  0000 C CNN
F 1 "TSR 0.5-2433" H 6000 6650 40  0000 C CNN
F 2 "SOT-23" H 6000 6550 35  0001 C CIN
F 3 "" H 6000 6450 60  0000 C CNN
F 4 "2451539" H 6000 6450 60  0001 C CNN "Farnell"
	1    6000 6450
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR04
U 1 1 5562BDA6
P 6000 6800
F 0 "#PWR04" H 6000 6550 50  0001 C CNN
F 1 "GND" H 6000 6650 50  0000 C CNN
F 2 "" H 6000 6800 60  0000 C CNN
F 3 "" H 6000 6800 60  0000 C CNN
	1    6000 6800
	1    0    0    -1  
$EndComp
$Comp
L +3V3 #PWR05
U 1 1 5562C50D
P 6650 6250
F 0 "#PWR05" H 6650 6100 50  0001 C CNN
F 1 "+3V3" H 6650 6390 50  0000 C CNN
F 2 "" H 6650 6250 60  0000 C CNN
F 3 "" H 6650 6250 60  0000 C CNN
	1    6650 6250
	1    0    0    -1  
$EndComp
$Comp
L SWD U102
U 1 1 5562CA87
P 9950 1350
F 0 "U102" V 9950 1350 60  0000 C CNN
F 1 "SWD" V 9850 1350 60  0000 C CNN
F 2 "common:FTSH-105-01-F-D-K" H 9850 1650 60  0001 C CNN
F 3 "" H 9850 1650 60  0001 C CNN
	1    9950 1350
	1    0    0    -1  
$EndComp
$Comp
L +3V3 #PWR06
U 1 1 5562CAD0
P 9250 1100
F 0 "#PWR06" H 9250 950 50  0001 C CNN
F 1 "+3V3" H 9250 1240 50  0000 C CNN
F 2 "" H 9250 1100 60  0000 C CNN
F 3 "" H 9250 1100 60  0000 C CNN
	1    9250 1100
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR07
U 1 1 5562CB22
P 9250 1600
F 0 "#PWR07" H 9250 1350 50  0001 C CNN
F 1 "GND" H 9250 1450 50  0000 C CNN
F 2 "" H 9250 1600 60  0000 C CNN
F 3 "" H 9250 1600 60  0000 C CNN
	1    9250 1600
	1    0    0    -1  
$EndComp
NoConn ~ 10550 1350
NoConn ~ 10550 1450
$Comp
L +3V3 #PWR08
U 1 1 5562D38F
P 8600 4875
F 0 "#PWR08" H 8600 4725 50  0001 C CNN
F 1 "+3V3" H 8600 5015 50  0000 C CNN
F 2 "" H 8600 4875 60  0000 C CNN
F 3 "" H 8600 4875 60  0000 C CNN
	1    8600 4875
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR09
U 1 1 5562D3AE
P 8300 5350
F 0 "#PWR09" H 8300 5100 50  0001 C CNN
F 1 "GND" H 8300 5200 50  0000 C CNN
F 2 "" H 8300 5350 60  0000 C CNN
F 3 "" H 8300 5350 60  0000 C CNN
	1    8300 5350
	1    0    0    -1  
$EndComp
$Comp
L C C109
U 1 1 5562D48A
P 8600 5125
F 0 "C109" H 8625 5200 50  0000 L CNN
F 1 "100n" H 8625 5050 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 8638 4975 30  0001 C CNN
F 3 "" H 8600 5125 60  0000 C CNN
F 4 "2407344" H 8600 5125 60  0001 C CNN "Farnell"
	1    8600 5125
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR010
U 1 1 5562DF61
P 10275 6300
F 0 "#PWR010" H 10275 6050 50  0001 C CNN
F 1 "GND" H 10275 6150 50  0000 C CNN
F 2 "" H 10275 6300 60  0000 C CNN
F 3 "" H 10275 6300 60  0000 C CNN
	1    10275 6300
	1    0    0    -1  
$EndComp
$Comp
L +3V3 #PWR011
U 1 1 5562E586
P 1450 3950
F 0 "#PWR011" H 1450 3800 50  0001 C CNN
F 1 "+3V3" H 1450 4090 50  0000 C CNN
F 2 "" H 1450 3950 60  0000 C CNN
F 3 "" H 1450 3950 60  0000 C CNN
	1    1450 3950
	1    0    0    -1  
$EndComp
$Comp
L C C105
U 1 1 5562E61B
P 1250 4700
F 0 "C105" V 1300 4750 50  0000 L CNN
F 1 "100n" V 1300 4450 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 1288 4550 30  0001 C CNN
F 3 "" H 1250 4700 60  0000 C CNN
F 4 "2407344" H 1250 4700 60  0001 C CNN "Farnell"
	1    1250 4700
	0    1    1    0   
$EndComp
$Comp
L C C107
U 1 1 5562E674
P 1250 5100
F 0 "C107" V 1300 5150 50  0000 L CNN
F 1 "10n" V 1300 4875 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 1288 4950 30  0001 C CNN
F 3 "" H 1250 5100 60  0000 C CNN
F 4 "2407341" H 1250 5100 60  0001 C CNN "Farnell"
	1    1250 5100
	0    1    1    0   
$EndComp
$Comp
L GND #PWR012
U 1 1 5562F2E9
P 1500 5500
F 0 "#PWR012" H 1500 5250 50  0001 C CNN
F 1 "GND" H 1500 5350 50  0000 C CNN
F 2 "" H 1500 5500 60  0000 C CNN
F 3 "" H 1500 5500 60  0000 C CNN
	1    1500 5500
	1    0    0    -1  
$EndComp
$Comp
L C C104
U 1 1 5562F811
P 4000 4600
F 0 "C104" H 3900 4700 50  0000 L CNN
F 1 "100n" H 3800 4500 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 4038 4450 30  0001 C CNN
F 3 "" H 4000 4600 60  0000 C CNN
F 4 "2407344" H 4000 4600 60  0001 C CNN "Farnell"
	1    4000 4600
	-1   0    0    -1  
$EndComp
$Comp
L GND #PWR013
U 1 1 5562F9AC
P 3950 4800
F 0 "#PWR013" H 3950 4550 50  0001 C CNN
F 1 "GND" H 3950 4650 50  0000 C CNN
F 2 "" H 3950 4800 60  0000 C CNN
F 3 "" H 3950 4800 60  0000 C CNN
	1    3950 4800
	-1   0    0    -1  
$EndComp
$Comp
L Crystal Y101
U 1 1 55630CBB
P 4350 5350
F 0 "Y101" H 4350 5500 50  0000 C CNN
F 1 "8MHz" H 4350 5200 50  0000 C CNN
F 2 "common:XTAL50x32" H 4350 5350 60  0001 C CNN
F 3 "" H 4350 5350 60  0000 C CNN
F 4 "2101329" H 4350 5350 60  0001 C CNN "Farnell"
	1    4350 5350
	0    -1   1    0   
$EndComp
$Comp
L R R105
U 1 1 55630D1D
P 3900 5400
F 0 "R105" V 3800 5250 50  0000 C CNN
F 1 "100R" V 3800 5500 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 3830 5400 30  0001 C CNN
F 3 "" H 3900 5400 30  0000 C CNN
F 4 "2331783" H 3900 5400 60  0001 C CNN "Farnell"
	1    3900 5400
	0    1    -1   0   
$EndComp
$Comp
L C C111
U 1 1 55631037
P 4650 5700
F 0 "C111" H 4675 5800 50  0000 L CNN
F 1 "15p" H 4675 5600 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 4688 5550 30  0001 C CNN
F 3 "" H 4650 5700 60  0000 C CNN
F 4 "1414668" H 4650 5700 60  0001 C CNN "Farnell"
	1    4650 5700
	-1   0    0    -1  
$EndComp
$Comp
L C C112
U 1 1 5563107B
P 4850 5700
F 0 "C112" H 4875 5800 50  0000 L CNN
F 1 "15p" H 4700 5600 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 4888 5550 30  0001 C CNN
F 3 "" H 4850 5700 60  0000 C CNN
F 4 "1414668" H 4850 5700 60  0001 C CNN "Farnell"
	1    4850 5700
	-1   0    0    -1  
$EndComp
$Comp
L GND #PWR014
U 1 1 5563121C
P 4750 5950
F 0 "#PWR014" H 4750 5700 50  0001 C CNN
F 1 "GND" H 4750 5800 50  0000 C CNN
F 2 "" H 4750 5950 60  0000 C CNN
F 3 "" H 4750 5950 60  0000 C CNN
	1    4750 5950
	-1   0    0    -1  
$EndComp
$Comp
L RFM95W U105
U 1 1 5562D355
P 9400 5550
F 0 "U105" V 9450 5550 60  0000 C CNN
F 1 "RFM95W" V 9350 5550 60  0000 C CNN
F 2 "common:RFM95W" H 9400 5600 60  0001 C CNN
F 3 "" H 9400 5600 60  0000 C CNN
	1    9400 5550
	1    0    0    -1  
$EndComp
$Comp
L STM32F071CBT6 U106
U 1 1 559C32F0
P 2650 6150
F 0 "U106" V 2700 6300 60  0000 C CNN
F 1 "STM32F071CBT6" V 2600 6300 60  0000 C CNN
F 2 "Housings_QFP:LQFP-48_7x7mm_Pitch0.5mm" H 2650 6350 60  0001 C CNN
F 3 "" H 2650 6350 60  0000 C CNN
F 4 "2432093" H 2650 6150 60  0001 C CNN "Farnell"
	1    2650 6150
	1    0    0    -1  
$EndComp
Text Notes 7300 1550 0    60   ~ 0
Bat: 3*4.2V\nMon: 3.1V
$Comp
L +3V3 #PWR015
U 1 1 559CAD6D
P 1900 4500
F 0 "#PWR015" H 1900 4350 50  0001 C CNN
F 1 "+3V3" H 1900 4640 50  0000 C CNN
F 2 "" H 1900 4500 60  0000 C CNN
F 3 "" H 1900 4500 60  0000 C CNN
	1    1900 4500
	1    0    0    -1  
$EndComp
$Comp
L C C102
U 1 1 559CBAB1
P 1250 4300
F 0 "C102" V 1300 4350 50  0000 L CNN
F 1 "100n" V 1300 4050 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 1288 4150 30  0001 C CNN
F 3 "" H 1250 4300 60  0000 C CNN
F 4 "2407344" H 1250 4300 60  0001 C CNN "Farnell"
	1    1250 4300
	0    1    1    0   
$EndComp
$Comp
L C C101
U 1 1 559CBB36
P 1250 4100
F 0 "C101" V 1300 4150 50  0000 L CNN
F 1 "100n" V 1300 3850 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 1288 3950 30  0001 C CNN
F 3 "" H 1250 4100 60  0000 C CNN
F 4 "2407344" H 1250 4100 60  0001 C CNN "Farnell"
	1    1250 4100
	0    1    1    0   
$EndComp
$Comp
L C C110
U 1 1 559CBA26
P 1250 5300
F 0 "C110" V 1300 5350 50  0000 L CNN
F 1 "1u" V 1300 5075 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 1288 5150 30  0001 C CNN
F 3 "" H 1250 5300 60  0000 C CNN
F 4 "9227792" H 1250 5300 60  0001 C CNN "Farnell"
	1    1250 5300
	0    1    1    0   
$EndComp
$Comp
L C C106
U 1 1 5562E645
P 1250 4900
F 0 "C106" V 1300 4950 50  0000 L CNN
F 1 "4u7" V 1300 4675 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 1288 4750 30  0001 C CNN
F 3 "" H 1250 4900 60  0000 C CNN
F 4 "1845742" H 1250 4900 60  0001 C CNN "Farnell"
	1    1250 4900
	0    1    1    0   
$EndComp
$Comp
L C C103
U 1 1 5562E5EC
P 1250 4500
F 0 "C103" V 1300 4550 50  0000 L CNN
F 1 "4u7" V 1300 4275 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 1288 4350 30  0001 C CNN
F 3 "" H 1250 4500 60  0000 C CNN
F 4 "1845742" H 1250 4500 60  0001 C CNN "Farnell"
	1    1250 4500
	0    1    1    0   
$EndComp
$Comp
L C C108
U 1 1 559D45ED
P 8375 5125
F 0 "C108" H 8250 5200 50  0000 L CNN
F 1 "100u" H 8175 5050 50  0000 L CNN
F 2 "Capacitors_SMD:C_1206_HandSoldering" H 8413 4975 30  0001 C CNN
F 3 "" H 8375 5125 60  0000 C CNN
F 4 "2362115" H 8375 5125 60  0001 C CNN "Farnell"
	1    8375 5125
	1    0    0    -1  
$EndComp
$Comp
L TVS_small D101
U 1 1 559D6477
P 4600 1050
F 0 "D101" V 4800 1050 50  0000 C CNN
F 1 "TVS_small" H 4600 975 50  0001 C CNN
F 2 "Resistors_SMD:R_0603_HandSoldering" H 4600 1050 60  0001 C CNN
F 3 "" H 4600 1050 60  0000 C CNN
F 4 "2368174" H 4600 1050 60  0001 C CNN "Farnell"
	1    4600 1050
	0    1    1    0   
$EndComp
$Comp
L TVS_small D103
U 1 1 559D90C7
P 10075 6175
F 0 "D103" H 10075 6250 50  0000 C CNN
F 1 "TVS_small" H 10075 6100 50  0001 C CNN
F 2 "Resistors_SMD:R_0603_HandSoldering" H 10075 6175 60  0001 C CNN
F 3 "" H 10075 6175 60  0000 C CNN
F 4 "2368174" H 10075 6175 60  0001 C CNN "Farnell"
	1    10075 6175
	0    1    1    0   
$EndComp
$Sheet
S 9750 2750 950  1200
U 559E15BF
F0 "firing" 60
F1 "firing.sch" 60
F2 "CONT_CH1" O L 9750 3300 60 
F3 "CONT_CH2" O L 9750 3400 60 
F4 "CONT_CH3" O L 9750 3500 60 
F5 "CONT_CH4" O L 9750 3600 60 
F6 "FIRE_CH4" I L 9750 3150 60 
F7 "FIRE_CH3" I L 9750 3050 60 
F8 "FIRE_CH2" I L 9750 2950 60 
F9 "FIRE_CH1" I L 9750 2850 60 
F10 "UPSTREAM_RELAY" I L 9750 3750 60 
F11 "RELAY_SENSE" O L 9750 3850 60 
$EndSheet
Text Label 9700 2850 2    60   ~ 0
FIRE_CH1
Text Label 9700 2950 2    60   ~ 0
FIRE_CH2
Text Label 9700 3050 2    60   ~ 0
FIRE_CH3
Text Label 9700 3150 2    60   ~ 0
FIRE_CH4
Text Label 9700 3300 2    60   ~ 0
CONT_CH1
Text Label 9700 3400 2    60   ~ 0
CONT_CH2
Text Label 9700 3500 2    60   ~ 0
CONT_CH3
Text Label 9700 3600 2    60   ~ 0
CONT_CH4
Text Label 9700 3750 2    60   ~ 0
UPSTREAM_RELAY
Text Label 9700 3850 2    60   ~ 0
RELAY_SENSE
Text Label 5900 3150 0    60   ~ 0
FIRE_CH1
Text Label 5900 3250 0    60   ~ 0
FIRE_CH2
Text Label 5900 3350 0    60   ~ 0
FIRE_CH3
Text Label 5900 3450 0    60   ~ 0
FIRE_CH4
Text Label 5900 3650 0    60   ~ 0
CONT_CH1
Text Label 5900 3750 0    60   ~ 0
CONT_CH2
Text Label 5900 3850 0    60   ~ 0
CONT_CH3
Text Label 5900 3950 0    60   ~ 0
CONT_CH4
Text Label 5900 4150 0    60   ~ 0
UPSTREAM_RELAY
Text Label 5900 4250 0    60   ~ 0
RELAY_SENSE
Text Label 3350 5600 0    60   ~ 0
SWDIO
Text Label 3350 5700 0    60   ~ 0
SWDCLK
Text Label 10600 1150 0    60   ~ 0
SWDIO
Text Label 10600 1250 0    60   ~ 0
SWDCLK
Text Label 10600 1550 0    60   ~ 0
nRST
Text Label 7250 1300 0    60   ~ 0
BATT_MON
Text Label 5900 2950 0    60   ~ 0
BATT_MON
Text Label 10050 5150 0    60   ~ 0
RFM_DIO0
Text Label 10050 5250 0    60   ~ 0
RFM_DIO1
Text Label 10050 5350 0    60   ~ 0
RFM_DIO2
Text Label 10050 5450 0    60   ~ 0
RFM_DIO3
Text Label 10050 5550 0    60   ~ 0
RFM_DIO4
Text Label 10050 5650 0    60   ~ 0
RFM_DIO5
Text Label 8750 5650 2    60   ~ 0
RFM_MISO
Text Label 8750 5750 2    60   ~ 0
RFM_MOSI
Text Label 8750 5850 2    60   ~ 0
RFM_SCK
Text Label 8750 5950 2    60   ~ 0
RFM_NSS
Text Label 8750 6050 2    60   ~ 0
RFM_RESET
Text Label 5900 4450 0    60   ~ 0
RFM_DIO0
Text Label 5900 4550 0    60   ~ 0
RFM_DIO1
Text Label 5900 4650 0    60   ~ 0
RFM_DIO2
Text Label 5900 4750 0    60   ~ 0
RFM_DIO3
Text Label 5900 4850 0    60   ~ 0
RFM_DIO4
Text Label 5900 4950 0    60   ~ 0
RFM_DIO5
Text Label 5900 5150 0    60   ~ 0
RFM_NSS
Text Label 5900 5250 0    60   ~ 0
RFM_SCK
Text Label 5900 5350 0    60   ~ 0
RFM_MISO
Text Label 5900 5450 0    60   ~ 0
RFM_MOSI
Text Label 5900 5550 0    60   ~ 0
RFM_RESET
Text Label 4200 4450 0    60   ~ 0
nRST
$Comp
L TestPoint TP101
U 1 1 559DC01E
P 6700 900
F 0 "TP101" H 6700 825 60  0000 C CNN
F 1 "BATT" H 6700 750 60  0000 C CNN
F 2 "common:TESTPOINT" H 6700 900 60  0001 C CNN
F 3 "" H 6700 900 60  0000 C CNN
	1    6700 900 
	-1   0    0    1   
$EndComp
$Comp
L TestPoint TP102
U 1 1 559DC0E8
P 6700 1000
F 0 "TP102" H 6700 925 60  0000 C CNN
F 1 "GND" H 6700 850 60  0000 C CNN
F 2 "common:TESTPOINT" H 6700 1000 60  0001 C CNN
F 3 "" H 6700 1000 60  0000 C CNN
	1    6700 1000
	1    0    0    -1  
$EndComp
$Comp
L TestPoint TP104
U 1 1 559DC42B
P 6550 6400
F 0 "TP104" H 6550 6500 60  0000 C CNN
F 1 "3v3" H 6550 6600 60  0000 C CNN
F 2 "common:TESTPOINT" H 6550 6400 60  0001 C CNN
F 3 "" H 6550 6400 60  0000 C CNN
	1    6550 6400
	-1   0    0    1   
$EndComp
$Comp
L TestPoint TP103
U 1 1 559DC7C1
P 3850 4450
F 0 "TP103" H 3850 4375 60  0000 C CNN
F 1 "nRST" H 3825 4275 60  0000 C CNN
F 2 "common:TESTPOINT" H 3850 4450 60  0001 C CNN
F 3 "" H 3850 4450 60  0000 C CNN
	1    3850 4450
	-1   0    0    1   
$EndComp
$Comp
L SMA P106
U 1 1 559E7125
P 10525 6050
F 0 "P106" H 10625 5950 60  0000 C CNN
F 1 "SMA" H 10625 6150 60  0000 C CNN
F 2 "common:SMA-142-0701-801" H 10525 6050 60  0001 C CNN
F 3 "" H 10525 6050 60  0000 C CNN
F 4 "1608592" H 10525 6050 60  0001 C CNN "Farnell"
	1    10525 6050
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR016
U 1 1 559E749A
P 10375 5850
F 0 "#PWR016" H 10375 5600 50  0001 C CNN
F 1 "GND" H 10375 5700 50  0001 C CNN
F 2 "" H 10375 5850 60  0000 C CNN
F 3 "" H 10375 5850 60  0000 C CNN
	1    10375 5850
	-1   0    0    1   
$EndComp
$Comp
L CONN_01X04 P103
U 1 1 55A17348
P 3000 1550
F 0 "P103" V 3000 1850 50  0000 C CNN
F 1 "CHARGE_CASE" V 3000 1050 50  0000 C CNN
F 2 "" H 3000 1550 60  0000 C CNN
F 3 "" H 3000 1550 60  0000 C CNN
	1    3000 1550
	0    -1   1    0   
$EndComp
$Comp
L FUSE F101
U 1 1 55A19884
P 4950 1000
F 0 "F101" H 4925 1125 50  0000 C CNN
F 1 "6.3x32" H 4825 950 50  0000 C CNN
F 2 "ignition:6x32_fuseclip" H 4950 1000 60  0001 C CNN
F 3 "" H 4950 1000 60  0000 C CNN
F 4 "1603990" H 4950 1000 60  0001 C CNN "Farnell"
	1    4950 1000
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR017
U 1 1 55A1A3D6
P 6950 1050
F 0 "#PWR017" H 6950 800 50  0001 C CNN
F 1 "GND" H 6950 900 50  0000 C CNN
F 2 "" H 6950 1050 60  0000 C CNN
F 3 "" H 6950 1050 60  0000 C CNN
	1    6950 1050
	1    0    0    -1  
$EndComp
$Comp
L +BATT #PWR018
U 1 1 55A1ACA2
P 6950 850
F 0 "#PWR018" H 6950 700 50  0001 C CNN
F 1 "+BATT" H 6950 990 50  0000 C CNN
F 2 "" H 6950 850 60  0000 C CNN
F 3 "" H 6950 850 60  0000 C CNN
	1    6950 850 
	1    0    0    -1  
$EndComp
$Comp
L CONN_01X04 P104
U 1 1 55A1E122
P 3000 1700
F 0 "P104" V 3000 2000 50  0000 C CNN
F 1 "CHARGE_CASE" V 3000 1200 50  0000 C CNN
F 2 "" H 3000 1700 60  0000 C CNN
F 3 "" H 3000 1700 60  0000 C CNN
	1    3000 1700
	0    -1   -1   0   
$EndComp
$Comp
L CONN_01X04 P105
U 1 1 55A1E609
P 3000 2150
F 0 "P105" V 3000 2450 50  0000 C CNN
F 1 "CHARGE_JST_XH" V 3000 1600 50  0000 C CNN
F 2 "" H 3000 2150 60  0000 C CNN
F 3 "" H 3000 2150 60  0000 C CNN
	1    3000 2150
	0    -1   1    0   
$EndComp
$Comp
L +BATT #PWR019
U 1 1 55A85387
P 5450 6300
F 0 "#PWR019" H 5450 6150 50  0001 C CNN
F 1 "+BATT" H 5450 6440 50  0000 C CNN
F 2 "" H 5450 6300 60  0000 C CNN
F 3 "" H 5450 6300 60  0000 C CNN
	1    5450 6300
	1    0    0    -1  
$EndComp
$Comp
L RELAY_SPST K101
U 1 1 55A865A8
P 5650 750
F 0 "K101" H 5550 1075 50  0000 C CNN
F 1 "16A SPST" H 5650 600 50  0000 C CNN
F 2 "" H 5650 750 60  0000 C CNN
F 3 "" H 5650 750 60  0000 C CNN
F 4 "1629044" H 5650 750 60  0001 C CNN "Farnell"
	1    5650 750 
	1    0    0    1   
$EndComp
$Comp
L CONN_01X03 P102
U 1 1 55A940BC
P 4300 1000
F 0 "P102" H 4300 1200 50  0000 C CNN
F 1 "BATT" H 4250 800 50  0000 C CNN
F 2 "" H 4300 1000 60  0000 C CNN
F 3 "" H 4300 1000 60  0000 C CNN
	1    4300 1000
	-1   0    0    1   
$EndComp
$Comp
L CONN_01X03 P101
U 1 1 55A942D9
P 4150 1000
F 0 "P101" H 4150 1200 50  0000 C CNN
F 1 "BATT" H 4100 800 50  0000 C CNN
F 2 "" H 4150 1000 60  0000 C CNN
F 3 "" H 4150 1000 60  0000 C CNN
	1    4150 1000
	1    0    0    1   
$EndComp
$Comp
L GND #PWR020
U 1 1 55A9677A
P 5250 850
F 0 "#PWR020" H 5250 600 50  0001 C CNN
F 1 "GND" H 5150 850 50  0001 C CNN
F 2 "" H 5250 850 60  0000 C CNN
F 3 "" H 5250 850 60  0000 C CNN
	1    5250 850 
	1    0    0    -1  
$EndComp
$Comp
L SPST_small SW101
U 1 1 55A8C95B
P 3650 900
F 0 "SW101" H 3650 1000 50  0000 C CNN
F 1 "LATCHING PUSH" H 3650 1100 50  0000 C CNN
F 2 "" H 3650 900 60  0000 C CNN
F 3 "" H 3650 900 60  0000 C CNN
F 4 "SW02719" H 3650 900 60  0001 C CNN "Farnell"
	1    3650 900 
	1    0    0    -1  
$EndComp
$Comp
L Led_x2 D102
U 1 1 55A8E3E4
P 3500 2800
F 0 "D102" H 3500 3025 50  0000 C CNN
F 1 "RED/GREEN" H 3500 2550 50  0000 C CNN
F 2 "" H 3500 2800 60  0000 C CNN
F 3 "" H 3500 2800 60  0000 C CNN
F 4 "2449761" H 3500 2800 60  0001 C CNN "Farnell"
	1    3500 2800
	-1   0    0    1   
$EndComp
$Comp
L GND #PWR021
U 1 1 55A8E7EB
P 3850 2850
F 0 "#PWR021" H 3850 2600 50  0001 C CNN
F 1 "GND" H 3850 2700 50  0000 C CNN
F 2 "" H 3850 2850 60  0000 C CNN
F 3 "" H 3850 2850 60  0000 C CNN
	1    3850 2850
	1    0    0    -1  
$EndComp
$Comp
L R R103
U 1 1 55A8E9D5
P 3050 2700
F 0 "R103" V 3130 2700 50  0000 C CNN
F 1 "68R" V 3050 2700 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 2980 2700 30  0001 C CNN
F 3 "" H 3050 2700 30  0000 C CNN
F 4 "2332055" H 3050 2700 60  0001 C CNN "Farnell"
	1    3050 2700
	0    1    1    0   
$EndComp
$Comp
L R R104
U 1 1 55A8EA84
P 3050 2900
F 0 "R104" V 3130 2900 50  0000 C CNN
F 1 "68R" V 3050 2900 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 2980 2900 30  0001 C CNN
F 3 "" H 3050 2900 30  0000 C CNN
F 4 "2332055" H 3050 2900 60  0001 C CNN "Farnell"
	1    3050 2900
	0    1    1    0   
$EndComp
Text Label 2900 2700 2    60   ~ 0
ARM_LED
Text Label 2900 2900 2    60   ~ 0
DISARM_LED
Text Label 5900 2850 0    60   ~ 0
ARM_LED
Text Label 5900 2750 0    60   ~ 0
DISARM_LED
$Comp
L PART U104
U 1 1 55A94007
P 3500 3300
F 0 "U104" H 3550 3300 60  0000 C CNN
F 1 "BEZEL" H 3600 3400 60  0000 C CNN
F 2 "" H 3500 3300 60  0000 C CNN
F 3 "" H 3500 3300 60  0000 C CNN
F 4 "SC11628" H 3500 3300 60  0001 C CNN "Farnell"
	1    3500 3300
	1    0    0    -1  
$EndComp
Text Notes 2450 2600 0    60   ~ 0
RED: 1.95V 20mA
Text Notes 2350 3100 0    60   ~ 0
GREEN: 2.1V 20mA
$Comp
L LiPO_3S BT101
U 1 1 55A171E6
P 2600 900
F 0 "BT101" H 2320 735 50  0000 L CNN
F 1 "LiPO_3S" H 2200 650 50  0000 L CNN
F 2 "" V 2600 940 60  0000 C CNN
F 3 "" V 2600 940 60  0000 C CNN
F 4 "http://hobbyking.com/hobbyking/store/uh_viewItem.asp?idProduct=21380" H 2600 900 60  0001 C CNN "URL"
	1    2600 900 
	1    0    0    -1  
$EndComp
$Comp
L PART U101
U 1 1 55AA60A6
P 4825 1325
F 0 "U101" H 4875 1325 60  0000 C CNN
F 1 "FUSE 15A" H 5000 1425 60  0000 C CNN
F 2 "" H 4825 1325 60  0000 C CNN
F 3 "" H 4825 1325 60  0000 C CNN
F 4 "1829561" H 4825 1325 60  0001 C CNN "Farnell"
	1    4825 1325
	1    0    0    -1  
$EndComp
$Comp
L PART U103
U 1 1 55AA6AFA
P 4825 1550
F 0 "U103" H 4875 1550 60  0000 C CNN
F 1 "CLIP" H 4925 1650 60  0000 C CNN
F 2 "" H 4825 1550 60  0000 C CNN
F 3 "" H 4825 1550 60  0000 C CNN
F 4 "1603990" H 4825 1550 60  0001 C CNN "Farnell"
	1    4825 1550
	1    0    0    -1  
$EndComp
Wire Wire Line
	6425 900  6425 850 
Wire Wire Line
	6425 1050 6425 1000
Connection ~ 6425 900 
Connection ~ 6425 1000
Wire Wire Line
	7150 1250 7150 1350
Wire Wire Line
	7150 1700 7150 1650
Wire Wire Line
	7150 1300 7250 1300
Connection ~ 7150 1300
Wire Wire Line
	6000 6800 6000 6750
Wire Wire Line
	9250 1100 9250 1150
Wire Wire Line
	9250 1150 9350 1150
Wire Wire Line
	9250 1550 9350 1550
Wire Wire Line
	9250 1250 9250 1600
Wire Wire Line
	9350 1350 9250 1350
Connection ~ 9250 1550
Wire Wire Line
	9350 1250 9250 1250
Connection ~ 9250 1350
Wire Wire Line
	10550 1150 10600 1150
Wire Wire Line
	10600 1550 10550 1550
Wire Wire Line
	8300 5350 8800 5350
Wire Wire Line
	8700 5350 8700 5450
Wire Wire Line
	8700 5450 8800 5450
Connection ~ 8700 5350
Wire Wire Line
	8750 5350 8750 5250
Wire Wire Line
	8750 5250 8800 5250
Connection ~ 8750 5350
Wire Wire Line
	8600 4875 8600 4975
Wire Wire Line
	8375 4925 8800 4925
Wire Wire Line
	8800 4925 8800 5150
Connection ~ 8600 4925
Wire Wire Line
	8600 5275 8600 5350
Connection ~ 8600 5350
Wire Wire Line
	10275 6150 10425 6150
Wire Wire Line
	10275 6150 10275 6300
Wire Wire Line
	10000 6050 10425 6050
Wire Wire Line
	1450 5100 1400 5100
Wire Wire Line
	1450 3950 1450 5300
Connection ~ 1450 4300
Connection ~ 1450 4500
Connection ~ 1450 4700
Connection ~ 1450 4900
Connection ~ 1450 5100
Wire Wire Line
	1000 5100 1100 5100
Wire Wire Line
	1000 4100 1000 5450
Wire Wire Line
	1000 4900 1100 4900
Connection ~ 1000 5100
Wire Wire Line
	1100 4700 1000 4700
Connection ~ 1000 4900
Wire Wire Line
	1000 4500 1100 4500
Connection ~ 1000 4700
Connection ~ 1000 4500
Wire Wire Line
	1000 5450 1950 5450
Wire Wire Line
	1900 5450 1900 5350
Wire Wire Line
	1900 5350 1950 5350
Connection ~ 1900 5450
Wire Wire Line
	1850 5450 1850 5550
Wire Wire Line
	1850 5550 1950 5550
Connection ~ 1850 5450
Wire Wire Line
	1800 5450 1800 5750
Wire Wire Line
	1800 5750 1950 5750
Connection ~ 1800 5450
Wire Wire Line
	3850 4750 4000 4750
Wire Wire Line
	3950 4750 3950 4800
Connection ~ 3950 4750
Wire Wire Line
	3750 4450 4200 4450
Wire Wire Line
	3750 4450 3750 4700
Wire Wire Line
	3750 4700 3300 4700
Wire Wire Line
	3850 4750 3850 4800
Wire Wire Line
	3850 4800 3300 4800
Wire Wire Line
	3300 5300 4100 5300
Wire Wire Line
	4100 5300 4100 5200
Wire Wire Line
	4100 5200 4850 5200
Wire Wire Line
	4100 5500 4650 5500
Wire Wire Line
	4100 5500 4100 5400
Wire Wire Line
	4650 5500 4650 5550
Connection ~ 4350 5500
Wire Wire Line
	4850 5200 4850 5550
Connection ~ 4350 5200
Wire Wire Line
	4850 5950 4850 5850
Wire Wire Line
	4650 5950 4850 5950
Wire Wire Line
	4650 5950 4650 5850
Connection ~ 4750 5950
Wire Wire Line
	3350 5700 3300 5700
Wire Wire Line
	1500 5450 1500 5500
Wire Wire Line
	6450 6400 6650 6400
Wire Wire Line
	1900 4500 1900 4550
Wire Wire Line
	1900 4550 1950 4550
Connection ~ 1500 5450
Wire Wire Line
	1400 4100 1450 4100
Connection ~ 1450 4100
Wire Wire Line
	1100 5300 1000 5300
Connection ~ 1000 5300
Wire Wire Line
	1100 4300 1000 4300
Wire Wire Line
	1000 4100 1100 4100
Connection ~ 1000 4300
Wire Wire Line
	1450 5300 1400 5300
Wire Wire Line
	1400 4300 1450 4300
Wire Wire Line
	1450 4500 1400 4500
Wire Wire Line
	1400 4700 1450 4700
Wire Wire Line
	1450 4900 1400 4900
Wire Wire Line
	1450 5200 1700 5200
Wire Wire Line
	1700 5200 1700 5150
Wire Wire Line
	1700 5150 1950 5150
Connection ~ 1450 5200
Connection ~ 1450 4800
Wire Wire Line
	1450 4800 1550 4800
Wire Wire Line
	1550 4800 1550 5050
Wire Wire Line
	1550 5050 1950 5050
Wire Wire Line
	1650 4850 1950 4850
Wire Wire Line
	1650 4400 1650 4850
Wire Wire Line
	1750 4750 1950 4750
Wire Wire Line
	1750 4200 1750 4750
Connection ~ 1450 4400
Wire Wire Line
	8375 5275 8375 5350
Wire Wire Line
	8375 4975 8375 4925
Wire Wire Line
	4550 950  4550 1000
Wire Wire Line
	4550 1100 4550 1150
Wire Wire Line
	4550 950  4650 950 
Wire Wire Line
	4600 950  4600 950 
Connection ~ 4600 950 
Wire Wire Line
	4550 1150 6200 1150
Wire Wire Line
	4600 1150 4600 1150
Connection ~ 4600 1150
Wire Wire Line
	10075 6050 10075 6075
Connection ~ 10075 6050
Wire Wire Line
	10075 6275 10275 6275
Connection ~ 10275 6275
Wire Wire Line
	9700 2850 9750 2850
Wire Wire Line
	9700 2950 9750 2950
Wire Wire Line
	9700 3050 9750 3050
Wire Wire Line
	9700 3150 9750 3150
Wire Wire Line
	9700 3850 9750 3850
Wire Wire Line
	9750 3750 9700 3750
Wire Wire Line
	9700 3600 9750 3600
Wire Wire Line
	9750 3500 9700 3500
Wire Wire Line
	9750 3400 9700 3400
Wire Wire Line
	9700 3300 9750 3300
Wire Wire Line
	3350 5600 3300 5600
Wire Wire Line
	10550 1250 10600 1250
Wire Wire Line
	7150 900  7150 950 
Wire Wire Line
	10050 5650 10000 5650
Wire Wire Line
	10000 5550 10050 5550
Wire Wire Line
	10050 5450 10000 5450
Wire Wire Line
	10000 5350 10050 5350
Wire Wire Line
	10050 5250 10000 5250
Wire Wire Line
	10000 5150 10050 5150
Connection ~ 8375 5350
Wire Wire Line
	8800 5650 8750 5650
Wire Wire Line
	8750 5750 8800 5750
Wire Wire Line
	8800 5850 8750 5850
Wire Wire Line
	8750 5950 8800 5950
Wire Wire Line
	8800 6050 8750 6050
Connection ~ 4000 4450
Connection ~ 6700 900 
Connection ~ 6700 1000
Connection ~ 6550 6400
Connection ~ 3850 4450
Wire Wire Line
	3300 5400 3750 5400
Wire Wire Line
	4100 5400 4050 5400
Wire Wire Line
	1650 4400 1450 4400
Wire Wire Line
	1450 4200 1750 4200
Connection ~ 1450 4200
Wire Wire Line
	10350 6150 10350 6200
Wire Wire Line
	10350 6200 10425 6200
Connection ~ 10350 6150
Wire Wire Line
	10375 5850 10375 5950
Wire Wire Line
	10375 5900 10425 5900
Wire Wire Line
	10375 5950 10425 5950
Connection ~ 10375 5900
Wire Wire Line
	2850 1250 2850 1350
Wire Wire Line
	2800 1050 2950 1050
Wire Wire Line
	2950 1050 2950 1350
Wire Wire Line
	2800 950  3050 950 
Wire Wire Line
	3050 950  3050 1350
Wire Wire Line
	3150 750  3150 1350
Wire Wire Line
	3300 750  3300 1000
Wire Wire Line
	3300 1000 3950 1000
Connection ~ 3150 750 
Wire Wire Line
	3300 1250 3300 1100
Connection ~ 2850 1250
Wire Wire Line
	2600 750  3300 750 
Wire Wire Line
	2600 1250 3300 1250
Wire Wire Line
	6950 1000 6950 1050
Wire Wire Line
	6950 900  6950 850 
Connection ~ 6950 900 
Wire Wire Line
	2850 1900 2850 1950
Wire Wire Line
	2950 1950 2950 1900
Wire Wire Line
	3050 1900 3050 1950
Wire Wire Line
	3150 1950 3150 1900
Wire Wire Line
	6050 900  7150 900 
Wire Wire Line
	5450 6300 5450 6400
Wire Wire Line
	5450 6400 5550 6400
Wire Wire Line
	3300 1100 3950 1100
Wire Wire Line
	6200 1000 6950 1000
Wire Wire Line
	4650 1000 4700 1000
Wire Wire Line
	4650 950  4650 1000
Wire Wire Line
	3450 900  3450 1000
Connection ~ 3450 1000
Wire Wire Line
	4500 1100 4550 1100
Wire Wire Line
	4550 1000 4500 1000
Wire Wire Line
	5200 1000 5250 1000
Wire Wire Line
	4500 900  4500 750 
Wire Wire Line
	4500 750  5250 750 
Wire Wire Line
	6200 1150 6200 1000
Wire Wire Line
	3800 900  3950 900 
Wire Wire Line
	3850 2850 3850 2800
Wire Wire Line
	3850 2800 3800 2800
Wire Wire Line
	6650 6400 6650 6250
$EndSCHEMATC
