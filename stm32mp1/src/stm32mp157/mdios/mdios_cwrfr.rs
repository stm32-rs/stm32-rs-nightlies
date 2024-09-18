///Register `MDIOS_CWRFR` reader
pub type R = crate::R<MDIOS_CWRFRrs>;
///Register `MDIOS_CWRFR` writer
pub type W = crate::W<MDIOS_CWRFRrs>;
///Field `CWRF` reader - CWRF
pub type CWRF_R = crate::FieldReader<u32>;
///Field `CWRF` writer - CWRF
pub type CWRF_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CWRF
    #[inline(always)]
    pub fn cwrf(&self) -> CWRF_R {
        CWRF_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDIOS_CWRFR")
            .field("cwrf", &self.cwrf())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CWRF
    #[inline(always)]
    #[must_use]
    pub fn cwrf(&mut self) -> CWRF_W<MDIOS_CWRFRrs> {
        CWRF_W::new(self, 0)
    }
}
/**MDIOS clear write flag register

You can [`read`](crate::Reg::read) this register and get [`mdios_cwrfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_cwrfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_CWRFR)*/
pub struct MDIOS_CWRFRrs;
impl crate::RegisterSpec for MDIOS_CWRFRrs {
    type Ux = u32;
}
///`read()` method returns [`mdios_cwrfr::R`](R) reader structure
impl crate::Readable for MDIOS_CWRFRrs {}
///`write(|w| ..)` method takes [`mdios_cwrfr::W`](W) writer structure
impl crate::Writable for MDIOS_CWRFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MDIOS_CWRFR to value 0
impl crate::Resettable for MDIOS_CWRFRrs {
    const RESET_VALUE: u32 = 0;
}
