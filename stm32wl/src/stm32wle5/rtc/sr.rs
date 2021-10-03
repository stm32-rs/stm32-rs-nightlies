#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "SSR underflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSRUF_A {
    #[doc = "1: This flag is set by hardware when the SSR rolls under 0. SSRUF is not set when SSCLR=1"]
    UNDERFLOW = 1,
}
impl From<SSRUF_A> for bool {
    #[inline(always)]
    fn from(variant: SSRUF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSRUF` reader - SSR underflow flag"]
pub struct SSRUF_R(crate::FieldReader<bool, SSRUF_A>);
impl SSRUF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSRUF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSRUF_A> {
        match self.bits {
            true => Some(SSRUF_A::UNDERFLOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNDERFLOW`"]
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        **self == SSRUF_A::UNDERFLOW
    }
}
impl core::ops::Deref for SSRUF_R {
    type Target = crate::FieldReader<bool, SSRUF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Internal timestamp flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITSF_A {
    #[doc = "1: This flag is set by hardware when a timestamp on the internal event occurs"]
    TIMESTAMPEVENT = 1,
}
impl From<ITSF_A> for bool {
    #[inline(always)]
    fn from(variant: ITSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITSF` reader - Internal timestamp flag"]
pub struct ITSF_R(crate::FieldReader<bool, ITSF_A>);
impl ITSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITSF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ITSF_A> {
        match self.bits {
            true => Some(ITSF_A::TIMESTAMPEVENT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIMESTAMPEVENT`"]
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        **self == ITSF_A::TIMESTAMPEVENT
    }
}
impl core::ops::Deref for ITSF_R {
    type Target = crate::FieldReader<bool, ITSF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Timestamp overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSOVF_A {
    #[doc = "1: This flag is set by hardware when a time-stamp event occurs while TSF is already set"]
    OVERFLOW = 1,
}
impl From<TSOVF_A> for bool {
    #[inline(always)]
    fn from(variant: TSOVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSOVF` reader - Timestamp overflow flag"]
pub struct TSOVF_R(crate::FieldReader<bool, TSOVF_A>);
impl TSOVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSOVF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSOVF_A> {
        match self.bits {
            true => Some(TSOVF_A::OVERFLOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OVERFLOW`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        **self == TSOVF_A::OVERFLOW
    }
}
impl core::ops::Deref for TSOVF_R {
    type Target = crate::FieldReader<bool, TSOVF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Timestamp flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSF_A {
    #[doc = "1: This flag is set by hardware when a time-stamp event occurs"]
    TIMESTAMPEVENT = 1,
}
impl From<TSF_A> for bool {
    #[inline(always)]
    fn from(variant: TSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSF` reader - Timestamp flag"]
pub struct TSF_R(crate::FieldReader<bool, TSF_A>);
impl TSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSF_A> {
        match self.bits {
            true => Some(TSF_A::TIMESTAMPEVENT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIMESTAMPEVENT`"]
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        **self == TSF_A::TIMESTAMPEVENT
    }
}
impl core::ops::Deref for TSF_R {
    type Target = crate::FieldReader<bool, TSF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Wakeup timer flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTF_A {
    #[doc = "1: This flag is set by hardware when the wakeup auto-reload counter reaches 0"]
    ZERO = 1,
}
impl From<WUTF_A> for bool {
    #[inline(always)]
    fn from(variant: WUTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTF` reader - Wakeup timer flag"]
pub struct WUTF_R(crate::FieldReader<bool, WUTF_A>);
impl WUTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WUTF_A> {
        match self.bits {
            true => Some(WUTF_A::ZERO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == WUTF_A::ZERO
    }
}
impl core::ops::Deref for WUTF_R {
    type Target = crate::FieldReader<bool, WUTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Alarm B flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRBF_A {
    #[doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm B register (RTC_ALRMBR)"]
    MATCH = 1,
}
impl From<ALRBF_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRBF` reader - Alarm B flag"]
pub struct ALRBF_R(crate::FieldReader<bool, ALRBF_A>);
impl ALRBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRBF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALRBF_A> {
        match self.bits {
            true => Some(ALRBF_A::MATCH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        **self == ALRBF_A::MATCH
    }
}
impl core::ops::Deref for ALRBF_R {
    type Target = crate::FieldReader<bool, ALRBF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Alarm A flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRAF_A {
    #[doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)"]
    MATCH = 1,
}
impl From<ALRAF_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAF` reader - Alarm A flag"]
pub struct ALRAF_R(crate::FieldReader<bool, ALRAF_A>);
impl ALRAF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRAF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALRAF_A> {
        match self.bits {
            true => Some(ALRAF_A::MATCH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        **self == ALRAF_A::MATCH
    }
}
impl core::ops::Deref for ALRAF_R {
    type Target = crate::FieldReader<bool, ALRAF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 6 - SSR underflow flag"]
    #[inline(always)]
    pub fn ssruf(&self) -> SSRUF_R {
        SSRUF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Internal timestamp flag"]
    #[inline(always)]
    pub fn itsf(&self) -> ITSF_R {
        ITSF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timestamp overflow flag"]
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timestamp flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer flag"]
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alarm B flag"]
    #[inline(always)]
    pub fn alrbf(&self) -> ALRBF_R {
        ALRBF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Alarm A flag"]
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Status register (interrupts)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
