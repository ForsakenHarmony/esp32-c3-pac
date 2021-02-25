#[doc = "Reader of register INT_ST_TIMERS"]
pub type R = crate::R<u32, super::INT_ST_TIMERS>;
#[doc = "Reader of field `WDT_INT_ST`"]
pub type WDT_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `T0_INT_ST`"]
pub type T0_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wdt_int_st(&self) -> WDT_INT_ST_R {
        WDT_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn t0_int_st(&self) -> T0_INT_ST_R {
        T0_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
