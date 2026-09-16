///Register `RTC_MISR` reader
pub type R = crate::R<RTC_MISRrs>;
///Field `ALRAMF` reader - Alarm A masked flag This flag is set by hardware when the alarm A interrupt occurs.
pub type ALRAMF_R = crate::BitReader;
///Field `TSMF` reader - Timestamp masked flag This flag is set by hardware when a timestamp interrupt occurs.
pub type TSMF_R = crate::BitReader;
///Field `TSOVMF` reader - Timestamp overflow masked flag This flag is set by hardware when a timestamp interrupt occurs while TSMF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
pub type TSOVMF_R = crate::BitReader;
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
        f.debug_struct("RTC_MISR")
            .field("alramf", &self.alramf())
            .field("tsmf", &self.tsmf())
            .field("tsovmf", &self.tsovmf())
            .finish()
    }
}
/**RTC masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`rtc_misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#RTC:RTC_MISR)*/
pub struct RTC_MISRrs;
impl crate::RegisterSpec for RTC_MISRrs {
    type Ux = u32;
}
///`read()` method returns [`rtc_misr::R`](R) reader structure
impl crate::Readable for RTC_MISRrs {}
///`reset()` method sets RTC_MISR to value 0
impl crate::Resettable for RTC_MISRrs {}
