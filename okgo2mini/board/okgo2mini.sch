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
LIBS:stm32f0xxkxtx
LIBS:swd
LIBS:swd_tc
LIBS:led_bicolour
LIBS:rfm95w
LIBS:esd_diode
LIBS:sma
LIBS:tps62130
LIBS:adp3335
LIBS:bq24232ha
LIBS:microusb
LIBS:switch
LIBS:okgo2mini-cache
EELAYER 26 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
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
L STM32F0xxKxTx IC?
U 1 1 5996E255
P 4200 3050
F 0 "IC?" H 4200 4275 50  0000 C CNN
F 1 "STM32F051K6T7" H 4200 4184 50  0000 C CNN
F 2 "agg:LQFP-32" H 3900 1850 50  0001 L CNN
F 3 "http://www.st.com/web/en/resource/technical/document/datasheet/DM00088500.pdf" H 3900 1750 50  0001 L CNN
F 4 "2432085" H 3900 1650 50  0001 L CNN "Farnell"
	1    4200 3050
	1    0    0    -1  
$EndComp
$Comp
L RFM95W U?
U 1 1 5996E379
P 9600 1550
F 0 "U?" H 9600 2237 60  0000 C CNN
F 1 "RFM95W" H 9600 2131 60  0000 C CNN
F 2 "" H 9600 1600 60  0000 C CNN
F 3 "" H 9600 1600 60  0000 C CNN
	1    9600 1550
	1    0    0    -1  
$EndComp
$Comp
L ESD_DIODE D?
U 1 1 5996E48D
P 10350 2150
F 0 "D?" V 10350 2208 50  0000 L CNN
F 1 "ESD_DIODE" H 10300 2050 50  0001 L CNN
F 2 "agg:0603" H 10300 1950 50  0001 L CNN
F 3 "" H 10250 2150 50  0001 C CNN
	1    10350 2150
	0    1    1    0   
$EndComp
$Comp
L SMA P?
U 1 1 5996E5C3
P 10800 2050
F 0 "P?" H 10978 2103 60  0000 L CNN
F 1 "SMA" H 10978 1997 60  0000 L CNN
F 2 "" H 10800 2050 60  0000 C CNN
F 3 "" H 10800 2050 60  0000 C CNN
	1    10800 2050
	1    0    0    -1  
$EndComp
$Comp
L TPS62130 IC?
U 1 1 5996EA29
P 8100 5350
F 0 "IC?" H 8100 6175 50  0000 C CNN
F 1 "TPS62130" H 8100 6084 50  0000 C CNN
F 2 "agg:QFN-16-EP-TI" H 7800 4550 50  0001 L CNN
F 3 "http://www.ti.com/lit/ds/symlink/tps62130.pdf" H 7800 4450 50  0001 L CNN
F 4 "2361246" H 7800 4350 50  0001 L CNN "Farnell"
	1    8100 5350
	1    0    0    -1  
$EndComp
$Comp
L L_Small L?
U 1 1 5996EBF6
P 8750 4750
F 0 "L?" V 8572 4750 50  0000 C CNN
F 1 "2u2" V 8663 4750 50  0000 C CNN
F 2 "agg:XFL4020" H 8750 4750 50  0001 C CNN
F 3 "" H 8750 4750 50  0001 C CNN
F 4 "2289216" H 8750 4750 60  0001 C CNN "Farnell"
	1    8750 4750
	0    1    1    0   
$EndComp
$Comp
L D_Schottky D?
U 1 1 5996EF3F
P 9300 5850
F 0 "D?" V 9254 5929 50  0000 L CNN
F 1 "PMEG2005AEA" V 9345 5929 50  0001 L CNN
F 2 "agg:SOD-323" H 9300 5850 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/684565.pdf" H 9300 5850 50  0001 C CNN
F 4 "8737843" H 9300 5850 60  0001 C CNN "Farnell"
	1    9300 5850
	0    1    1    0   
$EndComp
$Comp
L R R?
U 1 1 5996F007
P 9550 6000
F 0 "R?" V 9343 6000 50  0000 C CNN
F 1 "3k3" V 9434 6000 50  0000 C CNN
F 2 "" V 9480 6000 50  0001 C CNN
F 3 "" H 9550 6000 50  0001 C CNN
	1    9550 6000
	0    1    1    0   
$EndComp
$Comp
L R R?
U 1 1 5996F064
P 9950 6000
F 0 "R?" V 9743 6000 50  0000 C CNN
F 1 "3k3" V 9834 6000 50  0000 C CNN
F 2 "" V 9880 6000 50  0001 C CNN
F 3 "" H 9950 6000 50  0001 C CNN
	1    9950 6000
	0    1    1    0   
$EndComp
$Comp
L BQ24232HA IC?
U 1 1 5996F22A
P 2050 7050
F 0 "IC?" H 2050 7875 50  0000 C CNN
F 1 "BQ24232HA" H 2050 7784 50  0000 C CNN
F 2 "agg:QFN-16-EP-TI" H 1650 6250 50  0001 L CNN
F 3 "http://www.ti.com/lit/ds/symlink/bq24232ha.pdf" H 1650 6150 50  0001 L CNN
F 4 "2576286" H 1650 6050 50  0001 L CNN "Farnell"
	1    2050 7050
	1    0    0    -1  
$EndComp
$Comp
L ADP3335 IC?
U 1 1 5996F300
P 4900 6550
F 0 "IC?" H 4900 6875 50  0000 C CNN
F 1 "ADP3335" H 4900 6784 50  0000 C CNN
F 2 "agg:MSOP-8" H 4700 6150 50  0001 L CNN
F 3 "" H 5300 6150 50  0001 C CNN
F 4 "2067775" H 4700 6050 50  0001 L CNN "Farnell"
	1    4900 6550
	1    0    0    -1  
