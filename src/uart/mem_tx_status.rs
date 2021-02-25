#[doc = "Reader of register MEM_TX_STATUS"]
pub type R = crate::R<u32, super::MEM_TX_STATUS>;
#[doc = "Reader of field `TX_RADDR`"]
pub type TX_RADDR_R = crate::R<u16, u16>;
#[doc = "Reader of field `APB_TX_WADDR`"]
pub type APB_TX_WADDR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 11:20"]
    #[inline(always)]
    pub fn tx_raddr(&self) -> TX_RADDR_R {
        TX_RADDR_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn apb_tx_waddr(&self) -> APB_TX_WADDR_R {
        APB_TX_WADDR_R::new((self.bits & 0x03ff) as u16)
    }
}
