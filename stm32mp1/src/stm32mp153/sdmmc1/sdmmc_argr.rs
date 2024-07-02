///Register `SDMMC_ARGR` reader
pub type R = crate::R<SDMMC_ARGRrs>;
///Register `SDMMC_ARGR` writer
pub type W = crate::W<SDMMC_ARGRrs>;
///Field `CMDARG` reader - CMDARG
pub type CMDARG_R = crate::FieldReader<u32>;
///Field `CMDARG` writer - CMDARG
pub type CMDARG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CMDARG
    #[inline(always)]
    pub fn cmdarg(&self) -> CMDARG_R {
        CMDARG_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDMMC_ARGR")
            .field("cmdarg", &self.cmdarg())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CMDARG
    #[inline(always)]
    #[must_use]
    pub fn cmdarg(&mut self) -> CMDARG_W<SDMMC_ARGRrs> {
        CMDARG_W::new(self, 0)
    }
}
/**The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_argr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_argr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_ARGR)*/
pub struct SDMMC_ARGRrs;
impl crate::RegisterSpec for SDMMC_ARGRrs {
    type Ux = u32;
}
///`read()` method returns [`sdmmc_argr::R`](R) reader structure
impl crate::Readable for SDMMC_ARGRrs {}
///`write(|w| ..)` method takes [`sdmmc_argr::W`](W) writer structure
impl crate::Writable for SDMMC_ARGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SDMMC_ARGR to value 0
impl crate::Resettable for SDMMC_ARGRrs {
    const RESET_VALUE: u32 = 0;
}
