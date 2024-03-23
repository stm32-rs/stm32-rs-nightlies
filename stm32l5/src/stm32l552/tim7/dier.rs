#[doc = "Register `DIER` reader"]
pub type R = crate::R<DIERrs>;
#[doc = "Register `DIER` writer"]
pub type W = crate::W<DIERrs>;
#[doc = "Field `UIE` reader - Update interrupt enable"]
pub type UIE_R = crate::BitReader;
#[doc = "Field `UIE` writer - Update interrupt enable"]
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDE` reader - Update DMA request enable"]
pub type UDE_R = crate::BitReader;
#[doc = "Field `UDE` writer - Update DMA request enable"]
pub type UDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UIE_W<DIERrs> {
        UIE_W::new(self, 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ude(&mut self) -> UDE_W<DIERrs> {
        UDE_W::new(self, 8)
    }
}
#[doc = "DMA/Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIERrs;
impl crate::RegisterSpec for DIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dier::R`](R) reader structure"]
impl crate::Readable for DIERrs {}
#[doc = "`write(|w| ..)` method takes [`dier::W`](W) writer structure"]
impl crate::Writable for DIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIER to value 0"]
impl crate::Resettable for DIERrs {
    const RESET_VALUE: u32 = 0;
}
