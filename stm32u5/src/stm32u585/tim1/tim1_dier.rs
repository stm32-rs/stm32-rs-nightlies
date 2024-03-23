#[doc = "Register `TIM1_DIER` reader"]
pub type R = crate::R<TIM1_DIERrs>;
#[doc = "Register `TIM1_DIER` writer"]
pub type W = crate::W<TIM1_DIERrs>;
#[doc = "Field `UIE` reader - Update interrupt enable"]
pub type UIE_R = crate::BitReader;
#[doc = "Field `UIE` writer - Update interrupt enable"]
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IE` reader - Capture/compare 1 interrupt enable"]
pub type CC1IE_R = crate::BitReader;
#[doc = "Field `CC1IE` writer - Capture/compare 1 interrupt enable"]
pub type CC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IE` reader - Capture/compare 2 interrupt enable"]
pub type CC2IE_R = crate::BitReader;
#[doc = "Field `CC2IE` writer - Capture/compare 2 interrupt enable"]
pub type CC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IE` reader - Capture/compare 3 interrupt enable"]
pub type CC3IE_R = crate::BitReader;
#[doc = "Field `CC3IE` writer - Capture/compare 3 interrupt enable"]
pub type CC3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4IE` reader - Capture/compare 4 interrupt enable"]
pub type CC4IE_R = crate::BitReader;
#[doc = "Field `CC4IE` writer - Capture/compare 4 interrupt enable"]
pub type CC4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMIE` reader - COM interrupt enable"]
pub type COMIE_R = crate::BitReader;
#[doc = "Field `COMIE` writer - COM interrupt enable"]
pub type COMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - Trigger interrupt enable"]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - Trigger interrupt enable"]
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIE` reader - Break interrupt enable"]
pub type BIE_R = crate::BitReader;
#[doc = "Field `BIE` writer - Break interrupt enable"]
pub type BIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDE` reader - Update DMA request enable"]
pub type UDE_R = crate::BitReader;
#[doc = "Field `UDE` writer - Update DMA request enable"]
pub type UDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1DE` reader - Capture/compare 1 DMA request enable"]
pub type CC1DE_R = crate::BitReader;
#[doc = "Field `CC1DE` writer - Capture/compare 1 DMA request enable"]
pub type CC1DE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2DE` reader - Capture/compare 2 DMA request enable"]
pub type CC2DE_R = crate::BitReader;
#[doc = "Field `CC2DE` writer - Capture/compare 2 DMA request enable"]
pub type CC2DE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3DE` reader - Capture/compare 3 DMA request enable"]
pub type CC3DE_R = crate::BitReader;
#[doc = "Field `CC3DE` writer - Capture/compare 3 DMA request enable"]
pub type CC3DE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4DE` reader - Capture/compare 4 DMA request enable"]
pub type CC4DE_R = crate::BitReader;
#[doc = "Field `CC4DE` writer - Capture/compare 4 DMA request enable"]
pub type CC4DE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMDE` reader - COM DMA request enable"]
pub type COMDE_R = crate::BitReader;
#[doc = "Field `COMDE` writer - COM DMA request enable"]
pub type COMDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDE` reader - Trigger DMA request enable"]
pub type TDE_R = crate::BitReader;
#[doc = "Field `TDE` writer - Trigger DMA request enable"]
pub type TDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDXIE` reader - Index interrupt enable"]
pub type IDXIE_R = crate::BitReader;
#[doc = "Field `IDXIE` writer - Index interrupt enable"]
pub type IDXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRIE` reader - Direction change interrupt enable"]
pub type DIRIE_R = crate::BitReader;
#[doc = "Field `DIRIE` writer - Direction change interrupt enable"]
pub type DIRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IERRIE` reader - Index error interrupt enable"]
pub type IERRIE_R = crate::BitReader;
#[doc = "Field `IERRIE` writer - Index error interrupt enable"]
pub type IERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERRIE` reader - Transition error interrupt enable"]
pub type TERRIE_R = crate::BitReader;
#[doc = "Field `TERRIE` writer - Transition error interrupt enable"]
pub type TERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare 2 interrupt enable"]
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare 3 interrupt enable"]
    #[inline(always)]
    pub fn cc3ie(&self) -> CC3IE_R {
        CC3IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/compare 4 interrupt enable"]
    #[inline(always)]
    pub fn cc4ie(&self) -> CC4IE_R {
        CC4IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn comie(&self) -> COMIE_R {
        COMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/compare 1 DMA request enable"]
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/compare 2 DMA request enable"]
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/compare 3 DMA request enable"]
    #[inline(always)]
    pub fn cc3de(&self) -> CC3DE_R {
        CC3DE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/compare 4 DMA request enable"]
    #[inline(always)]
    pub fn cc4de(&self) -> CC4DE_R {
        CC4DE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline(always)]
    pub fn comde(&self) -> COMDE_R {
        COMDE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 20 - Index interrupt enable"]
    #[inline(always)]
    pub fn idxie(&self) -> IDXIE_R {
        IDXIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Direction change interrupt enable"]
    #[inline(always)]
    pub fn dirie(&self) -> DIRIE_R {
        DIRIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Index error interrupt enable"]
    #[inline(always)]
    pub fn ierrie(&self) -> IERRIE_R {
        IERRIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transition error interrupt enable"]
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UIE_W<TIM1_DIERrs> {
        UIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> CC1IE_W<TIM1_DIERrs> {
        CC1IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/compare 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2ie(&mut self) -> CC2IE_W<TIM1_DIERrs> {
        CC2IE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare 3 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc3ie(&mut self) -> CC3IE_W<TIM1_DIERrs> {
        CC3IE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/compare 4 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc4ie(&mut self) -> CC4IE_W<TIM1_DIERrs> {
        CC4IE_W::new(self, 4)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn comie(&mut self) -> COMIE_W<TIM1_DIERrs> {
        COMIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<TIM1_DIERrs> {
        TIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bie(&mut self) -> BIE_W<TIM1_DIERrs> {
        BIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ude(&mut self) -> UDE_W<TIM1_DIERrs> {
        UDE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/compare 1 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1de(&mut self) -> CC1DE_W<TIM1_DIERrs> {
        CC1DE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/compare 2 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2de(&mut self) -> CC2DE_W<TIM1_DIERrs> {
        CC2DE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/compare 3 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc3de(&mut self) -> CC3DE_W<TIM1_DIERrs> {
        CC3DE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/compare 4 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc4de(&mut self) -> CC4DE_W<TIM1_DIERrs> {
        CC4DE_W::new(self, 12)
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn comde(&mut self) -> COMDE_W<TIM1_DIERrs> {
        COMDE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn tde(&mut self) -> TDE_W<TIM1_DIERrs> {
        TDE_W::new(self, 14)
    }
    #[doc = "Bit 20 - Index interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn idxie(&mut self) -> IDXIE_W<TIM1_DIERrs> {
        IDXIE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Direction change interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirie(&mut self) -> DIRIE_W<TIM1_DIERrs> {
        DIRIE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Index error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ierrie(&mut self) -> IERRIE_W<TIM1_DIERrs> {
        IERRIE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Transition error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn terrie(&mut self) -> TERRIE_W<TIM1_DIERrs> {
        TERRIE_W::new(self, 23)
    }
}
#[doc = "TIM1 DMA/interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_dier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_dier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_DIERrs;
impl crate::RegisterSpec for TIM1_DIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_dier::R`](R) reader structure"]
impl crate::Readable for TIM1_DIERrs {}
#[doc = "`write(|w| ..)` method takes [`tim1_dier::W`](W) writer structure"]
impl crate::Writable for TIM1_DIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_DIER to value 0"]
impl crate::Resettable for TIM1_DIERrs {
    const RESET_VALUE: u32 = 0;
}
