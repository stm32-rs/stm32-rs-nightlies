#[doc = "Reader of register HASH_HR4"]
pub type R = crate::R<u32, super::HASH_HR4>;
#[doc = "Reader of field `H4`"]
pub type H4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - H4"]
    #[inline(always)]
    pub fn h4(&self) -> H4_R {
        H4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
