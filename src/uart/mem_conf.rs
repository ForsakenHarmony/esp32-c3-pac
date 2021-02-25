#[doc = "Reader of register MEM_CONF"]
pub type R = crate::R<u32, super::MEM_CONF>;
#[doc = "Writer for register MEM_CONF"]
pub type W = crate::W<u32, super::MEM_CONF>;
#[doc = "Register MEM_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_FORCE_PU`"]
pub type MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_FORCE_PU`"]
pub struct MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `MEM_FORCE_PD`"]
pub type MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_FORCE_PD`"]
pub struct MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `RX_TOUT_THRHD`"]
pub type RX_TOUT_THRHD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RX_TOUT_THRHD`"]
pub struct RX_TOUT_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TOUT_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RX_FLOW_THRHD`"]
pub type RX_FLOW_THRHD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RX_FLOW_THRHD`"]
pub struct RX_FLOW_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FLOW_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 7)) | (((value as u32) & 0x01ff) << 7);
        self.w
    }
}
#[doc = "Reader of field `TX_SIZE`"]
pub type TX_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_SIZE`"]
pub struct TX_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `RX_SIZE`"]
pub type RX_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_SIZE`"]
pub struct RX_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn mem_force_pu(&self) -> MEM_FORCE_PU_R {
        MEM_FORCE_PU_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn mem_force_pd(&self) -> MEM_FORCE_PD_R {
        MEM_FORCE_PD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_tout_thrhd(&self) -> RX_TOUT_THRHD_R {
        RX_TOUT_THRHD_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 7:15"]
    #[inline(always)]
    pub fn rx_flow_thrhd(&self) -> RX_FLOW_THRHD_R {
        RX_FLOW_THRHD_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tx_size(&self) -> TX_SIZE_R {
        TX_SIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn rx_size(&self) -> RX_SIZE_R {
        RX_SIZE_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn mem_force_pu(&mut self) -> MEM_FORCE_PU_W {
        MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn mem_force_pd(&mut self) -> MEM_FORCE_PD_W {
        MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_tout_thrhd(&mut self) -> RX_TOUT_THRHD_W {
        RX_TOUT_THRHD_W { w: self }
    }
    #[doc = "Bits 7:15"]
    #[inline(always)]
    pub fn rx_flow_thrhd(&mut self) -> RX_FLOW_THRHD_W {
        RX_FLOW_THRHD_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tx_size(&mut self) -> TX_SIZE_W {
        TX_SIZE_W { w: self }
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn rx_size(&mut self) -> RX_SIZE_W {
        RX_SIZE_W { w: self }
    }
}
