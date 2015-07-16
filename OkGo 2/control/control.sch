EESchema Schematic File Version 2
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
LIBS:swd
LIBS:lcd_hd44780
LIBS:stm32f071cbt6
LIBS:tvs_small
LIBS:testpoint
LIBS:adp3335
LIBS:sma
LIBS:meu1s0303zc
LIBS:part
LIBS:lipo_1s
LIBS:spst_small
LIBS:push_illum
LIBS:push_illum_straight
LIBS:control-cache
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
L GND #PWR104
U 1 1 556214DE
P 7000 1050
F 0 "#PWR104" H 7000 800 50  0001 C CNN
F 1 "GND" H 7000 900 50  0000 C CNN
F 2 "" H 7000 1050 60  0000 C CNN
F 3 "" H 7000 1050 60  0000 C CNN
	1    7000 1050
	1    0    0    -1  
$EndComp
$Comp
L +BATT #PWR103
U 1 1 556214F3
P 7000 850
F 0 "#PWR103" H 7000 700 50  0001 C CNN
F 1 "+BATT" H 7000 990 50  0000 C CNN
F 2 "" H 7000 850 60  0000 C CNN
F 3 "" H 7000 850 60  0000 C CNN
	1    7000 850 
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR108
U 1 1 556215CB
P 9500 1600
F 0 "#PWR108" H 9500 1350 50  0001 C CNN
F 1 "GND" H 9500 1450 50  0000 C CNN
F 2 "" H 9500 1600 60  0000 C CNN
F 3 "" H 9500 1600 60  0000 C CNN
	1    9500 1600
	1    0    0    -1  
$EndComp
$Comp
L C C102
U 1 1 556215E1
P 8500 1300
F 0 "C102" H 8525 1400 50  0000 L CNN
F 1 "2u2" H 8525 1200 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 8538 1150 30  0001 C CNN
F 3 "" H 8500 1300 60  0000 C CNN
F 4 "2362106" H 8500 1300 60  0001 C CNN "Farnell"
	1    8500 1300
	1    0    0    -1  
$EndComp
$Comp
L C C101
U 1 1 55621639
P 10250 1250
F 0 "C101" H 10275 1350 50  0000 L CNN
F 1 "2u2" H 10275 1150 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 10288 1100 30  0001 C CNN
F 3 "" H 10250 1250 60  0000 C CNN
F 4 "2362106" H 10250 1250 60  0001 C CNN "Farnell"
	1    10250 1250
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR107
U 1 1 556216C5
P 8500 1500
F 0 "#PWR107" H 8500 1250 50  0001 C CNN
F 1 "GND" H 8500 1350 50  0000 C CNN
F 2 "" H 8500 1500 60  0000 C CNN
F 3 "" H 8500 1500 60  0000 C CNN
	1    8500 1500
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR106
U 1 1 556216DE
P 10250 1450
F 0 "#PWR106" H 10250 1200 50  0001 C CNN
F 1 "GND" H 10250 1300 50  0000 C CNN
F 2 "" H 10250 1450 60  0000 C CNN
F 3 "" H 10250 1450 60  0000 C CNN
	1    10250 1450
	1    0    0    -1  
$EndComp
$Comp
L PWR_FLAG #FLG101
U 1 1 556217FC
P 5850 850
F 0 "#FLG101" H 5850 945 50  0001 C CNN
F 1 "PWR_FLAG" H 5850 1030 50  0000 C CNN
F 2 "" H 5850 850 60  0000 C CNN
F 3 "" H 5850 850 60  0000 C CNN
	1    5850 850 
	1    0    0    -1  
$EndComp
$Comp
L PWR_FLAG #FLG102
U 1 1 55621881
P 5850 1050
F 0 "#FLG102" H 5850 1145 50  0001 C CNN
F 1 "PWR_FLAG" H 5850 1230 50  0000 C CNN
F 2 "" H 5850 1050 60  0000 C CNN
F 3 "" H 5850 1050 60  0000 C CNN
	1    5850 1050
	-1   0    0    1   
$EndComp
$Comp
L GND #PWR131
U 1 1 55629E65
P 9000 6300
F 0 "#PWR131" H 9000 6050 50  0001 C CNN
F 1 "GND" H 9000 6150 50  0001 C CNN
F 2 "" H 9000 6300 60  0000 C CNN
F 3 "" H 9000 6300 60  0000 C CNN
	1    9000 6300
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR128
U 1 1 5562B214
P 7200 5350
F 0 "#PWR128" H 7200 5100 50  0001 C CNN
F 1 "GND" H 7200 5200 50  0000 C CNN
F 2 "" H 7200 5350 60  0000 C CNN
F 3 "" H 7200 5350 60  0000 C CNN
	1    7200 5350
	1    0    0    -1  
$EndComp
$Comp
L +3V3 #PWR127
U 1 1 5562B35A
P 7250 4900
F 0 "#PWR127" H 7250 4750 50  0001 C CNN
F 1 "+3V3" H 7250 5040 50  0000 C CNN
F 2 "" H 7250 4900 60  0000 C CNN
F 3 "" H 7250 4900 60  0000 C CNN
	1    7250 4900
	1    0    0    -1  
$EndComp
$Comp
L +3V3 #PWR102
U 1 1 55624CAA
P 10850 850
F 0 "#PWR102" H 10850 700 50  0001 C CNN
F 1 "+3V3" H 10850 990 50  0000 C CNN
F 2 "" H 10850 850 60  0000 C CNN
F 3 "" H 10850 850 60  0000 C CNN
	1    10850 850 
	1    0    0    -1  
$EndComp
$Comp
L +3V3 #PWR110
U 1 1 5562C6B7
P 1200 2575
F 0 "#PWR110" H 1200 2425 50  0001 C CNN
F 1 "+3V3" H 1200 2715 50  0000 C CNN
F 2 "" H 1200 2575 60  0000 C CNN
F 3 "" H 1200 2575 60  0000 C CNN
	1    1200 2575
	1    0    0    -1  
$EndComp
$Comp
L C C110
U 1 1 5562D332
P 1050 3925
F 0 "C110" V 1100 3975 50  0000 L CNN
F 1 "1u" V 1100 3775 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 1088 3775 30  0001 C CNN
F 3 "" H 1050 3925 60  0000 C CNN
F 4 "9227792" H 1050 3925 60  0001 C CNN "Farnell"
	1    1050 3925
	0    1    1    0   