$EndComp
$Comp
L CONN_01X03 J?
U 1 1 5996F555
P 2700 5550
F 0 "J?" H 2778 5591 50  0000 L CNN
F 1 "BATT" H 2778 5500 50  0000 L CNN
F 2 "" H 2700 5550 50  0001 C CNN
F 3 "" H 2700 5550 50  0001 C CNN
	1    2700 5550
	-1   0    0    -1  
$EndComp
$Comp
L CONN_01X02 J?
U 1 1 5996F878
P 10550 4800
F 0 "J?" H 10468 4525 50  0000 C CNN
F 1 "PYRO" H 10468 4616 50  0000 C CNN
F 2 "" H 10550 4800 50  0001 C CNN
F 3 "" H 10550 4800 50  0001 C CNN
	1    10550 4800
	1    0    0    1   
$EndComp
$Comp
L MICROUSB J?
U 1 1 5996FB8F
P 4150 5550
F 0 "J?" H 4231 5975 50  0000 C CNN
F 1 "MICROUSB STRAIGHT" H 4231 5884 50  0000 C CNN
F 2 "" H 4000 5050 50  0001 L CNN
F 3 "" H 4450 5750 50  0001 C CNN
F 4 "2614949" H 4000 4950 50  0001 L CNN "Farnell"
	1    4150 5550
	1    0    0    -1  
$EndComp
$Comp
L SWITCH_SPST SW?
U 1 1 59970363
P 3000 5450
F 0 "SW?" H 3000 5705 50  0000 C CNN
F 1 "SWITCH_SPST" H 3000 5614 50  0000 C CNN
F 2 "" H 3000 5450 50  0001 C CNN
F 3 "" H 3000 5450 50  0001 C CNN
	1    3000 5450
	1    0    0    -1  
$EndComp
$Comp
L LED_bicolour D?
U 1 1 59970692
P 1300 2850
F 0 "D?" V 1254 3008 50  0000 L CNN
F 1 "ARM" V 1345 3008 50  0000 L CNN
F 2 "" V 1300 2910 60  0000 C CNN
F 3 "" V 1300 2910 60  0000 C CNN
	1    1300 2850
	0    1    1    0   
$EndComp
$Comp
L LED D?
U 1 1 599715BA
P 1250 3700
F 0 "D?" V 1288 3582 50  0000 R CNN
F 1 "CHARGING" V 1197 3582 50  0000 R CNN
F 2 "" H 1250 3700 50  0001 C CNN
F 3 "" H 1250 3700 50  0001 C CNN
	1    1250 3700
	0    -1   -1   0   
$EndComp
$Comp
L GND #PWR?
U 1 1 59972393
P 2900 5650
F 0 "#PWR?" H 2900 5400 50  0001 C CNN
F 1 "GND" H 2905 5477 50  0000 C CNN
F 2 "" H 2900 5650 50  0001 C CNN
F 3 "" H 2900 5650 50  0001 C CNN
	1    2900 5650
	1    0    0    -1  
$EndComp
Text Label 2900 5550 0    60   ~ 0
BATT_TEMP
Text Label 3250 5450 0    60   ~ 0
VBAT
Text Label 1350 5100 2    60   ~ 0
VBAT
$Comp
L R R?
U 1 1 59972F26
P 1350 5250
F 0 "R?" H 1420 5296 50  0000 L CNN
F 1 "100k" H 1420 5205 50  0000 L CNN
F 2 "" V 1280 5250 50  0001 C CNN
F 3 "" H 1350 5250 50  0001 C CNN
	1    1350 5250
	1    0    0    -1  
$EndComp
$Comp
L R R?
U 1 1 59972FEF
P 1350 5650
F 0 "R?" H 1420 5696 50  0000 L CNN
F 1 "100k" H 1420 5605 50  0000 L CNN
F 2 "" V 1280 5650 50  0001 C CNN
F 3 "" H 1350 5650 50  0001 C CNN
	1    1350 5650
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR?
U 1 1 59973248
P 1350 5800
F 0 "#PWR?" H 1350 5550 50  0001 C CNN
F 1 "GND" H 1355 5627 50  0000 C CNN
F 2 "" H 1350 5800 50  0001 C CNN
F 3 "" H 1350 5800 50  0001 C CNN
	1    1350 5800
	1    0    0    -1  
$EndComp
Wire Wire Line
	1350 5400 1350 5500
Wire Wire Line
	1350 5450 1600 5450
Connection ~ 1350 5450
Text Label 1600 5450 0    60   ~ 0
BATT_MON
Text Label 5250 3000 0    60   ~ 0
BATT_MON
Wire Wire Line
	3100 5450 3250 5450
Text Label 3300 7100 0    60   ~ 0
VBAT
Wire Wire Line
	2550 7050 2600 7050
Wire Wire Line
	2600 7050 2600 7150
Wire Wire Line
	2600 7100 3300 7100
Wire Wire Line
	2600 7150 2550 7150
Connection ~ 2600 7100
Text Label 2550 7250 0    60   ~ 0
BATT_TEMP
NoConn ~ 2550 7450
Text Label 2550 7550 0    60   ~ 0
~CHG
Text Label 5350 3100 0    60   ~ 0
~CHG
NoConn ~ 1550 7350
$Comp
L GND #PWR?
U 1 1 59974F9E
P 1350 7350
F 0 "#PWR?" H 1350 7100 50  0001 C CNN
F 1 "GND" H 1355 7177 50  0000 C CNN
F 2 "" H 1350 7350 50  0001 C CNN
F 3 "" H 1350 7350 50  0001 C CNN
	1    1350 7350
	1    0    0    -1  
