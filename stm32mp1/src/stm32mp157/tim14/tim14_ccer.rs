#[doc = "Register `TIM14_CCER` reader"]
pub type R = crate::R<TIM14_CCERrs>;
#[doc = "Register `TIM14_CCER` writer"]
pub type W = crate::W<TIM14_CCERrs>;
#[doc = "Field `CC1E` reader - CC1E"]
pub type CC1E_R = crate::BitReader;
#[doc = "Field `CC1E` writer - CC1E"]
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1P` reader - CC1P"]
pub type CC1P_R = crate::BitReader;
#[doc = "Field `CC1P` writer - CC1P"]
pub type CC1P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1NP` reader - CC1NP"]
pub type CC1NP_R = crate::BitReader;
#[doc = "Field `CC1NP` writer - CC1NP"]
pub type CC1NP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CC1E"]
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC1P"]
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - CC1NP"]
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CC1E"]
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> CC1E_W<TIM14_CCERrs> {
        CC1E_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC1P"]
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CC1P_W<TIM14_CCERrs> {
        CC1P_W::new(self, 1)
    }
    #[doc = "Bit 3 - CC1NP"]
    #[inline(always)]
    #[must_use]
    pub fn cc1np(&mut self) -> CC1NP_W<TIM14_CCERrs> {
        CC1NP_W::new(self, 3)
    }
}
#[doc = "TIM14 capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim14_ccer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim14_ccer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM14_CCERrs;
impl crate::RegisterSpec for TIM14_CCERrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim14_ccer::R`](R) reader structure"]
impl crate::Readable for TIM14_CCERrs {}
#[doc = "`write(|w| ..)` method takes [`tim14_ccer::W`](W) writer structure"]
impl crate::Writable for TIM14_CCERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM14_CCER to value 0"]
impl crate::Resettable for TIM14_CCERrs {
    const RESET_VALUE: u16 = 0;
}
