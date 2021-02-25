#[doc = "Reader of register PGM_DATA7"]
pub type R = crate::R<u32, super::PGM_DATA7>;
#[doc = "Writer for register PGM_DATA7"]
pub type W = crate::W<u32, super::PGM_DATA7>;
#[doc = "Register PGM_DATA7 `reset()`'s with value 0"]
impl crate::ResetValue for super::PGM_DATA7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PGM_DATA_7`"]
pub type PGM_DATA_7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PGM_DATA_7`"]
pub struct PGM_DATA_7_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_DATA_7_W<'a> {
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
    pub fn pgm_data_7(&self) -> PGM_DATA_7_R {
        PGM_DATA_7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pgm_data_7(&mut self) -> PGM_DATA_7_W {
        PGM_DATA_7_W { w: self }
    }
}
