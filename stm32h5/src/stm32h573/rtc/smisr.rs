///Register `SMISR` reader
pub type R = crate::R<SMISRrs>;
///Field `ALRAMF` reader - Alarm A interrupt secure masked flag This flag is set by hardware when the alarm A secure interrupt occurs.
pub type ALRAMF_R = crate::BitReader;
///Field `ALRBMF` reader - Alarm B interrupt secure masked flag This flag is set by hardware when the alarm B secure interrupt occurs.
pub type ALRBMF_R = crate::BitReader;
///Field `WUTMF` reader - Wakeup timer interrupt secure masked flag This flag is set by hardware when the wakeup timer secure interrupt occurs. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again.
pub type WUTMF_R = crate::BitReader;
///Field `TSMF` reader - Timestamp interrupt secure masked flag This flag is set by hardware when a timestamp secure interrupt occurs. If ITSF flag is set, TSF must be cleared together with ITSF.
pub type TSMF_R = crate::BitReader;
///Field `TSOVMF` reader - Timestamp overflow interrupt secure masked flag This flag is set by hardware when a timestamp secure interrupt occurs while TSMF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
pub type TSOVMF_R = crate::BitReader;
///Field `ITSMF` reader - Internal timestamp interrupt secure masked flag This flag is set by hardware when a timestamp on the internal event occurs and timestamp secure interrupt is raised.
pub type ITSMF_R = crate::BitReader;
///Field `SSRUMF` reader - SSR underflow secure masked flag This flag is set by hardware when the SSR underflow secure interrupt occurs.
pub type SSRUMF_R = crate::BitReader;
impl R {
    ///Bit 0 - Alarm A interrupt secure masked flag This flag is set by hardware when the alarm A secure interrupt occurs.
    #[inline(always)]
    pub fn alramf(&self) -> ALRAMF_R {
        ALRAMF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm B interrupt secure masked flag This flag is set by hardware when the alarm B secure interrupt occurs.
    #[inline(always)]
    pub fn alrbmf(&self) -> ALRBMF_R {
        ALRBMF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup timer interrupt secure masked flag This flag is set by hardware when the wakeup timer secure interrupt occurs. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again.
    #[inline(always)]
    pub fn wutmf(&self) -> WUTMF_R {
        WUTMF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timestamp interrupt secure masked flag This flag is set by hardware when a timestamp secure interrupt occurs. If ITSF flag is set, TSF must be cleared together with ITSF.
    #[inline(always)]
    pub fn tsmf(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timestamp overflow interrupt secure masked flag This flag is set by hardware when a timestamp secure interrupt occurs while TSMF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
    #[inline(always)]
    pub fn tsovmf(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Internal timestamp interrupt secure masked flag This flag is set by hardware when a timestamp on the internal event occurs and timestamp secure interrupt is raised.
    #[inline(always)]
    pub fn itsmf(&self) -> ITSMF_R {
        ITSMF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SSR underflow secure masked flag This flag is set by hardware when the SSR underflow secure interrupt occurs.
    #[inline(always)]
    pub fn ssrumf(&self) -> SSRUMF_R {
        SSRUMF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMISR")
            .field("alramf", &self.alramf())
            .field("alrbmf", &self.alrbmf())
            .field("wutmf", &self.wutmf())
            .field("tsmf", &self.tsmf())
            .field("tsovmf", &self.tsovmf())
            .field("itsmf", &self.itsmf())
            .field("ssrumf", &self.ssrumf())
            .finish()
    }
}
/**RTC secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`smisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RTC:SMISR)*/
pub struct SMISRrs;
impl crate::RegisterSpec for SMISRrs {
    type Ux = u32;
}
///`read()` method returns [`smisr::R`](R) reader structure
impl crate::Readable for SMISRrs {}
///`reset()` method sets SMISR to value 0
impl crate::Resettable for SMISRrs {}
