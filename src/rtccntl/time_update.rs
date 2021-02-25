#[doc = "Reader of register TIME_UPDATE"]
pub type R = crate::R<u32, super::TIME_UPDATE>;
#[doc = "Writer for register TIME_UPDATE"]
pub type W = crate::W<u32, super::TIME_UPDATE>;
#[doc = "Register TIME_UPDATE `reset()`'s with value 0"]
impl crate::ResetValue for super::TIME_UPDATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TIME_UPDATE`"]
pub struct TIME_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_UPDATE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `TIMER_SYS_RST`"]
pub type TIMER_SYS_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_SYS_RST`"]
pub struct TIMER_SYS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_SYS_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `TIMER_XTL_OFF`"]
pub type TIMER_XTL_OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_XTL_OFF`"]
pub struct TIMER_XTL_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_XTL_OFF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `TIMER_SYS_STALL`"]
pub type TIMER_SYS_STALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER_SYS_STALL`"]
pub struct TIMER_SYS_STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_SYS_STALL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn timer_sys_rst(&self) -> TIMER_SYS_RST_R {
        TIMER_SYS_RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn timer_xtl_off(&self) -> TIMER_XTL_OFF_R {
        TIMER_XTL_OFF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn timer_sys_stall(&self) -> TIMER_SYS_STALL_R {
        TIMER_SYS_STALL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn time_update(&mut self) -> TIME_UPDATE_W {
        TIME_UPDATE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn timer_sys_rst(&mut self) -> TIMER_SYS_RST_W {
        TIMER_SYS_RST_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn timer_xtl_off(&mut self) -> TIMER_XTL_OFF_W {
        TIMER_XTL_OFF_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn timer_sys_stall(&mut self) -> TIMER_SYS_STALL_W {
        TIMER_SYS_STALL_W { w: self }
    }
}
