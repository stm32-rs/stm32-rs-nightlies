///Register `CPU_WAKEUPTIME` reader
pub type R = crate::R<CPU_WAKEUPTIMErs>;
///Register `CPU_WAKEUPTIME` writer
pub type W = crate::W<CPU_WAKEUPTIMErs>;
///Field `CPU_WAKEUPTIME` reader - (Absolute) Target time to wakeup the CPU.
pub type CPU_WAKEUPTIME_R = crate::FieldReader<u32>;
///Field `CPU_WAKEUPTIME` writer - (Absolute) Target time to wakeup the CPU.
pub type CPU_WAKEUPTIME_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    ///Bits 1:31 - (Absolute) Target time to wakeup the CPU.
    #[inline(always)]
    pub fn cpu_wakeuptime(&self) -> CPU_WAKEUPTIME_R {
        CPU_WAKEUPTIME_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_WAKEUPTIME")
            .field("cpu_wakeuptime", &self.cpu_wakeuptime())
            .finish()
    }
}
impl W {
    ///Bits 1:31 - (Absolute) Target time to wakeup the CPU.
    #[inline(always)]
    pub fn cpu_wakeuptime(&mut self) -> CPU_WAKEUPTIME_W<'_, CPU_WAKEUPTIMErs> {
        CPU_WAKEUPTIME_W::new(self, 1)
    }
}
/**CPU_WAKEUPTIME register

You can [`read`](crate::Reg::read) this register and get [`cpu_wakeuptime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_wakeuptime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RETAINED:CPU_WAKEUPTIME)*/
pub struct CPU_WAKEUPTIMErs;
impl crate::RegisterSpec for CPU_WAKEUPTIMErs {
    type Ux = u32;
}
///`read()` method returns [`cpu_wakeuptime::R`](R) reader structure
impl crate::Readable for CPU_WAKEUPTIMErs {}
///`write(|w| ..)` method takes [`cpu_wakeuptime::W`](W) writer structure
impl crate::Writable for CPU_WAKEUPTIMErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CPU_WAKEUPTIME to value 0
impl crate::Resettable for CPU_WAKEUPTIMErs {}
