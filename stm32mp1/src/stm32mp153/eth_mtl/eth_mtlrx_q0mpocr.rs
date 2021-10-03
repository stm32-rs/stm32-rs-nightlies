#[doc = "Register `ETH_MTLRxQ0MPOCR` reader"]
pub struct R(crate::R<ETH_MTLRXQ0MPOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLRXQ0MPOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLRXQ0MPOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLRXQ0MPOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OVFPKTCNT` reader - OVFPKTCNT"]
pub struct OVFPKTCNT_R(crate::FieldReader<u16, u16>);
impl OVFPKTCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        OVFPKTCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFPKTCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVFCNTOVF` reader - OVFCNTOVF"]
pub struct OVFCNTOVF_R(crate::FieldReader<bool, bool>);
impl OVFCNTOVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVFCNTOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFCNTOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MISPKTCNT` reader - MISPKTCNT"]
pub struct MISPKTCNT_R(crate::FieldReader<u16, u16>);
impl MISPKTCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        MISPKTCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISPKTCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MISCNTOVF` reader - MISCNTOVF"]
pub struct MISCNTOVF_R(crate::FieldReader<bool, bool>);
impl MISCNTOVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MISCNTOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISCNTOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:10 - OVFPKTCNT"]
    #[inline(always)]
    pub fn ovfpktcnt(&self) -> OVFPKTCNT_R {
        OVFPKTCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - OVFCNTOVF"]
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - MISPKTCNT"]
    #[inline(always)]
    pub fn mispktcnt(&self) -> MISPKTCNT_R {
        MISPKTCNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 27 - MISCNTOVF"]
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
#[doc = "Rx queue 0 missed packet and overflow counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlrx_q0mpocr](index.html) module"]
pub struct ETH_MTLRXQ0MPOCR_SPEC;
impl crate::RegisterSpec for ETH_MTLRXQ0MPOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtlrx_q0mpocr::R](R) reader structure"]
impl crate::Readable for ETH_MTLRXQ0MPOCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MTLRxQ0MPOCR to value 0"]
impl crate::Resettable for ETH_MTLRXQ0MPOCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
