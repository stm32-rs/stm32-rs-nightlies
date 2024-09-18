///Register `SDMMC_DLENR` reader
pub type R = crate::R<SDMMC_DLENRrs>;
///Register `SDMMC_DLENR` writer
pub type W = crate::W<SDMMC_DLENRrs>;
///Field `DATALENGTH` reader - DATALENGTH
pub type DATALENGTH_R = crate::FieldReader<u32>;
///Field `DATALENGTH` writer - DATALENGTH
pub type DATALENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bits 0:24 - DATALENGTH
    #[inline(always)]
    pub fn datalength(&self) -> DATALENGTH_R {
        DATALENGTH_R::new(self.bits & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDMMC_DLENR")
            .field("datalength", &self.datalength())
            .finish()
    }
}
impl W {
    ///Bits 0:24 - DATALENGTH
    #[inline(always)]
    #[must_use]
    pub fn datalength(&mut self) -> DATALENGTH_W<SDMMC_DLENRrs> {
        DATALENGTH_W::new(self, 0)
    }
}
/**The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_dlenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_dlenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SDMMC1:SDMMC_DLENR)*/
pub struct SDMMC_DLENRrs;
impl crate::RegisterSpec for SDMMC_DLENRrs {
    type Ux = u32;
}
///`read()` method returns [`sdmmc_dlenr::R`](R) reader structure
impl crate::Readable for SDMMC_DLENRrs {}
///`write(|w| ..)` method takes [`sdmmc_dlenr::W`](W) writer structure
impl crate::Writable for SDMMC_DLENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SDMMC_DLENR to value 0
impl crate::Resettable for SDMMC_DLENRrs {
    const RESET_VALUE: u32 = 0;
}
