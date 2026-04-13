///Register `TSCSDIF_CFGR` reader
pub type R = crate::R<TSCSDIF_CFGRrs>;
///Register `TSCSDIF_CFGR` writer
pub type W = crate::W<TSCSDIF_CFGRrs>;
///Field `SDIF_INHIBIT` reader - Serial data interface (SDIF) programming inhibit
pub type SDIF_INHIBIT_R = crate::FieldReader;
///Field `SDIF_INHIBIT` writer - Serial data interface (SDIF) programming inhibit
pub type SDIF_INHIBIT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Serial data interface (SDIF) programming inhibit
    #[inline(always)]
    pub fn sdif_inhibit(&self) -> SDIF_INHIBIT_R {
        SDIF_INHIBIT_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCSDIF_CFGR")
            .field("sdif_inhibit", &self.sdif_inhibit())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Serial data interface (SDIF) programming inhibit
    #[inline(always)]
    pub fn sdif_inhibit(&mut self) -> SDIF_INHIBIT_W<'_, TSCSDIF_CFGRrs> {
        SDIF_INHIBIT_W::new(self, 0)
    }
}
/**DTS TSC SDIF control register

You can [`read`](crate::Reg::read) this register and get [`tscsdif_cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscsdif_cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TSCSDIF_CFGR)*/
pub struct TSCSDIF_CFGRrs;
impl crate::RegisterSpec for TSCSDIF_CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`tscsdif_cfgr::R`](R) reader structure
impl crate::Readable for TSCSDIF_CFGRrs {}
///`write(|w| ..)` method takes [`tscsdif_cfgr::W`](W) writer structure
impl crate::Writable for TSCSDIF_CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSCSDIF_CFGR to value 0
impl crate::Resettable for TSCSDIF_CFGRrs {}
