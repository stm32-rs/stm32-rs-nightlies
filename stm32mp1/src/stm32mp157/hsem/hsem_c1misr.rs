#[doc = "Reader of register HSEM_C1MISR"]
pub type R = crate::R<u32, super::HSEM_C1MISR>;
#[doc = "Reader of field `MISF`"]
pub type MISF_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - MISF"]
    #[inline(always)]
    pub fn misf(&self) -> MISF_R {
        MISF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