$EndComp
$Comp
L C C109
U 1 1 5562D381
P 1050 3725
F 0 "C109" V 1100 3775 50  0000 L CNN
F 1 "10n" V 1100 3525 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 1088 3575 30  0001 C CNN
F 3 "" H 1050 3725 60  0000 C CNN
F 4 "2407341" H 1050 3725 60  0001 C CNN "Farnell"
	1    1050 3725
	0    1    1    0   
$EndComp
$Comp
L C C108
U 1 1 5562D3B3
P 1050 3525
F 0 "C108" V 1100 3575 50  0000 L CNN
F 1 "4u7" V 1100 3325 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 1088 3375 30  0001 C CNN
F 3 "" H 1050 3525 60  0000 C CNN
F 4 "1845742" H 1050 3525 60  0001 C CNN "Farnell"
	1    1050 3525
	0    1    1    0   
$EndComp
$Comp
L GND #PWR120
U 1 1 5562DCAE
P 1200 4275
F 0 "#PWR120" H 1200 4025 50  0001 C CNN
F 1 "GND" H 1200 4125 50  0000 C CNN
F 2 "" H 1200 4275 60  0000 C CNN
F 3 "" H 1200 4275 60  0000 C CNN
	1    1200 4275
	1    0    0    -1  
$EndComp
$Comp
L C C107
U 1 1 5562E486
P 3650 3325
F 0 "C107" H 3675 3425 50  0000 L CNN
F 1 "100n" H 3675 3225 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 3688 3175 30  0001 C CNN
F 3 "" H 3650 3325 60  0000 C CNN
F 4 "2407344" H 3650 3325 60  0001 C CNN "Farnell"
	1    3650 3325
	-1   0    0    -1  
$EndComp
$Comp
L GND #PWR115
U 1 1 5562E73A
P 3600 3525
F 0 "#PWR115" H 3600 3275 50  0001 C CNN
F 1 "GND" H 3600 3375 50  0000 C CNN
F 2 "" H 3600 3525 60  0000 C CNN
F 3 "" H 3600 3525 60  0000 C CNN
	1    3600 3525
	-1   0    0    -1  
$EndComp
$Comp
L Crystal Y101
U 1 1 55631E06
P 4150 4025
F 0 "Y101" H 4150 4175 50  0000 C CNN
F 1 "8MHz" H 4150 3875 50  0000 C CNN
F 2 "" H 4150 4025 60  0000 C CNN
F 3 "" H 4150 4025 60  0000 C CNN
F 4 "2101329" H 4150 4025 60  0001 C CNN "Farnell"
	1    4150 4025
	0    -1   1    0   
$EndComp
$Comp
L R R105
U 1 1 5563219C
P 3700 4075
F 0 "R105" V 3800 4175 50  0000 C CNN
F 1 "100R" V 3800 3925 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 3630 4075 30  0001 C CNN
F 3 "" H 3700 4075 30  0000 C CNN
F 4 "2331783" H 3700 4075 60  0001 C CNN "Farnell"
	1    3700 4075
	0    -1   1    0   
$EndComp
$Comp
L C C111
U 1 1 5563246F
P 4400 4375
F 0 "C111" H 4425 4475 50  0000 L CNN
F 1 "15p" H 4425 4275 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 4438 4225 30  0001 C CNN
F 3 "" H 4400 4375 60  0000 C CNN
F 4 "1414668" H 4400 4375 60  0001 C CNN "Farnell"
	1    4400 4375
	-1   0    0    -1  
$EndComp
$Comp
L C C112
U 1 1 55632519
P 4600 4375
F 0 "C112" H 4625 4475 50  0000 L CNN
F 1 "15p" H 4625 4275 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 4638 4225 30  0001 C CNN
F 3 "" H 4600 4375 60  0000 C CNN
F 4 "1414668" H 4600 4375 60  0001 C CNN "Farnell"
	1    4600 4375
	-1   0    0    -1  
$EndComp
$Comp
L GND #PWR124
U 1 1 5563278B
P 4500 4575
F 0 "#PWR124" H 4500 4325 50  0001 C CNN
F 1 "GND" H 4500 4425 50  0000 C CNN
F 2 "" H 4500 4575 60  0000 C CNN
F 3 "" H 4500 4575 60  0000 C CNN
	1    4500 4575
	-1   0    0    -1  
$EndComp
$Comp
L LED D102
U 1 1 55645DBD
P 10850 1250
F 0 "D102" H 10850 1350 50  0000 C CNN
F 1 "POWER" H 10900 1150 50  0000 C CNN
F 2 "" H 10850 1250 60  0000 C CNN
F 3 "" H 10850 1250 60  0000 C CNN
	1    10850 1250
	0    -1   -1   0   
$EndComp
$Comp
L R R103
U 1 1 55646094
P 10850 1650
F 0 "R103" V 10930 1650 50  0000 C CNN
F 1 "R" V 10850 1650 50  0000 C CNN
F 2 "" V 10780 1650 30  0000 C CNN
F 3 "" H 10850 1650 30  0000 C CNN
	1    10850 1650
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR111
U 1 1 556462D7
P 10850 1850
F 0 "#PWR111" H 10850 1600 50  0001 C CNN
F 1 "GND" H 10850 1700 50  0000 C CNN
F 2 "" H 10850 1850 60  0000 C CNN
F 3 "" H 10850 1850 60  0000 C CNN
	1    10850 1850
	1    0    0    -1  
$EndComp
$Comp
L SWD U102
U 1 1 55647D4E
P 1575 7000
F 0 "U102" V 1575 7000 60  0000 C CNN
F 1 "SWD" V 1475 7000 60  0000 C CNN
F 2 "common:FTSH-105-01-F-D-K" H 1475 7300 60  0001 C CNN
F 3 "" H 1475 7300 60  0000 C CNN
	1    1575 7000
	1    0    0    -1  
$EndComp
$Comp
L +3V3 #PWR101
U 1 1 556484DA
P 925 6750
F 0 "#PWR101" H 925 6600 50  0001 C CNN
F 1 "+3V3" H 925 6890 50  0000 C CNN
F 2 "" H 925 6750 60  0000 C CNN
F 3 "" H 925 6750 60  0000 C CNN
	1    925  6750
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR105
U 1 1 5564860C
P 875 7250
F 0 "#PWR105" H 875 7000 50  0001 C CNN
F 1 "GND" H 875 7100 50  0000 C CNN
F 2 "" H 875 7250 60  0000 C CNN
F 3 "" H 875 7250 60  0000 C CNN
	1    875  7250
	1    0    0    -1  
