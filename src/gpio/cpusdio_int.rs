#[doc = "Reader of register CPUSDIO_INT"]
pub type R = crate::R<u32, super::CPUSDIO_INT>;
#[doc = "Reader of field `SDIO_INT`"]
pub type SDIO_INT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn sdio_int(&self) -> SDIO_INT_R {
        SDIO_INT_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
