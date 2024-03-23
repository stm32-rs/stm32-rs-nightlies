#[doc = "Register `ETH_MACIER` reader"]
pub type R = crate::R<ETH_MACIERrs>;
#[doc = "Register `ETH_MACIER` writer"]
pub type W = crate::W<ETH_MACIERrs>;
#[doc = "Field `RGSMIIIE` reader - RGSMIIIE"]
pub type RGSMIIIE_R = crate::BitReader;
#[doc = "Field `RGSMIIIE` writer - RGSMIIIE"]
pub type RGSMIIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYIE` reader - PHYIE"]
pub type PHYIE_R = crate::BitReader;
#[doc = "Field `PHYIE` writer - PHYIE"]
pub type PHYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMTIE` reader - PMTIE"]
pub type PMTIE_R = crate::BitReader;
#[doc = "Field `PMTIE` writer - PMTIE"]
pub type PMTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPIIE` reader - LPIIE"]
pub type LPIIE_R = crate::BitReader;
#[doc = "Field `LPIIE` writer - LPIIE"]
pub type LPIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIE` reader - TSIE"]
pub type TSIE_R = crate::BitReader;
#[doc = "Field `TSIE` writer - TSIE"]
pub type TSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSTSIE` reader - TXSTSIE"]
pub type TXSTSIE_R = crate::BitReader;
#[doc = "Field `TXSTSIE` writer - TXSTSIE"]
pub type TXSTSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTSIE` reader - RXSTSIE"]
pub type RXSTSIE_R = crate::BitReader;
#[doc = "Field `RXSTSIE` writer - RXSTSIE"]
pub type RXSTSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RGSMIIIE"]
    #[inline(always)]
    pub fn rgsmiiie(&self) -> RGSMIIIE_R {
        RGSMIIIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - PHYIE"]
    #[inline(always)]
    pub fn phyie(&self) -> PHYIE_R {
        PHYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMTIE"]
    #[inline(always)]
    pub fn pmtie(&self) -> PMTIE_R {
        PMTIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPIIE"]
    #[inline(always)]
    pub fn lpiie(&self) -> LPIIE_R {
        LPIIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - TSIE"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TXSTSIE"]
    #[inline(always)]
    pub fn txstsie(&self) -> TXSTSIE_R {
        TXSTSIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RXSTSIE"]
    #[inline(always)]
    pub fn rxstsie(&self) -> RXSTSIE_R {
        RXSTSIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RGSMIIIE"]
    #[inline(always)]
    #[must_use]
    pub fn rgsmiiie(&mut self) -> RGSMIIIE_W<ETH_MACIERrs> {
        RGSMIIIE_W::new(self, 0)
    }
    #[doc = "Bit 3 - PHYIE"]
    #[inline(always)]
    #[must_use]
    pub fn phyie(&mut self) -> PHYIE_W<ETH_MACIERrs> {
        PHYIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - PMTIE"]
    #[inline(always)]
    #[must_use]
    pub fn pmtie(&mut self) -> PMTIE_W<ETH_MACIERrs> {
        PMTIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - LPIIE"]
    #[inline(always)]
    #[must_use]
    pub fn lpiie(&mut self) -> LPIIE_W<ETH_MACIERrs> {
        LPIIE_W::new(self, 5)
    }
    #[doc = "Bit 12 - TSIE"]
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TSIE_W<ETH_MACIERrs> {
        TSIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - TXSTSIE"]
    #[inline(always)]
    #[must_use]
    pub fn txstsie(&mut self) -> TXSTSIE_W<ETH_MACIERrs> {
        TXSTSIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - RXSTSIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxstsie(&mut self) -> RXSTSIE_W<ETH_MACIERrs> {
        RXSTSIE_W::new(self, 14)
    }
}
#[doc = "The Interrupt Enable register contains the masks for generating the interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACIERrs;
impl crate::RegisterSpec for ETH_MACIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macier::R`](R) reader structure"]
impl crate::Readable for ETH_MACIERrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macier::W`](W) writer structure"]
impl crate::Writable for ETH_MACIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACIER to value 0"]
impl crate::Resettable for ETH_MACIERrs {
    const RESET_VALUE: u32 = 0;
}
