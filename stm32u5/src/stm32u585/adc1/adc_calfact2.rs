#[doc = "Register `ADC_CALFACT2` reader"]
pub type R = crate::R<ADC_CALFACT2rs>;
#[doc = "Register `ADC_CALFACT2` writer"]
pub type W = crate::W<ADC_CALFACT2rs>;
#[doc = "Field `CALFACT` reader - Linearity or offset calibration factor These bits can be written either by hardware or by software. They contain the 32-bit offset or linearity calibration factor. When CAPTURE_COEF is set to 1, the calibration factor of the analog block is read back and stored in CALFACT\\[31:0\\], indexed by CALINDEX\\[3:0\\]
bits. When LATCH_COEF is set to 1, the calibration factor of the analog block is updated with the value programmed in CALFACT\\[31:0\\], indexed by CALINDEX\\[3:0\\]
bits. To read all calibration factors, perform nine accesses to the ADC_CALFACT2 register. To write all calibration factors, perform eight accesses to the ADC_CALFACT2 register. Note: The software is allowed to write these bits only when ADEN = 1, ADSTART = 0 and JADSTART = 0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing)."]
pub type CALFACT_R = crate::FieldReader<u32>;
#[doc = "Field `CALFACT` writer - Linearity or offset calibration factor These bits can be written either by hardware or by software. They contain the 32-bit offset or linearity calibration factor. When CAPTURE_COEF is set to 1, the calibration factor of the analog block is read back and stored in CALFACT\\[31:0\\], indexed by CALINDEX\\[3:0\\]
bits. When LATCH_COEF is set to 1, the calibration factor of the analog block is updated with the value programmed in CALFACT\\[31:0\\], indexed by CALINDEX\\[3:0\\]
bits. To read all calibration factors, perform nine accesses to the ADC_CALFACT2 register. To write all calibration factors, perform eight accesses to the ADC_CALFACT2 register. Note: The software is allowed to write these bits only when ADEN = 1, ADSTART = 0 and JADSTART = 0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing)."]
pub type CALFACT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Linearity or offset calibration factor These bits can be written either by hardware or by software. They contain the 32-bit offset or linearity calibration factor. When CAPTURE_COEF is set to 1, the calibration factor of the analog block is read back and stored in CALFACT\\[31:0\\], indexed by CALINDEX\\[3:0\\]
bits. When LATCH_COEF is set to 1, the calibration factor of the analog block is updated with the value programmed in CALFACT\\[31:0\\], indexed by CALINDEX\\[3:0\\]
bits. To read all calibration factors, perform nine accesses to the ADC_CALFACT2 register. To write all calibration factors, perform eight accesses to the ADC_CALFACT2 register. Note: The software is allowed to write these bits only when ADEN = 1, ADSTART = 0 and JADSTART = 0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing)."]
    #[inline(always)]
    pub fn calfact(&self) -> CALFACT_R {
        CALFACT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Linearity or offset calibration factor These bits can be written either by hardware or by software. They contain the 32-bit offset or linearity calibration factor. When CAPTURE_COEF is set to 1, the calibration factor of the analog block is read back and stored in CALFACT\\[31:0\\], indexed by CALINDEX\\[3:0\\]
bits. When LATCH_COEF is set to 1, the calibration factor of the analog block is updated with the value programmed in CALFACT\\[31:0\\], indexed by CALINDEX\\[3:0\\]
bits. To read all calibration factors, perform nine accesses to the ADC_CALFACT2 register. To write all calibration factors, perform eight accesses to the ADC_CALFACT2 register. Note: The software is allowed to write these bits only when ADEN = 1, ADSTART = 0 and JADSTART = 0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn calfact(&mut self) -> CALFACT_W<ADC_CALFACT2rs> {
        CALFACT_W::new(self, 0)
    }
}
#[doc = "ADC calibration factor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_calfact2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_calfact2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_CALFACT2rs;
impl crate::RegisterSpec for ADC_CALFACT2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_calfact2::R`](R) reader structure"]
impl crate::Readable for ADC_CALFACT2rs {}
#[doc = "`write(|w| ..)` method takes [`adc_calfact2::W`](W) writer structure"]
impl crate::Writable for ADC_CALFACT2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CALFACT2 to value 0"]
impl crate::Resettable for ADC_CALFACT2rs {
    const RESET_VALUE: u32 = 0;
}