$EndComp
Wire Wire Line
	1350 7350 1350 7250
Wire Wire Line
	1350 7250 1550 7250
$Comp
L R R?
U 1 1 599757DE
P 1200 7050
F 0 "R?" V 1407 7050 50  0000 C CNN
F 1 "4k32" V 1316 7050 50  0000 C CNN
F 2 "" V 1130 7050 50  0001 C CNN
F 3 "" H 1200 7050 50  0001 C CNN
	1    1200 7050
	0    -1   -1   0   
$EndComp
$Comp
L GND #PWR?
U 1 1 599758BA
P 1000 7150
F 0 "#PWR?" H 1000 6900 50  0001 C CNN
F 1 "GND" H 1005 6977 50  0000 C CNN
F 2 "" H 1000 7150 50  0001 C CNN
F 3 "" H 1000 7150 50  0001 C CNN
	1    1000 7150
	1    0    0    -1  
$EndComp
Wire Wire Line
	1000 7150 1000 7050
Wire Wire Line
	1000 7050 1050 7050
Wire Wire Line
	1350 7050 1550 7050
NoConn ~ 1550 6850
NoConn ~ 1550 6950
$Comp
L GND #PWR?
U 1 1 59975F57
P 1400 6650
F 0 "#PWR?" H 1400 6400 50  0001 C CNN
F 1 "GND" H 1405 6477 50  0000 C CNN
F 2 "" H 1400 6650 50  0001 C CNN
F 3 "" H 1400 6650 50  0001 C CNN
	1    1400 6650
	1    0    0    -1  
$EndComp
Wire Wire Line
	1400 6650 1550 6650
Wire Wire Line
	1550 6550 1400 6550
Wire Wire Line
	1400 6550 1400 6650
Connection ~ 1400 6650
$Comp
L +5V #PWR?
U 1 1 59976218
P 4800 5300
F 0 "#PWR?" H 4800 5150 50  0001 C CNN
F 1 "+5V" H 4815 5473 50  0000 C CNN
F 2 "" H 4800 5300 50  0001 C CNN
F 3 "" H 4800 5300 50  0001 C CNN
	1    4800 5300
	1    0    0    -1  
$EndComp
Wire Wire Line
	4450 5350 4800 5350
Wire Wire Line
	4800 5350 4800 5300
NoConn ~ 4450 5450
NoConn ~ 4450 5550
NoConn ~ 4450 5650
$Comp
L GND #PWR?
U 1 1 59976437
P 4600 5900
F 0 "#PWR?" H 4600 5650 50  0001 C CNN
F 1 "GND" H 4605 5727 50  0000 C CNN
F 2 "" H 4600 5900 50  0001 C CNN
F 3 "" H 4600 5900 50  0001 C CNN
	1    4600 5900
	1    0    0    -1  
$EndComp
Wire Wire Line
	4450 5750 4600 5750
Wire Wire Line
	4600 5750 4600 5900
NoConn ~ 4450 5850
$Comp
L +5V #PWR?
U 1 1 599768C4
P 900 6400
F 0 "#PWR?" H 900 6250 50  0001 C CNN
F 1 "+5V" H 915 6573 50  0000 C CNN
F 2 "" H 900 6400 50  0001 C CNN
F 3 "" H 900 6400 50  0001 C CNN
	1    900  6400
	1    0    0    -1  
$EndComp
Wire Wire Line
	900  6400 900  6450
Wire Wire Line
	900  6450 1550 6450
$Comp
L GND #PWR?
U 1 1 59976B92
P 2550 6850
F 0 "#PWR?" H 2550 6600 50  0001 C CNN
F 1 "GND" H 2555 6677 50  0001 C CNN
F 2 "" H 2550 6850 50  0001 C CNN
F 3 "" H 2550 6850 50  0001 C CNN
	1    2550 6850
	1    0    0    -1  
$EndComp
Wire Wire Line
	2550 6850 2550 6850
Wire Wire Line
	2650 6750 2550 6750
Wire Wire Line
	2650 6450 2650 6750
Wire Wire Line
	2650 6550 2550 6550
Wire Wire Line
	2550 6450 4600 6450
Connection ~ 2650 6550
Connection ~ 2650 6450
Text Label 3200 6450 0    60   ~ 0
PYRO_POWER
Wire Wire Line
	4500 6450 4500 6650
Wire Wire Line
	4500 6550 4600 6550
Connection ~ 4500 6450
Wire Wire Line
	4500 6650 4600 6650
Connection ~ 4500 6550
$Comp
L C C?
U 1 1 59977621
P 4200 6650
F 0 "C?" H 4315 6696 50  0000 L CNN
F 1 "1u" H 4315 6605 50  0000 L CNN
F 2 "" H 4238 6500 50  0001 C CNN
F 3 "" H 4200 6650 50  0001 C CNN
	1    4200 6650
	1    0    0    -1  
$EndComp
Wire Wire Line
	4200 6500 4200 6450
Connection ~ 4200 6450
$Comp
L GND #PWR?
U 1 1 59977735
P 4200 6800
F 0 "#PWR?" H 4200 6550 50  0001 C CNN
F 1 "GND" H 4205 6627 50  0000 C CNN
F 2 "" H 4200 6800 50  0001 C CNN
F 3 "" H 4200 6800 50  0001 C CNN
	1    4200 6800
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR?
U 1 1 599777AE
P 4600 6750
F 0 "#PWR?" H 4600 6500 50  0001 C CNN
F 1 "GND" H 4605 6577 50  0000 C CNN
F 2 "" H 4600 6750 50  0001 C CNN
F 3 "" H 4600 6750 50  0001 C CNN
	1    4600 6750
	1    0    0    -1  
