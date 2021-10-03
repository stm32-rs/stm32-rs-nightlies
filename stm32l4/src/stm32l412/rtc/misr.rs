#[doc = "Register `MISR` reader"]
pub struct R(crate::R<MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ALRAMF` reader - Alarm A masked flag"]
pub struct ALRAMF_R(crate::FieldReader<bool, bool>);
impl ALRAMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRAMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRAMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRBMF` reader - Alarm B masked flag"]
pub struct ALRBMF_R(crate::FieldReader<bool, bool>);
impl ALRBMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRBMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRBMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUTMF` reader - Wakeup timer masked flag"]
pub struct WUTMF_R(crate::FieldReader<bool, bool>);
impl WUTMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUTMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUTMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSMF` reader - Timestamp masked flag"]
pub struct TSMF_R(crate::FieldReader<bool, bool>);
impl TSMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSOVMF` reader - Timestamp overflow masked flag"]
pub struct TSOVMF_R(crate::FieldReader<bool, bool>);
impl TSOVMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSOVMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSOVMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITSMF` reader - Internal timestamp masked flag"]
pub struct ITSMF_R(crate::FieldReader<bool, bool>);
impl ITSMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITSMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITSMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Alarm A masked flag"]
    #[inline(always)]
    pub fn alramf(&self) -> ALRAMF_R {
        ALRAMF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alarm B masked flag"]
    #[inline(always)]
    pub fn alrbmf(&self) -> ALRBMF_R {
        ALRBMF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer masked flag"]
    #[inline(always)]
    pub fn wutmf(&self) -> WUTMF_R {
        WUTMF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timestamp masked flag"]
    #[inline(always)]
    pub fn tsmf(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timestamp overflow masked flag"]
    #[inline(always)]
    pub fn tsovmf(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Internal timestamp masked flag"]
    #[inline(always)]
    pub fn itsmf(&self) -> ITSMF_R {
        ITSMF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "RTC masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misr](index.html) module"]
pub struct MISR_SPEC;
impl crate::RegisterSpec for MISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misr::R](R) reader structure"]
impl crate::Readable for MISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
