#[doc = "Reader of register DMA_OUT_CONF1_CH1"]
pub type R = crate::R<u32, super::DMA_OUT_CONF1_CH1>;
#[doc = "Writer for register DMA_OUT_CONF1_CH1"]
pub type W = crate::W<u32, super::DMA_OUT_CONF1_CH1>;
#[doc = "Register DMA_OUT_CONF1_CH1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_OUT_CONF1_CH1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_OUT_CHECK_OWNER_CH1`"]
pub type DMA_OUT_CHECK_OWNER_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_OUT_CHECK_OWNER_CH1`"]
pub struct DMA_OUT_CHECK_OWNER_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUT_CHECK_OWNER_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dma_out_check_owner_ch1(&self) -> DMA_OUT_CHECK_OWNER_CH1_R {
        DMA_OUT_CHECK_OWNER_CH1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dma_out_check_owner_ch1(&mut self) -> DMA_OUT_CHECK_OWNER_CH1_W {
        DMA_OUT_CHECK_OWNER_CH1_W { w: self }
    }
}
