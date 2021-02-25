#[doc = "Reader of register SWD_WPROTECT"]
pub type R = crate::R<u32, super::SWD_WPROTECT>;
#[doc = "Writer for register SWD_WPROTECT"]
pub type W = crate::W<u32, super::SWD_WPROTECT>;
#[doc = "Register SWD_WPROTECT `reset()`'s with value 0"]
impl crate::ResetValue for super::SWD_WPROTECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWD_WKEY`"]
pub type SWD_WKEY_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SWD_WKEY`"]
pub struct SWD_WKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_WKEY_W<'a> {
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
    pub fn swd_wkey(&self) -> SWD_WKEY_R {
        SWD_WKEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn swd_wkey(&mut self) -> SWD_WKEY_W {
        SWD_WKEY_W { w: self }
    }
}
