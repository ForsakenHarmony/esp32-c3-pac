#[doc = "Reader of register RTCCALICFG2"]
pub type R = crate::R<u32, super::RTCCALICFG2>;
#[doc = "Writer for register RTCCALICFG2"]
pub type W = crate::W<u32, super::RTCCALICFG2>;
#[doc = "Register RTCCALICFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCCALICFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMEOUT_THRES`"]
pub type TIMEOUT_THRES_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TIMEOUT_THRES`"]
pub struct TIMEOUT_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
#[doc = "Reader of field `TIMEOUT_RST_CNT`"]
pub type TIMEOUT_RST_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMEOUT_RST_CNT`"]
pub struct TIMEOUT_RST_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_RST_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
        self.w
    }
}
#[doc = "Reader of field `TIMEOUT`"]
pub type TIMEOUT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 7:31"]
    #[inline(always)]
    pub fn timeout_thres(&self) -> TIMEOUT_THRES_R {
        TIMEOUT_THRES_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
    #[doc = "Bits 3:6"]
    #[inline(always)]
    pub fn timeout_rst_cnt(&self) -> TIMEOUT_RST_CNT_R {
        TIMEOUT_RST_CNT_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 7:31"]
    #[inline(always)]
    pub fn timeout_thres(&mut self) -> TIMEOUT_THRES_W {
        TIMEOUT_THRES_W { w: self }
    }
    #[doc = "Bits 3:6"]
    #[inline(always)]
    pub fn timeout_rst_cnt(&mut self) -> TIMEOUT_RST_CNT_W {
        TIMEOUT_RST_CNT_W { w: self }
    }
}
