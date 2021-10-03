#[doc = "Register `ETH_MACTxTSSSR` reader"]
pub struct R(crate::R<ETH_MACTXTSSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACTXTSSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACTXTSSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACTXTSSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXTSSHI` reader - TXTSSHI"]
pub struct TXTSSHI_R(crate::FieldReader<u32, u32>);
impl TXTSSHI_R {
    pub(crate) fn new(bits: u32) -> Self {
        TXTSSHI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTSSHI_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - TXTSSHI"]
    #[inline(always)]
    pub fn txtsshi(&self) -> TXTSSHI_R {
        TXTSSHI_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "The register contains the higher 32 bits of the timestamp (in seconds) captured when a PTP packet is transmitted.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactx_tsssr](index.html) module"]
pub struct ETH_MACTXTSSSR_SPEC;
impl crate::RegisterSpec for ETH_MACTXTSSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mactx_tsssr::R](R) reader structure"]
impl crate::Readable for ETH_MACTXTSSSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MACTxTSSSR to value 0"]
impl crate::Resettable for ETH_MACTXTSSSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
