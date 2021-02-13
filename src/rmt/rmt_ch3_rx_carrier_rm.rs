#[doc = "Reader of register RMT_CH3_RX_CARRIER_RM"]
pub type R = crate::R<u32, super::RMT_CH3_RX_CARRIER_RM>;
#[doc = "Writer for register RMT_CH3_RX_CARRIER_RM"]
pub type W = crate::W<u32, super::RMT_CH3_RX_CARRIER_RM>;
#[doc = "Register RMT_CH3_RX_CARRIER_RM `reset()`'s with value 0"]
impl crate::ResetValue for super::RMT_CH3_RX_CARRIER_RM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RMT_CARRIER_HIGH_THRES_CH3`"]
pub type RMT_CARRIER_HIGH_THRES_CH3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RMT_CARRIER_HIGH_THRES_CH3`"]
pub struct RMT_CARRIER_HIGH_THRES_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CARRIER_HIGH_THRES_CH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RMT_CARRIER_LOW_THRES_CH3`"]
pub type RMT_CARRIER_LOW_THRES_CH3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RMT_CARRIER_LOW_THRES_CH3`"]
pub struct RMT_CARRIER_LOW_THRES_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CARRIER_LOW_THRES_CH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rmt_carrier_high_thres_ch3(&self) -> RMT_CARRIER_HIGH_THRES_CH3_R {
        RMT_CARRIER_HIGH_THRES_CH3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rmt_carrier_low_thres_ch3(&self) -> RMT_CARRIER_LOW_THRES_CH3_R {
        RMT_CARRIER_LOW_THRES_CH3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rmt_carrier_high_thres_ch3(&mut self) -> RMT_CARRIER_HIGH_THRES_CH3_W {
        RMT_CARRIER_HIGH_THRES_CH3_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rmt_carrier_low_thres_ch3(&mut self) -> RMT_CARRIER_LOW_THRES_CH3_W {
        RMT_CARRIER_LOW_THRES_CH3_W { w: self }
    }
}
