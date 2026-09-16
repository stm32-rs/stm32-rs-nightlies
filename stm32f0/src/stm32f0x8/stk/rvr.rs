///Register `RVR` reader
pub type R = crate::R<RVRrs>;
///Register `RVR` writer
pub type W = crate::W<RVRrs>;
///Field `RELOAD` reader - RELOAD value
pub type RELOAD_R = crate::FieldReader<u32>;
///Field `RELOAD` writer - RELOAD value
pub type RELOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - RELOAD value
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RVR")
            .field("reload", &self.reload())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - RELOAD value
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W<'_, RVRrs> {
        RELOAD_W::new(self, 0)
    }
}
/**SysTick reload value register

You can [`read`](crate::Reg::read) this register and get [`rvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x8.html#STK:RVR)*/
pub struct RVRrs;
impl crate::RegisterSpec for RVRrs {
    type Ux = u32;
}
///`read()` method returns [`rvr::R`](R) reader structure
impl crate::Readable for RVRrs {}
///`write(|w| ..)` method takes [`rvr::W`](W) writer structure
impl crate::Writable for RVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RVR to value 0
impl crate::Resettable for RVRrs {}
