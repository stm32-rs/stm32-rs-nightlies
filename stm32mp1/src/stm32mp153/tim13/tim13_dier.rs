#[doc = "Register `TIM13_DIER` reader"]
pub type R = crate::R<TIM13_DIERrs>;
#[doc = "Register `TIM13_DIER` writer"]
pub type W = crate::W<TIM13_DIERrs>;
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
    pub fn uie(&mut self) -> UIE_W<TIM13_DIERrs> {
        UIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC1IE"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> CC1IE_W<TIM13_DIERrs> {
        CC1IE_W::new(self, 1)
    }
}
#[doc = "TIM13 Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim13_dier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim13_dier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM13_DIERrs;
impl crate::RegisterSpec for TIM13_DIERrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim13_dier::R`](R) reader structure"]
impl crate::Readable for TIM13_DIERrs {}
#[doc = "`write(|w| ..)` method takes [`tim13_dier::W`](W) writer structure"]
impl crate::Writable for TIM13_DIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM13_DIER to value 0"]
impl crate::Resettable for TIM13_DIERrs {
    const RESET_VALUE: u16 = 0;
}
