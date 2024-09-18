///Register `MDIOS_WRFR` reader
pub type R = crate::R<MDIOS_WRFRrs>;
///Field `WRF` reader - WRF
pub type WRF_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - WRF
    #[inline(always)]
    pub fn wrf(&self) -> WRF_R {
        WRF_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDIOS_WRFR")
            .field("wrf", &self.wrf())
            .finish()
    }
}
/**MDIOS write flag register

You can [`read`](crate::Reg::read) this register and get [`mdios_wrfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDIOS:MDIOS_WRFR)*/
pub struct MDIOS_WRFRrs;
impl crate::RegisterSpec for MDIOS_WRFRrs {
    type Ux = u32;
}
///`read()` method returns [`mdios_wrfr::R`](R) reader structure
impl crate::Readable for MDIOS_WRFRrs {}
///`reset()` method sets MDIOS_WRFR to value 0
impl crate::Resettable for MDIOS_WRFRrs {
    const RESET_VALUE: u32 = 0;
}
