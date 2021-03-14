#[doc = "Reader of register HASH_HR3"]
pub type R = crate::R<u32, super::HASH_HR3>;
#[doc = "Reader of field `H3`"]
pub type H3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - H3"]
    #[inline(always)]
    pub fn h3(&self) -> H3_R {
        H3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