$EndComp
NoConn ~ 2175 7000
NoConn ~ 2175 7100
$Comp
L C C105
U 1 1 5564FCE8
P 1050 3125
F 0 "C105" V 1100 3175 50  0000 L CNN
F 1 "4u7" V 1100 2925 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 1088 2975 30  0001 C CNN
F 3 "" H 1050 3125 60  0000 C CNN
F 4 "1845742" H 1050 3125 60  0001 C CNN "Farnell"
	1    1050 3125
	0    1    1    0   
$EndComp
$Comp
L R R101
U 1 1 5565208A
P 7250 1100
F 0 "R101" V 7330 1100 50  0000 C CNN
F 1 "3k3" V 7150 1100 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 7180 1100 30  0001 C CNN
F 3 "" H 7250 1100 30  0000 C CNN
F 4 "9237682" H 7250 1100 60  0001 C CNN "Farnell"
	1    7250 1100
	1    0    0    -1  
$EndComp
$Comp
L R R102
U 1 1 55652110
P 7250 1500
F 0 "R102" V 7330 1500 50  0000 C CNN
F 1 "10k" V 7150 1500 50  0000 C CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" V 7180 1500 30  0001 C CNN
F 3 "" H 7250 1500 30  0000 C CNN
F 4 "9237755" H 7250 1500 60  0001 C CNN "Farnell"
	1    7250 1500
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR109
U 1 1 556523AF
P 7250 1700
F 0 "#PWR109" H 7250 1450 50  0001 C CNN
F 1 "GND" H 7250 1550 50  0000 C CNN
F 2 "" H 7250 1700 60  0000 C CNN
F 3 "" H 7250 1700 60  0000 C CNN
	1    7250 1700
	1    0    0    -1  
$EndComp
$Comp
L LCD_HD44780 U103
U 1 1 55638A7C
P 8200 3750
F 0 "U103" V 8250 3750 60  0000 C CNN
F 1 "LCD_HD44780" V 8150 3750 60  0000 C CNN
F 2 "" H 7450 4400 60  0000 C CNN
F 3 "" H 7450 4400 60  0000 C CNN
	1    8200 3750
	1    0    0    -1  
$EndComp
$Comp
L +3V3 #PWR113
U 1 1 5563E071
P 7500 3250
F 0 "#PWR113" H 7500 3100 50  0001 C CNN
F 1 "+3V3" H 7500 3390 50  0000 C CNN
F 2 "" H 7500 3250 60  0000 C CNN
F 3 "" H 7500 3250 60  0000 C CNN
	1    7500 3250
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR114
U 1 1 5563E174
P 7500 3350
F 0 "#PWR114" H 7500 3100 50  0001 C CNN
F 1 "GND" H 7500 3200 50  0000 C CNN
F 2 "" H 7500 3350 60  0000 C CNN
F 3 "" H 7500 3350 60  0000 C CNN
	1    7500 3350
	1    0    0    -1  
$EndComp
$Comp
L POT RV101
U 1 1 5563E702
P 6800 3850
F 0 "RV101" H 6800 3750 50  0000 C CNN
F 1 "20k LIN" H 6800 3850 50  0000 C CNN
F 2 "" H 6800 3850 60  0000 C CNN
F 3 "" H 6800 3850 60  0000 C CNN
F 4 "9354824" H 6800 3850 60  0001 C CNN "Farnell"
	1    6800 3850
	0    1    1    0   
$EndComp
$Comp
L +3V3 #PWR116
U 1 1 5563E7A7
P 6800 3550
F 0 "#PWR116" H 6800 3400 50  0001 C CNN
F 1 "+3V3" H 6800 3690 50  0000 C CNN
F 2 "" H 6800 3550 60  0000 C CNN
F 3 "" H 6800 3550 60  0000 C CNN
	1    6800 3550
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR121
U 1 1 5563E8D6
P 6800 4150
F 0 "#PWR121" H 6800 3900 50  0001 C CNN
F 1 "GND" H 6800 4000 50  0000 C CNN
F 2 "" H 6800 4150 60  0000 C CNN
F 3 "" H 6800 4150 60  0000 C CNN
	1    6800 4150
	1    0    0    -1  
$EndComp
$Comp
L +BATT #PWR117
U 1 1 55643D33
P 7150 3550
F 0 "#PWR117" H 7150 3400 50  0001 C CNN
F 1 "+BATT" H 7150 3690 50  0000 C CNN
F 2 "" H 7150 3550 60  0000 C CNN
F 3 "" H 7150 3550 60  0000 C CNN
	1    7150 3550
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR118
U 1 1 55643E77
P 7150 3650
F 0 "#PWR118" H 7150 3400 50  0001 C CNN
F 1 "GND" H 7150 3500 50  0000 C CNN
F 2 "" H 7150 3650 60  0000 C CNN
F 3 "" H 7150 3650 60  0000 C CNN
	1    7150 3650
	1    0    0    -1  
$EndComp
$Comp
L RFM95W U105
U 1 1 55635406
P 8100 5550
F 0 "U105" V 8150 5550 60  0000 C CNN
F 1 "RFM95W" V 8050 5550 60  0000 C CNN
F 2 "" H 8100 5600 60  0000 C CNN
F 3 "" H 8100 5600 60  0000 C CNN
	1    8100 5550
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR122
U 1 1 55638AFF
P 7100 4150
F 0 "#PWR122" H 7100 3900 50  0001 C CNN
F 1 "GND" H 7100 4000 50  0000 C CNN
F 2 "" H 7100 4150 60  0000 C CNN
F 3 "" H 7100 4150 60  0000 C CNN
	1    7100 4150
	1    0    0    -1  
$EndComp
$Comp
L STM32F071CBT6 U104
U 1 1 559BF2FD
P 2400 4825
F 0 "U104" V 2450 4975 60  0000 C CNN
F 1 "STM32F071CBT6" V 2350 4975 60  0000 C CNN
F 2 "Housings_QFP:LQFP-48_7x7mm_Pitch0.5mm" H 2400 5025 60  0001 C CNN
F 3 "" H 2400 5025 60  0000 C CNN
F 4 "2432093" H 2400 4825 60  0001 C CNN "Farnell"
	1    2400 4825
	1    0    0    -1  
$EndComp
$Comp
L TVS_small D101
U 1 1 559DC9F3
P 5600 950
F 0 "D101" V 5800 950 50  0000 C CNN
F 1 "TVS_small" H 5600 875 50  0001 C CNN
F 2 "Resistors_SMD:R_0603_HandSoldering" H 5600 950 60  0001 C CNN
F 3 "" H 5600 950 60  0000 C CNN
F 4 "2368174" H 5600 950 60  0001 C CNN "Farnell"
	1    5600 950 
	0    1    1    0   
