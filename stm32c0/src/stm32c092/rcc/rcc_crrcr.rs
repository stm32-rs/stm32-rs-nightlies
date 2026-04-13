///Register `RCC_CRRCR` reader
pub type R = crate::R<RCC_CRRCRrs>;
///Field `HSIUSB48CAL` reader - HSIUSB48 clock calibration These bits are initialized at startup with the factory-programmed HSIUSB48 calibration trim value.
pub type HSIUSB48CAL_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:8 - HSIUSB48 clock calibration These bits are initialized at startup with the factory-programmed HSIUSB48 calibration trim value.
    #[inline(always)]
    pub fn hsiusb48cal(&self) -> HSIUSB48CAL_R {
        HSIUSB48CAL_R::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CRRCR")
            .field("hsiusb48cal", &self.hsiusb48cal())
            .finish()
    }
}
/**RCC clock recovery RC register

You can [`read`](crate::Reg::read) this register and get [`rcc_crrcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#RCC:RCC_CRRCR)*/
pub struct RCC_CRRCRrs;
impl crate::RegisterSpec for RCC_CRRCRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_crrcr::R`](R) reader structure
impl crate::Readable for RCC_CRRCRrs {}
///`reset()` method sets RCC_CRRCR to value 0
impl crate::Resettable for RCC_CRRCRrs {}
