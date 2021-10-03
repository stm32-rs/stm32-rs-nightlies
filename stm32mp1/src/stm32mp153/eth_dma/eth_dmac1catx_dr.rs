#[doc = "Register `ETH_DMAC1CATxDR` reader"]
pub struct R(crate::R<ETH_DMAC1CATXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC1CATXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC1CATXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC1CATXDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURTDESAPTR` reader - Application Transmit Descriptor Address Pointer"]
pub struct CURTDESAPTR_R(crate::FieldReader<u32, u32>);
impl CURTDESAPTR_R {
    pub(crate) fn new(bits: u32) -> Self {
        CURTDESAPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURTDESAPTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Application Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CURTDESAPTR_R {
        CURTDESAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Channel current application transmit descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac1catx_dr](index.html) module"]
pub struct ETH_DMAC1CATXDR_SPEC;
impl crate::RegisterSpec for ETH_DMAC1CATXDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac1catx_dr::R](R) reader structure"]
impl crate::Readable for ETH_DMAC1CATXDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_DMAC1CATxDR to value 0"]
impl crate::Resettable for ETH_DMAC1CATXDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
