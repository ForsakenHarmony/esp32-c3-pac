#[doc = "Reader of register EFUSE_DATE"]
pub type R = crate::R<u32, super::EFUSE_DATE>;
#[doc = "Writer for register EFUSE_DATE"]
pub type W = crate::W<u32, super::EFUSE_DATE>;
#[doc = "Register EFUSE_DATE `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_DATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_DATE`"]
pub type EFUSE_DATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EFUSE_DATE`"]
pub struct EFUSE_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | ((value as u32) & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn efuse_date(&self) -> EFUSE_DATE_R {
        EFUSE_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn efuse_date(&mut self) -> EFUSE_DATE_W {
        EFUSE_DATE_W { w: self }
    }
}
