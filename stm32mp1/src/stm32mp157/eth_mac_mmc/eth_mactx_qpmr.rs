#[doc = "Register `ETH_MACTxQPMR` reader"]
pub struct R(crate::R<ETH_MACTXQPMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACTXQPMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACTXQPMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACTXQPMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PSTQ0` reader - PSTQ0"]
pub struct PSTQ0_R(crate::FieldReader<u8, u8>);
impl PSTQ0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSTQ0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSTQ0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSTQ1` reader - PSTQ1"]
pub struct PSTQ1_R(crate::FieldReader<u8, u8>);
impl PSTQ1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSTQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSTQ1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - PSTQ0"]
    #[inline(always)]
    pub fn pstq0(&self) -> PSTQ0_R {
        PSTQ0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PSTQ1"]
    #[inline(always)]
    pub fn pstq1(&self) -> PSTQ1_R {
        PSTQ1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "The transmit queue priority mapping 0 register contains the priority values assigned to Tx queue 0 and tx queue 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactx_qpmr](index.html) module"]
pub struct ETH_MACTXQPMR_SPEC;
impl crate::RegisterSpec for ETH_MACTXQPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mactx_qpmr::R](R) reader structure"]
impl crate::Readable for ETH_MACTXQPMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MACTxQPMR to value 0"]
impl crate::Resettable for ETH_MACTXQPMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
