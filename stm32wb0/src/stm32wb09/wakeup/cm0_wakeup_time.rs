///Register `CM0_WAKEUP_TIME` reader
pub type R = crate::R<CM0_WAKEUP_TIMErs>;
///Register `CM0_WAKEUP_TIME` writer
pub type W = crate::W<CM0_WAKEUP_TIMErs>;
///Field `WAKEUP_TIME` reader - programmed wakeup time for CPU.
pub type WAKEUP_TIME_R = crate::FieldReader<u32>;
///Field `WAKEUP_TIME` writer - programmed wakeup time for CPU.
pub type WAKEUP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 4:31 - programmed wakeup time for CPU.
    #[inline(always)]
    pub fn wakeup_time(&self) -> WAKEUP_TIME_R {
        WAKEUP_TIME_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM0_WAKEUP_TIME")
            .field("wakeup_time", &self.wakeup_time())
            .finish()
    }
}
impl W {
    ///Bits 4:31 - programmed wakeup time for CPU.
    #[inline(always)]
    pub fn wakeup_time(&mut self) -> WAKEUP_TIME_W<'_, CM0_WAKEUP_TIMErs> {
        WAKEUP_TIME_W::new(self, 4)
    }
}
/**CM0_WAKEUP_TIME register

You can [`read`](crate::Reg::read) this register and get [`cm0_wakeup_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_wakeup_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#WAKEUP:CM0_WAKEUP_TIME)*/
pub struct CM0_WAKEUP_TIMErs;
impl crate::RegisterSpec for CM0_WAKEUP_TIMErs {
    type Ux = u32;
}
///`read()` method returns [`cm0_wakeup_time::R`](R) reader structure
impl crate::Readable for CM0_WAKEUP_TIMErs {}
///`write(|w| ..)` method takes [`cm0_wakeup_time::W`](W) writer structure
impl crate::Writable for CM0_WAKEUP_TIMErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CM0_WAKEUP_TIME to value 0
impl crate::Resettable for CM0_WAKEUP_TIMErs {}
