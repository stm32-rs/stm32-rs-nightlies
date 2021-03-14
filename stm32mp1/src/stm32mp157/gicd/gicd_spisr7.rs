#[doc = "Reader of register GICD_SPISR7"]
pub type R = crate::R<u32, super::GICD_SPISR7>;
#[doc = "Reader of field `SPISR7`"]
pub type SPISR7_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SPISR7"]
    #[inline(always)]
    pub fn spisr7(&self) -> SPISR7_R {
        SPISR7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
