#[doc = "Reader of register SWD_CONF"]
pub type R = crate::R<u32, super::SWD_CONF>;
#[doc = "Writer for register SWD_CONF"]
pub type W = crate::W<u32, super::SWD_CONF>;
#[doc = "Register SWD_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SWD_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWD_AUTO_FEED_EN`"]
pub type SWD_AUTO_FEED_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWD_AUTO_FEED_EN`"]
pub struct SWD_AUTO_FEED_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_AUTO_FEED_EN_W<'a> {
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
#[doc = "Reader of field `SWD_DISABLE`"]
pub type SWD_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWD_DISABLE`"]
pub struct SWD_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Write proxy for field `SWD_FEED`"]
pub struct SWD_FEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_FEED_W<'a> {
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
#[doc = "Write proxy for field `SWD_RST_FLAG_CLR`"]
pub struct SWD_RST_FLAG_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_RST_FLAG_CLR_W<'a> {
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
#[doc = "Reader of field `SWD_SIGNAL_WIDTH`"]
pub type SWD_SIGNAL_WIDTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SWD_SIGNAL_WIDTH`"]
pub struct SWD_SIGNAL_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_SIGNAL_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 18)) | (((value as u32) & 0x03ff) << 18);
        self.w
    }
}
#[doc = "Reader of field `SWD_BYPASS_RST`"]
pub type SWD_BYPASS_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWD_BYPASS_RST`"]
pub struct SWD_BYPASS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_BYPASS_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `SWD_FEED_INT`"]
pub type SWD_FEED_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWD_RESET_FLAG`"]
pub type SWD_RESET_FLAG_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn swd_auto_feed_en(&self) -> SWD_AUTO_FEED_EN_R {
        SWD_AUTO_FEED_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn swd_disable(&self) -> SWD_DISABLE_R {
        SWD_DISABLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 18:27"]
    #[inline(always)]
    pub fn swd_signal_width(&self) -> SWD_SIGNAL_WIDTH_R {
        SWD_SIGNAL_WIDTH_R::new(((self.bits >> 18) & 0x03ff) as u16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn swd_bypass_rst(&self) -> SWD_BYPASS_RST_R {
        SWD_BYPASS_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swd_feed_int(&self) -> SWD_FEED_INT_R {
        SWD_FEED_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swd_reset_flag(&self) -> SWD_RESET_FLAG_R {
        SWD_RESET_FLAG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn swd_auto_feed_en(&mut self) -> SWD_AUTO_FEED_EN_W {
        SWD_AUTO_FEED_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn swd_disable(&mut self) -> SWD_DISABLE_W {
        SWD_DISABLE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn swd_feed(&mut self) -> SWD_FEED_W {
        SWD_FEED_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn swd_rst_flag_clr(&mut self) -> SWD_RST_FLAG_CLR_W {
        SWD_RST_FLAG_CLR_W { w: self }
    }
    #[doc = "Bits 18:27"]
    #[inline(always)]
    pub fn swd_signal_width(&mut self) -> SWD_SIGNAL_WIDTH_W {
        SWD_SIGNAL_WIDTH_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn swd_bypass_rst(&mut self) -> SWD_BYPASS_RST_W {
        SWD_BYPASS_RST_W { w: self }
    }
}
