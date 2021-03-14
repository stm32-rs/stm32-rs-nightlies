#[doc = "Reader of register ETH_MTLTxQ0ESR"]
pub type R = crate::R<u32, super::ETH_MTLTXQ0ESR>;
#[doc = "Reader of field `ABS`"]
pub type ABS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - ABS"]
    #[inline(always)]
    pub fn abs(&self) -> ABS_R {
        ABS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
