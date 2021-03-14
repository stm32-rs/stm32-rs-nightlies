#[doc = "Reader of register GICD_SPISR6"]
pub type R = crate::R<u32, super::GICD_SPISR6>;
#[doc = "Reader of field `SPISR6`"]
pub type SPISR6_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SPISR6"]
    #[inline(always)]
    pub fn spisr6(&self) -> SPISR6_R {
        SPISR6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
