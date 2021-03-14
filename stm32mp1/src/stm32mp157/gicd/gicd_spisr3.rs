#[doc = "Reader of register GICD_SPISR3"]
pub type R = crate::R<u32, super::GICD_SPISR3>;
#[doc = "Reader of field `SPISR3`"]
pub type SPISR3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SPISR3"]
    #[inline(always)]
    pub fn spisr3(&self) -> SPISR3_R {
        SPISR3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
