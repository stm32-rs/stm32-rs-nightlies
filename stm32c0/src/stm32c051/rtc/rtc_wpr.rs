///Register `RTC_WPR` writer
pub type W = crate::W<RTC_WPRrs>;
///Field `KEY` writer - Write protection key This byte is written by software. Reading this byte always returns 0x00. Refer to RTC register write protection for a description of how to unlock RTC register write protection.
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<RTC_WPRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Write protection key This byte is written by software. Reading this byte always returns 0x00. Refer to RTC register write protection for a description of how to unlock RTC register write protection.
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, RTC_WPRrs> {
        KEY_W::new(self, 0)
    }
}
/**RTC write protection register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_wpr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RTC:RTC_WPR)*/
pub struct RTC_WPRrs;
impl crate::RegisterSpec for RTC_WPRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`rtc_wpr::W`](W) writer structure
impl crate::Writable for RTC_WPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTC_WPR to value 0
impl crate::Resettable for RTC_WPRrs {}
