#[doc = "Reader of register RD_SYS_PART2_DATA0"]
pub type R = crate::R<u32, super::RD_SYS_PART2_DATA0>;
#[doc = "Reader of field `SYS_DATA_PART2_0`"]
pub type SYS_DATA_PART2_0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sys_data_part2_0(&self) -> SYS_DATA_PART2_0_R {
        SYS_DATA_PART2_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