$EndComp
$Comp
L TVS_small D103
U 1 1 559DD518
P 8800 6175
F 0 "D103" H 8800 6250 50  0000 C CNN
F 1 "TVS_small" H 8800 6100 50  0001 C CNN
F 2 "Resistors_SMD:R_0603_HandSoldering" H 8800 6175 60  0001 C CNN
F 3 "" H 8800 6175 60  0000 C CNN
F 4 "2368174" H 8800 6175 60  0001 C CNN "Farnell"
	1    8800 6175
	0    -1   -1   0   
$EndComp
Text Label 3800 3125 0    60   ~ 0
nRST
Text Label 3150 4275 0    60   ~ 0
SWDIO
Text Label 3150 4375 0    60   ~ 0
SWDCLK
Text Label 2175 6800 0    60   ~ 0
SWDIO
Text Label 2175 6900 0    60   ~ 0
SWDCLK
Text Label 2175 7200 0    60   ~ 0
nRST
Text Label 8700 5150 0    60   ~ 0
RFM_DIO0
Text Label 8700 5250 0    60   ~ 0
RFM_DIO1
Text Label 8700 5350 0    60   ~ 0
RFM_DIO2
Text Label 8700 5450 0    60   ~ 0
RFM_DIO3
Text Label 8700 5550 0    60   ~ 0
RFM_DIO4
Text Label 8700 5650 0    60   ~ 0
RFM_DIO5
Text Label 7500 5650 2    60   ~ 0
RFM_MISO
Text Label 7500 5750 2    60   ~ 0
RFM_MOSI
Text Label 7500 5850 2    60   ~ 0
RFM_SCK
Text Label 7500 5950 2    60   ~ 0
RFM_NSS
Text Label 7500 6050 2    60   ~ 0
RFM_RESET
Text Label 5000 6900 0    60   ~ 0
RFM_DIO0
Text Label 5000 7000 0    60   ~ 0
RFM_DIO1
Text Label 5000 7100 0    60   ~ 0
RFM_DIO2
Text Label 5000 7200 0    60   ~ 0
RFM_DIO3
Text Label 5000 7300 0    60   ~ 0
RFM_DIO4
Text Label 5000 7400 0    60   ~ 0
RFM_DIO5
Text Label 5000 6400 0    60   ~ 0
RFM_NSS
Text Label 5000 6500 0    60   ~ 0
RFM_SCK
Text Label 5000 6600 0    60   ~ 0
RFM_MISO
Text Label 5000 6700 0    60   ~ 0
RFM_MOSI
Text Label 8750 3400 0    60   ~ 0
LCD_DB0
Text Label 8750 3500 0    60   ~ 0
LCD_DB1
Text Label 8750 3600 0    60   ~ 0
LCD_DB2
Text Label 8750 3700 0    60   ~ 0
LCD_DB3
Text Label 8750 3800 0    60   ~ 0
LCD_DB4
Text Label 8750 3900 0    60   ~ 0
LCD_DB5
Text Label 8750 4000 0    60   ~ 0
LCD_DB6
Text Label 8750 4100 0    60   ~ 0
LCD_DB7
Text Label 7600 4050 2    60   ~ 0
LCD_RS
Text Label 7600 4250 2    60   ~ 0
LCD_E
Text Label 7400 1300 0    60   ~ 0
BATT_MON
Text Label 5000 4600 0    60   ~ 0
CH1_SW
Text Label 5000 4700 0    60   ~ 0
CH2_SW
Text Label 5000 4800 0    60   ~ 0
CH3_SW
Text Label 5000 4900 0    60   ~ 0
CH4_SW
Text Label 5000 4100 0    60   ~ 0
CH1_LED
Text Label 5000 4200 0    60   ~ 0
CH2_LED
Text Label 5000 4300 0    60   ~ 0
CH3_LED
Text Label 5000 4400 0    60   ~ 0
CH4_LED
Text Label 5000 6300 0    60   ~ 0
RFM_RESET
Text Label 5000 5100 0    60   ~ 0
LCD_DB0
Text Label 5000 5200 0    60   ~ 0
LCD_DB1
Text Label 5000 5300 0    60   ~ 0
LCD_DB2
Text Label 5000 5400 0    60   ~ 0
LCD_DB3
Text Label 5000 5500 0    60   ~ 0
LCD_DB4
Text Label 5000 5600 0    60   ~ 0
LCD_DB5
Text Label 5000 5700 0    60   ~ 0
LCD_DB6
Text Label 5000 5800 0    60   ~ 0
LCD_DB7
Text Label 5000 6000 0    60   ~ 0
LCD_RS
Text Label 5000 6100 0    60   ~ 0
LCD_E
Text Label 5000 3900 0    60   ~ 0
BATT_MON
$Comp
L TestPoint TP101
U 1 1 559DAAF9
P 10650 950
F 0 "TP101" H 10650 875 60  0000 C CNN
F 1 "TestPoint" H 10625 775 60  0001 C CNN
F 2 "" H 10650 950 60  0000 C CNN
F 3 "" H 10650 950 60  0000 C CNN
	1    10650 950 
	-1   0    0    1   
$EndComp
$Comp
L TestPoint TP102
U 1 1 559DAED6
P 6700 900
F 0 "TP102" H 6700 825 60  0000 C CNN
F 1 "TestPoint" H 6675 725 60  0001 C CNN
F 2 "" H 6700 900 60  0000 C CNN
F 3 "" H 6700 900 60  0000 C CNN
	1    6700 900 
	-1   0    0    1   
$EndComp
$Comp
L TestPoint TP103
U 1 1 559DAFE3
P 6700 1000
F 0 "TP103" H 6700 925 60  0000 C CNN
F 1 "TestPoint" H 6675 825 60  0001 C CNN
F 2 "" H 6700 1000 60  0000 C CNN
F 3 "" H 6700 1000 60  0000 C CNN
	1    6700 1000
	1    0    0    -1  
$EndComp
$Comp
L TestPoint TP104
U 1 1 559DB26A
P 3500 3125
F 0 "TP104" H 3500 3050 60  0000 C CNN
F 1 "TestPoint" H 3475 2950 60  0001 C CNN
F 2 "" H 3500 3125 60  0000 C CNN
F 3 "" H 3500 3125 60  0000 C CNN
	1    3500 3125
	-1   0    0    1   
