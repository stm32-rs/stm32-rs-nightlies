#[doc = "Register `ADC_JSQR` reader"]
pub type R = crate::R<ADC_JSQRrs>;
#[doc = "Register `ADC_JSQR` writer"]
pub type W = crate::W<ADC_JSQRrs>;
#[doc = "Field `JL` reader - JL"]
pub type JL_R = crate::FieldReader;
#[doc = "Field `JL` writer - JL"]
pub type JL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JEXTSEL` reader - JEXTSEL"]
pub type JEXTSEL_R = crate::FieldReader;
#[doc = "Field `JEXTSEL` writer - JEXTSEL"]
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JEXTEN` reader - JEXTEN"]
pub type JEXTEN_R = crate::FieldReader;
#[doc = "Field `JEXTEN` writer - JEXTEN"]
pub type JEXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JSQ1` reader - JSQ1"]
pub type JSQ1_R = crate::FieldReader;
#[doc = "Field `JSQ1` writer - JSQ1"]
pub type JSQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JSQ2` reader - JSQ2"]
pub type JSQ2_R = crate::FieldReader;
#[doc = "Field `JSQ2` writer - JSQ2"]
pub type JSQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JSQ3` reader - JSQ3"]
pub type JSQ3_R = crate::FieldReader;
#[doc = "Field `JSQ3` writer - JSQ3"]
pub type JSQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JSQ4` reader - JSQ4"]
pub type JSQ4_R = crate::FieldReader;
#[doc = "Field `JSQ4` writer - JSQ4"]
pub type JSQ4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - JL"]
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:6 - JEXTSEL"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:8 - JEXTEN"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:13 - JSQ1"]
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ1_R {
        JSQ1_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - JSQ2"]
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - JSQ3"]
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - JSQ4"]
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ4_R {
        JSQ4_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - JL"]
    #[inline(always)]
    #[must_use]
    pub fn jl(&mut self) -> JL_W<ADC_JSQRrs> {
        JL_W::new(self, 0)
    }
    #[doc = "Bits 2:6 - JEXTSEL"]
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<ADC_JSQRrs> {
        JEXTSEL_W::new(self, 2)
    }
    #[doc = "Bits 7:8 - JEXTEN"]
    #[inline(always)]
    #[must_use]
    pub fn jexten(&mut self) -> JEXTEN_W<ADC_JSQRrs> {
        JEXTEN_W::new(self, 7)
    }
    #[doc = "Bits 9:13 - JSQ1"]
    #[inline(always)]
    #[must_use]
    pub fn jsq1(&mut self) -> JSQ1_W<ADC_JSQRrs> {
        JSQ1_W::new(self, 9)
    }
    #[doc = "Bits 15:19 - JSQ2"]
    #[inline(always)]
    #[must_use]
    pub fn jsq2(&mut self) -> JSQ2_W<ADC_JSQRrs> {
        JSQ2_W::new(self, 15)
    }
    #[doc = "Bits 21:25 - JSQ3"]
    #[inline(always)]
    #[must_use]
    pub fn jsq3(&mut self) -> JSQ3_W<ADC_JSQRrs> {
        JSQ3_W::new(self, 21)
    }
    #[doc = "Bits 27:31 - JSQ4"]
    #[inline(always)]
    #[must_use]
    pub fn jsq4(&mut self) -> JSQ4_W<ADC_JSQRrs> {
        JSQ4_W::new(self, 27)
    }
}
#[doc = "ADC injected sequence register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_jsqr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_jsqr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_JSQRrs;
impl crate::RegisterSpec for ADC_JSQRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_jsqr::R`](R) reader structure"]
impl crate::Readable for ADC_JSQRrs {}
#[doc = "`write(|w| ..)` method takes [`adc_jsqr::W`](W) writer structure"]
impl crate::Writable for ADC_JSQRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_JSQR to value 0"]
impl crate::Resettable for ADC_JSQRrs {
    const RESET_VALUE: u32 = 0;
}
