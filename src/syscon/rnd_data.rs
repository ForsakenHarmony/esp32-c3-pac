#[doc = "Reader of register RND_DATA"]
pub type R = crate::R<u32, super::RND_DATA>;
#[doc = "Reader of field `RND_DATA`"]
pub type RND_DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rnd_data(&self) -> RND_DATA_R {
        RND_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
