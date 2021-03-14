#[doc = "Reader of register ETH_DMAC0CARxDR"]
pub type R = crate::R<u32, super::ETH_DMAC0CARXDR>;
#[doc = "Reader of field `CURRDESAPTR`"]
pub type CURRDESAPTR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
