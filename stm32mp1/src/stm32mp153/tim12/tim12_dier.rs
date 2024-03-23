#[doc = "Register `TIM12_DIER` reader"]
pub type R = crate::R<TIM12_DIERrs>;
#[doc = "Register `TIM12_DIER` writer"]
pub type W = crate::W<TIM12_DIERrs>;
#[doc = "Field `UIE` reader - UIE"]
pub type UIE_R = crate::BitReader;
#[doc = "Field `UIE` writer - UIE"]
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IE` reader - CC1IE"]
pub type CC1IE_R = crate::BitReader;
#[doc = "Field `CC1IE` writer - CC1IE"]
pub type CC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IE` reader - CC2IE"]
pub type CC2IE_R = crate::BitReader;
#[doc = "Field `CC2IE` writer - CC2IE"]
pub type CC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - TIE"]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - TIE"]
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UIE"]
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC1IE"]
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC2IE"]
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - TIE"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UIE"]
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UIE_W<TIM12_DIERrs> {
        UIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC1IE"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> CC1IE_W<TIM12_DIERrs> {
        CC1IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - CC2IE"]
    #[inline(always)]
    #[must_use]
    pub fn cc2ie(&mut self) -> CC2IE_W<TIM12_DIERrs> {
        CC2IE_W::new(self, 2)
    }
    #[doc = "Bit 6 - TIE"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<TIM12_DIERrs> {
        TIE_W::new(self, 6)
    }
}
#[doc = "TIM12 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim12_dier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim12_dier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM12_DIERrs;
impl crate::RegisterSpec for TIM12_DIERrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim12_dier::R`](R) reader structure"]
impl crate::Readable for TIM12_DIERrs {}
#[doc = "`write(|w| ..)` method takes [`tim12_dier::W`](W) writer structure"]
impl crate::Writable for TIM12_DIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM12_DIER to value 0"]
impl crate::Resettable for TIM12_DIERrs {
    const RESET_VALUE: u16 = 0;
}
