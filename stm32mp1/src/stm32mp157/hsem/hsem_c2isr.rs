#[doc = "Reader of register HSEM_C2ISR"]
pub type R = crate::R<u32, super::HSEM_C2ISR>;
#[doc = "Reader of field `ISF`"]
pub type ISF_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ISF"]
    #[inline(always)]
    pub fn isf(&self) -> ISF_R {
        ISF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
