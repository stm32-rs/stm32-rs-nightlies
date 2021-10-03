#[doc = "Register `ETH_DMAC0CARxBR` reader"]
pub struct R(crate::R<ETH_DMAC0CARXBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC0CARXBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC0CARXBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC0CARXBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURRBUFAPTR` reader - Application Receive Buffer Address Pointer"]
pub struct CURRBUFAPTR_R(crate::FieldReader<u32, u32>);
impl CURRBUFAPTR_R {
    pub(crate) fn new(bits: u32) -> Self {
        CURRBUFAPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURRBUFAPTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Application Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn currbufaptr(&self) -> CURRBUFAPTR_R {
        CURRBUFAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Channel current application receive buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0carx_br](index.html) module"]
pub struct ETH_DMAC0CARXBR_SPEC;
impl crate::RegisterSpec for ETH_DMAC0CARXBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac0carx_br::R](R) reader structure"]
impl crate::Readable for ETH_DMAC0CARXBR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_DMAC0CARxBR to value 0"]
impl crate::Resettable for ETH_DMAC0CARXBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
