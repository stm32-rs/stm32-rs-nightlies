#[doc = "Reader of register M3CR"]
pub type R = crate::R<u32, super::M3CR>;
#[doc = "Reader of field `FADD`"]
pub type FADD_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
