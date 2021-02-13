#[doc = "Reader of register SYSCON_TICK_CONF"]
pub type R = crate::R<u32, super::SYSCON_TICK_CONF>;
#[doc = "Writer for register SYSCON_TICK_CONF"]
pub type W = crate::W<u32, super::SYSCON_TICK_CONF>;
#[doc = "Register SYSCON_TICK_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCON_TICK_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCON_TICK_ENABLE`"]
pub type SYSCON_TICK_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_TICK_ENABLE`"]
pub struct SYSCON_TICK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_TICK_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SYSCON_CK8M_TICK_NUM`"]
pub type SYSCON_CK8M_TICK_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCON_CK8M_TICK_NUM`"]
pub struct SYSCON_CK8M_TICK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_CK8M_TICK_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SYSCON_XTAL_TICK_NUM`"]
pub type SYSCON_XTAL_TICK_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCON_XTAL_TICK_NUM`"]
pub struct SYSCON_XTAL_TICK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_XTAL_TICK_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn syscon_tick_enable(&self) -> SYSCON_TICK_ENABLE_R {
        SYSCON_TICK_ENABLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn syscon_ck8m_tick_num(&self) -> SYSCON_CK8M_TICK_NUM_R {
        SYSCON_CK8M_TICK_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn syscon_xtal_tick_num(&self) -> SYSCON_XTAL_TICK_NUM_R {
        SYSCON_XTAL_TICK_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn syscon_tick_enable(&mut self) -> SYSCON_TICK_ENABLE_W {
        SYSCON_TICK_ENABLE_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn syscon_ck8m_tick_num(&mut self) -> SYSCON_CK8M_TICK_NUM_W {
        SYSCON_CK8M_TICK_NUM_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn syscon_xtal_tick_num(&mut self) -> SYSCON_XTAL_TICK_NUM_W {
        SYSCON_XTAL_TICK_NUM_W { w: self }
    }
}
