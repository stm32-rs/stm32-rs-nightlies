#[doc = "Register `DIER` reader"]
pub type R = crate::R<DIERrs>;
#[doc = "Register `DIER` writer"]
pub type W = crate::W<DIERrs>;
#[doc = "Field `UIE` reader - Update interrupt enable"]
pub type UIE_R = crate::BitReader;
#[doc = "Field `UIE` writer - Update interrupt enable"]
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IE` reader - Capture/Compare 1 interrupt enable"]
pub type CC1IE_R = crate::BitReader;
#[doc = "Field `CC1IE` writer - Capture/Compare 1 interrupt enable"]
pub type CC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UIE_W<DIERrs> {
        UIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> CC1IE_W<DIERrs> {
        CC1IE_W::new(self, 1)
    }
}
#[doc = "TIM14 Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIERrs;
impl crate::RegisterSpec for DIERrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dier::R`](R) reader structure"]
impl crate::Readable for DIERrs {}
#[doc = "`write(|w| ..)` method takes [`dier::W`](W) writer structure"]
impl crate::Writable for DIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DIER to value 0"]
impl crate::Resettable for DIERrs {
    const RESET_VALUE: u16 = 0;
}
