#[doc = "Reader of register ETH_MACSTNR"]
pub type R = crate::R<u32, super::ETH_MACSTNR>;
#[doc = "Reader of field `TSSS`"]
pub type TSSS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:30 - TSSS"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
