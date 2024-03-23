#[doc = "Register `MACIER` reader"]
pub type R = crate::R<MACIERrs>;
#[doc = "Register `MACIER` writer"]
pub type W = crate::W<MACIERrs>;
#[doc = "Field `PHYIE` reader - PHY Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of PHYIS bit in Interrupt status register (ETH_MACISR)."]
pub type PHYIE_R = crate::BitReader;
#[doc = "Field `PHYIE` writer - PHY Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of PHYIS bit in Interrupt status register (ETH_MACISR)."]
pub type PHYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMTIE` reader - PMT Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of PMTIS bit in Interrupt status register (ETH_MACISR)."]
pub type PMTIE_R = crate::BitReader;
#[doc = "Field `PMTIE` writer - PMT Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of PMTIS bit in Interrupt status register (ETH_MACISR)."]
pub type PMTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPIIE` reader - LPI Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of LPIIS bit in Interrupt status register (ETH_MACISR)."]
pub type LPIIE_R = crate::BitReader;
#[doc = "Field `LPIIE` writer - LPI Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of LPIIS bit in Interrupt status register (ETH_MACISR)."]
pub type LPIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIE` reader - Timestamp Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of TSIS bit in Interrupt status register (ETH_MACISR)."]
pub type TSIE_R = crate::BitReader;
#[doc = "Field `TSIE` writer - Timestamp Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of TSIS bit in Interrupt status register (ETH_MACISR)."]
pub type TSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSTSIE` reader - Transmit Status Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of TXSTSIS bit in the Interrupt status register (ETH_MACISR)."]
pub type TXSTSIE_R = crate::BitReader;
#[doc = "Field `TXSTSIE` writer - Transmit Status Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of TXSTSIS bit in the Interrupt status register (ETH_MACISR)."]
pub type TXSTSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTSIE` reader - Receive Status Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of RXSTSIS bit in the Interrupt status register (ETH_MACISR)."]
pub type RXSTSIE_R = crate::BitReader;
#[doc = "Field `RXSTSIE` writer - Receive Status Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of RXSTSIS bit in the Interrupt status register (ETH_MACISR)."]
pub type RXSTSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - PHY Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of PHYIS bit in Interrupt status register (ETH_MACISR)."]
    #[inline(always)]
    pub fn phyie(&self) -> PHYIE_R {
        PHYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMT Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of PMTIS bit in Interrupt status register (ETH_MACISR)."]
    #[inline(always)]
    pub fn pmtie(&self) -> PMTIE_R {
        PMTIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPI Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of LPIIS bit in Interrupt status register (ETH_MACISR)."]
    #[inline(always)]
    pub fn lpiie(&self) -> LPIIE_R {
        LPIIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - Timestamp Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of TSIS bit in Interrupt status register (ETH_MACISR)."]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Status Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of TXSTSIS bit in the Interrupt status register (ETH_MACISR)."]
    #[inline(always)]
    pub fn txstsie(&self) -> TXSTSIE_R {
        TXSTSIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Status Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of RXSTSIS bit in the Interrupt status register (ETH_MACISR)."]
    #[inline(always)]
    pub fn rxstsie(&self) -> RXSTSIE_R {
        RXSTSIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PHY Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of PHYIS bit in Interrupt status register (ETH_MACISR)."]
    #[inline(always)]
    #[must_use]
    pub fn phyie(&mut self) -> PHYIE_W<MACIERrs> {
        PHYIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - PMT Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of PMTIS bit in Interrupt status register (ETH_MACISR)."]
    #[inline(always)]
    #[must_use]
    pub fn pmtie(&mut self) -> PMTIE_W<MACIERrs> {
        PMTIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - LPI Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of LPIIS bit in Interrupt status register (ETH_MACISR)."]
    #[inline(always)]
    #[must_use]
    pub fn lpiie(&mut self) -> LPIIE_W<MACIERrs> {
        LPIIE_W::new(self, 5)
    }
    #[doc = "Bit 12 - Timestamp Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of TSIS bit in Interrupt status register (ETH_MACISR)."]
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TSIE_W<MACIERrs> {
        TSIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit Status Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of TXSTSIS bit in the Interrupt status register (ETH_MACISR)."]
    #[inline(always)]
    #[must_use]
    pub fn txstsie(&mut self) -> TXSTSIE_W<MACIERrs> {
        TXSTSIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Receive Status Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of RXSTSIS bit in the Interrupt status register (ETH_MACISR)."]
    #[inline(always)]
    #[must_use]
    pub fn rxstsie(&mut self) -> RXSTSIE_W<MACIERrs> {
        RXSTSIE_W::new(self, 14)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACIERrs;
impl crate::RegisterSpec for MACIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macier::R`](R) reader structure"]
impl crate::Readable for MACIERrs {}
#[doc = "`write(|w| ..)` method takes [`macier::W`](W) writer structure"]
impl crate::Writable for MACIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACIER to value 0"]
impl crate::Resettable for MACIERrs {
    const RESET_VALUE: u32 = 0;
}
