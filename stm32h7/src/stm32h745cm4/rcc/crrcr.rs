///Register `CRRCR` reader
pub type R = crate::R<CRRCRrs>;
///Field `HSI48CAL` reader - Internal RC 48 MHz clock calibration
pub type HSI48CAL_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:9 - Internal RC 48 MHz clock calibration
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRRCR")
            .field("hsi48cal", &self.hsi48cal())
            .finish()
    }
}
/**RCC Clock Recovery RC Register

You can [`read`](crate::Reg::read) this register and get [`crrcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#RCC:CRRCR)*/
pub struct CRRCRrs;
impl crate::RegisterSpec for CRRCRrs {
    type Ux = u32;
}
///`read()` method returns [`crrcr::R`](R) reader structure
impl crate::Readable for CRRCRrs {}
///`reset()` method sets CRRCR to value 0
impl crate::Resettable for CRRCRrs {}
