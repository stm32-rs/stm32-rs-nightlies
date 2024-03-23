#[doc = "Register `RCC_CRRCR` reader"]
pub type R = crate::R<RCC_CRRCRrs>;
#[doc = "Field `HSI48CAL` reader - HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value."]
pub type HSI48CAL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value."]
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "RCC clock recovery RC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_crrcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_CRRCRrs;
impl crate::RegisterSpec for RCC_CRRCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_crrcr::R`](R) reader structure"]
impl crate::Readable for RCC_CRRCRrs {}
#[doc = "`reset()` method sets RCC_CRRCR to value 0"]
impl crate::Resettable for RCC_CRRCRrs {
    const RESET_VALUE: u32 = 0;
}
