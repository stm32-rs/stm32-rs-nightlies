#[doc = "Reader of register HASH_HR6"]
pub type R = crate::R<u32, super::HASH_HR6>;
#[doc = "Reader of field `H6`"]
pub type H6_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - H6"]
    #[inline(always)]
    pub fn h6(&self) -> H6_R {
        H6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
