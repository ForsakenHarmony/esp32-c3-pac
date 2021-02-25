#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `SCL_STATE_LAST`"]
pub type SCL_STATE_LAST_R = crate::R<u8, u8>;
#[doc = "Reader of field `SCL_MAIN_STATE_LAST`"]
pub type SCL_MAIN_STATE_LAST_R = crate::R<u8, u8>;
#[doc = "Reader of field `TXFIFO_CNT`"]
pub type TXFIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `STRETCH_CAUSE`"]
pub type STRETCH_CAUSE_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXFIFO_CNT`"]
pub type RXFIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `SLAVE_ADDRESSED`"]
pub type SLAVE_ADDRESSED_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUS_BUSY`"]
pub type BUS_BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARB_LOST`"]
pub type ARB_LOST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLAVE_RW`"]
pub type SLAVE_RW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RESP_REC`"]
pub type RESP_REC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn scl_state_last(&self) -> SCL_STATE_LAST_R {
        SCL_STATE_LAST_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn scl_main_state_last(&self) -> SCL_MAIN_STATE_LAST_R {
        SCL_MAIN_STATE_LAST_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TXFIFO_CNT_R {
        TXFIFO_CNT_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn stretch_cause(&self) -> STRETCH_CAUSE_R {
        STRETCH_CAUSE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RXFIFO_CNT_R {
        RXFIFO_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slave_addressed(&self) -> SLAVE_ADDRESSED_R {
        SLAVE_ADDRESSED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slave_rw(&self) -> SLAVE_RW_R {
        SLAVE_RW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn resp_rec(&self) -> RESP_REC_R {
        RESP_REC_R::new((self.bits & 0x01) != 0)
    }
}
