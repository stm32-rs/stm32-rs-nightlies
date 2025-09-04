///Register `SR` reader
pub type R = crate::R<SRrs>;
/**Alarm %s flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAF {
    ///1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm register (RTC_ALRxBR)
    Match = 1,
}
impl From<ALRAF> for bool {
    #[inline(always)]
    fn from(variant: ALRAF) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRF(A,B)` reader - Alarm %s flag
pub type ALRF_R = crate::BitReader<ALRAF>;
impl ALRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ALRAF> {
        match self.bits {
            true => Some(ALRAF::Match),
            _ => None,
        }
    }
    ///This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm register (RTC_ALRxBR)
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ALRAF::Match
    }
}
/**Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches 0. If WUTOCLR\[15:0\] is different from 0x0000, WUTF is cleared by hardware when the wakeup auto-reload counter reaches WUTOCLR value. If WUTOCLR\[15:0\] is 0x0000, WUTF must be cleared by software. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again.

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
///Field `WUTF` reader - Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches 0. If WUTOCLR\[15:0\] is different from 0x0000, WUTF is cleared by hardware when the wakeup auto-reload counter reaches WUTOCLR value. If WUTOCLR\[15:0\] is 0x0000, WUTF must be cleared by software. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again.
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
/**Timestamp flag This flag is set by hardware when a timestamp event occurs. If ITSF flag is set, TSF must be cleared together with ITSF. Note: TSF is not set if TAMPTS = 1 and the tamper flag is read during the 3 ck_apre cycles following tamper event. Refer to for more details.

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
///Field `TSF` reader - Timestamp flag This flag is set by hardware when a timestamp event occurs. If ITSF flag is set, TSF must be cleared together with ITSF. Note: TSF is not set if TAMPTS = 1 and the tamper flag is read during the 3 ck_apre cycles following tamper event. Refer to for more details.
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
/**Timestamp overflow flag This flag is set by hardware when a timestamp event occurs while TSF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.

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
///Field `TSOVF` reader - Timestamp overflow flag This flag is set by hardware when a timestamp event occurs while TSF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
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
/**Internal timestamp flag This flag is set by hardware when a timestamp on the internal event occurs.

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
///Field `ITSF` reader - Internal timestamp flag This flag is set by hardware when a timestamp on the internal event occurs.
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
///Field `SSRUF` reader - SSR underflow flag This flag is set by hardware when the SSR rolls under 0. SSRUF is not set when SSCLR=1.
pub type SSRUF_R = crate::BitReader;
impl R {
    ///Alarm (A,B) flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ALRAF` field.</div>
    #[inline(always)]
    pub fn alrf(&self, n: u8) -> ALRF_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ALRF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Alarm (A,B) flag
    #[inline(always)]
    pub fn alrf_iter(&self) -> impl Iterator<Item = ALRF_R> + '_ {
        (0..2).map(move |n| ALRF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Alarm A flag
    #[inline(always)]
    pub fn alraf(&self) -> ALRF_R {
        ALRF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm B flag
    #[inline(always)]
    pub fn alrbf(&self) -> ALRF_R {
        ALRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches 0. If WUTOCLR\[15:0\] is different from 0x0000, WUTF is cleared by hardware when the wakeup auto-reload counter reaches WUTOCLR value. If WUTOCLR\[15:0\] is 0x0000, WUTF must be cleared by software. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again.
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timestamp flag This flag is set by hardware when a timestamp event occurs. If ITSF flag is set, TSF must be cleared together with ITSF. Note: TSF is not set if TAMPTS = 1 and the tamper flag is read during the 3 ck_apre cycles following tamper event. Refer to for more details.
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timestamp overflow flag This flag is set by hardware when a timestamp event occurs while TSF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Internal timestamp flag This flag is set by hardware when a timestamp on the internal event occurs.
    #[inline(always)]
    pub fn itsf(&self) -> ITSF_R {
        ITSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SSR underflow flag This flag is set by hardware when the SSR rolls under 0. SSRUF is not set when SSCLR=1.
    #[inline(always)]
    pub fn ssruf(&self) -> SSRUF_R {
        SSRUF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("alraf", &self.alraf())
            .field("alrbf", &self.alrbf())
            .field("wutf", &self.wutf())
            .field("tsf", &self.tsf())
            .field("tsovf", &self.tsovf())
            .field("itsf", &self.itsf())
            .field("ssruf", &self.ssruf())
            .finish()
    }
}
/**RTC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#RTC:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
