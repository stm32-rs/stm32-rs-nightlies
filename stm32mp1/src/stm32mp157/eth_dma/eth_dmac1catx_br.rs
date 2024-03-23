#[doc = "Register `ETH_DMAC1CATxBR` reader"]
pub type R = crate::R<ETH_DMAC1CATX_BRrs>;
#[doc = "Field `CURTBUFAPTR` reader - Application Transmit Buffer Address Pointer"]
pub type CURTBUFAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new(self.bits)
    }
}
#[doc = "Channel 0 current application transmit buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_dmac1catx_br::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_DMAC1CATX_BRrs;
impl crate::RegisterSpec for ETH_DMAC1CATX_BRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_dmac1catx_br::R`](R) reader structure"]
impl crate::Readable for ETH_DMAC1CATX_BRrs {}
#[doc = "`reset()` method sets ETH_DMAC1CATxBR to value 0"]
impl crate::Resettable for ETH_DMAC1CATX_BRrs {
    const RESET_VALUE: u32 = 0;
}
