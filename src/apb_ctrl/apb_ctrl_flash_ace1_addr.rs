#[doc = "Reader of register APB_CTRL_FLASH_ACE1_ADDR"]
pub type R = crate::R<u32, super::APB_CTRL_FLASH_ACE1_ADDR>;
#[doc = "Writer for register APB_CTRL_FLASH_ACE1_ADDR"]
pub type W = crate::W<u32, super::APB_CTRL_FLASH_ACE1_ADDR>;
#[doc = "Register APB_CTRL_FLASH_ACE1_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_CTRL_FLASH_ACE1_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_CTRL_FLASH_ACE1_ADDR_S`"]
pub type APB_CTRL_FLASH_ACE1_ADDR_S_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `APB_CTRL_FLASH_ACE1_ADDR_S`"]
pub struct APB_CTRL_FLASH_ACE1_ADDR_S_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_FLASH_ACE1_ADDR_S_W<'a> {
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
    pub fn apb_ctrl_flash_ace1_addr_s(&self) -> APB_CTRL_FLASH_ACE1_ADDR_S_R {
        APB_CTRL_FLASH_ACE1_ADDR_S_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn apb_ctrl_flash_ace1_addr_s(&mut self) -> APB_CTRL_FLASH_ACE1_ADDR_S_W {
        APB_CTRL_FLASH_ACE1_ADDR_S_W { w: self }
    }
}
