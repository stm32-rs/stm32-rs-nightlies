#[doc = "Reader of register MACATSSR"]
pub type R = crate::R<u32, super::MACATSSR>;
#[doc = "Reader of field `AUXTSHI`"]
pub type AUXTSHI_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - AUXTSHI"]
    #[inline(always)]
    pub fn auxtshi(&self) -> AUXTSHI_R {
        AUXTSHI_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
