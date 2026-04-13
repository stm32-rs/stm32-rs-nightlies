///Register `BCHICR` writer
pub type W = crate::W<BCHICRrs>;
///Field `CDUEF` writer - CDUEF
pub type CDUEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDERF` writer - CDERF
pub type CDERF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDEFF` writer - CDEFF
pub type CDEFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDSRF` writer - CDSRF
pub type CDSRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEPBRF` writer - CEPBRF
pub type CEPBRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BCHICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - CDUEF
    #[inline(always)]
    pub fn cduef(&mut self) -> CDUEF_W<'_, BCHICRrs> {
        CDUEF_W::new(self, 0)
    }
    ///Bit 1 - CDERF
    #[inline(always)]
    pub fn cderf(&mut self) -> CDERF_W<'_, BCHICRrs> {
        CDERF_W::new(self, 1)
    }
    ///Bit 2 - CDEFF
    #[inline(always)]
    pub fn cdeff(&mut self) -> CDEFF_W<'_, BCHICRrs> {
        CDEFF_W::new(self, 2)
    }
    ///Bit 3 - CDSRF
    #[inline(always)]
    pub fn cdsrf(&mut self) -> CDSRF_W<'_, BCHICRrs> {
        CDSRF_W::new(self, 3)
    }
    ///Bit 4 - CEPBRF
    #[inline(always)]
    pub fn cepbrf(&mut self) -> CEPBRF_W<'_, BCHICRrs> {
        CEPBRF_W::new(self, 4)
    }
}
/**FMC BCH Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bchicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:BCHICR)*/
pub struct BCHICRrs;
impl crate::RegisterSpec for BCHICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`bchicr::W`](W) writer structure
impl crate::Writable for BCHICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCHICR to value 0
impl crate::Resettable for BCHICRrs {}
