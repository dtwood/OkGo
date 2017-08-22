use f0;
use stm32f0xx;
use bare_metal::CriticalSection;
use lcd;

output!(LED_GREEN, B, 12);
output!(LED_YELLOW, A, 11);

output!(LED_ARM, A, 8);
output!(LED_DISARM, A, 9);

input!(SW_KEY, A, 9);

output!(LED_CH1, B, 0);
output!(LED_CH2, B, 10);
output!(LED_CH3, B, 12);
output!(LED_CH4, B, 14);

input!(SW_CH1, B, 2);
input!(SW_CH2, B, 11);
input!(SW_CH3, B, 13);
input!(SW_CH4, B, 15);

adc!(ADC);
adc_pin!(BATT_MON, ADC, B, 1, 9);


pub fn init(cs: &CriticalSection) {
    // Clock all GPIO peripherals
    let rcc = stm32f0xx::RCC.borrow(cs);
    rcc.ahbenr.write(|w| {
        w
        // GPIO A
        .iopaen().set_bit()
        // GPIO B
        .iopben().set_bit()
        // GPIO C
        .iopcen().set_bit()
    });
    rcc.apb2enr.write(|w| {
        w
        // ADC
        .adcen().set_bit()
    });

    // Debug LEDs.  Default off
    LED_GREEN.clear(cs);
    LED_YELLOW.clear(cs);
    LED_GREEN.setup(cs);
    LED_YELLOW.setup(cs);

    // Keyswitch
    SW_KEY.setup(cs, f0::gpio::PullUpDown::None);

    // Arm/disarm LED.  Default off
    LED_ARM.clear(cs);
    LED_DISARM.clear(cs);
    LED_ARM.setup(cs);
    LED_DISARM.set(cs);

    // Channel switches and LEDs.  Default LEDs off
    LED_CH1.clear(cs);
    LED_CH2.clear(cs);
    LED_CH3.clear(cs);
    LED_CH4.clear(cs);
    LED_CH1.setup(cs);
    LED_CH2.setup(cs);
    LED_CH3.setup(cs);
    LED_CH3.setup(cs);
    SW_CH1.setup(cs, f0::gpio::PullUpDown::None);
    SW_CH2.setup(cs, f0::gpio::PullUpDown::None);
    SW_CH3.setup(cs, f0::gpio::PullUpDown::None);
    SW_CH4.setup(cs, f0::gpio::PullUpDown::None);

    // Analog inputs
    ADC.setup(cs);
    BATT_MON.setup(cs);

    // Initialise display
    lcd::init(cs);
}


// // LCD
// #define LCD_RS          GPIO15
// #define LCD_RS_PORT     GPIOA
// #define LCD_E           GPIO3
// #define LCD_E_PORT      GPIOB
// #define LCD_DB4         GPIO4
// #define LCD_DB4_PORT    GPIOB
// #define LCD_DB5         GPIO5
// #define LCD_DB5_PORT    GPIOB
// #define LCD_DB6         GPIO6
// #define LCD_DB6_PORT    GPIOB
// #define LCD_DB7         GPIO7
// #define LCD_DB7_PORT    GPIOB
