///Register `DIER` reader
pub type R = crate::R<DIERrs>;
///Register `DIER` writer
pub type W = crate::W<DIERrs>;
///Field `UIE` reader - Update interrupt enable
pub type UIE_R = crate::BitReader;
///Field `UIE` writer - Update interrupt enable
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1IE` reader - Capture/Compare 1 interrupt enable
pub type CC1IE_R = crate::BitReader;
///Field `CC1IE` writer - Capture/Compare 1 interrupt enable
pub type CC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2IE` reader - Capture/Compare 2 interrupt enable
pub type CC2IE_R = crate::BitReader;
///Field `CC2IE` writer - Capture/Compare 2 interrupt enable
pub type CC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
///Field `CC1DE` reader - Capture/Compare 1 DMA request enable
pub type CC1DE_R = crate::BitReader;
///Field `CC1DE` writer - Capture/Compare 1 DMA request enable
pub type CC1DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2DE` reader - Capture/Compare 2 DMA request enable
pub type CC2DE_R = crate::BitReader;
///Field `CC2DE` writer - Capture/Compare 2 DMA request enable
pub type CC2DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMDE` reader - COM DMA request enable
pub type COMDE_R = crate::BitReader;
///Field `COMDE` writer - COM DMA request enable
pub type COMDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TDE` reader - Trigger DMA request enable
pub type TDE_R = crate::BitReader;
///Field `TDE` writer - Trigger DMA request enable
pub type TDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Capture/Compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 2) & 1) != 0)
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
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Capture/Compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 10) & 1) != 0)
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER")
            .field("uie", &self.uie())
            .field("cc1ie", &self.cc1ie())
            .field("cc2ie", &self.cc2ie())
            .field("comie", &self.comie())
            .field("tie", &self.tie())
            .field("bie", &self.bie())
            .field("ude", &self.ude())
            .field("cc1de", &self.cc1de())
            .field("cc2de", &self.cc2de())
            .field("comde", &self.comde())
            .field("tde", &self.tde())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<'_, DIERrs> {
        UIE_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W<'_, DIERrs> {
        CC1IE_W::new(self, 1)
    }
    ///Bit 2 - Capture/Compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W<'_, DIERrs> {
        CC2IE_W::new(self, 2)
    }
    ///Bit 5 - COM interrupt enable
    #[inline(always)]
    pub fn comie(&mut self) -> COMIE_W<'_, DIERrs> {
        COMIE_W::new(self, 5)
    }
    ///Bit 6 - Trigger interrupt enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<'_, DIERrs> {
        TIE_W::new(self, 6)
    }
    ///Bit 7 - Break interrupt enable
    #[inline(always)]
    pub fn bie(&mut self) -> BIE_W<'_, DIERrs> {
        BIE_W::new(self, 7)
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&mut self) -> UDE_W<'_, DIERrs> {
        UDE_W::new(self, 8)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&mut self) -> CC1DE_W<'_, DIERrs> {
        CC1DE_W::new(self, 9)
    }
    ///Bit 10 - Capture/Compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&mut self) -> CC2DE_W<'_, DIERrs> {
        CC2DE_W::new(self, 10)
    }
    ///Bit 13 - COM DMA request enable
    #[inline(always)]
    pub fn comde(&mut self) -> COMDE_W<'_, DIERrs> {
        COMDE_W::new(self, 13)
    }
    ///Bit 14 - Trigger DMA request enable
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W<'_, DIERrs> {
        TDE_W::new(self, 14)
    }
}
/**TIM15 DMA/interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#TIM15:DIER)*/
pub struct DIERrs;
impl crate::RegisterSpec for DIERrs {
    type Ux = u16;
}
///`read()` method returns [`dier::R`](R) reader structure
impl crate::Readable for DIERrs {}
///`write(|w| ..)` method takes [`dier::W`](W) writer structure
impl crate::Writable for DIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIER to value 0
impl crate::Resettable for DIERrs {}
