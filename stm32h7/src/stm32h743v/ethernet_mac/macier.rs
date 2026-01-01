///Register `MACIER` reader
pub type R = crate::R<MACIERrs>;
///Register `MACIER` writer
pub type W = crate::W<MACIERrs>;
///Field `PHYIE` reader - PHY Interrupt Enable
pub type PHYIE_R = crate::BitReader;
///Field `PHYIE` writer - PHY Interrupt Enable
pub type PHYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PMTIE` reader - PMT Interrupt Enable
pub type PMTIE_R = crate::BitReader;
///Field `PMTIE` writer - PMT Interrupt Enable
pub type PMTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPIIE` reader - LPI Interrupt Enable
pub type LPIIE_R = crate::BitReader;
///Field `LPIIE` writer - LPI Interrupt Enable
pub type LPIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSIE` reader - Timestamp Interrupt Enable
pub type TSIE_R = crate::BitReader;
///Field `TSIE` writer - Timestamp Interrupt Enable
pub type TSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXSTSIE` reader - Transmit Status Interrupt Enable
pub type TXSTSIE_R = crate::BitReader;
///Field `TXSTSIE` writer - Transmit Status Interrupt Enable
pub type TXSTSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXSTSIE` reader - Receive Status Interrupt Enable
pub type RXSTSIE_R = crate::BitReader;
///Field `RXSTSIE` writer - Receive Status Interrupt Enable
pub type RXSTSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - PHY Interrupt Enable
    #[inline(always)]
    pub fn phyie(&self) -> PHYIE_R {
        PHYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PMT Interrupt Enable
    #[inline(always)]
    pub fn pmtie(&self) -> PMTIE_R {
        PMTIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LPI Interrupt Enable
    #[inline(always)]
    pub fn lpiie(&self) -> LPIIE_R {
        LPIIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 12 - Timestamp Interrupt Enable
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Transmit Status Interrupt Enable
    #[inline(always)]
    pub fn txstsie(&self) -> TXSTSIE_R {
        TXSTSIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Receive Status Interrupt Enable
    #[inline(always)]
    pub fn rxstsie(&self) -> RXSTSIE_R {
        RXSTSIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACIER")
            .field("phyie", &self.phyie())
            .field("pmtie", &self.pmtie())
            .field("lpiie", &self.lpiie())
            .field("tsie", &self.tsie())
            .field("txstsie", &self.txstsie())
            .field("rxstsie", &self.rxstsie())
            .finish()
    }
}
impl W {
    ///Bit 3 - PHY Interrupt Enable
    #[inline(always)]
    pub fn phyie(&mut self) -> PHYIE_W<'_, MACIERrs> {
        PHYIE_W::new(self, 3)
    }
    ///Bit 4 - PMT Interrupt Enable
    #[inline(always)]
    pub fn pmtie(&mut self) -> PMTIE_W<'_, MACIERrs> {
        PMTIE_W::new(self, 4)
    }
    ///Bit 5 - LPI Interrupt Enable
    #[inline(always)]
    pub fn lpiie(&mut self) -> LPIIE_W<'_, MACIERrs> {
        LPIIE_W::new(self, 5)
    }
    ///Bit 12 - Timestamp Interrupt Enable
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W<'_, MACIERrs> {
        TSIE_W::new(self, 12)
    }
    ///Bit 13 - Transmit Status Interrupt Enable
    #[inline(always)]
    pub fn txstsie(&mut self) -> TXSTSIE_W<'_, MACIERrs> {
        TXSTSIE_W::new(self, 13)
    }
    ///Bit 14 - Receive Status Interrupt Enable
    #[inline(always)]
    pub fn rxstsie(&mut self) -> RXSTSIE_W<'_, MACIERrs> {
        RXSTSIE_W::new(self, 14)
    }
}
/**Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`macier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#Ethernet_MAC:MACIER)*/
pub struct MACIERrs;
impl crate::RegisterSpec for MACIERrs {
    type Ux = u32;
}
///`read()` method returns [`macier::R`](R) reader structure
impl crate::Readable for MACIERrs {}
///`write(|w| ..)` method takes [`macier::W`](W) writer structure
impl crate::Writable for MACIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACIER to value 0
impl crate::Resettable for MACIERrs {}
