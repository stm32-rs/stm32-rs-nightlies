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
///Field `COMIE` reader - COMIE: COM interrupt enable 0: COM interrupt disabled 1: COM interrupt enabled
pub type COMIE_R = crate::BitReader;
///Field `COMIE` writer - COMIE: COM interrupt enable 0: COM interrupt disabled 1: COM interrupt enabled
pub type COMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIE` reader - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled
pub type TIE_R = crate::BitReader;
///Field `TIE` writer - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIE` reader - BIE: Break interrupt enable 0: Break interrupt disabled 1: Break interrupt enabled
pub type BIE_R = crate::BitReader;
///Field `BIE` writer - BIE: Break interrupt enable 0: Break interrupt disabled 1: Break interrupt enabled
pub type BIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDE` reader - UDE: Update DMA request enable 0: Update DMA request disabled 1: Update DMA request enabled
pub type UDE_R = crate::BitReader;
///Field `UDE` writer - UDE: Update DMA request enable 0: Update DMA request disabled 1: Update DMA request enabled
pub type UDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1DE` reader - CC1DE: Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled 1: CC1 DMA request enabled
pub type CC1DE_R = crate::BitReader;
///Field `CC1DE` writer - CC1DE: Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled 1: CC1 DMA request enabled
pub type CC1DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCUDE` reader - CCUDE: CC-Update DMA request Enable. Not used in Blue51. Not available in IUM 0: CC-Update DMA request disabled. 1: CC-Update DMA request enabled.
pub type CCUDE_R = crate::BitReader;
///Field `CCUDE` writer - CCUDE: CC-Update DMA request Enable. Not used in Blue51. Not available in IUM 0: CC-Update DMA request disabled. 1: CC-Update DMA request enabled.
pub type CCUDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TDE` reader - TDE: Trigger DMA request enable 0: Trigger DMA request disabled 1: Trigger DMA request enabled
pub type TDE_R = crate::BitReader;
///Field `TDE` writer - TDE: Trigger DMA request enable 0: Trigger DMA request disabled 1: Trigger DMA request enabled
pub type TDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BDE` reader - BDE: Break DMA request Enable. Not used in Blue51. Not available in IUM 0: Break DMA request disabled. 1: Break DMA request enabled.
pub type BDE_R = crate::BitReader;
///Field `BDE` writer - BDE: Break DMA request Enable. Not used in Blue51. Not available in IUM 0: Break DMA request disabled. 1: Break DMA request enabled.
pub type BDE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 5 - COMIE: COM interrupt enable 0: COM interrupt disabled 1: COM interrupt enabled
    #[inline(always)]
    pub fn comie(&self) -> COMIE_R {
        COMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - BIE: Break interrupt enable 0: Break interrupt disabled 1: Break interrupt enabled
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 7) & 1) != 0)
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
    ///Bit 13 - CCUDE: CC-Update DMA request Enable. Not used in Blue51. Not available in IUM 0: CC-Update DMA request disabled. 1: CC-Update DMA request enabled.
    #[inline(always)]
    pub fn ccude(&self) -> CCUDE_R {
        CCUDE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TDE: Trigger DMA request enable 0: Trigger DMA request disabled 1: Trigger DMA request enabled
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - BDE: Break DMA request Enable. Not used in Blue51. Not available in IUM 0: Break DMA request disabled. 1: Break DMA request enabled.
    #[inline(always)]
    pub fn bde(&self) -> BDE_R {
        BDE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER")
            .field("uie", &self.uie())
            .field("cc1ie", &self.cc1ie())
            .field("comie", &self.comie())
            .field("tie", &self.tie())
            .field("bie", &self.bie())
            .field("ude", &self.ude())
            .field("cc1de", &self.cc1de())
            .field("ccude", &self.ccude())
            .field("tde", &self.tde())
            .field("bde", &self.bde())
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
    ///Bit 5 - COMIE: COM interrupt enable 0: COM interrupt disabled 1: COM interrupt enabled
    #[inline(always)]
    pub fn comie(&mut self) -> COMIE_W<'_, DIERrs> {
        COMIE_W::new(self, 5)
    }
    ///Bit 6 - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<'_, DIERrs> {
        TIE_W::new(self, 6)
    }
    ///Bit 7 - BIE: Break interrupt enable 0: Break interrupt disabled 1: Break interrupt enabled
    #[inline(always)]
    pub fn bie(&mut self) -> BIE_W<'_, DIERrs> {
        BIE_W::new(self, 7)
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
    ///Bit 13 - CCUDE: CC-Update DMA request Enable. Not used in Blue51. Not available in IUM 0: CC-Update DMA request disabled. 1: CC-Update DMA request enabled.
    #[inline(always)]
    pub fn ccude(&mut self) -> CCUDE_W<'_, DIERrs> {
        CCUDE_W::new(self, 13)
    }
    ///Bit 14 - TDE: Trigger DMA request enable 0: Trigger DMA request disabled 1: Trigger DMA request enabled
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W<'_, DIERrs> {
        TDE_W::new(self, 14)
    }
    ///Bit 15 - BDE: Break DMA request Enable. Not used in Blue51. Not available in IUM 0: Break DMA request disabled. 1: Break DMA request enabled.
    #[inline(always)]
    pub fn bde(&mut self) -> BDE_W<'_, DIERrs> {
        BDE_W::new(self, 15)
    }
}
/**DIER register

You can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#TIM16:DIER)*/
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
