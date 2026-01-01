///Register `RTC_SR` reader
pub type R = crate::R<RTC_SRrs>;
///Field `ALRAF` reader - Alarm A flag This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the alarm A register (RTC_ALRMAR).
pub type ALRAF_R = crate::BitReader;
///Field `TSF` reader - Timestamp flag This flag is set by hardware when a timestamp event occurs.
pub type TSF_R = crate::BitReader;
///Field `TSOVF` reader - Timestamp overflow flag This flag is set by hardware when a timestamp event occurs while TSF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
pub type TSOVF_R = crate::BitReader;
impl R {
    ///Bit 0 - Alarm A flag This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the alarm A register (RTC_ALRMAR).
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - Timestamp flag This flag is set by hardware when a timestamp event occurs.
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timestamp overflow flag This flag is set by hardware when a timestamp event occurs while TSF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_SR")
            .field("alraf", &self.alraf())
            .field("tsf", &self.tsf())
            .field("tsovf", &self.tsovf())
            .finish()
    }
}
/**RTC status register

You can [`read`](crate::Reg::read) this register and get [`rtc_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RTC:RTC_SR)*/
pub struct RTC_SRrs;
impl crate::RegisterSpec for RTC_SRrs {
    type Ux = u32;
}
///`read()` method returns [`rtc_sr::R`](R) reader structure
impl crate::Readable for RTC_SRrs {}
///`reset()` method sets RTC_SR to value 0
impl crate::Resettable for RTC_SRrs {}
