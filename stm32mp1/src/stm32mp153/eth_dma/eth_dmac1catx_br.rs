#[doc = "Register `ETH_DMAC1CATxBR` reader"]
pub struct R(crate::R<ETH_DMAC1CATXBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC1CATXBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC1CATXBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC1CATXBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURTBUFAPTR` reader - Application Transmit Buffer Address Pointer"]
pub struct CURTBUFAPTR_R(crate::FieldReader<u32, u32>);
impl CURTBUFAPTR_R {
    pub(crate) fn new(bits: u32) -> Self {
        CURTBUFAPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURTBUFAPTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Application Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Channel 0 current application transmit buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac1catx_br](index.html) module"]
pub struct ETH_DMAC1CATXBR_SPEC;
impl crate::RegisterSpec for ETH_DMAC1CATXBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac1catx_br::R](R) reader structure"]
impl crate::Readable for ETH_DMAC1CATXBR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_DMAC1CATxBR to value 0"]
impl crate::Resettable for ETH_DMAC1CATXBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
