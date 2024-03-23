#[doc = "Register `ETH_DMAC0CARxDR` reader"]
pub type R = crate::R<ETH_DMAC0CARX_DRrs>;
#[doc = "Field `CURRDESAPTR` reader - Application Transmit Descriptor Address Pointer"]
pub type CURRDESAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new(self.bits)
    }
}
#[doc = "Channel 0 current application receive descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_dmac0carx_dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_DMAC0CARX_DRrs;
impl crate::RegisterSpec for ETH_DMAC0CARX_DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_dmac0carx_dr::R`](R) reader structure"]
impl crate::Readable for ETH_DMAC0CARX_DRrs {}
#[doc = "`reset()` method sets ETH_DMAC0CARxDR to value 0"]
impl crate::Resettable for ETH_DMAC0CARX_DRrs {
    const RESET_VALUE: u32 = 0;
}
