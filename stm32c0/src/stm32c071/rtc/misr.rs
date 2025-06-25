///Register `MISR` reader
pub type R = crate::R<MISRrs>;
/**Alarm A masked flag This flag is set by hardware when the alarm A interrupt occurs.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAMF {
    ///1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm register (RTC_ALRMxR)
    Match = 1,
}
impl From<ALRAMF> for bool {
    #[inline(always)]
    fn from(variant: ALRAMF) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRAMF` reader - Alarm A masked flag This flag is set by hardware when the alarm A interrupt occurs.
pub type ALRAMF_R = crate::BitReader<ALRAMF>;
impl ALRAMF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ALRAMF> {
        match self.bits {
            true => Some(ALRAMF::Match),
            _ => None,
        }
    }
    ///This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm register (RTC_ALRMxR)
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ALRAMF::Match
    }
}
/**Timestamp masked flag This flag is set by hardware when a timestamp interrupt occurs.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSMF {
    ///1: This flag is set by hardware when a time-stamp event occurs
    TimestampEvent = 1,
}
impl From<TSMF> for bool {
    #[inline(always)]
    fn from(variant: TSMF) -> Self {
        variant as u8 != 0
    }
}
///Field `TSMF` reader - Timestamp masked flag This flag is set by hardware when a timestamp interrupt occurs.
pub type TSMF_R = crate::BitReader<TSMF>;
impl TSMF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSMF> {
        match self.bits {
            true => Some(TSMF::TimestampEvent),
            _ => None,
        }
    }
    ///This flag is set by hardware when a time-stamp event occurs
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        *self == TSMF::TimestampEvent
    }
}
/**Timestamp overflow masked flag This flag is set by hardware when a timestamp interrupt occurs while TSMF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSOVMF {
    ///1: This flag is set by hardware when a time-stamp event occurs while TSF is already set
    Overflow = 1,
}
impl From<TSOVMF> for bool {
    #[inline(always)]
    fn from(variant: TSOVMF) -> Self {
        variant as u8 != 0
    }
}
///Field `TSOVMF` reader - Timestamp overflow masked flag This flag is set by hardware when a timestamp interrupt occurs while TSMF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
pub type TSOVMF_R = crate::BitReader<TSOVMF>;
impl TSOVMF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSOVMF> {
        match self.bits {
            true => Some(TSOVMF::Overflow),
            _ => None,
        }
    }
    ///This flag is set by hardware when a time-stamp event occurs while TSF is already set
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == TSOVMF::Overflow
    }
}
impl R {
    ///Bit 0 - Alarm A masked flag This flag is set by hardware when the alarm A interrupt occurs.
    #[inline(always)]
    pub fn alramf(&self) -> ALRAMF_R {
        ALRAMF_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - Timestamp masked flag This flag is set by hardware when a timestamp interrupt occurs.
    #[inline(always)]
    pub fn tsmf(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timestamp overflow masked flag This flag is set by hardware when a timestamp interrupt occurs while TSMF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
    #[inline(always)]
    pub fn tsovmf(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISR")
            .field("alramf", &self.alramf())
            .field("tsmf", &self.tsmf())
            .field("tsovmf", &self.tsovmf())
            .finish()
    }
}
/**RTC masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#RTC:MISR)*/
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
///`read()` method returns [`misr::R`](R) reader structure
impl crate::Readable for MISRrs {}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISRrs {}
