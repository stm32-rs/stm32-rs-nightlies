#[doc = "Register `ADCPS2` reader"]
pub type R = crate::R<ADCPS2rs>;
#[doc = "Register `ADCPS2` writer"]
pub type W = crate::W<ADCPS2rs>;
#[doc = "Field `ADC6PSC` reader - ADC6PSC"]
pub type ADC6PSC_R = crate::FieldReader;
#[doc = "Field `ADC6PSC` writer - ADC6PSC"]
pub type ADC6PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC7PSC` reader - ADC7PSC"]
pub type ADC7PSC_R = crate::FieldReader;
#[doc = "Field `ADC7PSC` writer - ADC7PSC"]
pub type ADC7PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC8PSC` reader - ADC8PSC"]
pub type ADC8PSC_R = crate::FieldReader;
#[doc = "Field `ADC8PSC` writer - ADC8PSC"]
pub type ADC8PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC9PSC` reader - ADC9PSC"]
pub type ADC9PSC_R = crate::FieldReader;
#[doc = "Field `ADC9PSC` writer - ADC9PSC"]
pub type ADC9PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC10PSC` reader - ADC10PSC"]
pub type ADC10PSC_R = crate::FieldReader;
#[doc = "Field `ADC10PSC` writer - ADC10PSC"]
pub type ADC10PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - ADC6PSC"]
    #[inline(always)]
    pub fn adc6psc(&self) -> ADC6PSC_R {
        ADC6PSC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - ADC7PSC"]
    #[inline(always)]
    pub fn adc7psc(&self) -> ADC7PSC_R {
        ADC7PSC_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - ADC8PSC"]
    #[inline(always)]
    pub fn adc8psc(&self) -> ADC8PSC_R {
        ADC8PSC_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - ADC9PSC"]
    #[inline(always)]
    pub fn adc9psc(&self) -> ADC9PSC_R {
        ADC9PSC_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADC10PSC"]
    #[inline(always)]
    pub fn adc10psc(&self) -> ADC10PSC_R {
        ADC10PSC_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADC6PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc6psc(&mut self) -> ADC6PSC_W<ADCPS2rs> {
        ADC6PSC_W::new(self, 0)
    }
    #[doc = "Bits 6:10 - ADC7PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc7psc(&mut self) -> ADC7PSC_W<ADCPS2rs> {
        ADC7PSC_W::new(self, 6)
    }
    #[doc = "Bits 12:16 - ADC8PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc8psc(&mut self) -> ADC8PSC_W<ADCPS2rs> {
        ADC8PSC_W::new(self, 12)
    }
    #[doc = "Bits 18:22 - ADC9PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc9psc(&mut self) -> ADC9PSC_W<ADCPS2rs> {
        ADC9PSC_W::new(self, 18)
    }
    #[doc = "Bits 24:28 - ADC10PSC"]
    #[inline(always)]
    #[must_use]
    pub fn adc10psc(&mut self) -> ADC10PSC_W<ADCPS2rs> {
        ADC10PSC_W::new(self, 24)
    }
}
#[doc = "HRTIM ADC Post Scaler Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcps2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcps2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCPS2rs;
impl crate::RegisterSpec for ADCPS2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcps2::R`](R) reader structure"]
impl crate::Readable for ADCPS2rs {}
#[doc = "`write(|w| ..)` method takes [`adcps2::W`](W) writer structure"]
impl crate::Writable for ADCPS2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCPS2 to value 0"]
impl crate::Resettable for ADCPS2rs {
    const RESET_VALUE: u32 = 0;
}
