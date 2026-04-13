///Register `CM0_SLEEP_REQUEST_MODE` reader
pub type R = crate::R<CM0_SLEEP_REQUEST_MODErs>;
///Register `CM0_SLEEP_REQUEST_MODE` writer
pub type W = crate::W<CM0_SLEEP_REQUEST_MODErs>;
///Field `CPU_WAKEUP_EN` reader - CPU wakeup enable:
pub type CPU_WAKEUP_EN_R = crate::BitReader;
///Field `CPU_WAKEUP_EN` writer - CPU wakeup enable:
pub type CPU_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_SLEEPING` reader - CPU sleeping control:
pub type FORCE_SLEEPING_R = crate::BitReader;
///Field `FORCE_SLEEPING` writer - CPU sleeping control:
pub type FORCE_SLEEPING_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 30 - CPU wakeup enable:
    #[inline(always)]
    pub fn cpu_wakeup_en(&self) -> CPU_WAKEUP_EN_R {
        CPU_WAKEUP_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - CPU sleeping control:
    #[inline(always)]
    pub fn force_sleeping(&self) -> FORCE_SLEEPING_R {
        FORCE_SLEEPING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM0_SLEEP_REQUEST_MODE")
            .field("cpu_wakeup_en", &self.cpu_wakeup_en())
            .field("force_sleeping", &self.force_sleeping())
            .finish()
    }
}
impl W {
    ///Bit 30 - CPU wakeup enable:
    #[inline(always)]
    pub fn cpu_wakeup_en(&mut self) -> CPU_WAKEUP_EN_W<'_, CM0_SLEEP_REQUEST_MODErs> {
        CPU_WAKEUP_EN_W::new(self, 30)
    }
    ///Bit 31 - CPU sleeping control:
    #[inline(always)]
    pub fn force_sleeping(&mut self) -> FORCE_SLEEPING_W<'_, CM0_SLEEP_REQUEST_MODErs> {
        FORCE_SLEEPING_W::new(self, 31)
    }
}
/**CM0_SLEEP_REQUEST_MODE register

You can [`read`](crate::Reg::read) this register and get [`cm0_sleep_request_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_sleep_request_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#WAKEUP:CM0_SLEEP_REQUEST_MODE)*/
pub struct CM0_SLEEP_REQUEST_MODErs;
impl crate::RegisterSpec for CM0_SLEEP_REQUEST_MODErs {
    type Ux = u32;
}
///`read()` method returns [`cm0_sleep_request_mode::R`](R) reader structure
impl crate::Readable for CM0_SLEEP_REQUEST_MODErs {}
///`write(|w| ..)` method takes [`cm0_sleep_request_mode::W`](W) writer structure
impl crate::Writable for CM0_SLEEP_REQUEST_MODErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CM0_SLEEP_REQUEST_MODE to value 0x8000_0007
impl crate::Resettable for CM0_SLEEP_REQUEST_MODErs {
    const RESET_VALUE: u32 = 0x8000_0007;
}