$EndComp
$Comp
L C C?
U 1 1 59977CFB
P 5450 6600
F 0 "C?" H 5335 6554 50  0000 R CNN
F 1 "DNF" H 5335 6645 50  0000 R CNN
F 2 "agg:0603" H 5488 6450 50  0001 C CNN
F 3 "" H 5450 6600 50  0001 C CNN
	1    5450 6600
	-1   0    0    1   
$EndComp
Wire Wire Line
	5200 6750 5450 6750
Wire Wire Line
	5200 6550 5300 6550
Wire Wire Line
	5300 6450 5300 6650
Connection ~ 5300 6650
Wire Wire Line
	5200 6450 6300 6450
Connection ~ 5300 6550
$Comp
L C C?
U 1 1 59978CA1
P 6000 6600
F 0 "C?" H 6115 6646 50  0000 L CNN
F 1 "1u" H 6115 6555 50  0000 L CNN
F 2 "" H 6038 6450 50  0001 C CNN
F 3 "" H 6000 6600 50  0001 C CNN
	1    6000 6600
	1    0    0    -1  
$EndComp
Connection ~ 5300 6450
$Comp
L GND #PWR?
U 1 1 59978D78
P 6000 6750
F 0 "#PWR?" H 6000 6500 50  0001 C CNN
F 1 "GND" H 6005 6577 50  0000 C CNN
F 2 "" H 6000 6750 50  0001 C CNN
F 3 "" H 6000 6750 50  0001 C CNN
	1    6000 6750
	1    0    0    -1  
$EndComp
$Comp
L C C?
U 1 1 5997A0A5
P 2850 6600
F 0 "C?" H 2965 6646 50  0000 L CNN
F 1 "4u7" H 2965 6555 50  0000 L CNN
F 2 "" H 2888 6450 50  0001 C CNN
F 3 "" H 2850 6600 50  0001 C CNN
	1    2850 6600
	1    0    0    -1  
$EndComp
Connection ~ 2850 6450
$Comp
L GND #PWR?
U 1 1 5997A708
P 2850 6750
F 0 "#PWR?" H 2850 6500 50  0001 C CNN
F 1 "GND" H 2855 6577 50  0000 C CNN
F 2 "" H 2850 6750 50  0001 C CNN
F 3 "" H 2850 6750 50  0001 C CNN
	1    2850 6750
	1    0    0    -1  
$EndComp
$Comp
L C C?
U 1 1 5997B5C0
P 3250 7250
F 0 "C?" H 3365 7296 50  0000 L CNN
F 1 "4u7" H 3365 7205 50  0000 L CNN
F 2 "" H 3288 7100 50  0001 C CNN
F 3 "" H 3250 7250 50  0001 C CNN
	1    3250 7250
	1    0    0    -1  
$EndComp
Connection ~ 3250 7100
$Comp
L GND #PWR?
U 1 1 5997B6AA
P 3250 7400
F 0 "#PWR?" H 3250 7150 50  0001 C CNN
F 1 "GND" H 3255 7227 50  0000 C CNN
F 2 "" H 3250 7400 50  0001 C CNN
F 3 "" H 3250 7400 50  0001 C CNN
	1    3250 7400
	1    0    0    -1  
$EndComp
$Comp
L C C?
U 1 1 5997C05B
P 900 6600
F 0 "C?" H 1015 6646 50  0000 L CNN
F 1 "1u" H 1015 6555 50  0000 L CNN
F 2 "" H 938 6450 50  0001 C CNN
F 3 "" H 900 6600 50  0001 C CNN
	1    900  6600
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR?
U 1 1 5997C917
P 900 6750
F 0 "#PWR?" H 900 6500 50  0001 C CNN
F 1 "GND" H 905 6577 50  0000 C CNN
F 2 "" H 900 6750 50  0001 C CNN
F 3 "" H 900 6750 50  0001 C CNN
	1    900  6750
	1    0    0    -1  
$EndComp
Connection ~ 900  6450
Connection ~ 6000 6450
$Comp
L +3V3 #PWR?
U 1 1 5997CFBC
P 6300 6450
F 0 "#PWR?" H 6300 6300 50  0001 C CNN
F 1 "+3V3" V 6315 6578 50  0000 L CNN
F 2 "" H 6300 6450 50  0001 C CNN
F 3 "" H 6300 6450 50  0001 C CNN
	1    6300 6450
	0    1    1    0   
$EndComp
Connection ~ 5450 6450
Wire Wire Line
	5300 6650 5200 6650
Wire Wire Line
	8500 4750 8650 4750
Wire Wire Line
	8550 4850 8500 4850
Wire Wire Line
	8550 4750 8550 4950
Connection ~ 8550 4750
Wire Wire Line
	8550 4950 8500 4950
Connection ~ 8550 4850
$Comp
L C C?
U 1 1 5997F372
P 9250 4900
F 0 "C?" H 9365 4946 50  0000 L CNN
F 1 "22u" H 9365 4855 50  0000 L CNN
F 2 "" H 9288 4750 50  0001 C CNN
F 3 "" H 9250 4900 50  0001 C CNN
	1    9250 4900
	1    0    0    -1  
$EndComp
Wire Wire Line
	8850 4750 10350 4750
$Comp
L GND #PWR?
U 1 1 5997F4A8
P 9250 5050
F 0 "#PWR?" H 9250 4800 50  0001 C CNN
F 1 "GND" H 9255 4877 50  0000 C CNN
F 2 "" H 9250 5050 50  0001 C CNN
F 3 "" H 9250 5050 50  0001 C CNN
	1    9250 5050
	1    0    0    -1  
