#[doc = "Reader of register DATA"]
pub type R = crate::R<u32, super::DATA>;
#[doc = "Reader of field `FIFO_RDATA`"]
pub type FIFO_RDATA_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn fifo_rdata(&self) -> FIFO_RDATA_R {
        FIFO_RDATA_R::new((self.bits & 0xff) as u8)
    }
}
