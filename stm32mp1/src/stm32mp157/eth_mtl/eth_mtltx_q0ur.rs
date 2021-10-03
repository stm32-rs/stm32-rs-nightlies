#[doc = "Register `ETH_MTLTxQ0UR` reader"]
pub struct R(crate::R<ETH_MTLTXQ0UR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLTXQ0UR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLTXQ0UR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLTXQ0UR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UFFRMCNT` reader - UFFRMCNT"]
pub struct UFFRMCNT_R(crate::FieldReader<u16, u16>);
impl UFFRMCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        UFFRMCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UFFRMCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UFCNTOVF` reader - UFCNTOVF"]
pub struct UFCNTOVF_R(crate::FieldReader<bool, bool>);
impl UFCNTOVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        UFCNTOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UFCNTOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:10 - UFFRMCNT"]
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UFFRMCNT_R {
        UFFRMCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - UFCNTOVF"]
    #[inline(always)]
    pub fn ufcntovf(&self) -> UFCNTOVF_R {
        UFCNTOVF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
#[doc = "Tx queue 0 underflow register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q0ur](index.html) module"]
pub struct ETH_MTLTXQ0UR_SPEC;
impl crate::RegisterSpec for ETH_MTLTXQ0UR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtltx_q0ur::R](R) reader structure"]
impl crate::Readable for ETH_MTLTXQ0UR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MTLTxQ0UR to value 0"]
impl crate::Resettable for ETH_MTLTXQ0UR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
