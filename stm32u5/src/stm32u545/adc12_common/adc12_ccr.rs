#[doc = "Register `ADC12_CCR` reader"]
pub type R = crate::R<ADC12_CCRrs>;
#[doc = "Register `ADC12_CCR` writer"]
pub type W = crate::W<ADC12_CCRrs>;
#[doc = "Field `DUAL` reader - Dual ADC mode selection These bits are written by software to select the operating mode. All the ADCs are independent: The configurations 00001 to 01001 correspond to the following operating modes: Dual mode, master and slave ADCs working together: All other combinations are reserved and must not be programmed Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type DUAL_R = crate::FieldReader;
#[doc = "Field `DUAL` writer - Dual ADC mode selection These bits are written by software to select the operating mode. All the ADCs are independent: The configurations 00001 to 01001 correspond to the following operating modes: Dual mode, master and slave ADCs working together: All other combinations are reserved and must not be programmed Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type DUAL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DELAY` reader - Delay between the end of the master ADC sampling phase and the beginning of the slave ADC sampling phase. These bits are set and cleared by software. These bits are used in dual interleaved modes. Refer to for the value of ADC resolution versus DELAY bits values. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type DELAY_R = crate::FieldReader;
#[doc = "Field `DELAY` writer - Delay between the end of the master ADC sampling phase and the beginning of the slave ADC sampling phase. These bits are set and cleared by software. These bits are used in dual interleaved modes. Refer to for the value of ADC resolution versus DELAY bits values. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAMDF` reader - Dual ADC Mode Data Format This bit-field is set and cleared by software. It specifies the data format in the common data register ADC12_CDR. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type DAMDF_R = crate::FieldReader;
#[doc = "Field `DAMDF` writer - Dual ADC Mode Data Format This bit-field is set and cleared by software. It specifies the data format in the common data register ADC12_CDR. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
pub type DAMDF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRESC` reader - ADC prescaler These bits are set and cleared by software to select the frequency of the ADC clock. The clock is common to all ADCs. Others: Reserved, must not be used Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type PRESC_R = crate::FieldReader;
#[doc = "Field `PRESC` writer - ADC prescaler These bits are set and cleared by software to select the frequency of the ADC clock. The clock is common to all ADCs. Others: Reserved, must not be used Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VREFEN` reader - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT buffer. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type VREFEN_R = crate::BitReader;
#[doc = "Field `VREFEN` writer - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT buffer. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSENSESEL` reader - Temperature sensor voltage selection This bit is set and cleared by software to control the temperature sensor channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type VSENSESEL_R = crate::BitReader;
#[doc = "Field `VSENSESEL` writer - Temperature sensor voltage selection This bit is set and cleared by software to control the temperature sensor channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type VSENSESEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBATEN` reader - VBAT enable This bit is set and cleared by software to control the VBAT channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type VBATEN_R = crate::BitReader;
#[doc = "Field `VBATEN` writer - VBAT enable This bit is set and cleared by software to control the VBAT channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type VBATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Dual ADC mode selection These bits are written by software to select the operating mode. All the ADCs are independent: The configurations 00001 to 01001 correspond to the following operating modes: Dual mode, master and slave ADCs working together: All other combinations are reserved and must not be programmed Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Delay between the end of the master ADC sampling phase and the beginning of the slave ADC sampling phase. These bits are set and cleared by software. These bits are used in dual interleaved modes. Refer to for the value of ADC resolution versus DELAY bits values. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - Dual ADC Mode Data Format This bit-field is set and cleared by software. It specifies the data format in the common data register ADC12_CDR. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    pub fn damdf(&self) -> DAMDF_R {
        DAMDF_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 18:21 - ADC prescaler These bits are set and cleared by software to select the frequency of the ADC clock. The clock is common to all ADCs. Others: Reserved, must not be used Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT buffer. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Temperature sensor voltage selection This bit is set and cleared by software to control the temperature sensor channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn vsensesel(&self) -> VSENSESEL_R {
        VSENSESEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - VBAT enable This bit is set and cleared by software to control the VBAT channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Dual ADC mode selection These bits are written by software to select the operating mode. All the ADCs are independent: The configurations 00001 to 01001 correspond to the following operating modes: Dual mode, master and slave ADCs working together: All other combinations are reserved and must not be programmed Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn dual(&mut self) -> DUAL_W<ADC12_CCRrs> {
        DUAL_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Delay between the end of the master ADC sampling phase and the beginning of the slave ADC sampling phase. These bits are set and cleared by software. These bits are used in dual interleaved modes. Refer to for the value of ADC resolution versus DELAY bits values. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<ADC12_CCRrs> {
        DELAY_W::new(self, 8)
    }
    #[doc = "Bits 14:15 - Dual ADC Mode Data Format This bit-field is set and cleared by software. It specifies the data format in the common data register ADC12_CDR. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn damdf(&mut self) -> DAMDF_W<ADC12_CCRrs> {
        DAMDF_W::new(self, 14)
    }
    #[doc = "Bits 18:21 - ADC prescaler These bits are set and cleared by software to select the frequency of the ADC clock. The clock is common to all ADCs. Others: Reserved, must not be used Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<ADC12_CCRrs> {
        PRESC_W::new(self, 18)
    }
    #[doc = "Bit 22 - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT buffer. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<ADC12_CCRrs> {
        VREFEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Temperature sensor voltage selection This bit is set and cleared by software to control the temperature sensor channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn vsensesel(&mut self) -> VSENSESEL_W<ADC12_CCRrs> {
        VSENSESEL_W::new(self, 23)
    }
    #[doc = "Bit 24 - VBAT enable This bit is set and cleared by software to control the VBAT channel. Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn vbaten(&mut self) -> VBATEN_W<ADC12_CCRrs> {
        VBATEN_W::new(self, 24)
    }
}
#[doc = "ADC_CCR system control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12_ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12_ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC12_CCRrs;
impl crate::RegisterSpec for ADC12_CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc12_ccr::R`](R) reader structure"]
impl crate::Readable for ADC12_CCRrs {}
#[doc = "`write(|w| ..)` method takes [`adc12_ccr::W`](W) writer structure"]
impl crate::Writable for ADC12_CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC12_CCR to value 0"]
impl crate::Resettable for ADC12_CCRrs {
    const RESET_VALUE: u32 = 0;
}
