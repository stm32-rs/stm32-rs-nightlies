///Register `ICNCGCR` reader
pub type R = crate::R<ICNCGCRrs>;
///Register `ICNCGCR` writer
pub type W = crate::W<ICNCGCRrs>;
///Field `ICNCGCR` reader - When bit\[i\] is set to 1, ICN clock gating\[i\] is OFF.
pub type ICNCGCR_R = crate::FieldReader<u32>;
///Field `ICNCGCR` writer - When bit\[i\] is set to 1, ICN clock gating\[i\] is OFF.
pub type ICNCGCR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - When bit\[i\] is set to 1, ICN clock gating\[i\] is OFF.
    #[inline(always)]
    pub fn icncgcr(&self) -> ICNCGCR_R {
        ICNCGCR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICNCGCR")
            .field("icncgcr", &self.icncgcr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - When bit\[i\] is set to 1, ICN clock gating\[i\] is OFF.
    #[inline(always)]
    pub fn icncgcr(&mut self) -> ICNCGCR_W<'_, ICNCGCRrs> {
        ICNCGCR_W::new(self, 0)
    }
}
/**SYSCFG ICN clock gating control register

You can [`read`](crate::Reg::read) this register and get [`icncgcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icncgcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SYSCFG:ICNCGCR)*/
pub struct ICNCGCRrs;
impl crate::RegisterSpec for ICNCGCRrs {
    type Ux = u32;
}
///`read()` method returns [`icncgcr::R`](R) reader structure
impl crate::Readable for ICNCGCRrs {}
///`write(|w| ..)` method takes [`icncgcr::W`](W) writer structure
impl crate::Writable for ICNCGCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICNCGCR to value 0
impl crate::Resettable for ICNCGCRrs {}
