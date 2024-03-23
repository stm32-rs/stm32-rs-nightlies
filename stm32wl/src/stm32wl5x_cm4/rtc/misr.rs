#[doc = "Register `MISR` reader"]
pub type R = crate::R<MISRrs>;
#[doc = "Alarm A masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAMF {
    #[doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)"]
    Match = 1,
}
impl From<ALRAMF> for bool {
    #[inline(always)]
    fn from(variant: ALRAMF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAMF` reader - Alarm A masked flag"]
pub type ALRAMF_R = crate::BitReader<ALRAMF>;
impl ALRAMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ALRAMF> {
        match self.bits {
            true => Some(ALRAMF::Match),
            _ => None,
        }
    }
    #[doc = "This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ALRAMF::Match
    }
}
#[doc = "Alarm B masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRBMF {
    #[doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm B register (RTC_ALRMBR)"]
    Match = 1,
}
impl From<ALRBMF> for bool {
    #[inline(always)]
    fn from(variant: ALRBMF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRBMF` reader - Alarm B masked flag"]
pub type ALRBMF_R = crate::BitReader<ALRBMF>;
impl ALRBMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ALRBMF> {
        match self.bits {
            true => Some(ALRBMF::Match),
            _ => None,
        }
    }
    #[doc = "This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm B register (RTC_ALRMBR)"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ALRBMF::Match
    }
}
#[doc = "Wakeup timer masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTMF {
    #[doc = "1: This flag is set by hardware when the wakeup auto-reload counter reaches 0"]
    Zero = 1,
}
impl From<WUTMF> for bool {
    #[inline(always)]
    fn from(variant: WUTMF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTMF` reader - Wakeup timer masked flag"]
pub type WUTMF_R = crate::BitReader<WUTMF>;
impl WUTMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUTMF> {
        match self.bits {
            true => Some(WUTMF::Zero),
            _ => None,
        }
    }
    #[doc = "This flag is set by hardware when the wakeup auto-reload counter reaches 0"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == WUTMF::Zero
    }
}
#[doc = "Timestamp masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSMF {
    #[doc = "1: This flag is set by hardware when a time-stamp event occurs"]
    TimestampEvent = 1,
}
impl From<TSMF> for bool {
    #[inline(always)]
    fn from(variant: TSMF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSMF` reader - Timestamp masked flag"]
pub type TSMF_R = crate::BitReader<TSMF>;
impl TSMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSMF> {
        match self.bits {
            true => Some(TSMF::TimestampEvent),
            _ => None,
        }
    }
    #[doc = "This flag is set by hardware when a time-stamp event occurs"]
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        *self == TSMF::TimestampEvent
    }
}
#[doc = "Timestamp overflow masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSOVMF {
    #[doc = "1: This flag is set by hardware when a time-stamp event occurs while TSF is already set"]
    Overflow = 1,
}
impl From<TSOVMF> for bool {
    #[inline(always)]
    fn from(variant: TSOVMF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSOVMF` reader - Timestamp overflow masked flag"]
pub type TSOVMF_R = crate::BitReader<TSOVMF>;
impl TSOVMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSOVMF> {
        match self.bits {
            true => Some(TSOVMF::Overflow),
            _ => None,
        }
    }
    #[doc = "This flag is set by hardware when a time-stamp event occurs while TSF is already set"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == TSOVMF::Overflow
    }
}
#[doc = "Internal timestamp masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITSMF {
    #[doc = "1: This flag is set by hardware when a timestamp on the internal event occurs"]
    TimestampEvent = 1,
}
impl From<ITSMF> for bool {
    #[inline(always)]
    fn from(variant: ITSMF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITSMF` reader - Internal timestamp masked flag"]
pub type ITSMF_R = crate::BitReader<ITSMF>;
impl ITSMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ITSMF> {
        match self.bits {
            true => Some(ITSMF::TimestampEvent),
            _ => None,
        }
    }
    #[doc = "This flag is set by hardware when a timestamp on the internal event occurs"]
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        *self == ITSMF::TimestampEvent
    }
}
#[doc = "SSR underflow masked flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSRUMF {
    #[doc = "1: This flag is set by hardware when the SSR rolls under 0. SSRUF is not set when SSCLR=1"]
    Underflow = 1,
}
impl From<SSRUMF> for bool {
    #[inline(always)]
    fn from(variant: SSRUMF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSRUMF` reader - SSR underflow masked flag"]
pub type SSRUMF_R = crate::BitReader<SSRUMF>;
impl SSRUMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SSRUMF> {
        match self.bits {
            true => Some(SSRUMF::Underflow),
            _ => None,
        }
    }
    #[doc = "This flag is set by hardware when the SSR rolls under 0. SSRUF is not set when SSCLR=1"]
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        *self == SSRUMF::Underflow
    }
}
impl R {
    #[doc = "Bit 0 - Alarm A masked flag"]
    #[inline(always)]
    pub fn alramf(&self) -> ALRAMF_R {
        ALRAMF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B masked flag"]
    #[inline(always)]
    pub fn alrbmf(&self) -> ALRBMF_R {
        ALRBMF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer masked flag"]
    #[inline(always)]
    pub fn wutmf(&self) -> WUTMF_R {
        WUTMF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp masked flag"]
    #[inline(always)]
    pub fn tsmf(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp overflow masked flag"]
    #[inline(always)]
    pub fn tsovmf(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Internal timestamp masked flag"]
    #[inline(always)]
    pub fn itsmf(&self) -> ITSMF_R {
        ITSMF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SSR underflow masked flag"]
    #[inline(always)]
    pub fn ssrumf(&self) -> SSRUMF_R {
        SSRUMF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misr::R`](R) reader structure"]
impl crate::Readable for MISRrs {}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MISRrs {
    const RESET_VALUE: u32 = 0;
}
