///Register `WPR` reader
pub type R = crate::R<WPRrs>;
///Register `WPR` writer
pub type W = crate::W<WPRrs>;
///Field `KEY` writer - Write protection key This byte is written by software. Reading this byte always returns 0x00
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPR").finish()
    }
}
impl W {
    ///Bits 0:7 - Write protection key This byte is written by software. Reading this byte always returns 0x00
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, WPRrs> {
        KEY_W::new(self, 0)
    }
}
/**RTC_WPR register

You can [`read`](crate::Reg::read) this register and get [`wpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RTC:WPR)*/
pub struct WPRrs;
impl crate::RegisterSpec for WPRrs {
    type Ux = u32;
}
///`read()` method returns [`wpr::R`](R) reader structure
impl crate::Readable for WPRrs {}
///`write(|w| ..)` method takes [`wpr::W`](W) writer structure
impl crate::Writable for WPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPR to value 0
impl crate::Resettable for WPRrs {}
