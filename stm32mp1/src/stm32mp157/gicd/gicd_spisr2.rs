#[doc = "Reader of register GICD_SPISR2"]
pub type R = crate::R<u32, super::GICD_SPISR2>;
#[doc = "Reader of field `SPISR2`"]
pub type SPISR2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SPISR2"]
    #[inline(always)]
    pub fn spisr2(&self) -> SPISR2_R {
        SPISR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
