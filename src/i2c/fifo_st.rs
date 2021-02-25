#[doc = "Reader of register FIFO_ST"]
pub type R = crate::R<u32, super::FIFO_ST>;
#[doc = "Reader of field `SLAVE_RW_POINT`"]
pub type SLAVE_RW_POINT_R = crate::R<u8, u8>;
#[doc = "Reader of field `TXFIFO_WADDR`"]
pub type TXFIFO_WADDR_R = crate::R<u8, u8>;
#[doc = "Reader of field `TXFIFO_RADDR`"]
pub type TXFIFO_RADDR_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXFIFO_WADDR`"]
pub type RXFIFO_WADDR_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXFIFO_RADDR`"]
pub type RXFIFO_RADDR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 22:29"]
    #[inline(always)]
    pub fn slave_rw_point(&self) -> SLAVE_RW_POINT_R {
        SLAVE_RW_POINT_R::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn txfifo_waddr(&self) -> TXFIFO_WADDR_R {
        TXFIFO_WADDR_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn txfifo_raddr(&self) -> TXFIFO_RADDR_R {
        TXFIFO_RADDR_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn rxfifo_waddr(&self) -> RXFIFO_WADDR_R {
        RXFIFO_WADDR_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rxfifo_raddr(&self) -> RXFIFO_RADDR_R {
        RXFIFO_RADDR_R::new((self.bits & 0x1f) as u8)
    }
}
