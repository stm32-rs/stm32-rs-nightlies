///Register `WPR` writer
pub type W = crate::W<WPRrs>;
///Field `KEY` writer - Write protection key
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<WPRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Write protection key
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, WPRrs> {
        KEY_W::new(self, 0)
    }
}
/**write protection register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#RTC:WPR)*/
pub struct WPRrs;
impl crate::RegisterSpec for WPRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`wpr::W`](W) writer structure
impl crate::Writable for WPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPR to value 0
impl crate::Resettable for WPRrs {}
