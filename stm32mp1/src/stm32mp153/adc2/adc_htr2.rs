#[doc = "Register `ADC_HTR2` reader"]
pub type R = crate::R<ADC_HTR2rs>;
#[doc = "Register `ADC_HTR2` writer"]
pub type W = crate::W<ADC_HTR2rs>;
#[doc = "Field `HTR2` reader - HTR2"]
pub type HTR2_R = crate::FieldReader<u32>;
#[doc = "Field `HTR2` writer - HTR2"]
pub type HTR2_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - HTR2"]
    #[inline(always)]
    pub fn htr2(&self) -> HTR2_R {
        HTR2_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - HTR2"]
    #[inline(always)]
    #[must_use]
    pub fn htr2(&mut self) -> HTR2_W<ADC_HTR2rs> {
        HTR2_W::new(self, 0)
    }
}
#[doc = "ADC watchdog higher threshold register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_htr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_htr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_HTR2rs;
impl crate::RegisterSpec for ADC_HTR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_htr2::R`](R) reader structure"]
impl crate::Readable for ADC_HTR2rs {}
#[doc = "`write(|w| ..)` method takes [`adc_htr2::W`](W) writer structure"]
impl crate::Writable for ADC_HTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_HTR2 to value 0x03ff_ffff"]
impl crate::Resettable for ADC_HTR2rs {
    const RESET_VALUE: u32 = 0x03ff_ffff;
}
