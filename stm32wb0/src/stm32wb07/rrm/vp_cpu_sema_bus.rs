///Register `VP_CPU_SEMA_BUS` reader
pub type R = crate::R<VP_CPU_SEMA_BUSrs>;
///Register `VP_CPU_SEMA_BUS` writer
pub type W = crate::W<VP_CPU_SEMA_BUSrs>;
///Field `TAKE_PRIO` reader - semaphore priority: priority value (between 0 and 7) of the take request.
pub type TAKE_PRIO_R = crate::FieldReader;
///Field `TAKE_PRIO` writer - semaphore priority: priority value (between 0 and 7) of the take request.
pub type TAKE_PRIO_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TAKE_REQ` reader - semaphore token request:
pub type TAKE_REQ_R = crate::BitReader;
///Field `TAKE_REQ` writer - semaphore token request:
pub type TAKE_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - semaphore priority: priority value (between 0 and 7) of the take request.
    #[inline(always)]
    pub fn take_prio(&self) -> TAKE_PRIO_R {
        TAKE_PRIO_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - semaphore token request:
    #[inline(always)]
    pub fn take_req(&self) -> TAKE_REQ_R {
        TAKE_REQ_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VP_CPU_SEMA_BUS")
            .field("take_prio", &self.take_prio())
            .field("take_req", &self.take_req())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - semaphore priority: priority value (between 0 and 7) of the take request.
    #[inline(always)]
    pub fn take_prio(&mut self) -> TAKE_PRIO_W<'_, VP_CPU_SEMA_BUSrs> {
        TAKE_PRIO_W::new(self, 0)
    }
    ///Bit 3 - semaphore token request:
    #[inline(always)]
    pub fn take_req(&mut self) -> TAKE_REQ_W<'_, VP_CPU_SEMA_BUSrs> {
        TAKE_REQ_W::new(self, 3)
    }
}
/**VP_CPU_SEMA_BUS register

You can [`read`](crate::Reg::read) this register and get [`vp_cpu_sema_bus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vp_cpu_sema_bus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RRM:VP_CPU_SEMA_BUS)*/
pub struct VP_CPU_SEMA_BUSrs;
impl crate::RegisterSpec for VP_CPU_SEMA_BUSrs {
    type Ux = u32;
}
///`read()` method returns [`vp_cpu_sema_bus::R`](R) reader structure
impl crate::Readable for VP_CPU_SEMA_BUSrs {}
///`write(|w| ..)` method takes [`vp_cpu_sema_bus::W`](W) writer structure
impl crate::Writable for VP_CPU_SEMA_BUSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VP_CPU_SEMA_BUS to value 0
impl crate::Resettable for VP_CPU_SEMA_BUSrs {}
