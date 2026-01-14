///Register `WRCR` reader
pub type R = crate::R<WRCRrs>;
///Field `WRC` reader - Warm reset counter
pub type WRC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Warm reset counter
    #[inline(always)]
    pub fn wrc(&self) -> WRC_R {
        WRC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRCR").field("wrc", &self.wrc()).finish()
    }
}
/**BSEC warm reset count register

You can [`read`](crate::Reg::read) this register and get [`wrcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#BSEC:WRCR)*/
pub struct WRCRrs;
impl crate::RegisterSpec for WRCRrs {
    type Ux = u32;
}
///`read()` method returns [`wrcr::R`](R) reader structure
impl crate::Readable for WRCRrs {}
///`reset()` method sets WRCR to value 0
impl crate::Resettable for WRCRrs {}