$EndComp
$Comp
L C C106
U 1 1 559DAEBF
P 1050 3325
F 0 "C106" V 1000 3375 50  0000 L CNN
F 1 "100n" V 1000 3075 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 1088 3175 30  0001 C CNN
F 3 "" H 1050 3325 60  0000 C CNN
F 4 "2407344" H 1050 3325 60  0001 C CNN "Farnell"
	1    1050 3325
	0    1    -1   0   
$EndComp
$Comp
L C C104
U 1 1 559DB222
P 1050 2925
F 0 "C104" V 1000 2975 50  0000 L CNN
F 1 "100n" V 1000 2675 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 1088 2775 30  0001 C CNN
F 3 "" H 1050 2925 60  0000 C CNN
F 4 "2407344" H 1050 2925 60  0001 C CNN "Farnell"
	1    1050 2925
	0    1    -1   0   
$EndComp
$Comp
L C C103
U 1 1 559DB2C5
P 1050 2725
F 0 "C103" V 1000 2775 50  0000 L CNN
F 1 "100n" V 1000 2475 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 1088 2575 30  0001 C CNN
F 3 "" H 1050 2725 60  0000 C CNN
F 4 "2407344" H 1050 2725 60  0001 C CNN "Farnell"
	1    1050 2725
	0    1    -1   0   
$EndComp
$Comp
L C C114
U 1 1 559DBA6F
P 7250 5150
F 0 "C114" H 7100 5250 50  0000 L CNN
F 1 "100n" H 7050 5050 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 7288 5000 30  0001 C CNN
F 3 "" H 7250 5150 60  0000 C CNN
F 4 "2407344" H 7250 5150 60  0001 C CNN "Farnell"
	1    7250 5150
	1    0    0    1   
$EndComp
$Comp
L C C113
U 1 1 559DBFDA
P 6950 5150
F 0 "C113" H 6800 5250 50  0000 L CNN
F 1 "100u" H 6750 5050 50  0000 L CNN
F 2 "Capacitors_SMD:C_1206_HandSoldering" H 6988 5000 30  0001 C CNN
F 3 "" H 6950 5150 60  0000 C CNN
F 4 "2362115" H 6950 5150 60  0001 C CNN "Farnell"
	1    6950 5150
	1    0    0    -1  
$EndComp
$Comp
L ADP3335 U101
U 1 1 559E0082
P 9400 950
F 0 "U101" V 9500 950 60  0000 C CNN
F 1 "ADP3335" V 9400 950 60  0000 C CNN
F 2 "Housings_SSOP:MSOP-8_3x3mm_Pitch0.65mm" H 9400 1100 60  0001 C CNN
F 3 "" H 9400 1100 60  0000 C CNN
F 4 "2067775" H 9400 950 60  0001 C CNN "Farnell"
	1    9400 950 
	1    0    0    -1  
$EndComp
$Comp
L SMA P?
U 1 1 559E45E1
P 9150 6050
F 0 "P?" H 9250 5950 60  0000 C CNN
F 1 "SMA" H 9250 6150 60  0000 C CNN
F 2 "common:SMA-142-0701-801" H 9150 6050 60  0001 C CNN
F 3 "" H 9150 6050 60  0000 C CNN
F 4 "1608592" H 9150 6050 60  0001 C CNN "Farnell"
	1    9150 6050
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR?
U 1 1 559E503C
P 9000 5850
F 0 "#PWR?" H 9000 5600 50  0001 C CNN
F 1 "GND" H 9000 5700 50  0001 C CNN
F 2 "" H 9000 5850 60  0000 C CNN
F 3 "" H 9000 5850 60  0000 C CNN
	1    9000 5850
	-1   0    0    1   
$EndComp
$Comp
L MEU1S0303ZC U?
U 1 1 559EA3B4
P 3700 6950
F 0 "U?" H 3700 6600 60  0000 C CNN
F 1 "MEU1S0303ZC" H 3750 6900 60  0000 C CNN
F 2 "" H 3700 6950 60  0000 C CNN
F 3 "" H 3700 6950 60  0000 C CNN
F 4 "2366449" H 3700 6950 60  0001 C CNN "Farnell"
	1    3700 6950
	1    0    0    -1  
$EndComp
$Comp
L +3V3 #PWR?
U 1 1 559EA6BB
P 3150 7050
F 0 "#PWR?" H 3150 6900 50  0001 C CNN
F 1 "+3V3" H 3150 7190 50  0000 C CNN
F 2 "" H 3150 7050 60  0000 C CNN
F 3 "" H 3150 7050 60  0000 C CNN
	1    3150 7050
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR?
U 1 1 559EA828
P 3150 7250
F 0 "#PWR?" H 3150 7000 50  0001 C CNN
F 1 "GND" H 3150 7100 50  0000 C CNN
F 2 "" H 3150 7250 60  0000 C CNN
F 3 "" H 3150 7250 60  0000 C CNN
	1    3150 7250
	1    0    0    -1  
$EndComp
$Comp
L +5V #PWR?
U 1 1 559EAA6F
P 4350 7050
F 0 "#PWR?" H 4350 6900 50  0001 C CNN
F 1 "+5V" H 4350 7190 50  0000 C CNN
F 2 "" H 4350 7050 60  0000 C CNN
F 3 "" H 4350 7050 60  0000 C CNN
	1    4350 7050
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR?
U 1 1 559EABE0
P 4350 7250
F 0 "#PWR?" H 4350 7000 50  0001 C CNN
F 1 "GND" H 4350 7100 50  0000 C CNN
F 2 "" H 4350 7250 60  0000 C CNN
F 3 "" H 4350 7250 60  0000 C CNN
	1    4350 7250
	1    0    0    -1  
$EndComp
$Sheet
S 10200 4100 550  1050
U 559ED680
F0 "controls" 60
F1 "controls.sch" 60
F2 "CH1_LED" I L 10200 4200 60 
F3 "CH1_SW" O L 10200 4300 60 
F4 "CH2_LED" I L 10200 4450 60 
F5 "CH2_SW" O L 10200 4550 60 
F6 "CH3_LED" I L 10200 4700 60 
F7 "CH3_SW" O L 10200 4800 60 
F8 "CH4_LED" I L 10200 4950 60 
F9 "CH4_SW" O L 10200 5050 60 
$EndSheet
Text Label 10200 4200 2    60   ~ 0
CH1_LED
Text Label 10200 4300 2    60   ~ 0
CH1_SW
Text Label 10200 4450 2    60   ~ 0
CH2_LED
Text Label 10200 4550 2    60   ~ 0
CH2_SW
Text Label 10200 4700 2    60   ~ 0
CH3_LED
Text Label 10200 4800 2    60   ~ 0
CH3_SW
Text Label 10200 4950 2    60   ~ 0
CH4_LED
Text Label 10200 5050 2    60   ~ 0
CH4_SW
$Comp
L LiPO_1S BT?
U 1 1 55A20375
P 3925 950
F 0 "BT?" H 3625 1000 50  0000 L CNN
F 1 "LiPO_1S" H 3525 900 50  0000 L CNN
F 2 "" V 3925 990 60  0000 C CNN
F 3 "" V 3925 990 60  0000 C CNN
	1    3925 950 
	1    0    0    -1  
