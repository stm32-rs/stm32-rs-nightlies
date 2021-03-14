#[doc = "Reader of register HR2"]
pub type R = crate::R<u32, super::HR2>;
#[doc = "Reader of field `H2`"]
pub type H2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - H2"]
    #[inline(always)]
    pub fn h2(&self) -> H2_R {
        H2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
