#[doc = "Register `MTLTxQUR` reader"]
pub struct R(crate::R<MTLTXQUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLTXQUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLTXQUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLTXQUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UFFRMCNT` reader - Underflow Packet Counter"]
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
#[doc = "Field `UFCNTOVF` reader - Overflow Bit for Underflow Packet Counter"]
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
    #[doc = "Bits 0:10 - Underflow Packet Counter"]
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UFFRMCNT_R {
        UFFRMCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Overflow Bit for Underflow Packet Counter"]
    #[inline(always)]
    pub fn ufcntovf(&self) -> UFCNTOVF_R {
        UFCNTOVF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
#[doc = "Tx queue underflow register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtltx_qur](index.html) module"]
pub struct MTLTXQUR_SPEC;
impl crate::RegisterSpec for MTLTXQUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtltx_qur::R](R) reader structure"]
impl crate::Readable for MTLTXQUR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MTLTxQUR to value 0"]
impl crate::Resettable for MTLTXQUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
