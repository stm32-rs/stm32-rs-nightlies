#[doc = "Register `ADC_SMPR2` reader"]
pub type R = crate::R<ADC_SMPR2rs>;
#[doc = "Register `ADC_SMPR2` writer"]
pub type W = crate::W<ADC_SMPR2rs>;
#[doc = "Field `SMP10` reader - SMP10"]
pub type SMP10_R = crate::FieldReader;
#[doc = "Field `SMP10` writer - SMP10"]
pub type SMP10_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP11` reader - SMP11"]
pub type SMP11_R = crate::FieldReader;
#[doc = "Field `SMP11` writer - SMP11"]
pub type SMP11_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP12` reader - SMP12"]
pub type SMP12_R = crate::FieldReader;
#[doc = "Field `SMP12` writer - SMP12"]
pub type SMP12_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP13` reader - SMP13"]
pub type SMP13_R = crate::FieldReader;
#[doc = "Field `SMP13` writer - SMP13"]
pub type SMP13_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP14` reader - SMP14"]
pub type SMP14_R = crate::FieldReader;
#[doc = "Field `SMP14` writer - SMP14"]
pub type SMP14_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP15` reader - SMP15"]
pub type SMP15_R = crate::FieldReader;
#[doc = "Field `SMP15` writer - SMP15"]
pub type SMP15_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP16` reader - SMP16"]
pub type SMP16_R = crate::FieldReader;
#[doc = "Field `SMP16` writer - SMP16"]
pub type SMP16_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP17` reader - SMP17"]
pub type SMP17_R = crate::FieldReader;
#[doc = "Field `SMP17` writer - SMP17"]
pub type SMP17_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP18` reader - SMP18"]
pub type SMP18_R = crate::FieldReader;
#[doc = "Field `SMP18` writer - SMP18"]
pub type SMP18_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP19` reader - SMP19"]
pub type SMP19_R = crate::FieldReader;
#[doc = "Field `SMP19` writer - SMP19"]
pub type SMP19_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SMP10"]
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - SMP11"]
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - SMP12"]
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - SMP13"]
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SMP14"]
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - SMP15"]
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - SMP16"]
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - SMP17"]
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - SMP18"]
    #[inline(always)]
    pub fn smp18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - SMP19"]
    #[inline(always)]
    pub fn smp19(&self) -> SMP19_R {
        SMP19_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SMP10"]
    #[inline(always)]
    #[must_use]
    pub fn smp10(&mut self) -> SMP10_W<ADC_SMPR2rs> {
        SMP10_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - SMP11"]
    #[inline(always)]
    #[must_use]
    pub fn smp11(&mut self) -> SMP11_W<ADC_SMPR2rs> {
        SMP11_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - SMP12"]
    #[inline(always)]
    #[must_use]
    pub fn smp12(&mut self) -> SMP12_W<ADC_SMPR2rs> {
        SMP12_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - SMP13"]
    #[inline(always)]
    #[must_use]
    pub fn smp13(&mut self) -> SMP13_W<ADC_SMPR2rs> {
        SMP13_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - SMP14"]
    #[inline(always)]
    #[must_use]
    pub fn smp14(&mut self) -> SMP14_W<ADC_SMPR2rs> {
        SMP14_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - SMP15"]
    #[inline(always)]
    #[must_use]
    pub fn smp15(&mut self) -> SMP15_W<ADC_SMPR2rs> {
        SMP15_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - SMP16"]
    #[inline(always)]
    #[must_use]
    pub fn smp16(&mut self) -> SMP16_W<ADC_SMPR2rs> {
        SMP16_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - SMP17"]
    #[inline(always)]
    #[must_use]
    pub fn smp17(&mut self) -> SMP17_W<ADC_SMPR2rs> {
        SMP17_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - SMP18"]
    #[inline(always)]
    #[must_use]
    pub fn smp18(&mut self) -> SMP18_W<ADC_SMPR2rs> {
        SMP18_W::new(self, 24)
    }
    #[doc = "Bits 27:29 - SMP19"]
    #[inline(always)]
    #[must_use]
    pub fn smp19(&mut self) -> SMP19_W<ADC_SMPR2rs> {
        SMP19_W::new(self, 27)
    }
}
#[doc = "ADC sample time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_smpr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_smpr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_SMPR2rs;
impl crate::RegisterSpec for ADC_SMPR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_smpr2::R`](R) reader structure"]
impl crate::Readable for ADC_SMPR2rs {}
#[doc = "`write(|w| ..)` method takes [`adc_smpr2::W`](W) writer structure"]
impl crate::Writable for ADC_SMPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_SMPR2 to value 0"]
impl crate::Resettable for ADC_SMPR2rs {
    const RESET_VALUE: u32 = 0;
}
