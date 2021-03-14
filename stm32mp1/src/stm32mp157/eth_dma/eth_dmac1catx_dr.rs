#[doc = "Reader of register ETH_DMAC1CATxDR"]
pub type R = crate::R<u32, super::ETH_DMAC1CATXDR>;
#[doc = "Reader of field `CURTDESAPTR`"]
pub type CURTDESAPTR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CURTDESAPTR_R {
        CURTDESAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
