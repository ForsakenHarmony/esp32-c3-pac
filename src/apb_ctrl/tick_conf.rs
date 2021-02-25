#[doc = "Reader of register TICK_CONF"]
pub type R = crate::R<u32, super::TICK_CONF>;
#[doc = "Writer for register TICK_CONF"]
pub type W = crate::W<u32, super::TICK_CONF>;
#[doc = "Register TICK_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::TICK_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TICK_ENABLE`"]
pub type TICK_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TICK_ENABLE`"]
pub struct TICK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_ENABLE_W<'a> {
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
#[doc = "Reader of field `CK8M_TICK_NUM`"]
pub type CK8M_TICK_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CK8M_TICK_NUM`"]
pub struct CK8M_TICK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_TICK_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `XTAL_TICK_NUM`"]
pub type XTAL_TICK_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XTAL_TICK_NUM`"]
pub struct XTAL_TICK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_TICK_NUM_W<'a> {
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
    pub fn tick_enable(&self) -> TICK_ENABLE_R {
        TICK_ENABLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ck8m_tick_num(&self) -> CK8M_TICK_NUM_R {
        CK8M_TICK_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn xtal_tick_num(&self) -> XTAL_TICK_NUM_R {
        XTAL_TICK_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tick_enable(&mut self) -> TICK_ENABLE_W {
        TICK_ENABLE_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ck8m_tick_num(&mut self) -> CK8M_TICK_NUM_W {
        CK8M_TICK_NUM_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn xtal_tick_num(&mut self) -> XTAL_TICK_NUM_W {
        XTAL_TICK_NUM_W { w: self }
    }
}
