#[doc = "Register `HSPI_CALFCR` reader"]
pub type R = crate::R<HSPI_CALFCRrs>;
#[doc = "Field `FINE` reader - 6: 0\\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
pub type FINE_R = crate::FieldReader;
#[doc = "Field `COARSE` reader - 4: 0\\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
pub type COARSE_R = crate::FieldReader;
#[doc = "Field `CALMAX` reader - Max value This bit gets set when the memory-clock period is outside the range of DLLM, in which case HSPI_CALFCR and HSPI_CALSR are updated with the values for the maximum delay."]
pub type CALMAX_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - 6: 0\\]: Fine calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
    #[inline(always)]
    pub fn fine(&self) -> FINE_R {
        FINE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:20 - 4: 0\\]: Coarse calibration The unitary value of delay for this field depends on product technology (refer to the product datasheet)."]
    #[inline(always)]
    pub fn coarse(&self) -> COARSE_R {
        COARSE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Max value This bit gets set when the memory-clock period is outside the range of DLLM, in which case HSPI_CALFCR and HSPI_CALSR are updated with the values for the maximum delay."]
    #[inline(always)]
    pub fn calmax(&self) -> CALMAX_R {
        CALMAX_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "HSPI full-cycle calibration configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_calfcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSPI_CALFCRrs;
impl crate::RegisterSpec for HSPI_CALFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hspi_calfcr::R`](R) reader structure"]
impl crate::Readable for HSPI_CALFCRrs {}
#[doc = "`reset()` method sets HSPI_CALFCR to value 0"]
impl crate::Resettable for HSPI_CALFCRrs {
    const RESET_VALUE: u32 = 0;
}
