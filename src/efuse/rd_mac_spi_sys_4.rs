#[doc = "Reader of register RD_MAC_SPI_SYS_4"]
pub type R = crate::R<u32, super::RD_MAC_SPI_SYS_4>;
#[doc = "Reader of field `SYS_DATA_PART0_1`"]
pub type SYS_DATA_PART0_1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sys_data_part0_1(&self) -> SYS_DATA_PART0_1_R {
        SYS_DATA_PART0_1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
