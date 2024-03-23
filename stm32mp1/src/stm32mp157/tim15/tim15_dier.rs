#[doc = "Register `TIM15_DIER` reader"]
pub type R = crate::R<TIM15_DIERrs>;
#[doc = "Register `TIM15_DIER` writer"]
pub type W = crate::W<TIM15_DIERrs>;
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
#[doc = "Field `COMIE` reader - COMIE"]
pub type COMIE_R = crate::BitReader;
#[doc = "Field `COMIE` writer - COMIE"]
pub type COMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - TIE"]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - TIE"]
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIE` reader - BIE"]
pub type BIE_R = crate::BitReader;
#[doc = "Field `BIE` writer - BIE"]
pub type BIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDE` reader - UDE"]
pub type UDE_R = crate::BitReader;
#[doc = "Field `UDE` writer - UDE"]
pub type UDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1DE` reader - CC1DE"]
pub type CC1DE_R = crate::BitReader;
#[doc = "Field `CC1DE` writer - CC1DE"]
pub type CC1DE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2DE` reader - CC2DE"]
pub type CC2DE_R = crate::BitReader;
#[doc = "Field `CC2DE` writer - CC2DE"]
pub type CC2DE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMDE` reader - COMDE"]
pub type COMDE_R = crate::BitReader;
#[doc = "Field `COMDE` writer - COMDE"]
pub type COMDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDE` reader - TDE"]
pub type TDE_R = crate::BitReader;
#[doc = "Field `TDE` writer - TDE"]
pub type TDE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 5 - COMIE"]
    #[inline(always)]
    pub fn comie(&self) -> COMIE_R {
        COMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIE"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BIE"]
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UDE"]
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC1DE"]
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC2DE"]
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - COMDE"]
    #[inline(always)]
    pub fn comde(&self) -> COMDE_R {
        COMDE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TDE"]
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UIE"]
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UIE_W<TIM15_DIERrs> {
        UIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC1IE"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> CC1IE_W<TIM15_DIERrs> {
        CC1IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - CC2IE"]
    #[inline(always)]
    #[must_use]
    pub fn cc2ie(&mut self) -> CC2IE_W<TIM15_DIERrs> {
        CC2IE_W::new(self, 2)
    }
    #[doc = "Bit 5 - COMIE"]
    #[inline(always)]
    #[must_use]
    pub fn comie(&mut self) -> COMIE_W<TIM15_DIERrs> {
        COMIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - TIE"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<TIM15_DIERrs> {
        TIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - BIE"]
    #[inline(always)]
    #[must_use]
    pub fn bie(&mut self) -> BIE_W<TIM15_DIERrs> {
        BIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - UDE"]
    #[inline(always)]
    #[must_use]
    pub fn ude(&mut self) -> UDE_W<TIM15_DIERrs> {
        UDE_W::new(self, 8)
    }
    #[doc = "Bit 9 - CC1DE"]
    #[inline(always)]
    #[must_use]
    pub fn cc1de(&mut self) -> CC1DE_W<TIM15_DIERrs> {
        CC1DE_W::new(self, 9)
    }
    #[doc = "Bit 10 - CC2DE"]
    #[inline(always)]
    #[must_use]
    pub fn cc2de(&mut self) -> CC2DE_W<TIM15_DIERrs> {
        CC2DE_W::new(self, 10)
    }
    #[doc = "Bit 13 - COMDE"]
    #[inline(always)]
    #[must_use]
    pub fn comde(&mut self) -> COMDE_W<TIM15_DIERrs> {
        COMDE_W::new(self, 13)
    }
    #[doc = "Bit 14 - TDE"]
    #[inline(always)]
    #[must_use]
    pub fn tde(&mut self) -> TDE_W<TIM15_DIERrs> {
        TDE_W::new(self, 14)
    }
}
#[doc = "TIM15 DMA/interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_dier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_dier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM15_DIERrs;
impl crate::RegisterSpec for TIM15_DIERrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim15_dier::R`](R) reader structure"]
impl crate::Readable for TIM15_DIERrs {}
#[doc = "`write(|w| ..)` method takes [`tim15_dier::W`](W) writer structure"]
impl crate::Writable for TIM15_DIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM15_DIER to value 0"]
impl crate::Resettable for TIM15_DIERrs {
    const RESET_VALUE: u16 = 0;
}
