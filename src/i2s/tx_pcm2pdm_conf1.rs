#[doc = "Reader of register TX_PCM2PDM_CONF1"]
pub type R = crate::R<u32, super::TX_PCM2PDM_CONF1>;
#[doc = "Writer for register TX_PCM2PDM_CONF1"]
pub type W = crate::W<u32, super::TX_PCM2PDM_CONF1>;
#[doc = "Register TX_PCM2PDM_CONF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_PCM2PDM_CONF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_IIR_HP_MULT12_0`"]
pub type TX_IIR_HP_MULT12_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_IIR_HP_MULT12_0`"]
pub struct TX_IIR_HP_MULT12_0_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IIR_HP_MULT12_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Reader of field `TX_IIR_HP_MULT12_5`"]
pub type TX_IIR_HP_MULT12_5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_IIR_HP_MULT12_5`"]
pub struct TX_IIR_HP_MULT12_5_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IIR_HP_MULT12_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `TX_PDM_FS`"]
pub type TX_PDM_FS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TX_PDM_FS`"]
pub struct TX_PDM_FS_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_FS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `TX_PDM_FP`"]
pub type TX_PDM_FP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TX_PDM_FP`"]
pub struct TX_PDM_FP_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_FP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn tx_iir_hp_mult12_0(&self) -> TX_IIR_HP_MULT12_0_R {
        TX_IIR_HP_MULT12_0_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn tx_iir_hp_mult12_5(&self) -> TX_IIR_HP_MULT12_5_R {
        TX_IIR_HP_MULT12_5_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn tx_pdm_fs(&self) -> TX_PDM_FS_R {
        TX_PDM_FS_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_pdm_fp(&self) -> TX_PDM_FP_R {
        TX_PDM_FP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn tx_iir_hp_mult12_0(&mut self) -> TX_IIR_HP_MULT12_0_W {
        TX_IIR_HP_MULT12_0_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn tx_iir_hp_mult12_5(&mut self) -> TX_IIR_HP_MULT12_5_W {
        TX_IIR_HP_MULT12_5_W { w: self }
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn tx_pdm_fs(&mut self) -> TX_PDM_FS_W {
        TX_PDM_FS_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_pdm_fp(&mut self) -> TX_PDM_FP_W {
        TX_PDM_FP_W { w: self }
    }
}