$EndComp
Connection ~ 9250 4750
Wire Wire Line
	8500 5150 8950 5150
Wire Wire Line
	8950 5150 8950 4750
Connection ~ 8950 4750
NoConn ~ 8500 5250
$Comp
L GND #PWR?
U 1 1 59980B55
P 8500 5350
F 0 "#PWR?" H 8500 5100 50  0001 C CNN
F 1 "GND" H 8505 5177 50  0000 C CNN
F 2 "" H 8500 5350 50  0001 C CNN
F 3 "" H 8500 5350 50  0001 C CNN
	1    8500 5350
	1    0    0    -1  
$EndComp
Text Label 6300 4750 2    60   ~ 0
PYRO_POWER
$Comp
L C C?
U 1 1 59981AB3
P 6400 4900
F 0 "C?" H 6515 4946 50  0000 L CNN
F 1 "10u" H 6515 4855 50  0000 L CNN
F 2 "" H 6438 4750 50  0001 C CNN
F 3 "" H 6400 4900 50  0001 C CNN
	1    6400 4900
	1    0    0    -1  
$EndComp
$Comp
L C C?
U 1 1 59981BD5
P 6750 4900
F 0 "C?" H 6865 4946 50  0000 L CNN
F 1 "10u" H 6865 4855 50  0000 L CNN
F 2 "" H 6788 4750 50  0001 C CNN
F 3 "" H 6750 4900 50  0001 C CNN
	1    6750 4900
	1    0    0    -1  
$EndComp
$Comp
L C C?
U 1 1 59981CC4
P 7150 5100
F 0 "C?" H 7265 5146 50  0000 L CNN
F 1 "100n" H 7265 5055 50  0000 L CNN
F 2 "" H 7188 4950 50  0001 C CNN
F 3 "" H 7150 5100 50  0001 C CNN
	1    7150 5100
	1    0    0    -1  
$EndComp
Wire Wire Line
	6300 4750 7700 4750
Connection ~ 6400 4750
Connection ~ 6750 4750
$Comp
L GND #PWR?
U 1 1 59981E1B
P 6400 5050
F 0 "#PWR?" H 6400 4800 50  0001 C CNN
F 1 "GND" H 6405 4877 50  0000 C CNN
F 2 "" H 6400 5050 50  0001 C CNN
F 3 "" H 6400 5050 50  0001 C CNN
	1    6400 5050
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR?
U 1 1 59981E6B
P 6750 5050
F 0 "#PWR?" H 6750 4800 50  0001 C CNN
F 1 "GND" H 6755 4877 50  0000 C CNN
F 2 "" H 6750 5050 50  0001 C CNN
F 3 "" H 6750 5050 50  0001 C CNN
	1    6750 5050
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR?
U 1 1 59981EB4
P 7150 5250
F 0 "#PWR?" H 7150 5000 50  0001 C CNN
F 1 "GND" H 7155 5077 50  0000 C CNN
F 2 "" H 7150 5250 50  0001 C CNN
F 3 "" H 7150 5250 50  0001 C CNN
	1    7150 5250
	1    0    0    -1  
$EndComp
Wire Wire Line
	7150 4750 7150 4950
Connection ~ 7150 4750
Connection ~ 7150 4950
Wire Wire Line
	7150 4950 7700 4950
Wire Wire Line
	7600 4750 7600 4850
Wire Wire Line
	7600 4850 7700 4850
Connection ~ 7600 4750
Text Label 7700 5050 2    60   ~ 0
FIRE
Text Label 5500 3200 2    60   ~ 0
FIRE
NoConn ~ 7700 5250
$Comp
L GND #PWR?
U 1 1 599840FE
P 7600 6000
F 0 "#PWR?" H 7600 5750 50  0001 C CNN
F 1 "GND" H 7605 5827 50  0000 C CNN
F 2 "" H 7600 6000 50  0001 C CNN
F 3 "" H 7600 6000 50  0001 C CNN
	1    7600 6000
	1    0    0    -1  
$EndComp
Wire Wire Line
	7600 5450 7600 6000
Wire Wire Line
	7600 5950 7700 5950
Wire Wire Line
	7600 5850 7700 5850
Connection ~ 7600 5950
Wire Wire Line
	7600 5750 7700 5750
Connection ~ 7600 5850
Wire Wire Line
	7600 5650 7700 5650
Connection ~ 7600 5750
Wire Wire Line
	7600 5550 7700 5550
Connection ~ 7600 5650
Wire Wire Line
	7600 5450 7700 5450
Connection ~ 7600 5550
$Comp
L GND #PWR?
U 1 1 5998481A
P 10350 4850
F 0 "#PWR?" H 10350 4600 50  0001 C CNN
F 1 "GND" H 10355 4677 50  0000 C CNN
F 2 "" H 10350 4850 50  0001 C CNN
F 3 "" H 10350 4850 50  0001 C CNN
	1    10350 4850
	1    0    0    -1  
$EndComp
Wire Wire Line
	9750 4750 9750 6000
Connection ~ 9750 4750
Wire Wire Line
	9700 6000 9800 6000
Connection ~ 9750 6000
Wire Wire Line
	9250 6000 9400 6000
Wire Wire Line
	10100 6000 10350 6000
$Comp
L +3V3 #PWR?
U 1 1 59985B71
P 9300 5700
F 0 "#PWR?" H 9300 5550 50  0001 C CNN
F 1 "+3V3" H 9315 5873 50  0000 C CNN
F 2 "" H 9300 5700 50  0001 C CNN
F 3 "" H 9300 5700 50  0001 C CNN
	1    9300 5700
	1    0    0    -1  
