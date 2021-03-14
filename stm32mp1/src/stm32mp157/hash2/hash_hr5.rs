#[doc = "Reader of register HASH_HR5"]
pub type R = crate::R<u32, super::HASH_HR5>;
#[doc = "Reader of field `H5`"]
pub type H5_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - H5"]
    #[inline(always)]
    pub fn h5(&self) -> H5_R {
        H5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
