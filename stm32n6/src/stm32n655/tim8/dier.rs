///Register `DIER` reader
pub type R = crate::R<DIERrs>;
///Register `DIER` writer
pub type W = crate::W<DIERrs>;
///Field `UIE` reader - Update interrupt enable
pub type UIE_R = crate::BitReader;
///Field `UIE` writer - Update interrupt enable
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1IE` reader - Capture/compare 1 interrupt enable
pub type CC1IE_R = crate::BitReader;
///Field `CC1IE` writer - Capture/compare 1 interrupt enable
pub type CC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2IE` reader - Capture/compare 2 interrupt enable
pub type CC2IE_R = crate::BitReader;
///Field `CC2IE` writer - Capture/compare 2 interrupt enable
pub type CC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3IE` reader - Capture/compare 3 interrupt enable
pub type CC3IE_R = crate::BitReader;
///Field `CC3IE` writer - Capture/compare 3 interrupt enable
pub type CC3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4IE` reader - Capture/compare 4 interrupt enable
pub type CC4IE_R = crate::BitReader;
///Field `CC4IE` writer - Capture/compare 4 interrupt enable
pub type CC4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMIE` reader - COM interrupt enable
pub type COMIE_R = crate::BitReader;
///Field `COMIE` writer - COM interrupt enable
pub type COMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIE` reader - Trigger interrupt enable
pub type TIE_R = crate::BitReader;
///Field `TIE` writer - Trigger interrupt enable
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIE` reader - Break interrupt enable
pub type BIE_R = crate::BitReader;
///Field `BIE` writer - Break interrupt enable
pub type BIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDE` reader - Update DMA request enable
pub type UDE_R = crate::BitReader;
///Field `UDE` writer - Update DMA request enable
pub type UDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1DE` reader - Capture/compare 1 DMA request enable
pub type CC1DE_R = crate::BitReader;
///Field `CC1DE` writer - Capture/compare 1 DMA request enable
pub type CC1DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2DE` reader - Capture/compare 2 DMA request enable
pub type CC2DE_R = crate::BitReader;
///Field `CC2DE` writer - Capture/compare 2 DMA request enable
pub type CC2DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3DE` reader - Capture/compare 3 DMA request enable
pub type CC3DE_R = crate::BitReader;
///Field `CC3DE` writer - Capture/compare 3 DMA request enable
pub type CC3DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4DE` reader - Capture/compare 4 DMA request enable
pub type CC4DE_R = crate::BitReader;
///Field `CC4DE` writer - Capture/compare 4 DMA request enable
pub type CC4DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMDE` reader - COM DMA request enable
pub type COMDE_R = crate::BitReader;
///Field `COMDE` writer - COM DMA request enable
pub type COMDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TDE` reader - Trigger DMA request enable
pub type TDE_R = crate::BitReader;
///Field `TDE` writer - Trigger DMA request enable
pub type TDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDXIE` reader - Index interrupt enable
pub type IDXIE_R = crate::BitReader;
///Field `IDXIE` writer - Index interrupt enable
pub type IDXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIRIE` reader - Direction change interrupt enable
pub type DIRIE_R = crate::BitReader;
///Field `DIRIE` writer - Direction change interrupt enable
pub type DIRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IERRIE` reader - Index error interrupt enable
pub type IERRIE_R = crate::BitReader;
///Field `IERRIE` writer - Index error interrupt enable
pub type IERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TERRIE` reader - Transition error interrupt enable
pub type TERRIE_R = crate::BitReader;
///Field `TERRIE` writer - Transition error interrupt enable
pub type TERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Capture/compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/compare 3 interrupt enable
    #[inline(always)]
    pub fn cc3ie(&self) -> CC3IE_R {
        CC3IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Capture/compare 4 interrupt enable
    #[inline(always)]
    pub fn cc4ie(&self) -> CC4IE_R {
        CC4IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - COM interrupt enable
    #[inline(always)]
    pub fn comie(&self) -> COMIE_R {
        COMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Trigger interrupt enable
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Break interrupt enable
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Capture/compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Capture/compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Capture/compare 3 DMA request enable
    #[inline(always)]
    pub fn cc3de(&self) -> CC3DE_R {
        CC3DE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Capture/compare 4 DMA request enable
    #[inline(always)]
    pub fn cc4de(&self) -> CC4DE_R {
        CC4DE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - COM DMA request enable
    #[inline(always)]
    pub fn comde(&self) -> COMDE_R {
        COMDE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Trigger DMA request enable
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 20 - Index interrupt enable
    #[inline(always)]
    pub fn idxie(&self) -> IDXIE_R {
        IDXIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Direction change interrupt enable
    #[inline(always)]
    pub fn dirie(&self) -> DIRIE_R {
        DIRIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Index error interrupt enable
    #[inline(always)]
    pub fn ierrie(&self) -> IERRIE_R {
        IERRIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Transition error interrupt enable
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER")
            .field("uie", &self.uie())
            .field("cc1ie", &self.cc1ie())
            .field("cc2ie", &self.cc2ie())
            .field("cc3ie", &self.cc3ie())
            .field("cc4ie", &self.cc4ie())
            .field("comie", &self.comie())
            .field("tie", &self.tie())
            .field("bie", &self.bie())
            .field("ude", &self.ude())
            .field("cc1de", &self.cc1de())
            .field("cc2de", &self.cc2de())
            .field("cc3de", &self.cc3de())
            .field("cc4de", &self.cc4de())
            .field("comde", &self.comde())
            .field("tde", &self.tde())
            .field("idxie", &self.idxie())
            .field("dirie", &self.dirie())
            .field("ierrie", &self.ierrie())
            .field("terrie", &self.terrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<DIERrs> {
        UIE_W::new(self, 0)
    }
    ///Bit 1 - Capture/compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W<DIERrs> {
        CC1IE_W::new(self, 1)
    }
    ///Bit 2 - Capture/compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W<DIERrs> {
        CC2IE_W::new(self, 2)
    }
    ///Bit 3 - Capture/compare 3 interrupt enable
    #[inline(always)]
    pub fn cc3ie(&mut self) -> CC3IE_W<DIERrs> {
        CC3IE_W::new(self, 3)
    }
    ///Bit 4 - Capture/compare 4 interrupt enable
    #[inline(always)]
    pub fn cc4ie(&mut self) -> CC4IE_W<DIERrs> {
        CC4IE_W::new(self, 4)
    }
    ///Bit 5 - COM interrupt enable
    #[inline(always)]
    pub fn comie(&mut self) -> COMIE_W<DIERrs> {
        COMIE_W::new(self, 5)
    }
    ///Bit 6 - Trigger interrupt enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<DIERrs> {
        TIE_W::new(self, 6)
    }
    ///Bit 7 - Break interrupt enable
    #[inline(always)]
    pub fn bie(&mut self) -> BIE_W<DIERrs> {
        BIE_W::new(self, 7)
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&mut self) -> UDE_W<DIERrs> {
        UDE_W::new(self, 8)
    }
    ///Bit 9 - Capture/compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&mut self) -> CC1DE_W<DIERrs> {
        CC1DE_W::new(self, 9)
    }
    ///Bit 10 - Capture/compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&mut self) -> CC2DE_W<DIERrs> {
        CC2DE_W::new(self, 10)
    }
    ///Bit 11 - Capture/compare 3 DMA request enable
    #[inline(always)]
    pub fn cc3de(&mut self) -> CC3DE_W<DIERrs> {
        CC3DE_W::new(self, 11)
    }
    ///Bit 12 - Capture/compare 4 DMA request enable
    #[inline(always)]
    pub fn cc4de(&mut self) -> CC4DE_W<DIERrs> {
        CC4DE_W::new(self, 12)
    }
    ///Bit 13 - COM DMA request enable
    #[inline(always)]
    pub fn comde(&mut self) -> COMDE_W<DIERrs> {
        COMDE_W::new(self, 13)
    }
    ///Bit 14 - Trigger DMA request enable
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W<DIERrs> {
        TDE_W::new(self, 14)
    }
    ///Bit 20 - Index interrupt enable
    #[inline(always)]
    pub fn idxie(&mut self) -> IDXIE_W<DIERrs> {
        IDXIE_W::new(self, 20)
    }
    ///Bit 21 - Direction change interrupt enable
    #[inline(always)]
    pub fn dirie(&mut self) -> DIRIE_W<DIERrs> {
        DIRIE_W::new(self, 21)
    }
    ///Bit 22 - Index error interrupt enable
    #[inline(always)]
    pub fn ierrie(&mut self) -> IERRIE_W<DIERrs> {
        IERRIE_W::new(self, 22)
    }
    ///Bit 23 - Transition error interrupt enable
    #[inline(always)]
    pub fn terrie(&mut self) -> TERRIE_W<DIERrs> {
        TERRIE_W::new(self, 23)
    }
}
/**TIM8 DMA/interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#TIM8:DIER)*/
pub struct DIERrs;
impl crate::RegisterSpec for DIERrs {
    type Ux = u32;
}
///`read()` method returns [`dier::R`](R) reader structure
impl crate::Readable for DIERrs {}
///`write(|w| ..)` method takes [`dier::W`](W) writer structure
impl crate::Writable for DIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIER to value 0
impl crate::Resettable for DIERrs {}
