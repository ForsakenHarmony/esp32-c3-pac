#[doc = "Reader of register PERI_BACKUP_CONFIG"]
pub type R = crate::R<u32, super::PERI_BACKUP_CONFIG>;
#[doc = "Writer for register PERI_BACKUP_CONFIG"]
pub type W = crate::W<u32, super::PERI_BACKUP_CONFIG>;
#[doc = "Register PERI_BACKUP_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::PERI_BACKUP_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PERI_BACKUP_ENA`"]
pub type PERI_BACKUP_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PERI_BACKUP_ENA`"]
pub struct PERI_BACKUP_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_BACKUP_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `PERI_BACKUP_TO_MEM`"]
pub type PERI_BACKUP_TO_MEM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PERI_BACKUP_TO_MEM`"]
pub struct PERI_BACKUP_TO_MEM_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_BACKUP_TO_MEM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Write proxy for field `PERI_BACKUP_START`"]
pub struct PERI_BACKUP_START_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_BACKUP_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `PERI_BACKUP_SIZE`"]
pub type PERI_BACKUP_SIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PERI_BACKUP_SIZE`"]
pub struct PERI_BACKUP_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_BACKUP_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 19)) | (((value as u32) & 0x03ff) << 19);
        self.w
    }
}
#[doc = "Reader of field `PERI_BACKUP_TOUT_THRES`"]
pub type PERI_BACKUP_TOUT_THRES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PERI_BACKUP_TOUT_THRES`"]
pub struct PERI_BACKUP_TOUT_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_BACKUP_TOUT_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 9)) | (((value as u32) & 0x03ff) << 9);
        self.w
    }
}
#[doc = "Reader of field `PERI_BACKUP_BURST_LIMIT`"]
pub type PERI_BACKUP_BURST_LIMIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PERI_BACKUP_BURST_LIMIT`"]
pub struct PERI_BACKUP_BURST_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_BACKUP_BURST_LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PERI_BACKUP_FLOW_ERR`"]
pub type PERI_BACKUP_FLOW_ERR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn peri_backup_ena(&self) -> PERI_BACKUP_ENA_R {
        PERI_BACKUP_ENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn peri_backup_to_mem(&self) -> PERI_BACKUP_TO_MEM_R {
        PERI_BACKUP_TO_MEM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    pub fn peri_backup_size(&self) -> PERI_BACKUP_SIZE_R {
        PERI_BACKUP_SIZE_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 9:18"]
    #[inline(always)]
    pub fn peri_backup_tout_thres(&self) -> PERI_BACKUP_TOUT_THRES_R {
        PERI_BACKUP_TOUT_THRES_R::new(((self.bits >> 9) & 0x03ff) as u16)
    }
    #[doc = "Bits 4:8"]
    #[inline(always)]
    pub fn peri_backup_burst_limit(&self) -> PERI_BACKUP_BURST_LIMIT_R {
        PERI_BACKUP_BURST_LIMIT_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn peri_backup_flow_err(&self) -> PERI_BACKUP_FLOW_ERR_R {
        PERI_BACKUP_FLOW_ERR_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn peri_backup_ena(&mut self) -> PERI_BACKUP_ENA_W {
        PERI_BACKUP_ENA_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn peri_backup_to_mem(&mut self) -> PERI_BACKUP_TO_MEM_W {
        PERI_BACKUP_TO_MEM_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn peri_backup_start(&mut self) -> PERI_BACKUP_START_W {
        PERI_BACKUP_START_W { w: self }
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    pub fn peri_backup_size(&mut self) -> PERI_BACKUP_SIZE_W {
        PERI_BACKUP_SIZE_W { w: self }
    }
    #[doc = "Bits 9:18"]
    #[inline(always)]
    pub fn peri_backup_tout_thres(&mut self) -> PERI_BACKUP_TOUT_THRES_W {
        PERI_BACKUP_TOUT_THRES_W { w: self }
    }
    #[doc = "Bits 4:8"]
    #[inline(always)]
    pub fn peri_backup_burst_limit(&mut self) -> PERI_BACKUP_BURST_LIMIT_W {
        PERI_BACKUP_BURST_LIMIT_W { w: self }
    }
}