$EndComp
$Comp
L +3V3 #PWR?
U 1 1 59985BF0
P 10200 5700
F 0 "#PWR?" H 10200 5550 50  0001 C CNN
F 1 "+3V3" H 10215 5873 50  0000 C CNN
F 2 "" H 10200 5700 50  0001 C CNN
F 3 "" H 10200 5700 50  0001 C CNN
	1    10200 5700
	1    0    0    -1  
$EndComp
Text Label 9250 6000 2    60   ~ 0
CONT_EN
Connection ~ 9300 6000
Text Label 5550 3350 2    60   ~ 0
CONT_EN
Text Label 10350 6000 0    60   ~ 0
CONT
Connection ~ 10200 6000
Text Label 5300 3500 0    60   ~ 0
CONT
$Comp
L D_Schottky D?
U 1 1 59987172
P 10200 5850
F 0 "D?" V 10154 5929 50  0000 L CNN
F 1 "PMEG2005AEA" V 10245 5929 50  0001 L CNN
F 2 "agg:SOD-323" H 10200 5850 50  0001 C CNN
F 3 "http://www.farnell.com/datasheets/684565.pdf" H 10200 5850 50  0001 C CNN
F 4 "8737843" H 10200 5850 60  0001 C CNN "Farnell"
	1    10200 5850
	0    1    1    0   
$EndComp
$Comp
L C C?
U 1 1 599892AB
P 8650 1100
F 0 "C?" H 8765 1146 50  0000 L CNN
F 1 "100n" H 8765 1055 50  0000 L CNN
F 2 "" H 8688 950 50  0001 C CNN
F 3 "" H 8650 1100 50  0001 C CNN
	1    8650 1100
	1    0    0    -1  
$EndComp
$Comp
L C C?
U 1 1 599895DF
P 8250 1100
F 0 "C?" H 8365 1146 50  0000 L CNN
F 1 "10u" H 8365 1055 50  0000 L CNN
F 2 "" H 8288 950 50  0001 C CNN
F 3 "" H 8250 1100 50  0001 C CNN
	1    8250 1100
	1    0    0    -1  
$EndComp
Wire Wire Line
	8100 950  9000 950 
Wire Wire Line
	9000 950  9000 1150
Connection ~ 8650 950 
$Comp
L +3V3 #PWR?
U 1 1 59989A30
P 8100 950
F 0 "#PWR?" H 8100 800 50  0001 C CNN
F 1 "+3V3" H 8115 1123 50  0000 C CNN
F 2 "" H 8100 950 50  0001 C CNN
F 3 "" H 8100 950 50  0001 C CNN
	1    8100 950 
	1    0    0    -1  
$EndComp
Connection ~ 8250 950 
Wire Wire Line
	8100 1250 9000 1250
Wire Wire Line
	8950 1250 8950 1450
Wire Wire Line
	8950 1350 9000 1350
Connection ~ 8950 1250
Wire Wire Line
	8950 1450 9000 1450
Connection ~ 8950 1350
Connection ~ 8650 1250
Connection ~ 8250 1250
$Comp
L GND #PWR?
U 1 1 5998A0B7
P 8100 1250
F 0 "#PWR?" H 8100 1000 50  0001 C CNN
F 1 "GND" H 8105 1077 50  0000 C CNN
F 2 "" H 8100 1250 50  0001 C CNN
F 3 "" H 8100 1250 50  0001 C CNN
	1    8100 1250
	1    0    0    -1  
$EndComp
Text Label 9000 1650 2    60   ~ 0
RFM_MISO
Text Label 9000 1750 2    60   ~ 0
RFM_MOSI
Text Label 9000 1850 2    60   ~ 0
RFM_SCK
Text Label 9000 1950 2    60   ~ 0
RFM_NSS
Text Label 9000 2050 2    60   ~ 0
RFM_RESET
NoConn ~ 10200 1150
NoConn ~ 10200 1250
NoConn ~ 10200 1350
NoConn ~ 10200 1450
NoConn ~ 10200 1550
NoConn ~ 10200 1650
Wire Wire Line
	10200 2050 10700 2050
Connection ~ 10350 2050
$Comp
L GND #PWR?
U 1 1 5998B464
P 10700 2250
F 0 "#PWR?" H 10700 2000 50  0001 C CNN
F 1 "GND" H 10705 2077 50  0000 C CNN
F 2 "" H 10700 2250 50  0001 C CNN
F 3 "" H 10700 2250 50  0001 C CNN
	1    10700 2250
	1    0    0    -1  
$EndComp
Wire Wire Line
	10700 1900 10700 2250
Connection ~ 10700 2200
Connection ~ 10700 2150
Connection ~ 10700 1950
$Comp
L GND #PWR?
U 1 1 5998B869
P 10350 2250
F 0 "#PWR?" H 10350 2000 50  0001 C CNN
F 1 "GND" H 10355 2077 50  0000 C CNN
F 2 "" H 10350 2250 50  0001 C CNN
F 3 "" H 10350 2250 50  0001 C CNN
	1    10350 2250
	1    0    0    -1  
