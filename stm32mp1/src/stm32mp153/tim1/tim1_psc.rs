#[doc = "Register `TIM1_PSC` reader"]
pub type R = crate::R<TIM1_PSCrs>;
#[doc = "Register `TIM1_PSC` writer"]
pub type W = crate::W<TIM1_PSCrs>;
#[doc = "Field `PSC` reader - PSC"]
pub type PSC_R = crate::FieldReader<u16>;
#[doc = "Field `PSC` writer - PSC"]
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PSC"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - PSC"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<TIM1_PSCrs> {
        PSC_W::new(self, 0)
    }
}
#[doc = "TIM1 prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_psc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_psc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_PSCrs;
impl crate::RegisterSpec for TIM1_PSCrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim1_psc::R`](R) reader structure"]
impl crate::Readable for TIM1_PSCrs {}
#[doc = "`write(|w| ..)` method takes [`tim1_psc::W`](W) writer structure"]
impl crate::Writable for TIM1_PSCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM1_PSC to value 0"]
impl crate::Resettable for TIM1_PSCrs {
    const RESET_VALUE: u16 = 0;
}
