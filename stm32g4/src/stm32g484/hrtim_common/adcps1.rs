///Register `ADCPS1` reader
pub type R = crate::R<ADCPS1rs>;
///Register `ADCPS1` writer
pub type W = crate::W<ADCPS1rs>;
///Field `ADC1PSC` reader - ADC1PSC
pub type ADC1PSC_R = crate::FieldReader;
///Field `ADC1PSC` writer - ADC1PSC
pub type ADC1PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ADC2PSC` reader - ADC2PSC
pub type ADC2PSC_R = crate::FieldReader;
///Field `ADC2PSC` writer - ADC2PSC
pub type ADC2PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ADC3PSC` reader - ADC3PSC
pub type ADC3PSC_R = crate::FieldReader;
///Field `ADC3PSC` writer - ADC3PSC
pub type ADC3PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ADC4PSC` reader - ADC4PSC
pub type ADC4PSC_R = crate::FieldReader;
///Field `ADC4PSC` writer - ADC4PSC
pub type ADC4PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ADC5PSC` reader - ADC5PSC
pub type ADC5PSC_R = crate::FieldReader;
///Field `ADC5PSC` writer - ADC5PSC
pub type ADC5PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - ADC1PSC
    #[inline(always)]
    pub fn adc1psc(&self) -> ADC1PSC_R {
        ADC1PSC_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:10 - ADC2PSC
    #[inline(always)]
    pub fn adc2psc(&self) -> ADC2PSC_R {
        ADC2PSC_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 12:16 - ADC3PSC
    #[inline(always)]
    pub fn adc3psc(&self) -> ADC3PSC_R {
        ADC3PSC_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    ///Bits 18:22 - ADC4PSC
    #[inline(always)]
    pub fn adc4psc(&self) -> ADC4PSC_R {
        ADC4PSC_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bits 24:28 - ADC5PSC
    #[inline(always)]
    pub fn adc5psc(&self) -> ADC5PSC_R {
        ADC5PSC_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCPS1")
            .field("adc5psc", &self.adc5psc())
            .field("adc4psc", &self.adc4psc())
            .field("adc3psc", &self.adc3psc())
            .field("adc2psc", &self.adc2psc())
            .field("adc1psc", &self.adc1psc())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - ADC1PSC
    #[inline(always)]
    pub fn adc1psc(&mut self) -> ADC1PSC_W<'_, ADCPS1rs> {
        ADC1PSC_W::new(self, 0)
    }
    ///Bits 6:10 - ADC2PSC
    #[inline(always)]
    pub fn adc2psc(&mut self) -> ADC2PSC_W<'_, ADCPS1rs> {
        ADC2PSC_W::new(self, 6)
    }
    ///Bits 12:16 - ADC3PSC
    #[inline(always)]
    pub fn adc3psc(&mut self) -> ADC3PSC_W<'_, ADCPS1rs> {
        ADC3PSC_W::new(self, 12)
    }
    ///Bits 18:22 - ADC4PSC
    #[inline(always)]
    pub fn adc4psc(&mut self) -> ADC4PSC_W<'_, ADCPS1rs> {
        ADC4PSC_W::new(self, 18)
    }
    ///Bits 24:28 - ADC5PSC
    #[inline(always)]
    pub fn adc5psc(&mut self) -> ADC5PSC_W<'_, ADCPS1rs> {
        ADC5PSC_W::new(self, 24)
    }
}
/**HRTIM ADC Post Scaler Register 1

You can [`read`](crate::Reg::read) this register and get [`adcps1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcps1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#HRTIM_Common:ADCPS1)*/
pub struct ADCPS1rs;
impl crate::RegisterSpec for ADCPS1rs {
    type Ux = u32;
}
///`read()` method returns [`adcps1::R`](R) reader structure
impl crate::Readable for ADCPS1rs {}
///`write(|w| ..)` method takes [`adcps1::W`](W) writer structure
impl crate::Writable for ADCPS1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCPS1 to value 0
impl crate::Resettable for ADCPS1rs {}