$EndComp
Text Label 5650 2450 2    60   ~ 0
RFM_MISO
Text Label 5650 2550 2    60   ~ 0
RFM_MOSI
Text Label 5650 2650 2    60   ~ 0
RFM_SCK
Text Label 5650 2750 2    60   ~ 0
RFM_NSS
Text Label 5650 2850 2    60   ~ 0
RFM_RESET
$Comp
L ESD_DIODE D?
U 1 1 5998CFE8
P 10000 4850
F 0 "D?" V 10000 4908 50  0000 L CNN
F 1 "ESD_DIODE" H 9950 4750 50  0001 L CNN
F 2 "agg:0603" H 9950 4650 50  0001 L CNN
F 3 "" H 9900 4850 50  0001 C CNN
	1    10000 4850
	0    1    1    0   
$EndComp
$Comp
L GND #PWR?
U 1 1 5998DA91
P 10000 4950
F 0 "#PWR?" H 10000 4700 50  0001 C CNN
F 1 "GND" H 10005 4777 50  0000 C CNN
F 2 "" H 10000 4950 50  0001 C CNN
F 3 "" H 10000 4950 50  0001 C CNN
	1    10000 4950
	1    0    0    -1  
$EndComp
Connection ~ 10000 4750
$Comp
L +3V3 #PWR?
U 1 1 5998E71C
P 3700 2000
F 0 "#PWR?" H 3700 1850 50  0001 C CNN
F 1 "+3V3" H 3715 2173 50  0000 C CNN
F 2 "" H 3700 2000 50  0001 C CNN
F 3 "" H 3700 2000 50  0001 C CNN
	1    3700 2000
	1    0    0    -1  
$EndComp
Wire Wire Line
	3700 2000 3700 2250
Wire Wire Line
	3700 2050 3800 2050
Wire Wire Line
	3700 2150 3800 2150
Connection ~ 3700 2050
Wire Wire Line
	3700 2250 3800 2250
Connection ~ 3700 2150
$Comp
L GND #PWR?
U 1 1 5998ED76
P 3600 2350
F 0 "#PWR?" H 3600 2100 50  0001 C CNN
F 1 "GND" H 3605 2177 50  0000 C CNN
F 2 "" H 3600 2350 50  0001 C CNN
F 3 "" H 3600 2350 50  0001 C CNN
	1    3600 2350
	1    0    0    -1  
$EndComp
Wire Wire Line
	3600 2350 3800 2350
Wire Wire Line
	3800 2450 3750 2450
Wire Wire Line
	3750 2450 3750 2350
Connection ~ 3750 2350
$Comp
L SWD P?
U 1 1 5998F106
P 4600 1050
F 0 "P?" H 4600 1475 50  0000 C CNN
F 1 "SWD" H 4600 1384 50  0000 C CNN
F 2 "agg:FTSH-105-01-F-D-K" H 4200 650 50  0001 L CNN
F 3 "" H 4950 850 50  0001 C CNN
F 4 "FTSH-105-01-F-D-K" H 4200 550 50  0001 L CNN "Toby"
	1    4600 1050
	1    0    0    -1  
$EndComp
$Comp
L SWD_TC P?
U 1 1 5998F224
P 6450 1050
F 0 "P?" H 6450 1375 50  0000 C CNN
F 1 "SWD_TC" H 6450 1284 50  0000 C CNN
F 2 "agg:TC2030-NL" H 6150 750 50  0001 L CNN
F 3 "" H 6050 1150 50  0001 C CNN
	1    6450 1050
	1    0    0    -1  
$EndComp
$Comp
L +3V3 #PWR?
U 1 1 5998F5C9
P 4100 850
F 0 "#PWR?" H 4100 700 50  0001 C CNN
F 1 "+3V3" H 4115 1023 50  0000 C CNN
F 2 "" H 4100 850 50  0001 C CNN
F 3 "" H 4100 850 50  0001 C CNN
	1    4100 850 
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR?
U 1 1 5998F8AC
P 4000 1250
F 0 "#PWR?" H 4000 1000 50  0001 C CNN
F 1 "GND" H 4005 1077 50  0000 C CNN
F 2 "" H 4000 1250 50  0001 C CNN
F 3 "" H 4000 1250 50  0001 C CNN
	1    4000 1250
	1    0    0    -1  
$EndComp
Wire Wire Line
	4100 1250 4000 1250
Wire Wire Line
	4050 950  4050 1250
Wire Wire Line
	4050 1050 4100 1050
Connection ~ 4050 1250
Wire Wire Line
	4050 950  4100 950 
Connection ~ 4050 1050
Text Label 5100 850  0    60   ~ 0
SWDIO
Text Label 5100 950  0    60   ~ 0
SWDCLK
NoConn ~ 5100 1050
NoConn ~ 5100 1150
Text Label 5100 1250 0    60   ~ 0
nRST
Text Label 4600 3350 0    60   ~ 0
SWDIO
Text Label 4600 3450 0    60   ~ 0
SWDCLK
Text Label 3800 2650 2    60   ~ 0
nRST
Text Label 6850 950  0    60   ~ 0
SWDIO
Text Label 6850 1050 0    60   ~ 0
SWDCLK
Text Label 6050 1050 2    60   ~ 0
nRST
NoConn ~ 6850 1150
$Comp
L +3V3 #PWR?
U 1 1 59990FD8
P 6050 950
F 0 "#PWR?" H 6050 800 50  0001 C CNN
F 1 "+3V3" H 6065 1123 50  0000 C CNN
F 2 "" H 6050 950 50  0001 C CNN
F 3 "" H 6050 950 50  0001 C CNN
	1    6050 950 
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR?
U 1 1 59991393
P 6050 1150
F 0 "#PWR?" H 6050 900 50  0001 C CNN
F 1 "GND" H 6055 977 50  0000 C CNN
F 2 "" H 6050 1150 50  0001 C CNN
F 3 "" H 6050 1150 50  0001 C CNN
	1    6050 1150
	1    0    0    -1  
