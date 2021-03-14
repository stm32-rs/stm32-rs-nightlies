#[doc = "Reader of register GICD_SPISR5"]
pub type R = crate::R<u32, super::GICD_SPISR5>;
#[doc = "Reader of field `SPISR5`"]
pub type SPISR5_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SPISR5"]
    #[inline(always)]
    pub fn spisr5(&self) -> SPISR5_R {
        SPISR5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
