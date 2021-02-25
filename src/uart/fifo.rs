#[doc = "Reader of register FIFO"]
pub type R = crate::R<u32, super::FIFO>;
#[doc = "Reader of field `RXFIFO_RD_BYTE`"]
pub type RXFIFO_RD_BYTE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rxfifo_rd_byte(&self) -> RXFIFO_RD_BYTE_R {
        RXFIFO_RD_BYTE_R::new((self.bits & 0xff) as u8)
    }
}
