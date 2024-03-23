#[doc = "Register `TIM12_CCER` reader"]
pub type R = crate::R<TIM12_CCERrs>;
#[doc = "Register `TIM12_CCER` writer"]
pub type W = crate::W<TIM12_CCERrs>;
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
#[doc = "Field `CC2E` reader - CC2E"]
pub type CC2E_R = crate::BitReader;
#[doc = "Field `CC2E` writer - CC2E"]
pub type CC2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2P` reader - CC2P"]
pub type CC2P_R = crate::BitReader;
#[doc = "Field `CC2P` writer - CC2P"]
pub type CC2P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NP` reader - CC2NP"]
pub type CC2NP_R = crate::BitReader;
#[doc = "Field `CC2NP` writer - CC2NP"]
pub type CC2NP_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 4 - CC2E"]
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CC2P"]
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - CC2NP"]
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CC1E"]
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> CC1E_W<TIM12_CCERrs> {
        CC1E_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC1P"]
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CC1P_W<TIM12_CCERrs> {
        CC1P_W::new(self, 1)
    }
    #[doc = "Bit 3 - CC1NP"]
    #[inline(always)]
    #[must_use]
    pub fn cc1np(&mut self) -> CC1NP_W<TIM12_CCERrs> {
        CC1NP_W::new(self, 3)
    }
    #[doc = "Bit 4 - CC2E"]
    #[inline(always)]
    #[must_use]
    pub fn cc2e(&mut self) -> CC2E_W<TIM12_CCERrs> {
        CC2E_W::new(self, 4)
    }
    #[doc = "Bit 5 - CC2P"]
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> CC2P_W<TIM12_CCERrs> {
        CC2P_W::new(self, 5)
    }
    #[doc = "Bit 7 - CC2NP"]
    #[inline(always)]
    #[must_use]
    pub fn cc2np(&mut self) -> CC2NP_W<TIM12_CCERrs> {
        CC2NP_W::new(self, 7)
    }
}
#[doc = "TIM12 capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim12_ccer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim12_ccer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM12_CCERrs;
impl crate::RegisterSpec for TIM12_CCERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim12_ccer::R`](R) reader structure"]
impl crate::Readable for TIM12_CCERrs {}
#[doc = "`write(|w| ..)` method takes [`tim12_ccer::W`](W) writer structure"]
impl crate::Writable for TIM12_CCERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM12_CCER to value 0"]
impl crate::Resettable for TIM12_CCERrs {
    const RESET_VALUE: u32 = 0;
}
