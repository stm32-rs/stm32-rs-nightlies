///Register `MISR` reader
pub type R = crate::R<MISRrs>;
/**Alarm %s masked flag

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
///Field `ALRMF(A,B)` reader - Alarm %s masked flag
pub type ALRMF_R = crate::BitReader<ALRAMF>;
impl ALRMF_R {
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
/**Wakeup timer masked flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTMF {
    ///1: This flag is set by hardware when the wakeup auto-reload counter reaches 0
    Zero = 1,
}
impl From<WUTMF> for bool {
    #[inline(always)]
    fn from(variant: WUTMF) -> Self {
        variant as u8 != 0
    }
}
///Field `WUTMF` reader - Wakeup timer masked flag
pub type WUTMF_R = crate::BitReader<WUTMF>;
impl WUTMF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUTMF> {
        match self.bits {
            true => Some(WUTMF::Zero),
            _ => None,
        }
    }
    ///This flag is set by hardware when the wakeup auto-reload counter reaches 0
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == WUTMF::Zero
    }
}
/**Timestamp masked flag

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
///Field `TSMF` reader - Timestamp masked flag
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
/**Timestamp overflow masked flag

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
///Field `TSOVMF` reader - Timestamp overflow masked flag
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
/**Internal timestamp masked flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITSMF {
    ///1: This flag is set by hardware when a timestamp on the internal event occurs
    TimestampEvent = 1,
}
impl From<ITSMF> for bool {
    #[inline(always)]
    fn from(variant: ITSMF) -> Self {
        variant as u8 != 0
    }
}
///Field `ITSMF` reader - Internal timestamp masked flag
pub type ITSMF_R = crate::BitReader<ITSMF>;
impl ITSMF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ITSMF> {
        match self.bits {
            true => Some(ITSMF::TimestampEvent),
            _ => None,
        }
    }
    ///This flag is set by hardware when a timestamp on the internal event occurs
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        *self == ITSMF::TimestampEvent
    }
}
/**SSR underflow masked flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSRUMF {
    ///1: This flag is set by hardware when the SSR rolls under 0. SSRUF is not set when SSCLR=1
    Underflow = 1,
}
impl From<SSRUMF> for bool {
    #[inline(always)]
    fn from(variant: SSRUMF) -> Self {
        variant as u8 != 0
    }
}
///Field `SSRUMF` reader - SSR underflow masked flag
pub type SSRUMF_R = crate::BitReader<SSRUMF>;
impl SSRUMF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SSRUMF> {
        match self.bits {
            true => Some(SSRUMF::Underflow),
            _ => None,
        }
    }
    ///This flag is set by hardware when the SSR rolls under 0. SSRUF is not set when SSCLR=1
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        *self == SSRUMF::Underflow
    }
}
impl R {
    ///Alarm (A,B) masked flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ALRAMF` field.</div>
    #[inline(always)]
    pub fn alrmf(&self, n: u8) -> ALRMF_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ALRMF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Alarm (A,B) masked flag
    #[inline(always)]
    pub fn alrmf_iter(&self) -> impl Iterator<Item = ALRMF_R> + '_ {
        (0..2).map(move |n| ALRMF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Alarm A masked flag
    #[inline(always)]
    pub fn alramf(&self) -> ALRMF_R {
        ALRMF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm B masked flag
    #[inline(always)]
    pub fn alrbmf(&self) -> ALRMF_R {
        ALRMF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup timer masked flag
    #[inline(always)]
    pub fn wutmf(&self) -> WUTMF_R {
        WUTMF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timestamp masked flag
    #[inline(always)]
    pub fn tsmf(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timestamp overflow masked flag
    #[inline(always)]
    pub fn tsovmf(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Internal timestamp masked flag
    #[inline(always)]
    pub fn itsmf(&self) -> ITSMF_R {
        ITSMF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SSR underflow masked flag
    #[inline(always)]
    pub fn ssrumf(&self) -> SSRUMF_R {
        SSRUMF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISR")
            .field("ssrumf", &self.ssrumf())
            .field("itsmf", &self.itsmf())
            .field("tsovmf", &self.tsovmf())
            .field("tsmf", &self.tsmf())
            .field("wutmf", &self.wutmf())
            .field("alramf", &self.alramf())
            .field("alrbmf", &self.alrbmf())
            .finish()
    }
}
/**Masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#RTC:MISR)*/
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
///`read()` method returns [`misr::R`](R) reader structure
impl crate::Readable for MISRrs {}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISRrs {}
