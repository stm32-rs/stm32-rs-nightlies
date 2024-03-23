#[doc = "Register `DMACIER` reader"]
pub type R = crate::R<DMACIERrs>;
#[doc = "Register `DMACIER` writer"]
pub type W = crate::W<DMACIERrs>;
#[doc = "Field `TIE` reader - Transmit Interrupt Enable When this bit is set along with the NIE bit, the Transmit Interrupt is enabled. When this bit is reset, the Transmit Interrupt is disabled."]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit Interrupt Enable When this bit is set along with the NIE bit, the Transmit Interrupt is enabled. When this bit is reset, the Transmit Interrupt is disabled."]
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSE` reader - Transmit Stopped Enable When this bit is set along with the AIE bit, the Transmission Stopped interrupt is enabled. When this bit is reset, the Transmission Stopped interrupt is disabled."]
pub type TXSE_R = crate::BitReader;
#[doc = "Field `TXSE` writer - Transmit Stopped Enable When this bit is set along with the AIE bit, the Transmission Stopped interrupt is enabled. When this bit is reset, the Transmission Stopped interrupt is disabled."]
pub type TXSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBUE` reader - Transmit Buffer Unavailable Enable When this bit is set along with the NIE bit, the Transmit Buffer Unavailable interrupt is enabled. When this bit is reset, the Transmit Buffer Unavailable interrupt is disabled."]
pub type TBUE_R = crate::BitReader;
#[doc = "Field `TBUE` writer - Transmit Buffer Unavailable Enable When this bit is set along with the NIE bit, the Transmit Buffer Unavailable interrupt is enabled. When this bit is reset, the Transmit Buffer Unavailable interrupt is disabled."]
pub type TBUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - Receive Interrupt Enable When this bit is set along with the NIE bit, the Receive Interrupt is enabled. When this bit is reset, the Receive Interrupt is disabled."]
pub type RIE_R = crate::BitReader;
#[doc = "Field `RIE` writer - Receive Interrupt Enable When this bit is set along with the NIE bit, the Receive Interrupt is enabled. When this bit is reset, the Receive Interrupt is disabled."]
pub type RIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBUE` reader - Receive Buffer Unavailable Enable When this bit is set along with the AIE bit, the Receive Buffer Unavailable interrupt is enabled. When this bit is reset, the Receive Buffer Unavailable interrupt is disabled."]
pub type RBUE_R = crate::BitReader;
#[doc = "Field `RBUE` writer - Receive Buffer Unavailable Enable When this bit is set along with the AIE bit, the Receive Buffer Unavailable interrupt is enabled. When this bit is reset, the Receive Buffer Unavailable interrupt is disabled."]
pub type RBUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSE` reader - Receive Stopped Enable When this bit is set along with the AIE bit, the Receive Stopped Interrupt is enabled. When this bit is reset, the Receive Stopped interrupt is disabled."]
pub type RSE_R = crate::BitReader;
#[doc = "Field `RSE` writer - Receive Stopped Enable When this bit is set along with the AIE bit, the Receive Stopped Interrupt is enabled. When this bit is reset, the Receive Stopped interrupt is disabled."]
pub type RSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWTE` reader - Receive Watchdog Timeout Enable When this bit is set along with the AIE bit, the Receive Watchdog Timeout interrupt is enabled. When this bit is reset, the Receive Watchdog Timeout interrupt is disabled."]
pub type RWTE_R = crate::BitReader;
#[doc = "Field `RWTE` writer - Receive Watchdog Timeout Enable When this bit is set along with the AIE bit, the Receive Watchdog Timeout interrupt is enabled. When this bit is reset, the Receive Watchdog Timeout interrupt is disabled."]
pub type RWTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETIE` reader - Early Transmit Interrupt Enable When this bit is set along with the AIE bit, the Early Transmit interrupt is enabled. When this bit is reset, the Early Transmit interrupt is disabled."]
pub type ETIE_R = crate::BitReader;
#[doc = "Field `ETIE` writer - Early Transmit Interrupt Enable When this bit is set along with the AIE bit, the Early Transmit interrupt is enabled. When this bit is reset, the Early Transmit interrupt is disabled."]
pub type ETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERIE` reader - Early Receive Interrupt Enable When this bit is set along with the NIE bit, the Early Receive interrupt is enabled. When this bit is reset, the Early Receive interrupt is disabled."]
pub type ERIE_R = crate::BitReader;
#[doc = "Field `ERIE` writer - Early Receive Interrupt Enable When this bit is set along with the NIE bit, the Early Receive interrupt is enabled. When this bit is reset, the Early Receive interrupt is disabled."]
pub type ERIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBEE` reader - Fatal Bus Error Enable When this bit is set along with the AIE bit, the Fatal Bus error interrupt is enabled. When this bit is reset, the Fatal Bus Error error interrupt is disabled."]
pub type FBEE_R = crate::BitReader;
#[doc = "Field `FBEE` writer - Fatal Bus Error Enable When this bit is set along with the AIE bit, the Fatal Bus error interrupt is enabled. When this bit is reset, the Fatal Bus Error error interrupt is disabled."]
pub type FBEE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDEE` reader - Context Descriptor Error Enable When this bit is set along with the AIE bit, the Context Descriptor error interrupt is enabled. When this bit is reset, the Context Descriptor error interrupt is disabled."]
pub type CDEE_R = crate::BitReader;
#[doc = "Field `CDEE` writer - Context Descriptor Error Enable When this bit is set along with the AIE bit, the Context Descriptor error interrupt is enabled. When this bit is reset, the Context Descriptor error interrupt is disabled."]
pub type CDEE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIE` reader - Abnormal Interrupt Summary Enable When this bit is set, the abnormal interrupt summary is enabled. This bit enables the following interrupts in the Channel status register (ETH_DMACSR): Bit 1: Transmit Process Stopped Bit 7: Rx Buffer Unavailable Bit 8: Receive Process Stopped Bit 9: Receive Watchdog Timeout Bit 10: Early Transmit Interrupt Bit 12: Fatal Bus Error When this bit is reset, the abnormal interrupt summary is disabled."]
pub type AIE_R = crate::BitReader;
#[doc = "Field `AIE` writer - Abnormal Interrupt Summary Enable When this bit is set, the abnormal interrupt summary is enabled. This bit enables the following interrupts in the Channel status register (ETH_DMACSR): Bit 1: Transmit Process Stopped Bit 7: Rx Buffer Unavailable Bit 8: Receive Process Stopped Bit 9: Receive Watchdog Timeout Bit 10: Early Transmit Interrupt Bit 12: Fatal Bus Error When this bit is reset, the abnormal interrupt summary is disabled."]
pub type AIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIE` reader - Normal Interrupt Summary Enable When this bit is set, the normal interrupt summary is enabled. This bit enables the following interrupts in the Channel status register (ETH_DMACSR): Bit 0: Transmit Interrupt Bit 2: Transmit Buffer Unavailable Bit 6: Receive Interrupt Bit 11: Early Receive Interrupt When this bit is reset, the normal interrupt summary is disabled."]
pub type NIE_R = crate::BitReader;
#[doc = "Field `NIE` writer - Normal Interrupt Summary Enable When this bit is set, the normal interrupt summary is enabled. This bit enables the following interrupts in the Channel status register (ETH_DMACSR): Bit 0: Transmit Interrupt Bit 2: Transmit Buffer Unavailable Bit 6: Receive Interrupt Bit 11: Early Receive Interrupt When this bit is reset, the normal interrupt summary is disabled."]
pub type NIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable When this bit is set along with the NIE bit, the Transmit Interrupt is enabled. When this bit is reset, the Transmit Interrupt is disabled."]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable When this bit is set along with the AIE bit, the Transmission Stopped interrupt is enabled. When this bit is reset, the Transmission Stopped interrupt is disabled."]
    #[inline(always)]
    pub fn txse(&self) -> TXSE_R {
        TXSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable When this bit is set along with the NIE bit, the Transmit Buffer Unavailable interrupt is enabled. When this bit is reset, the Transmit Buffer Unavailable interrupt is disabled."]
    #[inline(always)]
    pub fn tbue(&self) -> TBUE_R {
        TBUE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable When this bit is set along with the NIE bit, the Receive Interrupt is enabled. When this bit is reset, the Receive Interrupt is disabled."]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable When this bit is set along with the AIE bit, the Receive Buffer Unavailable interrupt is enabled. When this bit is reset, the Receive Buffer Unavailable interrupt is disabled."]
    #[inline(always)]
    pub fn rbue(&self) -> RBUE_R {
        RBUE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Stopped Enable When this bit is set along with the AIE bit, the Receive Stopped Interrupt is enabled. When this bit is reset, the Receive Stopped interrupt is disabled."]
    #[inline(always)]
    pub fn rse(&self) -> RSE_R {
        RSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable When this bit is set along with the AIE bit, the Receive Watchdog Timeout interrupt is enabled. When this bit is reset, the Receive Watchdog Timeout interrupt is disabled."]
    #[inline(always)]
    pub fn rwte(&self) -> RWTE_R {
        RWTE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable When this bit is set along with the AIE bit, the Early Transmit interrupt is enabled. When this bit is reset, the Early Transmit interrupt is disabled."]
    #[inline(always)]
    pub fn etie(&self) -> ETIE_R {
        ETIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Early Receive Interrupt Enable When this bit is set along with the NIE bit, the Early Receive interrupt is enabled. When this bit is reset, the Early Receive interrupt is disabled."]
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fatal Bus Error Enable When this bit is set along with the AIE bit, the Fatal Bus error interrupt is enabled. When this bit is reset, the Fatal Bus Error error interrupt is disabled."]
    #[inline(always)]
    pub fn fbee(&self) -> FBEE_R {
        FBEE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Context Descriptor Error Enable When this bit is set along with the AIE bit, the Context Descriptor error interrupt is enabled. When this bit is reset, the Context Descriptor error interrupt is disabled."]
    #[inline(always)]
    pub fn cdee(&self) -> CDEE_R {
        CDEE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary Enable When this bit is set, the abnormal interrupt summary is enabled. This bit enables the following interrupts in the Channel status register (ETH_DMACSR): Bit 1: Transmit Process Stopped Bit 7: Rx Buffer Unavailable Bit 8: Receive Process Stopped Bit 9: Receive Watchdog Timeout Bit 10: Early Transmit Interrupt Bit 12: Fatal Bus Error When this bit is reset, the abnormal interrupt summary is disabled."]
    #[inline(always)]
    pub fn aie(&self) -> AIE_R {
        AIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary Enable When this bit is set, the normal interrupt summary is enabled. This bit enables the following interrupts in the Channel status register (ETH_DMACSR): Bit 0: Transmit Interrupt Bit 2: Transmit Buffer Unavailable Bit 6: Receive Interrupt Bit 11: Early Receive Interrupt When this bit is reset, the normal interrupt summary is disabled."]
    #[inline(always)]
    pub fn nie(&self) -> NIE_R {
        NIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable When this bit is set along with the NIE bit, the Transmit Interrupt is enabled. When this bit is reset, the Transmit Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<DMACIERrs> {
        TIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable When this bit is set along with the AIE bit, the Transmission Stopped interrupt is enabled. When this bit is reset, the Transmission Stopped interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn txse(&mut self) -> TXSE_W<DMACIERrs> {
        TXSE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable When this bit is set along with the NIE bit, the Transmit Buffer Unavailable interrupt is enabled. When this bit is reset, the Transmit Buffer Unavailable interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn tbue(&mut self) -> TBUE_W<DMACIERrs> {
        TBUE_W::new(self, 2)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable When this bit is set along with the NIE bit, the Receive Interrupt is enabled. When this bit is reset, the Receive Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<DMACIERrs> {
        RIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable When this bit is set along with the AIE bit, the Receive Buffer Unavailable interrupt is enabled. When this bit is reset, the Receive Buffer Unavailable interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rbue(&mut self) -> RBUE_W<DMACIERrs> {
        RBUE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Stopped Enable When this bit is set along with the AIE bit, the Receive Stopped Interrupt is enabled. When this bit is reset, the Receive Stopped interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rse(&mut self) -> RSE_W<DMACIERrs> {
        RSE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable When this bit is set along with the AIE bit, the Receive Watchdog Timeout interrupt is enabled. When this bit is reset, the Receive Watchdog Timeout interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rwte(&mut self) -> RWTE_W<DMACIERrs> {
        RWTE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable When this bit is set along with the AIE bit, the Early Transmit interrupt is enabled. When this bit is reset, the Early Transmit interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn etie(&mut self) -> ETIE_W<DMACIERrs> {
        ETIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Early Receive Interrupt Enable When this bit is set along with the NIE bit, the Early Receive interrupt is enabled. When this bit is reset, the Early Receive interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn erie(&mut self) -> ERIE_W<DMACIERrs> {
        ERIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Fatal Bus Error Enable When this bit is set along with the AIE bit, the Fatal Bus error interrupt is enabled. When this bit is reset, the Fatal Bus Error error interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fbee(&mut self) -> FBEE_W<DMACIERrs> {
        FBEE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Context Descriptor Error Enable When this bit is set along with the AIE bit, the Context Descriptor error interrupt is enabled. When this bit is reset, the Context Descriptor error interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn cdee(&mut self) -> CDEE_W<DMACIERrs> {
        CDEE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary Enable When this bit is set, the abnormal interrupt summary is enabled. This bit enables the following interrupts in the Channel status register (ETH_DMACSR): Bit 1: Transmit Process Stopped Bit 7: Rx Buffer Unavailable Bit 8: Receive Process Stopped Bit 9: Receive Watchdog Timeout Bit 10: Early Transmit Interrupt Bit 12: Fatal Bus Error When this bit is reset, the abnormal interrupt summary is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn aie(&mut self) -> AIE_W<DMACIERrs> {
        AIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary Enable When this bit is set, the normal interrupt summary is enabled. This bit enables the following interrupts in the Channel status register (ETH_DMACSR): Bit 0: Transmit Interrupt Bit 2: Transmit Buffer Unavailable Bit 6: Receive Interrupt Bit 11: Early Receive Interrupt When this bit is reset, the normal interrupt summary is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn nie(&mut self) -> NIE_W<DMACIERrs> {
        NIE_W::new(self, 15)
    }
}
#[doc = "Channel interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACIERrs;
impl crate::RegisterSpec for DMACIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacier::R`](R) reader structure"]
impl crate::Readable for DMACIERrs {}
#[doc = "`write(|w| ..)` method takes [`dmacier::W`](W) writer structure"]
impl crate::Writable for DMACIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACIER to value 0"]
impl crate::Resettable for DMACIERrs {
    const RESET_VALUE: u32 = 0;
}
