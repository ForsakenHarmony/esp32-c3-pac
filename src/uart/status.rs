#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `TXD`"]
pub type TXD_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTSN`"]
pub type RTSN_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTRN`"]
pub type DTRN_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFIFO_CNT`"]
pub type TXFIFO_CNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `RXD`"]
pub type RXD_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTSN`"]
pub type CTSN_R = crate::R<bool, bool>;
#[doc = "Reader of field `DSRN`"]
pub type DSRN_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFO_CNT`"]
pub type RXFIFO_CNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn txd(&self) -> TXD_R {
        TXD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtsn(&self) -> RTSN_R {
        RTSN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dtrn(&self) -> DTRN_R {
        DTRN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TXFIFO_CNT_R {
        TXFIFO_CNT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ctsn(&self) -> CTSN_R {
        CTSN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dsrn(&self) -> DSRN_R {
        DSRN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RXFIFO_CNT_R {
        RXFIFO_CNT_R::new((self.bits & 0x03ff) as u16)
    }
}
