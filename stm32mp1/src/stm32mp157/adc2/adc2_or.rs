#[doc = "Register `ADC2_OR` reader"]
pub type R = crate::R<ADC2_ORrs>;
#[doc = "Register `ADC2_OR` writer"]
pub type W = crate::W<ADC2_ORrs>;
#[doc = "Field `VDDCOREEN` reader - VDDCOREEN"]
pub type VDDCOREEN_R = crate::BitReader;
#[doc = "Field `VDDCOREEN` writer - VDDCOREEN"]
pub type VDDCOREEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VDDCOREEN"]
    #[inline(always)]
    pub fn vddcoreen(&self) -> VDDCOREEN_R {
        VDDCOREEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VDDCOREEN"]
    #[inline(always)]
    #[must_use]
    pub fn vddcoreen(&mut self) -> VDDCOREEN_W<ADC2_ORrs> {
        VDDCOREEN_W::new(self, 0)
    }
}
#[doc = "ADC2 option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc2_or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc2_or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC2_ORrs;
impl crate::RegisterSpec for ADC2_ORrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc2_or::R`](R) reader structure"]
impl crate::Readable for ADC2_ORrs {}
#[doc = "`write(|w| ..)` method takes [`adc2_or::W`](W) writer structure"]
impl crate::Writable for ADC2_ORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC2_OR to value 0"]
impl crate::Resettable for ADC2_ORrs {
    const RESET_VALUE: u32 = 0;
}
