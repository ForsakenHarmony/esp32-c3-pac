#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - interrupt of WiFi MAC, level"]
    WIFI_MAC_INTR = 0,
    #[doc = "1 - interrupt of WiFi MAC, NMI, use if MAC have bug to fix in NMI"]
    WIFI_MAC_NMI = 1,
    #[doc = "2 - interrupt of WiFi BB, level, we can do some calibration"]
    WIFI_BB_INTR = 2,
    #[doc = "3 - will be cancelled"]
    BT_MAC_INTR = 3,
    #[doc = "4 - interrupt of BT BB, level"]
    BT_BB_INTR = 4,
    #[doc = "5 - interrupt of BT BB, NMI, use if BB have bug to fix in NMI"]
    BT_BB_NMI = 5,
    #[doc = "6 - interrupt of RWBT, level"]
    RWBT_INTR = 6,
    #[doc = "7 - interrupt of RWBLE, level"]
    RWBLE_INTR = 7,
    #[doc = "8 - interrupt of RWBT, NMI, use if RWBT have bug to fix in NMI"]
    RWBT_NMI = 8,
    #[doc = "9 - interrupt of RWBLE, NMI, use if RWBT have bug to fix in NMI"]
    RWBLE_NMI = 9,
    #[doc = "12 - interrupt of UHCI0, level"]
    UHCI0_INTR = 12,
    #[doc = "13 - interrupt of UHCI1, level"]
    UHCI1_INTR = 13,
    #[doc = "14 - interrupt of TIMER_GROUP0, TIMER0, level, we would like use EDGE for timer if permission"]
    TG0_T0_LEVEL_INTR = 14,
    #[doc = "15 - interrupt of TIMER_GROUP0, TIMER1, level, we would like use EDGE for timer if permission"]
    TG0_T1_LEVEL_INTR = 15,
    #[doc = "16 - interrupt of TIMER_GROUP0, WATCHDOG, level"]
    TG0_WDT_LEVEL_INTR = 16,
    #[doc = "17 - interrupt of TIMER_GROUP0, LACT, level"]
    TG0_LACT_LEVEL_INTR = 17,
    #[doc = "18 - interrupt of TIMER_GROUP1, TIMER0, level, we would like use EDGE for timer if permission"]
    TG1_T0_LEVEL_INTR = 18,
    #[doc = "19 - interrupt of TIMER_GROUP1, TIMER1, level, we would like use EDGE for timer if permission"]
    TG1_T1_LEVEL_INTR = 19,
    #[doc = "20 - interrupt of TIMER_GROUP1, WATCHDOG, level"]
    TG1_WDT_LEVEL_INTR = 20,
    #[doc = "21 - interrupt of TIMER_GROUP1, LACT, level"]
    TG1_LACT_LEVEL_INTR = 21,
    #[doc = "22 - interrupt of GPIO, level"]
    GPIO_INTR = 22,
    #[doc = "23 - interrupt of GPIO, NMI"]
    GPIO_NMI = 23,
    #[doc = "28 - interrupt of SPI0, level, SPI0 is for Cache Access, do not use this"]
    SPI0_INTR = 28,
    #[doc = "29 - interrupt of SPI1, level, SPI1 is for flash read/write, do not use this"]
    SPI1_INTR = 29,
    #[doc = "30 - interrupt of SPI2, level"]
    SPI2_INTR = 30,
    #[doc = "31 - interrupt of SPI3, level"]
    SPI3_INTR = 31,
    #[doc = "32 - interrupt of I2S0, level"]
    I2S0_INTR = 32,
    #[doc = "33 - interrupt of I2S1, level"]
    I2S1_INTR = 33,
    #[doc = "34 - interrupt of UART0, level"]
    UART0_INTR = 34,
    #[doc = "35 - interrupt of UART1, level"]
    UART1_INTR = 35,
    #[doc = "36 - interrupt of UART2, level"]
    UART2_INTR = 36,
    #[doc = "37 - interrupt of SD/SDIO/MMC HOST, level"]
    SDIO_HOST_INTR = 37,
    #[doc = "38 - interrupt of ethernet mac, level"]
    ETH_MAC_INTR = 38,
    #[doc = "39 - interrupt of PWM0, level, Reserved"]
    PWM0_INTR = 39,
    #[doc = "40 - interrupt of PWM1, level, Reserved"]
    PWM1_INTR = 40,
    #[doc = "41 - interrupt of PWM2, level"]
    PWM2_INTR = 41,
    #[doc = "42 - interrupt of PWM3, level"]
    PWM3_INTR = 42,
    #[doc = "43 - interrupt of LED PWM, level"]
    LEDC_INTR = 43,
    #[doc = "44 - interrupt of efuse, level, not likely to use"]
    EFUSE_INTR = 44,
    #[doc = "46 - interrupt of rtc core, level, include rtc watchdog"]
    RTC_CORE_INTR = 46,
    #[doc = "47 - interrupt of remote controller, level"]
    RMT_INTR = 47,
    #[doc = "49 - interrupt of I2C controller0, level"]
    I2C_EXT0_INTR = 49,
    #[doc = "50 - interrupt of I2C controller1, level"]
    I2C_EXT1_INTR = 50,
    #[doc = "51 - interrupt of RSA accelerator, level"]
    RSA_INTR = 51,
    #[doc = "52 - interrupt of SPI1 DMA, SPI1 is for flash read/write, do not use this"]
    SPI1_DMA_INTR = 52,
    #[doc = "53 - interrupt of SPI2 DMA, level"]
    SPI2_DMA_INTR = 53,
    #[doc = "54 - interrupt of SPI3 DMA, level"]
    SPI3_DMA_INTR = 54,
    #[doc = "55 - will be cancelled"]
    WDT_INTR = 55,
    #[doc = "58 - interrupt of TIMER_GROUP0, TIMER0, EDGE"]
    TG0_T0_EDGE_INTR = 58,
    #[doc = "59 - interrupt of TIMER_GROUP0, TIMER1, EDGE"]
    TG0_T1_EDGE_INTR = 59,
    #[doc = "60 - interrupt of TIMER_GROUP0, WATCH DOG, EDGE"]
    TG0_WDT_EDGE_INTR = 60,
    #[doc = "61 - interrupt of TIMER_GROUP0, LACT, EDGE"]
    TG0_LACT_EDGE_INTR = 61,
    #[doc = "62 - interrupt of TIMER_GROUP1, TIMER0, EDGE"]
    TG1_T0_EDGE_INTR = 62,
    #[doc = "63 - interrupt of TIMER_GROUP1, TIMER1, EDGE"]
    TG1_T1_EDGE_INTR = 63,
    #[doc = "64 - interrupt of TIMER_GROUP1, WATCHDOG, EDGE"]
    TG1_WDT_EDGE_INTR = 64,
    #[doc = "65 - interrupt of TIMER_GROUP0, LACT, EDGE"]
    TG1_LACT_EDGE_INTR = 65,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            0 => Ok(Interrupt::WIFI_MAC_INTR),
            1 => Ok(Interrupt::WIFI_MAC_NMI),
            2 => Ok(Interrupt::WIFI_BB_INTR),
            3 => Ok(Interrupt::BT_MAC_INTR),
            4 => Ok(Interrupt::BT_BB_INTR),
            5 => Ok(Interrupt::BT_BB_NMI),
            6 => Ok(Interrupt::RWBT_INTR),
            7 => Ok(Interrupt::RWBLE_INTR),
            8 => Ok(Interrupt::RWBT_NMI),
            9 => Ok(Interrupt::RWBLE_NMI),
            12 => Ok(Interrupt::UHCI0_INTR),
            13 => Ok(Interrupt::UHCI1_INTR),
            14 => Ok(Interrupt::TG0_T0_LEVEL_INTR),
            15 => Ok(Interrupt::TG0_T1_LEVEL_INTR),
            16 => Ok(Interrupt::TG0_WDT_LEVEL_INTR),
            17 => Ok(Interrupt::TG0_LACT_LEVEL_INTR),
            18 => Ok(Interrupt::TG1_T0_LEVEL_INTR),
            19 => Ok(Interrupt::TG1_T1_LEVEL_INTR),
            20 => Ok(Interrupt::TG1_WDT_LEVEL_INTR),
            21 => Ok(Interrupt::TG1_LACT_LEVEL_INTR),
            22 => Ok(Interrupt::GPIO_INTR),
            23 => Ok(Interrupt::GPIO_NMI),
            28 => Ok(Interrupt::SPI0_INTR),
            29 => Ok(Interrupt::SPI1_INTR),
            30 => Ok(Interrupt::SPI2_INTR),
            31 => Ok(Interrupt::SPI3_INTR),
            32 => Ok(Interrupt::I2S0_INTR),
            33 => Ok(Interrupt::I2S1_INTR),
            34 => Ok(Interrupt::UART0_INTR),
            35 => Ok(Interrupt::UART1_INTR),
            36 => Ok(Interrupt::UART2_INTR),
            37 => Ok(Interrupt::SDIO_HOST_INTR),
            38 => Ok(Interrupt::ETH_MAC_INTR),
            39 => Ok(Interrupt::PWM0_INTR),
            40 => Ok(Interrupt::PWM1_INTR),
            41 => Ok(Interrupt::PWM2_INTR),
            42 => Ok(Interrupt::PWM3_INTR),
            43 => Ok(Interrupt::LEDC_INTR),
            44 => Ok(Interrupt::EFUSE_INTR),
            46 => Ok(Interrupt::RTC_CORE_INTR),
            47 => Ok(Interrupt::RMT_INTR),
            49 => Ok(Interrupt::I2C_EXT0_INTR),
            50 => Ok(Interrupt::I2C_EXT1_INTR),
            51 => Ok(Interrupt::RSA_INTR),
            52 => Ok(Interrupt::SPI1_DMA_INTR),
            53 => Ok(Interrupt::SPI2_DMA_INTR),
            54 => Ok(Interrupt::SPI3_DMA_INTR),
            55 => Ok(Interrupt::WDT_INTR),
            58 => Ok(Interrupt::TG0_T0_EDGE_INTR),
            59 => Ok(Interrupt::TG0_T1_EDGE_INTR),
            60 => Ok(Interrupt::TG0_WDT_EDGE_INTR),
            61 => Ok(Interrupt::TG0_LACT_EDGE_INTR),
            62 => Ok(Interrupt::TG1_T0_EDGE_INTR),
            63 => Ok(Interrupt::TG1_T1_EDGE_INTR),
            64 => Ok(Interrupt::TG1_WDT_EDGE_INTR),
            65 => Ok(Interrupt::TG1_LACT_EDGE_INTR),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