$EndComp
$Comp
L C C?
U 1 1 599917F3
P 1800 900
F 0 "C?" H 1915 946 50  0000 L CNN
F 1 "10n" H 1915 855 50  0000 L CNN
F 2 "" H 1838 750 50  0001 C CNN
F 3 "" H 1800 900 50  0001 C CNN
	1    1800 900 
	1    0    0    -1  
$EndComp
$Comp
L C C?
U 1 1 59991857
P 2150 900
F 0 "C?" H 2265 946 50  0000 L CNN
F 1 "1u" H 2265 855 50  0000 L CNN
F 2 "" H 2188 750 50  0001 C CNN
F 3 "" H 2150 900 50  0001 C CNN
	1    2150 900 
	1    0    0    -1  
$EndComp
Text Notes 1700 700  0    60   ~ 0
VDDA
$Comp
L C C?
U 1 1 59991E9E
P 2500 900
F 0 "C?" H 2615 946 50  0000 L CNN
F 1 "4u7" H 2615 855 50  0000 L CNN
F 2 "" H 2538 750 50  0001 C CNN
F 3 "" H 2500 900 50  0001 C CNN
	1    2500 900 
	1    0    0    -1  
$EndComp
$Comp
L C C?
U 1 1 59991F7A
P 2850 900
F 0 "C?" H 2965 946 50  0000 L CNN
F 1 "100n" H 2965 855 50  0000 L CNN
F 2 "" H 2888 750 50  0001 C CNN
F 3 "" H 2850 900 50  0001 C CNN
	1    2850 900 
	1    0    0    -1  
$EndComp
$Comp
L C C?
U 1 1 5999203E
P 3250 900
F 0 "C?" H 3365 946 50  0000 L CNN
F 1 "100n" H 3365 855 50  0000 L CNN
F 2 "" H 3288 750 50  0001 C CNN
F 3 "" H 3250 900 50  0001 C CNN
	1    3250 900 
	1    0    0    -1  
$EndComp
Text Notes 2400 700  0    60   ~ 0
VDD
Text Notes 2750 700  0    60   ~ 0
VDD
Text Notes 3150 700  0    60   ~ 0
VDD
Text Notes 2050 700  0    60   ~ 0
VDDA
$Comp
L +3V3 #PWR?
U 1 1 59992E97
P 1550 750
F 0 "#PWR?" H 1550 600 50  0001 C CNN
F 1 "+3V3" H 1565 923 50  0000 C CNN
F 2 "" H 1550 750 50  0001 C CNN
F 3 "" H 1550 750 50  0001 C CNN
	1    1550 750 
	1    0    0    -1  
$EndComp
Wire Wire Line
	1550 750  3250 750 
Connection ~ 1800 750 
Connection ~ 2150 750 
Connection ~ 2500 750 
Connection ~ 2850 750 
$Comp
L GND #PWR?
U 1 1 599932A9
P 1550 1050
F 0 "#PWR?" H 1550 800 50  0001 C CNN
F 1 "GND" H 1555 877 50  0000 C CNN
F 2 "" H 1550 1050 50  0001 C CNN
F 3 "" H 1550 1050 50  0001 C CNN
	1    1550 1050
	1    0    0    -1  
$EndComp
Wire Wire Line
	1550 1050 3250 1050
Connection ~ 1800 1050
Connection ~ 2150 1050
Connection ~ 2500 1050
Connection ~ 2850 1050
$Comp
L GND #PWR?
U 1 1 5999402F
P 3800 2750
F 0 "#PWR?" H 3800 2500 50  0001 C CNN
F 1 "GND" V 3805 2622 50  0000 R CNN
F 2 "" H 3800 2750 50  0001 C CNN
F 3 "" H 3800 2750 50  0001 C CNN
	1    3800 2750
	0    1    1    0   
$EndComp
$Comp
L R R?
U 1 1 59995FDC
P 1250 3400
F 0 "R?" H 1320 3446 50  0000 L CNN
F 1 "R" H 1320 3355 50  0000 L CNN
F 2 "" V 1180 3400 50  0001 C CNN
F 3 "" H 1250 3400 50  0001 C CNN
	1    1250 3400
	1    0    0    -1  
$EndComp
Text Label 1250 3250 2    60   ~ 0
LED_CHARGE
$Comp
L GND #PWR?
U 1 1 59996486
P 1250 3850
F 0 "#PWR?" H 1250 3600 50  0001 C CNN
F 1 "GND" H 1255 3677 50  0000 C CNN
F 2 "" H 1250 3850 50  0001 C CNN
F 3 "" H 1250 3850 50  0001 C CNN
	1    1250 3850
	1    0    0    -1  
$EndComp
$Comp
L R R?
U 1 1 59997190
P 1300 2550
F 0 "R?" H 1370 2596 50  0000 L CNN
F 1 "R" H 1370 2505 50  0000 L CNN
F 2 "" V 1230 2550 50  0001 C CNN
F 3 "" H 1300 2550 50  0001 C CNN
	1    1300 2550
	1    0    0    -1  
$EndComp
Text Label 1050 2300 2    60   ~ 0
LED_ARM
Wire Wire Line
	1050 2300 1300 2300
Wire Wire Line
	1300 2300 1300 2400
Text Label 1050 2400 2    60   ~ 0
LED_DISARM
Wire Wire Line
	1050 2400 1050 3000
Wire Wire Line
	1050 3000 1300 3000
Text Label 5700 3600 2    60   ~ 0
LED_ARM
Text Label 5700 3700 2    60   ~ 0
LED_DISARM
Text Label 5650 3800 2    60   ~ 0
LED_CHARGE
$EndSCHEMATC
