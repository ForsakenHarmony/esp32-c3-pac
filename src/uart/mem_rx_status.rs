#[doc = "Reader of register MEM_RX_STATUS"]
pub type R = crate::R<u32, super::MEM_RX_STATUS>;
#[doc = "Reader of field `RX_WADDR`"]
pub type RX_WADDR_R = crate::R<u16, u16>;
#[doc = "Reader of field `APB_RX_RADDR`"]
pub type APB_RX_RADDR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 11:20"]
    #[inline(always)]
    pub fn rx_waddr(&self) -> RX_WADDR_R {
        RX_WADDR_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn apb_rx_raddr(&self) -> APB_RX_RADDR_R {
        APB_RX_RADDR_R::new((self.bits & 0x03ff) as u16)
    }
}
