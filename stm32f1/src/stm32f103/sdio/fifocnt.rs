#[doc = "Reader of register FIFOCNT"]
pub type R = crate::R<u32, super::FIFOCNT>;
#[doc = "Reader of field `FIF0COUNT`"]
pub type FIF0COUNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - FIF0COUNT"]
    #[inline(always)]
    pub fn fif0count(&self) -> FIF0COUNT_R {
        FIF0COUNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
