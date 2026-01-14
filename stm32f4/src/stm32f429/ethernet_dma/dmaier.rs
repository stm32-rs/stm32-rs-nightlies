///Register `DMAIER` reader
pub type R = crate::R<DMAIERrs>;
///Register `DMAIER` writer
pub type W = crate::W<DMAIERrs>;
///Field `TIE` reader - Transmit interrupt enable
pub type TIE_R = crate::BitReader;
///Field `TIE` writer - Transmit interrupt enable
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TPSIE` reader - Transmit process stopped interrupt enable
pub type TPSIE_R = crate::BitReader;
///Field `TPSIE` writer - Transmit process stopped interrupt enable
pub type TPSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TBUIE` reader - Transmit buffer unavailable interrupt enable
pub type TBUIE_R = crate::BitReader;
///Field `TBUIE` writer - Transmit buffer unavailable interrupt enable
pub type TBUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TJTIE` reader - Transmit jabber timeout interrupt enable
pub type TJTIE_R = crate::BitReader;
///Field `TJTIE` writer - Transmit jabber timeout interrupt enable
pub type TJTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROIE` reader - Receive overflow interrupt enable
pub type ROIE_R = crate::BitReader;
///Field `ROIE` writer - Receive overflow interrupt enable
pub type ROIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TUIE` reader - Transmit underflow interrupt enable
pub type TUIE_R = crate::BitReader;
///Field `TUIE` writer - Transmit underflow interrupt enable
pub type TUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RIE` reader - Receive interrupt enable
pub type RIE_R = crate::BitReader;
///Field `RIE` writer - Receive interrupt enable
pub type RIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RBUIE` reader - Receive buffer unavailable interrupt enable
pub type RBUIE_R = crate::BitReader;
///Field `RBUIE` writer - Receive buffer unavailable interrupt enable
pub type RBUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPSIE` reader - Receive process stopped interrupt enable
pub type RPSIE_R = crate::BitReader;
///Field `RPSIE` writer - Receive process stopped interrupt enable
pub type RPSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWTIE` reader - Receive watchdog timeout interrupt enable
pub type RWTIE_R = crate::BitReader;
///Field `RWTIE` writer - Receive watchdog timeout interrupt enable
pub type RWTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETIE` reader - Early transmit interrupt enable
pub type ETIE_R = crate::BitReader;
///Field `ETIE` writer - Early transmit interrupt enable
pub type ETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FBEIE` reader - Fatal bus error interrupt enable
pub type FBEIE_R = crate::BitReader;
///Field `FBEIE` writer - Fatal bus error interrupt enable
pub type FBEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERIE` reader - Early receive interrupt enable
pub type ERIE_R = crate::BitReader;
///Field `ERIE` writer - Early receive interrupt enable
pub type ERIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AISE` reader - Abnormal interrupt summary enable
pub type AISE_R = crate::BitReader;
///Field `AISE` writer - Abnormal interrupt summary enable
pub type AISE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NISE` reader - Normal interrupt summary enable
pub type NISE_R = crate::BitReader;
///Field `NISE` writer - Normal interrupt summary enable
pub type NISE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Transmit interrupt enable
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit process stopped interrupt enable
    #[inline(always)]
    pub fn tpsie(&self) -> TPSIE_R {
        TPSIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transmit buffer unavailable interrupt enable
    #[inline(always)]
    pub fn tbuie(&self) -> TBUIE_R {
        TBUIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmit jabber timeout interrupt enable
    #[inline(always)]
    pub fn tjtie(&self) -> TJTIE_R {
        TJTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Receive overflow interrupt enable
    #[inline(always)]
    pub fn roie(&self) -> ROIE_R {
        ROIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Transmit underflow interrupt enable
    #[inline(always)]
    pub fn tuie(&self) -> TUIE_R {
        TUIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Receive interrupt enable
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Receive buffer unavailable interrupt enable
    #[inline(always)]
    pub fn rbuie(&self) -> RBUIE_R {
        RBUIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Receive process stopped interrupt enable
    #[inline(always)]
    pub fn rpsie(&self) -> RPSIE_R {
        RPSIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Receive watchdog timeout interrupt enable
    #[inline(always)]
    pub fn rwtie(&self) -> RWTIE_R {
        RWTIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Early transmit interrupt enable
    #[inline(always)]
    pub fn etie(&self) -> ETIE_R {
        ETIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - Fatal bus error interrupt enable
    #[inline(always)]
    pub fn fbeie(&self) -> FBEIE_R {
        FBEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Early receive interrupt enable
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Abnormal interrupt summary enable
    #[inline(always)]
    pub fn aise(&self) -> AISE_R {
        AISE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Normal interrupt summary enable
    #[inline(always)]
    pub fn nise(&self) -> NISE_R {
        NISE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAIER")
            .field("tie", &self.tie())
            .field("tpsie", &self.tpsie())
            .field("tbuie", &self.tbuie())
            .field("tjtie", &self.tjtie())
            .field("roie", &self.roie())
            .field("tuie", &self.tuie())
            .field("rie", &self.rie())
            .field("rbuie", &self.rbuie())
            .field("rpsie", &self.rpsie())
            .field("rwtie", &self.rwtie())
            .field("etie", &self.etie())
            .field("fbeie", &self.fbeie())
            .field("erie", &self.erie())
            .field("aise", &self.aise())
            .field("nise", &self.nise())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transmit interrupt enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<'_, DMAIERrs> {
        TIE_W::new(self, 0)
    }
    ///Bit 1 - Transmit process stopped interrupt enable
    #[inline(always)]
    pub fn tpsie(&mut self) -> TPSIE_W<'_, DMAIERrs> {
        TPSIE_W::new(self, 1)
    }
    ///Bit 2 - Transmit buffer unavailable interrupt enable
    #[inline(always)]
    pub fn tbuie(&mut self) -> TBUIE_W<'_, DMAIERrs> {
        TBUIE_W::new(self, 2)
    }
    ///Bit 3 - Transmit jabber timeout interrupt enable
    #[inline(always)]
    pub fn tjtie(&mut self) -> TJTIE_W<'_, DMAIERrs> {
        TJTIE_W::new(self, 3)
    }
    ///Bit 4 - Receive overflow interrupt enable
    #[inline(always)]
    pub fn roie(&mut self) -> ROIE_W<'_, DMAIERrs> {
        ROIE_W::new(self, 4)
    }
    ///Bit 5 - Transmit underflow interrupt enable
    #[inline(always)]
    pub fn tuie(&mut self) -> TUIE_W<'_, DMAIERrs> {
        TUIE_W::new(self, 5)
    }
    ///Bit 6 - Receive interrupt enable
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W<'_, DMAIERrs> {
        RIE_W::new(self, 6)
    }
    ///Bit 7 - Receive buffer unavailable interrupt enable
    #[inline(always)]
    pub fn rbuie(&mut self) -> RBUIE_W<'_, DMAIERrs> {
        RBUIE_W::new(self, 7)
    }
    ///Bit 8 - Receive process stopped interrupt enable
    #[inline(always)]
    pub fn rpsie(&mut self) -> RPSIE_W<'_, DMAIERrs> {
        RPSIE_W::new(self, 8)
    }
    ///Bit 9 - Receive watchdog timeout interrupt enable
    #[inline(always)]
    pub fn rwtie(&mut self) -> RWTIE_W<'_, DMAIERrs> {
        RWTIE_W::new(self, 9)
    }
    ///Bit 10 - Early transmit interrupt enable
    #[inline(always)]
    pub fn etie(&mut self) -> ETIE_W<'_, DMAIERrs> {
        ETIE_W::new(self, 10)
    }
    ///Bit 13 - Fatal bus error interrupt enable
    #[inline(always)]
    pub fn fbeie(&mut self) -> FBEIE_W<'_, DMAIERrs> {
        FBEIE_W::new(self, 13)
    }
    ///Bit 14 - Early receive interrupt enable
    #[inline(always)]
    pub fn erie(&mut self) -> ERIE_W<'_, DMAIERrs> {
        ERIE_W::new(self, 14)
    }
    ///Bit 15 - Abnormal interrupt summary enable
    #[inline(always)]
    pub fn aise(&mut self) -> AISE_W<'_, DMAIERrs> {
        AISE_W::new(self, 15)
    }
    ///Bit 16 - Normal interrupt summary enable
    #[inline(always)]
    pub fn nise(&mut self) -> NISE_W<'_, DMAIERrs> {
        NISE_W::new(self, 16)
    }
}
/**Ethernet DMA interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dmaier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F429.html#Ethernet_DMA:DMAIER)*/
pub struct DMAIERrs;
impl crate::RegisterSpec for DMAIERrs {
    type Ux = u32;
}
///`read()` method returns [`dmaier::R`](R) reader structure
impl crate::Readable for DMAIERrs {}
///`write(|w| ..)` method takes [`dmaier::W`](W) writer structure
impl crate::Writable for DMAIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAIER to value 0
impl crate::Resettable for DMAIERrs {}
