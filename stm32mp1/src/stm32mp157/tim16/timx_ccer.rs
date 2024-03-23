#[doc = "Register `TIMx_CCER` reader"]
pub type R = crate::R<TIMX_CCERrs>;
#[doc = "Register `TIMx_CCER` writer"]
pub type W = crate::W<TIMX_CCERrs>;
#[doc = "Field `CC1E` reader - CC1E"]
pub type CC1E_R = crate::BitReader;
#[doc = "Field `CC1E` writer - CC1E"]
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1P` reader - CC1P"]
pub type CC1P_R = crate::BitReader;
#[doc = "Field `CC1P` writer - CC1P"]
pub type CC1P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1NE` reader - CC1NE"]
pub type CC1NE_R = crate::BitReader;
#[doc = "Field `CC1NE` writer - CC1NE"]
pub type CC1NE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - CC1NE"]
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 1) != 0)
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
    pub fn cc1e(&mut self) -> CC1E_W<TIMX_CCERrs> {
        CC1E_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC1P"]
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CC1P_W<TIMX_CCERrs> {
        CC1P_W::new(self, 1)
    }
    #[doc = "Bit 2 - CC1NE"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ne(&mut self) -> CC1NE_W<TIMX_CCERrs> {
        CC1NE_W::new(self, 2)
    }
    #[doc = "Bit 3 - CC1NP"]
    #[inline(always)]
    #[must_use]
    pub fn cc1np(&mut self) -> CC1NP_W<TIMX_CCERrs> {
        CC1NP_W::new(self, 3)
    }
}
#[doc = "TIM16/TIM17 capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timx_ccer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_ccer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMX_CCERrs;
impl crate::RegisterSpec for TIMX_CCERrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`timx_ccer::R`](R) reader structure"]
impl crate::Readable for TIMX_CCERrs {}
#[doc = "`write(|w| ..)` method takes [`timx_ccer::W`](W) writer structure"]
impl crate::Writable for TIMX_CCERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIMx_CCER to value 0"]
impl crate::Resettable for TIMX_CCERrs {
    const RESET_VALUE: u16 = 0;
}
