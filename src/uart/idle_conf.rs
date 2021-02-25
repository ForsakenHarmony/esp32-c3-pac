#[doc = "Reader of register IDLE_CONF"]
pub type R = crate::R<u32, super::IDLE_CONF>;
#[doc = "Writer for register IDLE_CONF"]
pub type W = crate::W<u32, super::IDLE_CONF>;
#[doc = "Register IDLE_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::IDLE_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_IDLE_NUM`"]
pub type TX_IDLE_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TX_IDLE_NUM`"]
pub struct TX_IDLE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IDLE_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `RX_IDLE_THRHD`"]
pub type RX_IDLE_THRHD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RX_IDLE_THRHD`"]
pub struct RX_IDLE_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_IDLE_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn tx_idle_num(&self) -> TX_IDLE_NUM_R {
        TX_IDLE_NUM_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_idle_thrhd(&self) -> RX_IDLE_THRHD_R {
        RX_IDLE_THRHD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn tx_idle_num(&mut self) -> TX_IDLE_NUM_W {
        TX_IDLE_NUM_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_idle_thrhd(&mut self) -> RX_IDLE_THRHD_W {
        RX_IDLE_THRHD_W { w: self }
    }
}
