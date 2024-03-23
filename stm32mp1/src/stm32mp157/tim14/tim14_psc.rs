#[doc = "Register `TIM14_PSC` reader"]
pub type R = crate::R<TIM14_PSCrs>;
#[doc = "Register `TIM14_PSC` writer"]
pub type W = crate::W<TIM14_PSCrs>;
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
    pub fn psc(&mut self) -> PSC_W<TIM14_PSCrs> {
        PSC_W::new(self, 0)
    }
}
#[doc = "TIM14 prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim14_psc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim14_psc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM14_PSCrs;
impl crate::RegisterSpec for TIM14_PSCrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim14_psc::R`](R) reader structure"]
impl crate::Readable for TIM14_PSCrs {}
#[doc = "`write(|w| ..)` method takes [`tim14_psc::W`](W) writer structure"]
impl crate::Writable for TIM14_PSCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM14_PSC to value 0"]
impl crate::Resettable for TIM14_PSCrs {
    const RESET_VALUE: u16 = 0;
}
