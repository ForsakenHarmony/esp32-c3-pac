#[doc = "Reader of register PERI_BACKUP_MEM_ADDR"]
pub type R = crate::R<u32, super::PERI_BACKUP_MEM_ADDR>;
#[doc = "Writer for register PERI_BACKUP_MEM_ADDR"]
pub type W = crate::W<u32, super::PERI_BACKUP_MEM_ADDR>;
#[doc = "Register PERI_BACKUP_MEM_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::PERI_BACKUP_MEM_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BACKUP_MEM_START_ADDR`"]
pub type BACKUP_MEM_START_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BACKUP_MEM_START_ADDR`"]
pub struct BACKUP_MEM_START_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_MEM_START_ADDR_W<'a> {
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
    pub fn backup_mem_start_addr(&self) -> BACKUP_MEM_START_ADDR_R {
        BACKUP_MEM_START_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn backup_mem_start_addr(&mut self) -> BACKUP_MEM_START_ADDR_W {
        BACKUP_MEM_START_ADDR_W { w: self }
    }
}
