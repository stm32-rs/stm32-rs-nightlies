#[doc = "Register `ADC_CALFACT` reader"]
pub type R = crate::R<ADC_CALFACTrs>;
#[doc = "Register `ADC_CALFACT` writer"]
pub type W = crate::W<ADC_CALFACTrs>;
#[doc = "Field `I_APB_ADDR` reader - Delayed write access address This bitfield contains the address that is being written during delayed write accesses."]
pub type I_APB_ADDR_R = crate::FieldReader;
#[doc = "Field `I_APB_DATA` reader - Delayed write access data This bitfield contains the data that are being written during delayed write accesses."]
pub type I_APB_DATA_R = crate::FieldReader;
#[doc = "Field `VALIDITY` reader - Delayed write access status bit This bit indicates the communication status between the ADC digital and analog blocks."]
pub type VALIDITY_R = crate::BitReader;
#[doc = "Field `LATCH_COEF` reader - Calibration factor latch enable bit This bit latches the calibration factor in the CALFACT\\[31:0\\]
bits."]
pub type LATCH_COEF_R = crate::BitReader;
#[doc = "Field `LATCH_COEF` writer - Calibration factor latch enable bit This bit latches the calibration factor in the CALFACT\\[31:0\\]
bits."]
pub type LATCH_COEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPTURE_COEF` reader - Calibration factor capture enable bit This bit enables the internal calibration factor capture."]
pub type CAPTURE_COEF_R = crate::BitReader;
#[doc = "Field `CAPTURE_COEF` writer - Calibration factor capture enable bit This bit enables the internal calibration factor capture."]
pub type CAPTURE_COEF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Delayed write access address This bitfield contains the address that is being written during delayed write accesses."]
    #[inline(always)]
    pub fn i_apb_addr(&self) -> I_APB_ADDR_R {
        I_APB_ADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delayed write access data This bitfield contains the data that are being written during delayed write accesses."]
    #[inline(always)]
    pub fn i_apb_data(&self) -> I_APB_DATA_R {
        I_APB_DATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Delayed write access status bit This bit indicates the communication status between the ADC digital and analog blocks."]
    #[inline(always)]
    pub fn validity(&self) -> VALIDITY_R {
        VALIDITY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Calibration factor latch enable bit This bit latches the calibration factor in the CALFACT\\[31:0\\]
bits."]
    #[inline(always)]
    pub fn latch_coef(&self) -> LATCH_COEF_R {
        LATCH_COEF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Calibration factor capture enable bit This bit enables the internal calibration factor capture."]
    #[inline(always)]
    pub fn capture_coef(&self) -> CAPTURE_COEF_R {
        CAPTURE_COEF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Calibration factor latch enable bit This bit latches the calibration factor in the CALFACT\\[31:0\\]
bits."]
    #[inline(always)]
    #[must_use]
    pub fn latch_coef(&mut self) -> LATCH_COEF_W<ADC_CALFACTrs> {
        LATCH_COEF_W::new(self, 24)
    }
    #[doc = "Bit 25 - Calibration factor capture enable bit This bit enables the internal calibration factor capture."]
    #[inline(always)]
    #[must_use]
    pub fn capture_coef(&mut self) -> CAPTURE_COEF_W<ADC_CALFACTrs> {
        CAPTURE_COEF_W::new(self, 25)
    }
}
#[doc = "ADC user control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_calfact::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_calfact::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_CALFACTrs;
impl crate::RegisterSpec for ADC_CALFACTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_calfact::R`](R) reader structure"]
impl crate::Readable for ADC_CALFACTrs {}
#[doc = "`write(|w| ..)` method takes [`adc_calfact::W`](W) writer structure"]
impl crate::Writable for ADC_CALFACTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CALFACT to value 0"]
impl crate::Resettable for ADC_CALFACTrs {
    const RESET_VALUE: u32 = 0;
}
