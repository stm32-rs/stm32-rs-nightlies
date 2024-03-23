#[doc = "Register `TIM14_DIER` reader"]
pub type R = crate::R<TIM14_DIERrs>;
#[doc = "Register `TIM14_DIER` writer"]
pub type W = crate::W<TIM14_DIERrs>;
#[doc = "Field `UIE` reader - UIE"]
pub type UIE_R = crate::BitReader;
#[doc = "Field `UIE` writer - UIE"]
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IE` reader - CC1IE"]
pub type CC1IE_R = crate::BitReader;
#[doc = "Field `CC1IE` writer - CC1IE"]
pub type CC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
}
impl W {
    #[doc = "Bit 0 - UIE"]
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UIE_W<TIM14_DIERrs> {
        UIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC1IE"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> CC1IE_W<TIM14_DIERrs> {
        CC1IE_W::new(self, 1)
    }
}
#[doc = "TIM14 Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim14_dier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim14_dier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM14_DIERrs;
impl crate::RegisterSpec for TIM14_DIERrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim14_dier::R`](R) reader structure"]
impl crate::Readable for TIM14_DIERrs {}
#[doc = "`write(|w| ..)` method takes [`tim14_dier::W`](W) writer structure"]
impl crate::Writable for TIM14_DIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM14_DIER to value 0"]
impl crate::Resettable for TIM14_DIERrs {
    const RESET_VALUE: u16 = 0;
}
