#[doc = "Reader of register INT_RAW"]
pub type R = crate::R<u32, super::INT_RAW>;
#[doc = "Reader of field `BBPLL_CAL_INT_RAW`"]
pub type BBPLL_CAL_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `GLITCH_DET_INT_RAW`"]
pub type GLITCH_DET_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `XTAL32K_DEAD_INT_RAW`"]
pub type XTAL32K_DEAD_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWD_INT_RAW`"]
pub type SWD_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAIN_TIMER_INT_RAW`"]
pub type MAIN_TIMER_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `BROWN_OUT_INT_RAW`"]
pub type BROWN_OUT_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDT_INT_RAW`"]
pub type WDT_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLP_REJECT_INT_RAW`"]
pub type SLP_REJECT_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLP_WAKEUP_INT_RAW`"]
pub type SLP_WAKEUP_INT_RAW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn bbpll_cal_int_raw(&self) -> BBPLL_CAL_INT_RAW_R {
        BBPLL_CAL_INT_RAW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn glitch_det_int_raw(&self) -> GLITCH_DET_INT_RAW_R {
        GLITCH_DET_INT_RAW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn xtal32k_dead_int_raw(&self) -> XTAL32K_DEAD_INT_RAW_R {
        XTAL32K_DEAD_INT_RAW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn swd_int_raw(&self) -> SWD_INT_RAW_R {
        SWD_INT_RAW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn main_timer_int_raw(&self) -> MAIN_TIMER_INT_RAW_R {
        MAIN_TIMER_INT_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn brown_out_int_raw(&self) -> BROWN_OUT_INT_RAW_R {
        BROWN_OUT_INT_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wdt_int_raw(&self) -> WDT_INT_RAW_R {
        WDT_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slp_reject_int_raw(&self) -> SLP_REJECT_INT_RAW_R {
        SLP_REJECT_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slp_wakeup_int_raw(&self) -> SLP_WAKEUP_INT_RAW_R {
        SLP_WAKEUP_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