$EndComp
$Comp
L CONN_01X02 P?
U 1 1 55A20420
P 4125 1400
F 0 "P?" V 4125 1600 50  0000 C CNN
F 1 "BATT_CASE" V 4125 1100 50  0000 C CNN
F 2 "" H 4125 1400 60  0000 C CNN
F 3 "" H 4125 1400 60  0000 C CNN
	1    4125 1400
	0    -1   1    0   
$EndComp
$Comp
L CONN_01X02 P?
U 1 1 55A21CFA
P 4125 1550
F 0 "P?" V 4125 1750 50  0000 C CNN
F 1 "BATT_CASE" V 4125 1250 50  0000 C CNN
F 2 "" H 4125 1550 60  0000 C CNN
F 3 "" H 4125 1550 60  0000 C CNN
	1    4125 1550
	0    -1   -1   0   
$EndComp
$Comp
L CONN_01X02 P?
U 1 1 55A2205A
P 4125 2000
F 0 "P?" V 4125 2200 50  0000 C CNN
F 1 "BATT_JST_XH" V 4125 1650 50  0000 C CNN
F 2 "" H 4125 2000 60  0000 C CNN
F 3 "" H 4125 2000 60  0000 C CNN
	1    4125 2000
	0    -1   1    0   
$EndComp
$Comp
L FUSE F?
U 1 1 55A241B6
P 6250 900
F 0 "F?" H 6350 950 50  0000 C CNN
F 1 "FUSE" H 6150 850 50  0000 C CNN
F 2 "" H 6250 900 60  0000 C CNN
F 3 "" H 6250 900 60  0000 C CNN
	1    6250 900 
	1    0    0    -1  
$EndComp
$Comp
L +BATT #PWR?
U 1 1 55A7EBCE
P 8300 900
F 0 "#PWR?" H 8300 750 50  0001 C CNN
F 1 "+BATT" H 8300 1040 50  0000 C CNN
F 2 "" H 8300 900 60  0000 C CNN
F 3 "" H 8300 900 60  0000 C CNN
	1    8300 900 
	1    0    0    -1  
$EndComp
$Comp
L SPST_small SW?
U 1 1 55A7F874
P 1100 1400
F 0 "SW?" H 1100 1500 50  0000 C CNN
F 1 "KEYSWITCH" H 1100 1600 50  0000 C CNN
F 2 "" H 1100 1400 60  0000 C CNN
F 3 "" H 1100 1400 60  0000 C CNN
	1    1100 1400
	1    0    0    -1  
$EndComp
$Comp
L CONN_01X02 P?
U 1 1 55A7FC01
P 1600 1450
F 0 "P?" H 1600 1600 50  0000 C CNN
F 1 "KEYSW" H 1550 1300 50  0000 C CNN
F 2 "" H 1600 1450 60  0000 C CNN
F 3 "" H 1600 1450 60  0000 C CNN
	1    1600 1450
	1    0    0    1   
$EndComp
$Comp
L CONN_01X02 P?
U 1 1 55A7FE33
P 1750 1450
F 0 "P?" H 1750 1600 50  0000 C CNN
F 1 "KEYSW" H 1700 1300 50  0000 C CNN
F 2 "" H 1750 1450 60  0000 C CNN
F 3 "" H 1750 1450 60  0000 C CNN
	1    1750 1450
	-1   0    0    1   
$EndComp
$Comp
L GND #PWR?
U 1 1 55A805A4
P 2000 1550
F 0 "#PWR?" H 2000 1300 50  0001 C CNN
F 1 "GND" H 2000 1400 50  0000 C CNN
F 2 "" H 2000 1550 60  0000 C CNN
F 3 "" H 2000 1550 60  0000 C CNN
	1    2000 1550
	1    0    0    -1  
$EndComp
$Comp
L R R?
U 1 1 55A80750
P 2050 1200
F 0 "R?" V 2130 1200 50  0000 C CNN
F 1 "10k" V 2050 1200 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 1980 1200 30  0001 C CNN
F 3 "" H 2050 1200 30  0000 C CNN
F 4 "2331808" H 2050 1200 60  0001 C CNN "Farnell"
	1    2050 1200
	1    0    0    -1  
$EndComp
$Comp
L +3.3V #PWR?
U 1 1 55A80891
P 2050 1000
F 0 "#PWR?" H 2050 850 50  0001 C CNN
F 1 "+3.3V" H 2050 1140 50  0000 C CNN
F 2 "" H 2050 1000 60  0000 C CNN
F 3 "" H 2050 1000 60  0000 C CNN
	1    2050 1000
	1    0    0    -1  
$EndComp
Text Label 2200 1400 0    60   ~ 0
ARM_SW
Wire Wire Line
	9500 1550 9500 1600
Wire Wire Line
	8500 1000 8500 1150
Connection ~ 8500 1000
Wire Wire Line
	8500 1450 8500 1500
Wire Wire Line
	10250 1400 10250 1450
Wire Wire Line
	9950 950  10850 950 
Wire Wire Line
	10250 950  10250 1100
Connection ~ 10250 950 
Wire Wire Line
	10850 850  10850 1050
Wire Wire Line
	5650 1000 7000 1000
Wire Wire Line
	5850 850  5850 900 
Connection ~ 5850 900 
Wire Wire Line
	5850 1050 5850 1000
Connection ~ 5850 1000
Wire Wire Line
	7000 900  7000 850 
Wire Wire Line
	7000 1000 7000 1050
Wire Wire Line
	6950 5350 7500 5350
Wire Wire Line
	7450 5350 7450 5450
Wire Wire Line
	7450 5450 7500 5450
Connection ~ 7450 5350
Wire Wire Line
	7500 5250 7400 5250
Wire Wire Line
	7400 5250 7400 5350
Connection ~ 7400 5350
Wire Wire Line
	7250 4900 7250 5000
