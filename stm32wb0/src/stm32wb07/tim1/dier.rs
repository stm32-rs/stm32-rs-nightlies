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
///Field `COMIE` reader - COM interrupt enable
pub type COMIE_R = crate::BitReader;
///Field `COMIE` writer - COM interrupt enable
pub type COMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIE` reader - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled
pub type TIE_R = crate::BitReader;
///Field `TIE` writer - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIE` reader - Break interrupt enable.
pub type BIE_R = crate::BitReader;
///Field `BIE` writer - Break interrupt enable.
pub type BIE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 5 - COM interrupt enable
    #[inline(always)]
    pub fn comie(&self) -> COMIE_R {
        COMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Break interrupt enable.
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 7) & 1) != 0)
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
            .finish()
    }
}
impl W {
    ///Bit 0 - UIE: Update interrupt enable 0: Update interrupt disabled 1: Update interrupt enabled
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<DIERrs> {
        UIE_W::new(self, 0)
    }
    ///Bit 1 - CC1IE: Capture/Compare 1 interrupt enable 0: CC1 interrupt disabled. 1: CC1 interrupt enabled
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W<DIERrs> {
        CC1IE_W::new(self, 1)
    }
    ///Bit 2 - CC2IE: Capture/Compare 2 interrupt enable 0: CC2 interrupt disabled 1: CC2 interrupt enabled
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W<DIERrs> {
        CC2IE_W::new(self, 2)
    }
    ///Bit 3 - CC3IE: Capture/Compare 3 interrupt enable 0: CC3 interrupt disabled 1: CC3 interrupt enabled
    #[inline(always)]
    pub fn cc3ie(&mut self) -> CC3IE_W<DIERrs> {
        CC3IE_W::new(self, 3)
    }
    ///Bit 4 - CC4IE: Capture/Compare 4 interrupt enable 0: CC4 interrupt disabled 1: CC4 interrupt enabled
    #[inline(always)]
    pub fn cc4ie(&mut self) -> CC4IE_W<DIERrs> {
        CC4IE_W::new(self, 4)
    }
    ///Bit 5 - COM interrupt enable
    #[inline(always)]
    pub fn comie(&mut self) -> COMIE_W<DIERrs> {
        COMIE_W::new(self, 5)
    }
    ///Bit 6 - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<DIERrs> {
        TIE_W::new(self, 6)
    }
    ///Bit 7 - Break interrupt enable.
    #[inline(always)]
    pub fn bie(&mut self) -> BIE_W<DIERrs> {
        BIE_W::new(self, 7)
    }
}
/**DIER register

You can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#TIM1:DIER)*/
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
