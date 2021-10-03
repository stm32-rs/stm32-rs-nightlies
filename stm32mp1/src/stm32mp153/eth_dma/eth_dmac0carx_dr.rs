#[doc = "Register `ETH_DMAC0CARxDR` reader"]
pub struct R(crate::R<ETH_DMAC0CARXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC0CARXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC0CARXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC0CARXDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURRDESAPTR` reader - Application Transmit Descriptor Address Pointer"]
pub struct CURRDESAPTR_R(crate::FieldReader<u32, u32>);
impl CURRDESAPTR_R {
    pub(crate) fn new(bits: u32) -> Self {
        CURRDESAPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURRDESAPTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Application Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Channel 0 current application receive descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0carx_dr](index.html) module"]
pub struct ETH_DMAC0CARXDR_SPEC;
impl crate::RegisterSpec for ETH_DMAC0CARXDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac0carx_dr::R](R) reader structure"]
impl crate::Readable for ETH_DMAC0CARXDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_DMAC0CARxDR to value 0"]
impl crate::Resettable for ETH_DMAC0CARXDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
