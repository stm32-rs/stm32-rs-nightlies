#[doc = "Register `ETH_MACATSSR` reader"]
pub struct R(crate::R<ETH_MACATSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACATSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACATSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACATSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AUXTSHI` reader - AUXTSHI"]
pub struct AUXTSHI_R(crate::FieldReader<u32, u32>);
impl AUXTSHI_R {
    pub(crate) fn new(bits: u32) -> Self {
        AUXTSHI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXTSHI_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - AUXTSHI"]
    #[inline(always)]
    pub fn auxtshi(&self) -> AUXTSHI_R {
        AUXTSHI_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "The Auxiliary Timestamp - Seconds register contains the lower 32 bits of the Seconds field of the auxiliary timestamp register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macatssr](index.html) module"]
pub struct ETH_MACATSSR_SPEC;
impl crate::RegisterSpec for ETH_MACATSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macatssr::R](R) reader structure"]
impl crate::Readable for ETH_MACATSSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MACATSSR to value 0"]
impl crate::Resettable for ETH_MACATSSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
