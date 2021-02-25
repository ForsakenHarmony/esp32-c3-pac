#[doc = "Reader of register RD_SYS_PART1_DATA4"]
pub type R = crate::R<u32, super::RD_SYS_PART1_DATA4>;
#[doc = "Reader of field `SYS_DATA_PART1_4`"]
pub type SYS_DATA_PART1_4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sys_data_part1_4(&self) -> SYS_DATA_PART1_4_R {
        SYS_DATA_PART1_4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
