#[doc = "Register `ETH_DMAC1MFCR` reader"]
pub struct R(crate::R<ETH_DMAC1MFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC1MFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC1MFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC1MFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MFC` reader - Dropped Packet Counters"]
pub struct MFC_R(crate::FieldReader<u16, u16>);
impl MFC_R {
    pub(crate) fn new(bits: u16) -> Self {
        MFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MFCO` reader - Overflow status of the MFC Counter"]
pub struct MFCO_R(crate::FieldReader<bool, bool>);
impl MFCO_R {
    pub(crate) fn new(bits: bool) -> Self {
        MFCO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFCO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:10 - Dropped Packet Counters"]
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Overflow status of the MFC Counter"]
    #[inline(always)]
    pub fn mfco(&self) -> MFCO_R {
        MFCO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Channel missed frame count register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac1mfcr](index.html) module"]
pub struct ETH_DMAC1MFCR_SPEC;
impl crate::RegisterSpec for ETH_DMAC1MFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac1mfcr::R](R) reader structure"]
impl crate::Readable for ETH_DMAC1MFCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_DMAC1MFCR to value 0"]
impl crate::Resettable for ETH_DMAC1MFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
