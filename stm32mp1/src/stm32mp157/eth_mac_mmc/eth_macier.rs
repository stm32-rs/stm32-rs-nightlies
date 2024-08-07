///Register `ETH_MACIER` reader
pub type R = crate::R<ETH_MACIERrs>;
///Register `ETH_MACIER` writer
pub type W = crate::W<ETH_MACIERrs>;
///Field `RGSMIIIE` reader - RGSMIIIE
pub type RGSMIIIE_R = crate::BitReader;
///Field `RGSMIIIE` writer - RGSMIIIE
pub type RGSMIIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PHYIE` reader - PHYIE
pub type PHYIE_R = crate::BitReader;
///Field `PHYIE` writer - PHYIE
pub type PHYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PMTIE` reader - PMTIE
pub type PMTIE_R = crate::BitReader;
///Field `PMTIE` writer - PMTIE
pub type PMTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPIIE` reader - LPIIE
pub type LPIIE_R = crate::BitReader;
///Field `LPIIE` writer - LPIIE
pub type LPIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSIE` reader - TSIE
pub type TSIE_R = crate::BitReader;
///Field `TSIE` writer - TSIE
pub type TSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXSTSIE` reader - TXSTSIE
pub type TXSTSIE_R = crate::BitReader;
///Field `TXSTSIE` writer - TXSTSIE
pub type TXSTSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXSTSIE` reader - RXSTSIE
pub type RXSTSIE_R = crate::BitReader;
///Field `RXSTSIE` writer - RXSTSIE
pub type RXSTSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RGSMIIIE
    #[inline(always)]
    pub fn rgsmiiie(&self) -> RGSMIIIE_R {
        RGSMIIIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - PHYIE
    #[inline(always)]
    pub fn phyie(&self) -> PHYIE_R {
        PHYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PMTIE
    #[inline(always)]
    pub fn pmtie(&self) -> PMTIE_R {
        PMTIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LPIIE
    #[inline(always)]
    pub fn lpiie(&self) -> LPIIE_R {
        LPIIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 12 - TSIE
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TXSTSIE
    #[inline(always)]
    pub fn txstsie(&self) -> TXSTSIE_R {
        TXSTSIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - RXSTSIE
    #[inline(always)]
    pub fn rxstsie(&self) -> RXSTSIE_R {
        RXSTSIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_MACIER")
            .field("rgsmiiie", &self.rgsmiiie())
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
    ///Bit 0 - RGSMIIIE
    #[inline(always)]
    #[must_use]
    pub fn rgsmiiie(&mut self) -> RGSMIIIE_W<ETH_MACIERrs> {
        RGSMIIIE_W::new(self, 0)
    }
    ///Bit 3 - PHYIE
    #[inline(always)]
    #[must_use]
    pub fn phyie(&mut self) -> PHYIE_W<ETH_MACIERrs> {
        PHYIE_W::new(self, 3)
    }
    ///Bit 4 - PMTIE
    #[inline(always)]
    #[must_use]
    pub fn pmtie(&mut self) -> PMTIE_W<ETH_MACIERrs> {
        PMTIE_W::new(self, 4)
    }
    ///Bit 5 - LPIIE
    #[inline(always)]
    #[must_use]
    pub fn lpiie(&mut self) -> LPIIE_W<ETH_MACIERrs> {
        LPIIE_W::new(self, 5)
    }
    ///Bit 12 - TSIE
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TSIE_W<ETH_MACIERrs> {
        TSIE_W::new(self, 12)
    }
    ///Bit 13 - TXSTSIE
    #[inline(always)]
    #[must_use]
    pub fn txstsie(&mut self) -> TXSTSIE_W<ETH_MACIERrs> {
        TXSTSIE_W::new(self, 13)
    }
    ///Bit 14 - RXSTSIE
    #[inline(always)]
    #[must_use]
    pub fn rxstsie(&mut self) -> RXSTSIE_W<ETH_MACIERrs> {
        RXSTSIE_W::new(self, 14)
    }
}
/**The Interrupt Enable register contains the masks for generating the interrupts.

You can [`read`](crate::Reg::read) this register and get [`eth_macier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_macier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:ETH_MACIER)*/
pub struct ETH_MACIERrs;
impl crate::RegisterSpec for ETH_MACIERrs {
    type Ux = u32;
}
///`read()` method returns [`eth_macier::R`](R) reader structure
impl crate::Readable for ETH_MACIERrs {}
///`write(|w| ..)` method takes [`eth_macier::W`](W) writer structure
impl crate::Writable for ETH_MACIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_MACIER to value 0
impl crate::Resettable for ETH_MACIERrs {
    const RESET_VALUE: u32 = 0;
}
