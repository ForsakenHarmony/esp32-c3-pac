#[doc = "Reader of register LOW_POWER_ST"]
pub type R = crate::R<u32, super::LOW_POWER_ST>;
#[doc = "Reader of field `MAIN_STATE`"]
pub type MAIN_STATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAIN_STATE_IN_IDLE`"]
pub type MAIN_STATE_IN_IDLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAIN_STATE_IN_SLP`"]
pub type MAIN_STATE_IN_SLP_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAIN_STATE_IN_WAIT_XTL`"]
pub type MAIN_STATE_IN_WAIT_XTL_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAIN_STATE_IN_WAIT_PLL`"]
pub type MAIN_STATE_IN_WAIT_PLL_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAIN_STATE_IN_WAIT_8M`"]
pub type MAIN_STATE_IN_WAIT_8M_R = crate::R<bool, bool>;
#[doc = "Reader of field `IN_LOW_POWER_STATE`"]
pub type IN_LOW_POWER_STATE_R = crate::R<bool, bool>;
#[doc = "Reader of field `IN_WAKEUP_STATE`"]
pub type IN_WAKEUP_STATE_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAIN_STATE_WAIT_END`"]
pub type MAIN_STATE_WAIT_END_R = crate::R<bool, bool>;
#[doc = "Reader of field `RDY_FOR_WAKEUP`"]
pub type RDY_FOR_WAKEUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAIN_STATE_PLL_ON`"]
pub type MAIN_STATE_PLL_ON_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAIN_STATE_XTAL_ISO`"]
pub type MAIN_STATE_XTAL_ISO_R = crate::R<bool, bool>;
#[doc = "Reader of field `COCPU_STATE_DONE`"]
pub type COCPU_STATE_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `COCPU_STATE_SLP`"]
pub type COCPU_STATE_SLP_R = crate::R<bool, bool>;
#[doc = "Reader of field `COCPU_STATE_SWITCH`"]
pub type COCPU_STATE_SWITCH_R = crate::R<bool, bool>;
#[doc = "Reader of field `COCPU_STATE_START`"]
pub type COCPU_STATE_START_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOUCH_STATE_DONE`"]
pub type TOUCH_STATE_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOUCH_STATE_SLP`"]
pub type TOUCH_STATE_SLP_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOUCH_STATE_SWITCH`"]
pub type TOUCH_STATE_SWITCH_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOUCH_STATE_START`"]
pub type TOUCH_STATE_START_R = crate::R<bool, bool>;
#[doc = "Reader of field `XPD_DIG`"]
pub type XPD_DIG_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIG_ISO`"]
pub type DIG_ISO_R = crate::R<bool, bool>;
#[doc = "Reader of field `XPD_WIFI`"]
pub type XPD_WIFI_R = crate::R<bool, bool>;
#[doc = "Reader of field `WIFI_ISO`"]
pub type WIFI_ISO_R = crate::R<bool, bool>;
#[doc = "Reader of field `XPD_RTC_PERI`"]
pub type XPD_RTC_PERI_R = crate::R<bool, bool>;
#[doc = "Reader of field `PERI_ISO`"]
pub type PERI_ISO_R = crate::R<bool, bool>;
#[doc = "Reader of field `XPD_DIG_DCDC`"]
pub type XPD_DIG_DCDC_R = crate::R<bool, bool>;
#[doc = "Reader of field `XPD_ROM0`"]
pub type XPD_ROM0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn main_state(&self) -> MAIN_STATE_R {
        MAIN_STATE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn main_state_in_idle(&self) -> MAIN_STATE_IN_IDLE_R {
        MAIN_STATE_IN_IDLE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn main_state_in_slp(&self) -> MAIN_STATE_IN_SLP_R {
        MAIN_STATE_IN_SLP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn main_state_in_wait_xtl(&self) -> MAIN_STATE_IN_WAIT_XTL_R {
        MAIN_STATE_IN_WAIT_XTL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn main_state_in_wait_pll(&self) -> MAIN_STATE_IN_WAIT_PLL_R {
        MAIN_STATE_IN_WAIT_PLL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn main_state_in_wait_8m(&self) -> MAIN_STATE_IN_WAIT_8M_R {
        MAIN_STATE_IN_WAIT_8M_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn in_low_power_state(&self) -> IN_LOW_POWER_STATE_R {
        IN_LOW_POWER_STATE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn in_wakeup_state(&self) -> IN_WAKEUP_STATE_R {
        IN_WAKEUP_STATE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn main_state_wait_end(&self) -> MAIN_STATE_WAIT_END_R {
        MAIN_STATE_WAIT_END_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rdy_for_wakeup(&self) -> RDY_FOR_WAKEUP_R {
        RDY_FOR_WAKEUP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn main_state_pll_on(&self) -> MAIN_STATE_PLL_ON_R {
        MAIN_STATE_PLL_ON_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn main_state_xtal_iso(&self) -> MAIN_STATE_XTAL_ISO_R {
        MAIN_STATE_XTAL_ISO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cocpu_state_done(&self) -> COCPU_STATE_DONE_R {
        COCPU_STATE_DONE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cocpu_state_slp(&self) -> COCPU_STATE_SLP_R {
        COCPU_STATE_SLP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cocpu_state_switch(&self) -> COCPU_STATE_SWITCH_R {
        COCPU_STATE_SWITCH_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cocpu_state_start(&self) -> COCPU_STATE_START_R {
        COCPU_STATE_START_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn touch_state_done(&self) -> TOUCH_STATE_DONE_R {
        TOUCH_STATE_DONE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn touch_state_slp(&self) -> TOUCH_STATE_SLP_R {
        TOUCH_STATE_SLP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn touch_state_switch(&self) -> TOUCH_STATE_SWITCH_R {
        TOUCH_STATE_SWITCH_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn touch_state_start(&self) -> TOUCH_STATE_START_R {
        TOUCH_STATE_START_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn xpd_dig(&self) -> XPD_DIG_R {
        XPD_DIG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dig_iso(&self) -> DIG_ISO_R {
        DIG_ISO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn xpd_wifi(&self) -> XPD_WIFI_R {
        XPD_WIFI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn wifi_iso(&self) -> WIFI_ISO_R {
        WIFI_ISO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn xpd_rtc_peri(&self) -> XPD_RTC_PERI_R {
        XPD_RTC_PERI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn peri_iso(&self) -> PERI_ISO_R {
        PERI_ISO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xpd_dig_dcdc(&self) -> XPD_DIG_DCDC_R {
        XPD_DIG_DCDC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xpd_rom0(&self) -> XPD_ROM0_R {
        XPD_ROM0_R::new((self.bits & 0x01) != 0)
    }
}
