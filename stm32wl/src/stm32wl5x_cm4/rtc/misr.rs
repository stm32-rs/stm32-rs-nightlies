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
#[doc = "SSR underflow masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSRUMF_A {
    #[doc = "1: This flag is set by hardware when the SSR rolls under 0. SSRUF is not set when SSCLR=1"]
    UNDERFLOW = 1,
}
impl From<SSRUMF_A> for bool {
    #[inline(always)]
    fn from(variant: SSRUMF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSRUMF` reader - SSR underflow masked flag"]
pub struct SSRUMF_R(crate::FieldReader<bool, SSRUMF_A>);
impl SSRUMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSRUMF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSRUMF_A> {
        match self.bits {
            true => Some(SSRUMF_A::UNDERFLOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNDERFLOW`"]
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        **self == SSRUMF_A::UNDERFLOW
    }
}
impl core::ops::Deref for SSRUMF_R {
    type Target = crate::FieldReader<bool, SSRUMF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Internal timestamp masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITSMF_A {
    #[doc = "1: This flag is set by hardware when a timestamp on the internal event occurs"]
    TIMESTAMPEVENT = 1,
}
impl From<ITSMF_A> for bool {
    #[inline(always)]
    fn from(variant: ITSMF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITSMF` reader - Internal timestamp masked flag"]
pub struct ITSMF_R(crate::FieldReader<bool, ITSMF_A>);
impl ITSMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITSMF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ITSMF_A> {
        match self.bits {
            true => Some(ITSMF_A::TIMESTAMPEVENT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIMESTAMPEVENT`"]
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        **self == ITSMF_A::TIMESTAMPEVENT
    }
}
impl core::ops::Deref for ITSMF_R {
    type Target = crate::FieldReader<bool, ITSMF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Timestamp overflow masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSOVMF_A {
    #[doc = "1: This flag is set by hardware when a time-stamp event occurs while TSF is already set"]
    OVERFLOW = 1,
}
impl From<TSOVMF_A> for bool {
    #[inline(always)]
    fn from(variant: TSOVMF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSOVMF` reader - Timestamp overflow masked flag"]
pub struct TSOVMF_R(crate::FieldReader<bool, TSOVMF_A>);
impl TSOVMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSOVMF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSOVMF_A> {
        match self.bits {
            true => Some(TSOVMF_A::OVERFLOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OVERFLOW`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        **self == TSOVMF_A::OVERFLOW
    }
}
impl core::ops::Deref for TSOVMF_R {
    type Target = crate::FieldReader<bool, TSOVMF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Timestamp masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSMF_A {
    #[doc = "1: This flag is set by hardware when a time-stamp event occurs"]
    TIMESTAMPEVENT = 1,
}
impl From<TSMF_A> for bool {
    #[inline(always)]
    fn from(variant: TSMF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSMF` reader - Timestamp masked flag"]
pub struct TSMF_R(crate::FieldReader<bool, TSMF_A>);
impl TSMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSMF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSMF_A> {
        match self.bits {
            true => Some(TSMF_A::TIMESTAMPEVENT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIMESTAMPEVENT`"]
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        **self == TSMF_A::TIMESTAMPEVENT
    }
}
impl core::ops::Deref for TSMF_R {
    type Target = crate::FieldReader<bool, TSMF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Wakeup timer masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTMF_A {
    #[doc = "1: This flag is set by hardware when the wakeup auto-reload counter reaches 0"]
    ZERO = 1,
}
impl From<WUTMF_A> for bool {
    #[inline(always)]
    fn from(variant: WUTMF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTMF` reader - Wakeup timer masked flag"]
pub struct WUTMF_R(crate::FieldReader<bool, WUTMF_A>);
impl WUTMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUTMF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WUTMF_A> {
        match self.bits {
            true => Some(WUTMF_A::ZERO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == WUTMF_A::ZERO
    }
}
impl core::ops::Deref for WUTMF_R {
    type Target = crate::FieldReader<bool, WUTMF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Alarm B masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRBMF_A {
    #[doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm B register (RTC_ALRMBR)"]
    MATCH = 1,
}
impl From<ALRBMF_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBMF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRBMF` reader - Alarm B masked flag"]
pub struct ALRBMF_R(crate::FieldReader<bool, ALRBMF_A>);
impl ALRBMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRBMF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALRBMF_A> {
        match self.bits {
            true => Some(ALRBMF_A::MATCH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        **self == ALRBMF_A::MATCH
    }
}
impl core::ops::Deref for ALRBMF_R {
    type Target = crate::FieldReader<bool, ALRBMF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Alarm A masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRAMF_A {
    #[doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)"]
    MATCH = 1,
}
impl From<ALRAMF_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAMF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAMF` reader - Alarm A masked flag"]
pub struct ALRAMF_R(crate::FieldReader<bool, ALRAMF_A>);
impl ALRAMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRAMF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALRAMF_A> {
        match self.bits {
            true => Some(ALRAMF_A::MATCH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        **self == ALRAMF_A::MATCH
    }
}
impl core::ops::Deref for ALRAMF_R {
    type Target = crate::FieldReader<bool, ALRAMF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 6 - SSR underflow masked flag"]
    #[inline(always)]
    pub fn ssrumf(&self) -> SSRUMF_R {
        SSRUMF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Internal timestamp masked flag"]
    #[inline(always)]
    pub fn itsmf(&self) -> ITSMF_R {
        ITSMF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timestamp overflow masked flag"]
    #[inline(always)]
    pub fn tsovmf(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timestamp masked flag"]
    #[inline(always)]
    pub fn tsmf(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer masked flag"]
    #[inline(always)]
    pub fn wutmf(&self) -> WUTMF_R {
        WUTMF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alarm B masked flag"]
    #[inline(always)]
    pub fn alrbmf(&self) -> ALRBMF_R {
        ALRBMF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Alarm A masked flag"]
    #[inline(always)]
    pub fn alramf(&self) -> ALRAMF_R {
        ALRAMF_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misr](index.html) module"]
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
