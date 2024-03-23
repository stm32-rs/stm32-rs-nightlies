#[doc = "Register `ADC2ALT` reader"]
pub type R = crate::R<ADC2ALTrs>;
#[doc = "Register `ADC2ALT` writer"]
pub type W = crate::W<ADC2ALTrs>;
#[doc = "Field `ADC2_ROUT0` reader - ADC2 V_INP16 alternate connection"]
pub type ADC2_ROUT0_R = crate::BitReader;
#[doc = "Field `ADC2_ROUT0` writer - ADC2 V_INP16 alternate connection"]
pub type ADC2_ROUT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ROUT1` reader - ADC2 V_INP17 alternate connection"]
pub type ADC2_ROUT1_R = crate::BitReader;
#[doc = "Field `ADC2_ROUT1` writer - ADC2 V_INP17 alternate connection"]
pub type ADC2_ROUT1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC2 V_INP16 alternate connection"]
    #[inline(always)]
    pub fn adc2_rout0(&self) -> ADC2_ROUT0_R {
        ADC2_ROUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC2 V_INP17 alternate connection"]
    #[inline(always)]
    pub fn adc2_rout1(&self) -> ADC2_ROUT1_R {
        ADC2_ROUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC2 V_INP16 alternate connection"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_rout0(&mut self) -> ADC2_ROUT0_W<ADC2ALTrs> {
        ADC2_ROUT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC2 V_INP17 alternate connection"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_rout1(&mut self) -> ADC2_ROUT1_W<ADC2ALTrs> {
        ADC2_ROUT1_W::new(self, 1)
    }
}
#[doc = "ADC2 internal input alternate connection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc2alt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc2alt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC2ALTrs;
impl crate::RegisterSpec for ADC2ALTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc2alt::R`](R) reader structure"]
impl crate::Readable for ADC2ALTrs {}
#[doc = "`write(|w| ..)` method takes [`adc2alt::W`](W) writer structure"]
impl crate::Writable for ADC2ALTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC2ALT to value 0"]
impl crate::Resettable for ADC2ALTrs {
    const RESET_VALUE: u32 = 0;
}
