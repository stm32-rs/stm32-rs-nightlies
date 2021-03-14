#[doc = "Reader of register MACTxTSSSR"]
pub type R = crate::R<u32, super::MACTXTSSSR>;
#[doc = "Reader of field `TXTSSHI`"]
pub type TXTSSHI_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - TXTSSHI"]
    #[inline(always)]
    pub fn txtsshi(&self) -> TXTSSHI_R {
        TXTSSHI_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
