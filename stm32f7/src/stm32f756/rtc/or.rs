///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
///Field `RTC_ALARM_TYPE` reader - RTC_ALARM on PC13 output type
pub type RTC_ALARM_TYPE_R = crate::BitReader;
///Field `RTC_ALARM_TYPE` writer - RTC_ALARM on PC13 output type
pub type RTC_ALARM_TYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTC_OUT_RMP` reader - RTC_OUT remap
pub type RTC_OUT_RMP_R = crate::BitReader;
///Field `RTC_OUT_RMP` writer - RTC_OUT remap
pub type RTC_OUT_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RTC_ALARM on PC13 output type
    #[inline(always)]
    pub fn rtc_alarm_type(&self) -> RTC_ALARM_TYPE_R {
        RTC_ALARM_TYPE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RTC_OUT remap
    #[inline(always)]
    pub fn rtc_out_rmp(&self) -> RTC_OUT_RMP_R {
        RTC_OUT_RMP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("rtc_alarm_type", &self.rtc_alarm_type())
            .field("rtc_out_rmp", &self.rtc_out_rmp())
            .finish()
    }
}
impl W {
    ///Bit 0 - RTC_ALARM on PC13 output type
    #[inline(always)]
    pub fn rtc_alarm_type(&mut self) -> RTC_ALARM_TYPE_W<'_, ORrs> {
        RTC_ALARM_TYPE_W::new(self, 0)
    }
    ///Bit 1 - RTC_OUT remap
    #[inline(always)]
    pub fn rtc_out_rmp(&mut self) -> RTC_OUT_RMP_W<'_, ORrs> {
        RTC_OUT_RMP_W::new(self, 1)
    }
}
/**option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F756.html#RTC:OR)*/
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
///`read()` method returns [`or::R`](R) reader structure
impl crate::Readable for ORrs {}
///`write(|w| ..)` method takes [`or::W`](W) writer structure
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for ORrs {}
