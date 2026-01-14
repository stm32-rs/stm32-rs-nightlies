///Register `BLUE_WAKEUP_TIME` reader
pub type R = crate::R<BLUE_WAKEUP_TIMErs>;
///Register `BLUE_WAKEUP_TIME` writer
pub type W = crate::W<BLUE_WAKEUP_TIMErs>;
///Field `WAKEUP_TIME` reader - programmed wakeup time for the IP_BLE.
pub type WAKEUP_TIME_R = crate::FieldReader<u32>;
///Field `WAKEUP_TIME` writer - programmed wakeup time for the IP_BLE.
pub type WAKEUP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - programmed wakeup time for the IP_BLE.
    #[inline(always)]
    pub fn wakeup_time(&self) -> WAKEUP_TIME_R {
        WAKEUP_TIME_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLUE_WAKEUP_TIME")
            .field("wakeup_time", &self.wakeup_time())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - programmed wakeup time for the IP_BLE.
    #[inline(always)]
    pub fn wakeup_time(&mut self) -> WAKEUP_TIME_W<'_, BLUE_WAKEUP_TIMErs> {
        WAKEUP_TIME_W::new(self, 0)
    }
}
/**BLUE_WAKEUP_TIME register

You can [`read`](crate::Reg::read) this register and get [`blue_wakeup_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blue_wakeup_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#WAKEUP:BLUE_WAKEUP_TIME)*/
pub struct BLUE_WAKEUP_TIMErs;
impl crate::RegisterSpec for BLUE_WAKEUP_TIMErs {
    type Ux = u32;
}
///`read()` method returns [`blue_wakeup_time::R`](R) reader structure
impl crate::Readable for BLUE_WAKEUP_TIMErs {}
///`write(|w| ..)` method takes [`blue_wakeup_time::W`](W) writer structure
impl crate::Writable for BLUE_WAKEUP_TIMErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BLUE_WAKEUP_TIME to value 0
impl crate::Resettable for BLUE_WAKEUP_TIMErs {}
