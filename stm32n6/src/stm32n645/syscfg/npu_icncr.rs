///Register `NPU_ICNCR` reader
pub type R = crate::R<NPU_ICNCRrs>;
///Register `NPU_ICNCR` writer
pub type W = crate::W<NPU_ICNCRrs>;
///Field `INTERLEAVING_ACTIVE` reader - Control interleaving on NPU RAMs
pub type INTERLEAVING_ACTIVE_R = crate::BitReader;
///Field `INTERLEAVING_ACTIVE` writer - Control interleaving on NPU RAMs
pub type INTERLEAVING_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Control interleaving on NPU RAMs
    #[inline(always)]
    pub fn interleaving_active(&self) -> INTERLEAVING_ACTIVE_R {
        INTERLEAVING_ACTIVE_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NPU_ICNCR")
            .field("interleaving_active", &self.interleaving_active())
            .finish()
    }
}
impl W {
    ///Bit 0 - Control interleaving on NPU RAMs
    #[inline(always)]
    pub fn interleaving_active(&mut self) -> INTERLEAVING_ACTIVE_W<'_, NPU_ICNCRrs> {
        INTERLEAVING_ACTIVE_W::new(self, 0)
    }
}
/**SYSCFG NPU RAM interleaving control register

You can [`read`](crate::Reg::read) this register and get [`npu_icncr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`npu_icncr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#SYSCFG:NPU_ICNCR)*/
pub struct NPU_ICNCRrs;
impl crate::RegisterSpec for NPU_ICNCRrs {
    type Ux = u32;
}
///`read()` method returns [`npu_icncr::R`](R) reader structure
impl crate::Readable for NPU_ICNCRrs {}
///`write(|w| ..)` method takes [`npu_icncr::W`](W) writer structure
impl crate::Writable for NPU_ICNCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NPU_ICNCR to value 0
impl crate::Resettable for NPU_ICNCRrs {}
