#[doc = "Register `DMAIER` reader"]
pub type R = crate::R<DMAIERrs>;
#[doc = "Register `DMAIER` writer"]
pub type W = crate::W<DMAIERrs>;
#[doc = "Field `TIE` reader - Transmit interrupt enable"]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit interrupt enable"]
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPSIE` reader - Transmit process stopped interrupt enable"]
pub type TPSIE_R = crate::BitReader;
#[doc = "Field `TPSIE` writer - Transmit process stopped interrupt enable"]
pub type TPSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBUIE` reader - Transmit buffer unavailable interrupt enable"]
pub type TBUIE_R = crate::BitReader;
#[doc = "Field `TBUIE` writer - Transmit buffer unavailable interrupt enable"]
pub type TBUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TJTIE` reader - Transmit jabber timeout interrupt enable"]
pub type TJTIE_R = crate::BitReader;
#[doc = "Field `TJTIE` writer - Transmit jabber timeout interrupt enable"]
pub type TJTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROIE` reader - Receive overflow interrupt enable"]
pub type ROIE_R = crate::BitReader;
#[doc = "Field `ROIE` writer - Receive overflow interrupt enable"]
pub type ROIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUIE` reader - Transmit underflow interrupt enable"]
pub type TUIE_R = crate::BitReader;
#[doc = "Field `TUIE` writer - Transmit underflow interrupt enable"]
pub type TUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - Receive interrupt enable"]
pub type RIE_R = crate::BitReader;
#[doc = "Field `RIE` writer - Receive interrupt enable"]
pub type RIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBUIE` reader - Receive buffer unavailable interrupt enable"]
pub type RBUIE_R = crate::BitReader;
#[doc = "Field `RBUIE` writer - Receive buffer unavailable interrupt enable"]
pub type RBUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPSIE` reader - Receive process stopped interrupt enable"]
pub type RPSIE_R = crate::BitReader;
#[doc = "Field `RPSIE` writer - Receive process stopped interrupt enable"]
pub type RPSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWTIE` reader - Receive watchdog timeout interrupt enable"]
pub type RWTIE_R = crate::BitReader;
#[doc = "Field `RWTIE` writer - Receive watchdog timeout interrupt enable"]
pub type RWTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETIE` reader - Early transmit interrupt enable"]
pub type ETIE_R = crate::BitReader;
#[doc = "Field `ETIE` writer - Early transmit interrupt enable"]
pub type ETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBEIE` reader - Fatal bus error interrupt enable"]
pub type FBEIE_R = crate::BitReader;
#[doc = "Field `FBEIE` writer - Fatal bus error interrupt enable"]
pub type FBEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERIE` reader - Early receive interrupt enable"]
pub type ERIE_R = crate::BitReader;
#[doc = "Field `ERIE` writer - Early receive interrupt enable"]
pub type ERIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AISE` reader - Abnormal interrupt summary enable"]
pub type AISE_R = crate::BitReader;
#[doc = "Field `AISE` writer - Abnormal interrupt summary enable"]
pub type AISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NISE` reader - Normal interrupt summary enable"]
pub type NISE_R = crate::BitReader;
#[doc = "Field `NISE` writer - Normal interrupt summary enable"]
pub type NISE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit process stopped interrupt enable"]
    #[inline(always)]
    pub fn tpsie(&self) -> TPSIE_R {
        TPSIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable interrupt enable"]
    #[inline(always)]
    pub fn tbuie(&self) -> TBUIE_R {
        TBUIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit jabber timeout interrupt enable"]
    #[inline(always)]
    pub fn tjtie(&self) -> TJTIE_R {
        TJTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive overflow interrupt enable"]
    #[inline(always)]
    pub fn roie(&self) -> ROIE_R {
        ROIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit underflow interrupt enable"]
    #[inline(always)]
    pub fn tuie(&self) -> TUIE_R {
        TUIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive buffer unavailable interrupt enable"]
    #[inline(always)]
    pub fn rbuie(&self) -> RBUIE_R {
        RBUIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive process stopped interrupt enable"]
    #[inline(always)]
    pub fn rpsie(&self) -> RPSIE_R {
        RPSIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive watchdog timeout interrupt enable"]
    #[inline(always)]
    pub fn rwtie(&self) -> RWTIE_R {
        RWTIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable"]
    #[inline(always)]
    pub fn etie(&self) -> ETIE_R {
        ETIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal bus error interrupt enable"]
    #[inline(always)]
    pub fn fbeie(&self) -> FBEIE_R {
        FBEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early receive interrupt enable"]
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary enable"]
    #[inline(always)]
    pub fn aise(&self) -> AISE_R {
        AISE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal interrupt summary enable"]
    #[inline(always)]
    pub fn nise(&self) -> NISE_R {
        NISE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<DMAIERrs> {
        TIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit process stopped interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpsie(&mut self) -> TPSIE_W<DMAIERrs> {
        TPSIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbuie(&mut self) -> TBUIE_W<DMAIERrs> {
        TBUIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit jabber timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tjtie(&mut self) -> TJTIE_W<DMAIERrs> {
        TJTIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn roie(&mut self) -> ROIE_W<DMAIERrs> {
        ROIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit underflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tuie(&mut self) -> TUIE_W<DMAIERrs> {
        TUIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<DMAIERrs> {
        RIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive buffer unavailable interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbuie(&mut self) -> RBUIE_W<DMAIERrs> {
        RBUIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive process stopped interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rpsie(&mut self) -> RPSIE_W<DMAIERrs> {
        RPSIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive watchdog timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwtie(&mut self) -> RWTIE_W<DMAIERrs> {
        RWTIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn etie(&mut self) -> ETIE_W<DMAIERrs> {
        ETIE_W::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal bus error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fbeie(&mut self) -> FBEIE_W<DMAIERrs> {
        FBEIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Early receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn erie(&mut self) -> ERIE_W<DMAIERrs> {
        ERIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary enable"]
    #[inline(always)]
    #[must_use]
    pub fn aise(&mut self) -> AISE_W<DMAIERrs> {
        AISE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Normal interrupt summary enable"]
    #[inline(always)]
    #[must_use]
    pub fn nise(&mut self) -> NISE_W<DMAIERrs> {
        NISE_W::new(self, 16)
    }
}
#[doc = "Ethernet DMA interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAIERrs;
impl crate::RegisterSpec for DMAIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaier::R`](R) reader structure"]
impl crate::Readable for DMAIERrs {}
#[doc = "`write(|w| ..)` method takes [`dmaier::W`](W) writer structure"]
impl crate::Writable for DMAIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAIER to value 0"]
impl crate::Resettable for DMAIERrs {
    const RESET_VALUE: u32 = 0;
}
