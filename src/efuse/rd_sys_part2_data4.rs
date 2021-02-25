#[doc = "Reader of register RD_SYS_PART2_DATA4"]
pub type R = crate::R<u32, super::RD_SYS_PART2_DATA4>;
#[doc = "Reader of field `SYS_DATA_PART2_4`"]
pub type SYS_DATA_PART2_4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sys_data_part2_4(&self) -> SYS_DATA_PART2_4_R {
        SYS_DATA_PART2_4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
