#[doc = "Reader of register XTAL32K_CONF"]
pub type R = crate::R<u32, super::XTAL32K_CONF>;
#[doc = "Writer for register XTAL32K_CONF"]
pub type W = crate::W<u32, super::XTAL32K_CONF>;
#[doc = "Register XTAL32K_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::XTAL32K_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XTAL32K_STABLE_THRES`"]
pub type XTAL32K_STABLE_THRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XTAL32K_STABLE_THRES`"]
pub struct XTAL32K_STABLE_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_STABLE_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `XTAL32K_WDT_TIMEOUT`"]
pub type XTAL32K_WDT_TIMEOUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XTAL32K_WDT_TIMEOUT`"]
pub struct XTAL32K_WDT_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_WDT_TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | (((value as u32) & 0xff) << 20);
        self.w
    }
}
#[doc = "Reader of field `XTAL32K_RESTART_WAIT`"]
pub type XTAL32K_RESTART_WAIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XTAL32K_RESTART_WAIT`"]
pub struct XTAL32K_RESTART_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_RESTART_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 4)) | (((value as u32) & 0xffff) << 4);
        self.w
    }
}
#[doc = "Reader of field `XTAL32K_RETURN_WAIT`"]
pub type XTAL32K_RETURN_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XTAL32K_RETURN_WAIT`"]
pub struct XTAL32K_RETURN_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_RETURN_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn xtal32k_stable_thres(&self) -> XTAL32K_STABLE_THRES_R {
        XTAL32K_STABLE_THRES_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27"]
    #[inline(always)]
    pub fn xtal32k_wdt_timeout(&self) -> XTAL32K_WDT_TIMEOUT_R {
        XTAL32K_WDT_TIMEOUT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    pub fn xtal32k_restart_wait(&self) -> XTAL32K_RESTART_WAIT_R {
        XTAL32K_RESTART_WAIT_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn xtal32k_return_wait(&self) -> XTAL32K_RETURN_WAIT_R {
        XTAL32K_RETURN_WAIT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn xtal32k_stable_thres(&mut self) -> XTAL32K_STABLE_THRES_W {
        XTAL32K_STABLE_THRES_W { w: self }
    }
    #[doc = "Bits 20:27"]
    #[inline(always)]
    pub fn xtal32k_wdt_timeout(&mut self) -> XTAL32K_WDT_TIMEOUT_W {
        XTAL32K_WDT_TIMEOUT_W { w: self }
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    pub fn xtal32k_restart_wait(&mut self) -> XTAL32K_RESTART_WAIT_W {
        XTAL32K_RESTART_WAIT_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn xtal32k_return_wait(&mut self) -> XTAL32K_RETURN_WAIT_W {
        XTAL32K_RETURN_WAIT_W { w: self }
    }
}
