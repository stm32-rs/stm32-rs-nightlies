#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "Field `CKMODE` reader - ADC clock mode These bits are set and cleared by software to define the ADC clock scheme (which is common to both master and slave ADCs): In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type CKMODE_R = crate::FieldReader;
#[doc = "Field `CKMODE` writer - ADC clock mode These bits are set and cleared by software to define the ADC clock scheme (which is common to both master and slave ADCs): In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type CKMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRESC` reader - ADC prescaler These bits are set and cleared by software to select the frequency of the clock to the ADC. The clock is common for all the ADCs. other: reserved Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0). The ADC prescaler value is applied only when CKMODE\\[1:0\\]
= 0b00."]
pub type PRESC_R = crate::FieldReader;
#[doc = "Field `PRESC` writer - ADC prescaler These bits are set and cleared by software to select the frequency of the clock to the ADC. The clock is common for all the ADCs. other: reserved Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0). The ADC prescaler value is applied only when CKMODE\\[1:0\\]
= 0b00."]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VREFEN` reader - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT channel."]
pub type VREFEN_R = crate::BitReader;
#[doc = "Field `VREFEN` writer - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT channel."]
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEN` reader - VSENSE enable This bit is set and cleared by software to control VSENSE."]
pub type TSEN_R = crate::BitReader;
#[doc = "Field `TSEN` writer - VSENSE enable This bit is set and cleared by software to control VSENSE."]
pub type TSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBATEN` reader - VBAT enable This bit is set and cleared by software to control."]
pub type VBATEN_R = crate::BitReader;
#[doc = "Field `VBATEN` writer - VBAT enable This bit is set and cleared by software to control."]
pub type VBATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:17 - ADC clock mode These bits are set and cleared by software to define the ADC clock scheme (which is common to both master and slave ADCs): In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - ADC prescaler These bits are set and cleared by software to select the frequency of the clock to the ADC. The clock is common for all the ADCs. other: reserved Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0). The ADC prescaler value is applied only when CKMODE\\[1:0\\]
= 0b00."]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT channel."]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - VSENSE enable This bit is set and cleared by software to control VSENSE."]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - VBAT enable This bit is set and cleared by software to control."]
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - ADC clock mode These bits are set and cleared by software to define the ADC clock scheme (which is common to both master and slave ADCs): In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADCs are disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<CCRrs> {
        CKMODE_W::new(self, 16)
    }
    #[doc = "Bits 18:21 - ADC prescaler These bits are set and cleared by software to select the frequency of the clock to the ADC. The clock is common for all the ADCs. other: reserved Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0). The ADC prescaler value is applied only when CKMODE\\[1:0\\]
= 0b00."]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<CCRrs> {
        PRESC_W::new(self, 18)
    }
    #[doc = "Bit 22 - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT channel."]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<CCRrs> {
        VREFEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - VSENSE enable This bit is set and cleared by software to control VSENSE."]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<CCRrs> {
        TSEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - VBAT enable This bit is set and cleared by software to control."]
    #[inline(always)]
    #[must_use]
    pub fn vbaten(&mut self) -> VBATEN_W<CCRrs> {
        VBATEN_W::new(self, 24)
    }
}
#[doc = "ADC common control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCRrs {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0;
}
