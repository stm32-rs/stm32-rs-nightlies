#[doc = "Reader of register DOUTR"]
pub type R = crate::R<u32, super::DOUTR>;
#[doc = "Reader of field `DOUTR`"]
pub type DOUTR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data output"]
    #[inline(always)]
    pub fn doutr(&self) -> DOUTR_R {
        DOUTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
