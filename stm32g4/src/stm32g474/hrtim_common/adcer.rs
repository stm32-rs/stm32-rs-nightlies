#[doc = "Register `ADCER` reader"]
pub type R = crate::R<ADCERrs>;
#[doc = "Register `ADCER` writer"]
pub type W = crate::W<ADCERrs>;
#[doc = "Field `ADC5TRG` reader - ADC5TRG"]
pub type ADC5TRG_R = crate::FieldReader;
#[doc = "Field `ADC5TRG` writer - ADC5TRG"]
pub type ADC5TRG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC6TRG` reader - ADC6TRG"]
pub type ADC6TRG_R = crate::FieldReader;
#[doc = "Field `ADC6TRG` writer - ADC6TRG"]
pub type ADC6TRG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC7TRG` reader - ADC7TRG"]
pub type ADC7TRG_R = crate::FieldReader;
#[doc = "Field `ADC7TRG` writer - ADC7TRG"]
pub type ADC7TRG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC8TRG` reader - ADC8TRG"]
pub type ADC8TRG_R = crate::FieldReader;
#[doc = "Field `ADC8TRG` writer - ADC8TRG"]
pub type ADC8TRG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC9TRG` reader - ADC9TRG"]
pub type ADC9TRG_R = crate::FieldReader;
#[doc = "Field `ADC9TRG` writer - ADC9TRG"]
pub type ADC9TRG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADC10TRG` reader - ADC10TRG"]
pub type ADC10TRG_R = crate::FieldReader;
#[doc = "Field `ADC10TRG` writer - ADC10TRG"]
pub type ADC10TRG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - ADC5TRG"]
    #[inline(always)]
    pub fn adc5trg(&self) -> ADC5TRG_R {
        ADC5TRG_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - ADC6TRG"]
    #[inline(always)]
    pub fn adc6trg(&self) -> ADC6TRG_R {
        ADC6TRG_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - ADC7TRG"]
    #[inline(always)]
    pub fn adc7trg(&self) -> ADC7TRG_R {
        ADC7TRG_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - ADC8TRG"]
    #[inline(always)]
    pub fn adc8trg(&self) -> ADC8TRG_R {
        ADC8TRG_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - ADC9TRG"]
    #[inline(always)]
    pub fn adc9trg(&self) -> ADC9TRG_R {
        ADC9TRG_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:30 - ADC10TRG"]
    #[inline(always)]
    pub fn adc10trg(&self) -> ADC10TRG_R {
        ADC10TRG_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADC5TRG"]
    #[inline(always)]
    #[must_use]
    pub fn adc5trg(&mut self) -> ADC5TRG_W<ADCERrs> {
        ADC5TRG_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - ADC6TRG"]
    #[inline(always)]
    #[must_use]
    pub fn adc6trg(&mut self) -> ADC6TRG_W<ADCERrs> {
        ADC6TRG_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - ADC7TRG"]
    #[inline(always)]
    #[must_use]
    pub fn adc7trg(&mut self) -> ADC7TRG_W<ADCERrs> {
        ADC7TRG_W::new(self, 10)
    }
    #[doc = "Bits 16:20 - ADC8TRG"]
    #[inline(always)]
    #[must_use]
    pub fn adc8trg(&mut self) -> ADC8TRG_W<ADCERrs> {
        ADC8TRG_W::new(self, 16)
    }
    #[doc = "Bits 21:25 - ADC9TRG"]
    #[inline(always)]
    #[must_use]
    pub fn adc9trg(&mut self) -> ADC9TRG_W<ADCERrs> {
        ADC9TRG_W::new(self, 21)
    }
    #[doc = "Bits 26:30 - ADC10TRG"]
    #[inline(always)]
    #[must_use]
    pub fn adc10trg(&mut self) -> ADC10TRG_W<ADCERrs> {
        ADC10TRG_W::new(self, 26)
    }
}
#[doc = "HRTIM ADC Extended Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCERrs;
impl crate::RegisterSpec for ADCERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcer::R`](R) reader structure"]
impl crate::Readable for ADCERrs {}
#[doc = "`write(|w| ..)` method takes [`adcer::W`](W) writer structure"]
impl crate::Writable for ADCERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCER to value 0"]
impl crate::Resettable for ADCERrs {
    const RESET_VALUE: u32 = 0;
}
