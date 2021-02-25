#[doc = "Reader of register WIFI_BB_CFG"]
pub type R = crate::R<u32, super::WIFI_BB_CFG>;
#[doc = "Writer for register WIFI_BB_CFG"]
pub type W = crate::W<u32, super::WIFI_BB_CFG>;
#[doc = "Register WIFI_BB_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::WIFI_BB_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WIFI_BB_CFG`"]
pub type WIFI_BB_CFG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WIFI_BB_CFG`"]
pub struct WIFI_BB_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_BB_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn wifi_bb_cfg(&self) -> WIFI_BB_CFG_R {
        WIFI_BB_CFG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn wifi_bb_cfg(&mut self) -> WIFI_BB_CFG_W {
        WIFI_BB_CFG_W { w: self }
    }
}
