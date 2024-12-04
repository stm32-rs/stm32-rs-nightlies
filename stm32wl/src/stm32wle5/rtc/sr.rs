///Register `SR` reader
pub type R = crate::R<SRrs>;
/**Alarm A flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAF {
    ///1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)
    Match = 1,
}
impl From<ALRAF> for bool {
    #[inline(always)]
    fn from(variant: ALRAF) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRAF` reader - Alarm A flag
pub type ALRAF_R = crate::BitReader<ALRAF>;
impl ALRAF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ALRAF> {
        match self.bits {
            true => Some(ALRAF::Match),
            _ => None,
        }
    }
    ///This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ALRAF::Match
    }
}
/**Alarm B flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRBF {
    ///1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm B register (RTC_ALRMBR)
    Match = 1,
}
impl From<ALRBF> for bool {
    #[inline(always)]
    fn from(variant: ALRBF) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRBF` reader - Alarm B flag
pub type ALRBF_R = crate::BitReader<ALRBF>;
impl ALRBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ALRBF> {
        match self.bits {
            true => Some(ALRBF::Match),
            _ => None,
        }
    }
    ///This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm B register (RTC_ALRMBR)
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ALRBF::Match
    }
}
/**Wakeup timer flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTF {
    ///1: This flag is set by hardware when the wakeup auto-reload counter reaches 0
    Zero = 1,
}
impl From<WUTF> for bool {
    #[inline(always)]
    fn from(variant: WUTF) -> Self {
        variant as u8 != 0
    }
}
///Field `WUTF` reader - Wakeup timer flag
pub type WUTF_R = crate::BitReader<WUTF>;
impl WUTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUTF> {
        match self.bits {
            true => Some(WUTF::Zero),
            _ => None,
        }
    }
    ///This flag is set by hardware when the wakeup auto-reload counter reaches 0
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == WUTF::Zero
    }
}
/**Timestamp flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSF {
    ///1: This flag is set by hardware when a time-stamp event occurs
    TimestampEvent = 1,
}
impl From<TSF> for bool {
    #[inline(always)]
    fn from(variant: TSF) -> Self {
        variant as u8 != 0
    }
}
///Field `TSF` reader - Timestamp flag
pub type TSF_R = crate::BitReader<TSF>;
impl TSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSF> {
        match self.bits {
            true => Some(TSF::TimestampEvent),
            _ => None,
        }
    }
    ///This flag is set by hardware when a time-stamp event occurs
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        *self == TSF::TimestampEvent
    }
}
/**Timestamp overflow flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSOVF {
    ///1: This flag is set by hardware when a time-stamp event occurs while TSF is already set
    Overflow = 1,
}
impl From<TSOVF> for bool {
    #[inline(always)]
    fn from(variant: TSOVF) -> Self {
        variant as u8 != 0
    }
}
///Field `TSOVF` reader - Timestamp overflow flag
pub type TSOVF_R = crate::BitReader<TSOVF>;
impl TSOVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSOVF> {
        match self.bits {
            true => Some(TSOVF::Overflow),
            _ => None,
        }
    }
    ///This flag is set by hardware when a time-stamp event occurs while TSF is already set
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == TSOVF::Overflow
    }
}
/**Internal timestamp flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITSF {
    ///1: This flag is set by hardware when a timestamp on the internal event occurs
    TimestampEvent = 1,
}
impl From<ITSF> for bool {
    #[inline(always)]
    fn from(variant: ITSF) -> Self {
        variant as u8 != 0
    }
}
///Field `ITSF` reader - Internal timestamp flag
pub type ITSF_R = crate::BitReader<ITSF>;
impl ITSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ITSF> {
        match self.bits {
            true => Some(ITSF::TimestampEvent),
            _ => None,
        }
    }
    ///This flag is set by hardware when a timestamp on the internal event occurs
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        *self == ITSF::TimestampEvent
    }
}
/**SSR underflow flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSRUF {
    ///1: This flag is set by hardware when the SSR rolls under 0. SSRUF is not set when SSCLR=1
    Underflow = 1,
}
impl From<SSRUF> for bool {
    #[inline(always)]
    fn from(variant: SSRUF) -> Self {
        variant as u8 != 0
    }
}
///Field `SSRUF` reader - SSR underflow flag
pub type SSRUF_R = crate::BitReader<SSRUF>;
impl SSRUF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SSRUF> {
        match self.bits {
            true => Some(SSRUF::Underflow),
            _ => None,
        }
    }
    ///This flag is set by hardware when the SSR rolls under 0. SSRUF is not set when SSCLR=1
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        *self == SSRUF::Underflow
    }
}
impl R {
    ///Bit 0 - Alarm A flag
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm B flag
    #[inline(always)]
    pub fn alrbf(&self) -> ALRBF_R {
        ALRBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup timer flag
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timestamp flag
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timestamp overflow flag
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Internal timestamp flag
    #[inline(always)]
    pub fn itsf(&self) -> ITSF_R {
        ITSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SSR underflow flag
    #[inline(always)]
    pub fn ssruf(&self) -> SSRUF_R {
        SSRUF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("ssruf", &self.ssruf())
            .field("itsf", &self.itsf())
            .field("tsovf", &self.tsovf())
            .field("tsf", &self.tsf())
            .field("wutf", &self.wutf())
            .field("alrbf", &self.alrbf())
            .field("alraf", &self.alraf())
            .finish()
    }
}
/**Status register (interrupts)

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#RTC:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
