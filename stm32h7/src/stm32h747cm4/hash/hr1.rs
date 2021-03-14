#[doc = "Reader of register HR1"]
pub type R = crate::R<u32, super::HR1>;
#[doc = "Reader of field `H1`"]
pub type H1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - H1"]
    #[inline(always)]
    pub fn h1(&self) -> H1_R {
        H1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
