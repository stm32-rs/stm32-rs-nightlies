#[doc = "Reader of register ETH_DMAC0CATxBR"]
pub type R = crate::R<u32, super::ETH_DMAC0CATXBR>;
#[doc = "Reader of field `CURTBUFAPTR`"]
pub type CURTBUFAPTR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
