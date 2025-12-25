///Register `DIER` reader
pub type R = crate::R<DIERrs>;
///Register `DIER` writer
pub type W = crate::W<DIERrs>;
///Field `UIE` reader - UIE: Update interrupt enable 0: Update interrupt disabled 1: Update interrupt enabled
pub type UIE_R = crate::BitReader;
///Field `UIE` writer - UIE: Update interrupt enable 0: Update interrupt disabled 1: Update interrupt enabled
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1IE` reader - CC1IE: Capture/Compare 1 interrupt enable 0: CC1 interrupt disabled. 1: CC1 interrupt enabled
pub type CC1IE_R = crate::BitReader;
///Field `CC1IE` writer - CC1IE: Capture/Compare 1 interrupt enable 0: CC1 interrupt disabled. 1: CC1 interrupt enabled
pub type CC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2IE` reader - CC2IE: Capture/Compare 2 interrupt enable 0: CC2 interrupt disabled 1: CC2 interrupt enabled
pub type CC2IE_R = crate::BitReader;
///Field `CC2IE` writer - CC2IE: Capture/Compare 2 interrupt enable 0: CC2 interrupt disabled 1: CC2 interrupt enabled
pub type CC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3IE` reader - CC3IE: Capture/Compare 3 interrupt enable 0: CC3 interrupt disabled 1: CC3 interrupt enabled
pub type CC3IE_R = crate::BitReader;
///Field `CC3IE` writer - CC3IE: Capture/Compare 3 interrupt enable 0: CC3 interrupt disabled 1: CC3 interrupt enabled
pub type CC3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4IE` reader - CC4IE: Capture/Compare 4 interrupt enable 0: CC4 interrupt disabled 1: CC4 interrupt enabled
pub type CC4IE_R = crate::BitReader;
///Field `CC4IE` writer - CC4IE: Capture/Compare 4 interrupt enable 0: CC4 interrupt disabled 1: CC4 interrupt enabled
pub type CC4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIE` reader - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled
pub type TIE_R = crate::BitReader;
///Field `TIE` writer - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDE` reader - UDE: Update DMA request enable 0: Update DMA request disabled 1: Update DMA request enabled
pub type UDE_R = crate::BitReader;
///Field `UDE` writer - UDE: Update DMA request enable 0: Update DMA request disabled 1: Update DMA request enabled
pub type UDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1DE` reader - CC1DE: Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled 1: CC1 DMA request enabled
pub type CC1DE_R = crate::BitReader;
///Field `CC1DE` writer - CC1DE: Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled 1: CC1 DMA request enabled
pub type CC1DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2DE` reader - CC2DE: Capture/Compare 2 DMA request enable 0: CC2 DMA request disabled 1: CC2 DMA request enabled
pub type CC2DE_R = crate::BitReader;
///Field `CC2DE` writer - CC2DE: Capture/Compare 2 DMA request enable 0: CC2 DMA request disabled 1: CC2 DMA request enabled
pub type CC2DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3DE` reader - CC3DE: Capture/Compare 3 DMA request enable 0: CC3 DMA request disabled 1: CC3 DMA request enabled
pub type CC3DE_R = crate::BitReader;
///Field `CC3DE` writer - CC3DE: Capture/Compare 3 DMA request enable 0: CC3 DMA request disabled 1: CC3 DMA request enabled
pub type CC3DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4DE` reader - CC4DE: Capture/Compare 4 DMA request enable 0: CC4 DMA request disabled 1: CC4 DMA request enabled
pub type CC4DE_R = crate::BitReader;
///Field `CC4DE` writer - CC4DE: Capture/Compare 4 DMA request enable 0: CC4 DMA request disabled 1: CC4 DMA request enabled
pub type CC4DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TDE` reader - TDE: Trigger DMA request enable 0: Trigger DMA request disabled 1: Trigger DMA request enabled
pub type TDE_R = crate::BitReader;
///Field `TDE` writer - TDE: Trigger DMA request enable 0: Trigger DMA request disabled 1: Trigger DMA request enabled
pub type TDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - UIE: Update interrupt enable 0: Update interrupt disabled 1: Update interrupt enabled
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CC1IE: Capture/Compare 1 interrupt enable 0: CC1 interrupt disabled. 1: CC1 interrupt enabled
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CC2IE: Capture/Compare 2 interrupt enable 0: CC2 interrupt disabled 1: CC2 interrupt enabled
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CC3IE: Capture/Compare 3 interrupt enable 0: CC3 interrupt disabled 1: CC3 interrupt enabled
    #[inline(always)]
    pub fn cc3ie(&self) -> CC3IE_R {
        CC3IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CC4IE: Capture/Compare 4 interrupt enable 0: CC4 interrupt disabled 1: CC4 interrupt enabled
    #[inline(always)]
    pub fn cc4ie(&self) -> CC4IE_R {
        CC4IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - UDE: Update DMA request enable 0: Update DMA request disabled 1: Update DMA request enabled
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CC1DE: Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled 1: CC1 DMA request enabled
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CC2DE: Capture/Compare 2 DMA request enable 0: CC2 DMA request disabled 1: CC2 DMA request enabled
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CC3DE: Capture/Compare 3 DMA request enable 0: CC3 DMA request disabled 1: CC3 DMA request enabled
    #[inline(always)]
    pub fn cc3de(&self) -> CC3DE_R {
        CC3DE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CC4DE: Capture/Compare 4 DMA request enable 0: CC4 DMA request disabled 1: CC4 DMA request enabled
    #[inline(always)]
    pub fn cc4de(&self) -> CC4DE_R {
        CC4DE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - TDE: Trigger DMA request enable 0: Trigger DMA request disabled 1: Trigger DMA request enabled
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 14) & 1) != 0)
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
            .field("tie", &self.tie())
            .field("ude", &self.ude())
            .field("cc1de", &self.cc1de())
            .field("cc2de", &self.cc2de())
            .field("cc3de", &self.cc3de())
            .field("cc4de", &self.cc4de())
            .field("tde", &self.tde())
            .finish()
    }
}
impl W {
    ///Bit 0 - UIE: Update interrupt enable 0: Update interrupt disabled 1: Update interrupt enabled
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<'_, DIERrs> {
        UIE_W::new(self, 0)
    }
    ///Bit 1 - CC1IE: Capture/Compare 1 interrupt enable 0: CC1 interrupt disabled. 1: CC1 interrupt enabled
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W<'_, DIERrs> {
        CC1IE_W::new(self, 1)
    }
    ///Bit 2 - CC2IE: Capture/Compare 2 interrupt enable 0: CC2 interrupt disabled 1: CC2 interrupt enabled
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W<'_, DIERrs> {
        CC2IE_W::new(self, 2)
    }
    ///Bit 3 - CC3IE: Capture/Compare 3 interrupt enable 0: CC3 interrupt disabled 1: CC3 interrupt enabled
    #[inline(always)]
    pub fn cc3ie(&mut self) -> CC3IE_W<'_, DIERrs> {
        CC3IE_W::new(self, 3)
    }
    ///Bit 4 - CC4IE: Capture/Compare 4 interrupt enable 0: CC4 interrupt disabled 1: CC4 interrupt enabled
    #[inline(always)]
    pub fn cc4ie(&mut self) -> CC4IE_W<'_, DIERrs> {
        CC4IE_W::new(self, 4)
    }
    ///Bit 6 - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<'_, DIERrs> {
        TIE_W::new(self, 6)
    }
    ///Bit 8 - UDE: Update DMA request enable 0: Update DMA request disabled 1: Update DMA request enabled
    #[inline(always)]
    pub fn ude(&mut self) -> UDE_W<'_, DIERrs> {
        UDE_W::new(self, 8)
    }
    ///Bit 9 - CC1DE: Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled 1: CC1 DMA request enabled
    #[inline(always)]
    pub fn cc1de(&mut self) -> CC1DE_W<'_, DIERrs> {
        CC1DE_W::new(self, 9)
    }
    ///Bit 10 - CC2DE: Capture/Compare 2 DMA request enable 0: CC2 DMA request disabled 1: CC2 DMA request enabled
    #[inline(always)]
    pub fn cc2de(&mut self) -> CC2DE_W<'_, DIERrs> {
        CC2DE_W::new(self, 10)
    }
    ///Bit 11 - CC3DE: Capture/Compare 3 DMA request enable 0: CC3 DMA request disabled 1: CC3 DMA request enabled
    #[inline(always)]
    pub fn cc3de(&mut self) -> CC3DE_W<'_, DIERrs> {
        CC3DE_W::new(self, 11)
    }
    ///Bit 12 - CC4DE: Capture/Compare 4 DMA request enable 0: CC4 DMA request disabled 1: CC4 DMA request enabled
    #[inline(always)]
    pub fn cc4de(&mut self) -> CC4DE_W<'_, DIERrs> {
        CC4DE_W::new(self, 12)
    }
    ///Bit 14 - TDE: Trigger DMA request enable 0: Trigger DMA request disabled 1: Trigger DMA request enabled
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W<'_, DIERrs> {
        TDE_W::new(self, 14)
    }
}
/**DIER register

You can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#TIM2:DIER)*/
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
