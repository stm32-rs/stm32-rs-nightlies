#[doc = "Register `ETH_DMAC0IER` reader"]
pub type R = crate::R<ETH_DMAC0IERrs>;
#[doc = "Register `ETH_DMAC0IER` writer"]
pub type W = crate::W<ETH_DMAC0IERrs>;
#[doc = "Field `TIE` reader - Transmit Interrupt Enable"]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit Interrupt Enable"]
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSE` reader - Transmit Stopped Enable"]
pub type TXSE_R = crate::BitReader;
#[doc = "Field `TXSE` writer - Transmit Stopped Enable"]
pub type TXSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBUE` reader - Transmit Buffer Unavailable Enable"]
pub type TBUE_R = crate::BitReader;
#[doc = "Field `TBUE` writer - Transmit Buffer Unavailable Enable"]
pub type TBUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - Receive Interrupt Enable"]
pub type RIE_R = crate::BitReader;
#[doc = "Field `RIE` writer - Receive Interrupt Enable"]
pub type RIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBUE` reader - Receive Buffer Unavailable Enable"]
pub type RBUE_R = crate::BitReader;
#[doc = "Field `RBUE` writer - Receive Buffer Unavailable Enable"]
pub type RBUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSE` reader - Receive Stopped Enable"]
pub type RSE_R = crate::BitReader;
#[doc = "Field `RSE` writer - Receive Stopped Enable"]
pub type RSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWTE` reader - Receive Watchdog Timeout Enable"]
pub type RWTE_R = crate::BitReader;
#[doc = "Field `RWTE` writer - Receive Watchdog Timeout Enable"]
pub type RWTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETIE` reader - Early Transmit Interrupt Enable"]
pub type ETIE_R = crate::BitReader;
#[doc = "Field `ETIE` writer - Early Transmit Interrupt Enable"]
pub type ETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERIE` reader - Early Receive Interrupt Enable"]
pub type ERIE_R = crate::BitReader;
#[doc = "Field `ERIE` writer - Early Receive Interrupt Enable"]
pub type ERIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBEE` reader - Fatal Bus Error Enable"]
pub type FBEE_R = crate::BitReader;
#[doc = "Field `FBEE` writer - Fatal Bus Error Enable"]
pub type FBEE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDEE` reader - Context Descriptor Error Enable"]
pub type CDEE_R = crate::BitReader;
#[doc = "Field `CDEE` writer - Context Descriptor Error Enable"]
pub type CDEE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIE` reader - Abnormal Interrupt Summary Enable"]
pub type AIE_R = crate::BitReader;
#[doc = "Field `AIE` writer - Abnormal Interrupt Summary Enable"]
pub type AIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIE` reader - Normal Interrupt Summary Enable"]
pub type NIE_R = crate::BitReader;
#[doc = "Field `NIE` writer - Normal Interrupt Summary Enable"]
pub type NIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    pub fn txse(&self) -> TXSE_R {
        TXSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn tbue(&self) -> TBUE_R {
        TBUE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn rbue(&self) -> RBUE_R {
        RBUE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    pub fn rse(&self) -> RSE_R {
        RSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    pub fn rwte(&self) -> RWTE_R {
        RWTE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn etie(&self) -> ETIE_R {
        ETIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Early Receive Interrupt Enable"]
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fatal Bus Error Enable"]
    #[inline(always)]
    pub fn fbee(&self) -> FBEE_R {
        FBEE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Context Descriptor Error Enable"]
    #[inline(always)]
    pub fn cdee(&self) -> CDEE_R {
        CDEE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn aie(&self) -> AIE_R {
        AIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn nie(&self) -> NIE_R {
        NIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<ETH_DMAC0IERrs> {
        TIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txse(&mut self) -> TXSE_W<ETH_DMAC0IERrs> {
        TXSE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbue(&mut self) -> TBUE_W<ETH_DMAC0IERrs> {
        TBUE_W::new(self, 2)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<ETH_DMAC0IERrs> {
        RIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbue(&mut self) -> RBUE_W<ETH_DMAC0IERrs> {
        RBUE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rse(&mut self) -> RSE_W<ETH_DMAC0IERrs> {
        RSE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwte(&mut self) -> RWTE_W<ETH_DMAC0IERrs> {
        RWTE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn etie(&mut self) -> ETIE_W<ETH_DMAC0IERrs> {
        ETIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Early Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erie(&mut self) -> ERIE_W<ETH_DMAC0IERrs> {
        ERIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Fatal Bus Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fbee(&mut self) -> FBEE_W<ETH_DMAC0IERrs> {
        FBEE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Context Descriptor Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cdee(&mut self) -> CDEE_W<ETH_DMAC0IERrs> {
        CDEE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aie(&mut self) -> AIE_W<ETH_DMAC0IERrs> {
        AIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nie(&mut self) -> NIE_W<ETH_DMAC0IERrs> {
        NIE_W::new(self, 15)
    }
}
#[doc = "Channel interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_dmac0ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_dmac0ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_DMAC0IERrs;
impl crate::RegisterSpec for ETH_DMAC0IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_dmac0ier::R`](R) reader structure"]
impl crate::Readable for ETH_DMAC0IERrs {}
#[doc = "`write(|w| ..)` method takes [`eth_dmac0ier::W`](W) writer structure"]
impl crate::Writable for ETH_DMAC0IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_DMAC0IER to value 0x8000"]
impl crate::Resettable for ETH_DMAC0IERrs {
    const RESET_VALUE: u32 = 0x8000;
}
