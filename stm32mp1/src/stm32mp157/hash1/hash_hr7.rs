#[doc = "Reader of register HASH_HR7"]
pub type R = crate::R<u32, super::HASH_HR7>;
#[doc = "Reader of field `H7`"]
pub type H7_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - H7"]
    #[inline(always)]
    pub fn h7(&self) -> H7_R {
        H7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
