#[doc = "Reader of register GICD_SPISR1"]
pub type R = crate::R<u32, super::GICD_SPISR1>;
#[doc = "Reader of field `SPISR1`"]
pub type SPISR1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SPISR1"]
    #[inline(always)]
    pub fn spisr1(&self) -> SPISR1_R {
        SPISR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