Wire Wire Line
	6950 4950 7450 4950
Wire Wire Line
	7450 4950 7450 5150
Wire Wire Line
	7450 5150 7500 5150
Connection ~ 7250 4950
Wire Wire Line
	7250 5300 7250 5350
Connection ~ 7250 5350
Wire Wire Line
	8700 6050 9050 6050
Wire Wire Line
	1700 4025 1600 4025
Wire Wire Line
	1600 4025 1600 4425
Wire Wire Line
	1600 4125 1700 4125
Wire Wire Line
	1600 4225 1700 4225
Connection ~ 1600 4125
Wire Wire Line
	1600 4425 1700 4425
Connection ~ 1600 4225
Wire Wire Line
	800  4175 1600 4175
Wire Wire Line
	1200 4175 1200 4275
Connection ~ 1600 4175
Wire Wire Line
	3650 3125 3650 3175
Connection ~ 3650 3125
Wire Wire Line
	3050 3475 3650 3475
Wire Wire Line
	3600 3475 3600 3525
Wire Wire Line
	3400 3125 3800 3125
Wire Wire Line
	3400 3375 3050 3375
Wire Wire Line
	3050 3975 3950 3975
Wire Wire Line
	3950 3975 3950 3875
Wire Wire Line
	3950 3875 4600 3875
Wire Wire Line
	3950 4075 3950 4175
Wire Wire Line
	3950 4175 4400 4175
Wire Wire Line
	4400 4175 4400 4225
Connection ~ 4150 4175
Wire Wire Line
	4600 3875 4600 4225
Connection ~ 4150 3875
Wire Wire Line
	4600 4575 4600 4525
Wire Wire Line
	4400 4575 4600 4575
Connection ~ 4500 4575
Wire Wire Line
	4400 4575 4400 4525
Connection ~ 10850 950 
Wire Wire Line
	10850 1450 10850 1500
Wire Wire Line
	10850 1800 10850 1850
Wire Wire Line
	925  6750 925  6800
Wire Wire Line
	925  6800 975  6800
Wire Wire Line
	975  6900 875  6900
Wire Wire Line
	875  6900 875  7250
Wire Wire Line
	875  7200 975  7200
Connection ~ 875  7200
Wire Wire Line
	975  7000 875  7000
Connection ~ 875  7000
Wire Wire Line
	7250 900  7250 950 
Wire Wire Line
	7250 1650 7250 1700
Wire Wire Line
	7250 1250 7250 1350
Wire Wire Line
	7250 1300 7400 1300
Connection ~ 7250 1300
Wire Wire Line
	7600 3350 7500 3350
Wire Wire Line
	6800 3550 6800 3600
Wire Wire Line
	6800 4150 6800 4100
Wire Wire Line
	6950 3850 7600 3850
Wire Wire Line
	7150 3550 7600 3550
Wire Wire Line
	7150 3650 7600 3650
Wire Wire Line
	7500 3250 7600 3250
Wire Wire Line
	7100 4150 7600 4150
Wire Wire Line
	3400 3125 3400 3375
Connection ~ 3600 3475
Wire Wire Line
	5550 900  5550 850 
Wire Wire Line
	5550 850  5650 850 
Wire Wire Line
	5600 850  5600 850 
Wire Wire Line
	5650 850  5650 900 
Connection ~ 5600 850 
Wire Wire Line
	5550 1000 5550 1050
Wire Wire Line
	5550 1050 5650 1050
Wire Wire Line
	5600 1050 5600 1050
Wire Wire Line
	5650 1050 5650 1000
Connection ~ 5600 1050
Connection ~ 7250 900 
Wire Wire Line
	8800 6050 8800 6075
Connection ~ 8800 6050
Wire Wire Line
	3150 4275 3050 4275
Wire Wire Line
	3050 4375 3150 4375
Connection ~ 10650 950 
Connection ~ 6700 900 
Connection ~ 6700 1000
Connection ~ 3500 3125
Wire Wire Line
	3050 4075 3550 4075
Wire Wire Line
	3850 4075 3950 4075
Connection ~ 1200 4175
Wire Wire Line
	800  3325 900  3325
Wire Wire Line
	800  2725 800  4175
Wire Wire Line
	900  3925 800  3925
Connection ~ 800  3925
Wire Wire Line
	800  3725 900  3725
Connection ~ 800  3725
Wire Wire Line
	900  3525 800  3525
Connection ~ 800  3525
Wire Wire Line
	800  3125 900  3125
Connection ~ 800  3325
Wire Wire Line
	800  2925 900  2925
Connection ~ 800  3125
Wire Wire Line
	800  2725 900  2725
Connection ~ 800  2925
Wire Wire Line
	1200 2575 1200 3925
Wire Wire Line
	1200 3825 1700 3825
Connection ~ 1200 3825
Connection ~ 1200 3525
Connection ~ 1200 3725
Wire Wire Line
	1200 3425 1300 3425
Wire Wire Line
	1300 3425 1300 3725
Wire Wire Line
	1300 3725 1700 3725
Connection ~ 1200 3425
Connection ~ 1200 3325
Connection ~ 1200 3125
Connection ~ 1200 2925
Wire Wire Line
	1200 3025 1450 3025
Wire Wire Line
	1450 3025 1450 3525
Wire Wire Line
	1450 3525 1700 3525
Connection ~ 1200 3025
Wire Wire Line
	1200 2825 1550 2825
Wire Wire Line
	1550 2825 1550 3425
Wire Wire Line
	1550 3425 1700 3425
Connection ~ 1200 2825
Connection ~ 1200 2725
Wire Wire Line
	1200 2625 1700 2625
Wire Wire Line
	1700 2625 1700 3225
Connection ~ 1200 2625
Wire Wire Line
	6950 5000 6950 4950
Wire Wire Line
	6950 5300 6950 5350
Connection ~ 7200 5350
Wire Wire Line
	8900 950  8900 1050
Wire Wire Line
	8900 950  8950 950 
Wire Wire Line
	8900 1050 8950 1050
Connection ~ 8900 1000
Wire Wire Line
	10050 1050 9950 1050
Connection ~ 10050 950 
Wire Wire Line
	9950 850  10000 850 
Wire Wire Line
	10000 850  10000 950 
Connection ~ 10000 950 
Wire Wire Line
	10050 950  10050 1050
Wire Wire Line
	8800 1000 8800 1550
Wire Wire Line
	8800 1550 9400 1550
Connection ~ 8800 1000
Wire Wire Line
	9000 6200 9050 6200
Wire Wire Line
	9000 6150 9000 6300
