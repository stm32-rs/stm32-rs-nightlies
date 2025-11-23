///Register `WRFR` reader
pub type R = crate::R<WRFRrs>;
///Field `WRF` reader - Write flags for MDIO registers 0 to 31
pub type WRF_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Write flags for MDIO registers 0 to 31
    #[inline(always)]
    pub fn wrf(&self) -> WRF_R {
        WRF_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRFR").field("wrf", &self.wrf()).finish()
    }
}
/**MDIOS write flag register

You can [`read`](crate::Reg::read) this register and get [`wrfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#MDIOS:WRFR)*/
pub struct WRFRrs;
impl crate::RegisterSpec for WRFRrs {
    type Ux = u32;
}
///`read()` method returns [`wrfr::R`](R) reader structure
impl crate::Readable for WRFRrs {}
///`reset()` method sets WRFR to value 0
impl crate::Resettable for WRFRrs {}
