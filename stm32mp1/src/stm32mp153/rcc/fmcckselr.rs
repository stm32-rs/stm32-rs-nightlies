///Register `FMCCKSELR` reader
pub type R = crate::R<FMCCKSELRrs>;
///Register `FMCCKSELR` writer
pub type W = crate::W<FMCCKSELRrs>;
///Field `FMCSRC` reader - FMCSRC
pub type FMCSRC_R = crate::FieldReader;
///Field `FMCSRC` writer - FMCSRC
pub type FMCSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - FMCSRC
    #[inline(always)]
    pub fn fmcsrc(&self) -> FMCSRC_R {
        FMCSRC_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMCCKSELR")
            .field("fmcsrc", &self.fmcsrc())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - FMCSRC
    #[inline(always)]
    pub fn fmcsrc(&mut self) -> FMCSRC_W<'_, FMCCKSELRrs> {
        FMCSRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the FMC block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`fmcckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmcckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:FMCCKSELR)*/
pub struct FMCCKSELRrs;
impl crate::RegisterSpec for FMCCKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`fmcckselr::R`](R) reader structure
impl crate::Readable for FMCCKSELRrs {}
///`write(|w| ..)` method takes [`fmcckselr::W`](W) writer structure
impl crate::Writable for FMCCKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FMCCKSELR to value 0
impl crate::Resettable for FMCCKSELRrs {}