Wire Wire Line
	9000 6150 9050 6150
Connection ~ 9000 6200
Wire Wire Line
	9000 5850 9000 5950
Wire Wire Line
	9000 5900 9050 5900
Wire Wire Line
	9000 5950 9050 5950
Connection ~ 9000 5900
Wire Wire Line
	8800 6275 9000 6275
Connection ~ 9000 6275
Wire Wire Line
	3150 7050 3150 7100
Wire Wire Line
	3150 7100 3200 7100
Wire Wire Line
	3150 7250 3150 7200
Wire Wire Line
	3150 7200 3200 7200
Wire Wire Line
	4350 7050 4350 7100
Wire Wire Line
	4350 7100 4300 7100
Wire Wire Line
	4350 7250 4350 7200
Wire Wire Line
	4350 7200 4300 7200
Wire Wire Line
	4075 1100 4075 1200
Wire Wire Line
	4175 800  4175 1200
Connection ~ 4175 800 
Connection ~ 4075 1100
Wire Wire Line
	4075 1750 4075 1800
Wire Wire Line
	4175 1800 4175 1750
Wire Wire Line
	5650 900  6000 900 
Wire Wire Line
	6500 900  7250 900 
Connection ~ 7000 900 
Wire Wire Line
	8300 900  8300 1000
Wire Wire Line
	8300 1000 8900 1000
Wire Wire Line
	900  1400 900  1500
Wire Wire Line
	900  1500 1400 1500
Wire Wire Line
	1250 1400 1400 1400
Wire Wire Line
	1950 1500 2000 1500
Wire Wire Line
	2000 1500 2000 1550
Wire Wire Line
	2050 1000 2050 1050
Wire Wire Line
	1950 1400 2200 1400
Wire Wire Line
	2050 1400 2050 1350
Connection ~ 2050 1400
Wire Wire Line
	5400 1000 5550 1000
Wire Wire Line
	3925 1100 4375 1100
Wire Wire Line
	4375 1100 4375 1000
Wire Wire Line
	4375 1000 4850 1000
$Comp
L CONN_01X02 P?
U 1 1 55A9232C
P 5200 950
F 0 "P?" H 5200 1100 50  0000 C CNN
F 1 "BATT" H 5150 800 50  0000 C CNN
F 2 "" H 5200 950 60  0000 C CNN
F 3 "" H 5200 950 60  0000 C CNN
	1    5200 950 
	-1   0    0    1   
$EndComp
$Comp
L CONN_01X02 P?
U 1 1 55A92786
P 5050 950
F 0 "P?" H 5050 1100 50  0000 C CNN
F 1 "BATT" H 5000 800 50  0000 C CNN
F 2 "" H 5050 950 60  0000 C CNN
F 3 "" H 5050 950 60  0000 C CNN
	1    5050 950 
	1    0    0    1   
$EndComp
Wire Wire Line
	5400 900  5550 900 
Wire Wire Line
	4750 900  4850 900 
Wire Wire Line
	3925 800  4325 800 
Wire Wire Line
	4325 800  4325 900 
Wire Wire Line
	4325 900  4400 900 
$Comp
L SPST_small SW?
U 1 1 55A87CDE
P 4600 900
F 0 "SW?" H 4600 1000 50  0000 C CNN
F 1 "LATCHING PUSH" H 4600 1075 50  0000 C CNN
F 2 "" H 4600 900 60  0000 C CNN
F 3 "" H 4600 900 60  0000 C CNN
F 4 "SW02719" H 4600 900 60  0001 C CNN "Farnell"
	1    4600 900 
	1    0    0    -1  
$EndComp
$Comp
L Led_x2 D?
U 1 1 55A88ECE
P 6250 2450
F 0 "D?" H 6250 2675 50  0000 C CNN
F 1 "Led_x2" H 6250 2200 50  0000 C CNN
F 2 "" H 6250 2450 60  0000 C CNN
F 3 "" H 6250 2450 60  0000 C CNN
F 4 "2449761" H 6250 2450 60  0001 C CNN "Farnell"
	1    6250 2450
	-1   0    0    1   
$EndComp
Text Label 5650 2350 2    60   ~ 0
ARM_LED
Text Label 5650 2550 2    60   ~ 0
DISARM_LED
$Comp
L R R?
U 1 1 55A89898
P 5800 2350
F 0 "R?" V 5880 2350 50  0000 C CNN
F 1 "68R" V 5800 2350 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 5730 2350 30  0001 C CNN
F 3 "" H 5800 2350 30  0000 C CNN
F 4 "2332055" H 5800 2350 60  0001 C CNN "Farnell"
	1    5800 2350
	0    1    1    0   
$EndComp
$Comp
L R R?
U 1 1 55A8994B
P 5800 2550
F 0 "R?" V 5880 2550 50  0000 C CNN
F 1 "68R" V 5800 2550 50  0000 C CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 5730 2550 30  0001 C CNN
F 3 "" H 5800 2550 30  0000 C CNN
F 4 "2332055" H 5800 2550 60  0001 C CNN "Farnell"
	1    5800 2550
	0    1    1    0   
$EndComp
$Comp
L GND #PWR?
U 1 1 55A89B8A
P 6600 2500
F 0 "#PWR?" H 6600 2250 50  0001 C CNN
F 1 "GND" H 6600 2350 50  0000 C CNN
F 2 "" H 6600 2500 60  0000 C CNN
F 3 "" H 6600 2500 60  0000 C CNN
	1    6600 2500
	1    0    0    -1  
$EndComp
Wire Wire Line
	6550 2450 6600 2450
Wire Wire Line
	6600 2450 6600 2500
Text Label 5000 3800 0    60   ~ 0
ARM_SW
Text Label 5000 3700 0    60   ~ 0
DISARM_LED
Text Label 5000 3600 0    60   ~ 0
ARM_LED
$Comp
L PART U?
U 1 1 55A91744
P 6050 2900
F 0 "U?" H 6100 2900 60  0000 C CNN
F 1 "BEZEL" H 6150 3000 60  0000 C CNN
F 2 "" H 6050 2900 60  0000 C CNN
F 3 "" H 6050 2900 60  0000 C CNN
F 4 "SC11628" H 6050 2900 60  0001 C CNN "Farnell"
	1    6050 2900
	1    0    0    -1  
$EndComp
Text Notes 5150 2250 0    60   ~ 0
RED: 1.95V 20mA
Text Notes 5050 2750 0    60   ~ 0
GREEN: 2.1V 20mA
$EndSCHEMATC
