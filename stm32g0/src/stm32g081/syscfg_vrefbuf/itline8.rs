#[doc = "Register `ITLINE8` reader"]
pub struct R(crate::R<ITLINE8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UCPD1` reader - UCPD1"]
pub struct UCPD1_R(crate::FieldReader<bool, bool>);
impl UCPD1_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCPD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCPD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCPD2` reader - UCPD2"]
pub struct UCPD2_R(crate::FieldReader<bool, bool>);
impl UCPD2_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCPD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCPD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - UCPD1"]
    #[inline(always)]
    pub fn ucpd1(&self) -> UCPD1_R {
        UCPD1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UCPD2"]
    #[inline(always)]
    pub fn ucpd2(&self) -> UCPD2_R {
        UCPD2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "interrupt line 8 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline8](index.html) module"]
pub struct ITLINE8_SPEC;
impl crate::RegisterSpec for ITLINE8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline8::R](R) reader structure"]
impl crate::Readable for ITLINE8_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE8 to value 0"]
impl crate::Resettable for ITLINE8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
