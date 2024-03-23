#[doc = "Register `TIM4_PSC` reader"]
pub type R = crate::R<TIM4_PSCrs>;
#[doc = "Register `TIM4_PSC` writer"]
pub type W = crate::W<TIM4_PSCrs>;
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
    pub fn psc(&mut self) -> PSC_W<TIM4_PSCrs> {
        PSC_W::new(self, 0)
    }
}
#[doc = "TIM4 prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim4_psc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim4_psc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM4_PSCrs;
impl crate::RegisterSpec for TIM4_PSCrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim4_psc::R`](R) reader structure"]
impl crate::Readable for TIM4_PSCrs {}
#[doc = "`write(|w| ..)` method takes [`tim4_psc::W`](W) writer structure"]
impl crate::Writable for TIM4_PSCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM4_PSC to value 0"]
impl crate::Resettable for TIM4_PSCrs {
    const RESET_VALUE: u16 = 0;
}
