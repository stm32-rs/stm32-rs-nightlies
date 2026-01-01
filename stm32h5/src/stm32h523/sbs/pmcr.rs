///Register `PMCR` reader
pub type R = crate::R<PMCRrs>;
///Register `PMCR` writer
pub type W = crate::W<PMCRrs>;
///Field `PB6_FMP` reader - Fast-mode Plus driving capability activation on PB6
pub type PB6_FMP_R = crate::BitReader;
///Field `PB6_FMP` writer - Fast-mode Plus driving capability activation on PB6
pub type PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB7_FMP` reader - Fast-mode Plus driving capability activation on PB7
pub type PB7_FMP_R = crate::BitReader;
///Field `PB7_FMP` writer - Fast-mode Plus driving capability activation on PB7
pub type PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB8_FMP` reader - Fast-mode Plus driving capability activation on PB8
pub type PB8_FMP_R = crate::BitReader;
///Field `PB8_FMP` writer - Fast-mode Plus driving capability activation on PB8
pub type PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB9_FMP` reader - Fast-mode Plus driving capability activation on PB9
pub type PB9_FMP_R = crate::BitReader;
///Field `PB9_FMP` writer - Fast-mode Plus driving capability activation on PB9
pub type PB9_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH_SEL_PHY` reader - Ethernet PHY interface selection
pub type ETH_SEL_PHY_R = crate::FieldReader;
///Field `ETH_SEL_PHY` writer - Ethernet PHY interface selection
pub type ETH_SEL_PHY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 16 - Fast-mode Plus driving capability activation on PB6
    #[inline(always)]
    pub fn pb6_fmp(&self) -> PB6_FMP_R {
        PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Fast-mode Plus driving capability activation on PB7
    #[inline(always)]
    pub fn pb7_fmp(&self) -> PB7_FMP_R {
        PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Fast-mode Plus driving capability activation on PB8
    #[inline(always)]
    pub fn pb8_fmp(&self) -> PB8_FMP_R {
        PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Fast-mode Plus driving capability activation on PB9
    #[inline(always)]
    pub fn pb9_fmp(&self) -> PB9_FMP_R {
        PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 21:23 - Ethernet PHY interface selection
    #[inline(always)]
    pub fn eth_sel_phy(&self) -> ETH_SEL_PHY_R {
        ETH_SEL_PHY_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMCR")
            .field("pb6_fmp", &self.pb6_fmp())
            .field("pb7_fmp", &self.pb7_fmp())
            .field("pb8_fmp", &self.pb8_fmp())
            .field("pb9_fmp", &self.pb9_fmp())
            .field("eth_sel_phy", &self.eth_sel_phy())
            .finish()
    }
}
impl W {
    ///Bit 16 - Fast-mode Plus driving capability activation on PB6
    #[inline(always)]
    pub fn pb6_fmp(&mut self) -> PB6_FMP_W<'_, PMCRrs> {
        PB6_FMP_W::new(self, 16)
    }
    ///Bit 17 - Fast-mode Plus driving capability activation on PB7
    #[inline(always)]
    pub fn pb7_fmp(&mut self) -> PB7_FMP_W<'_, PMCRrs> {
        PB7_FMP_W::new(self, 17)
    }
    ///Bit 18 - Fast-mode Plus driving capability activation on PB8
    #[inline(always)]
    pub fn pb8_fmp(&mut self) -> PB8_FMP_W<'_, PMCRrs> {
        PB8_FMP_W::new(self, 18)
    }
    ///Bit 19 - Fast-mode Plus driving capability activation on PB9
    #[inline(always)]
    pub fn pb9_fmp(&mut self) -> PB9_FMP_W<'_, PMCRrs> {
        PB9_FMP_W::new(self, 19)
    }
    ///Bits 21:23 - Ethernet PHY interface selection
    #[inline(always)]
    pub fn eth_sel_phy(&mut self) -> ETH_SEL_PHY_W<'_, PMCRrs> {
        ETH_SEL_PHY_W::new(self, 21)
    }
}
/**SBS product mode and configuration register

You can [`read`](crate::Reg::read) this register and get [`pmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#SBS:PMCR)*/
pub struct PMCRrs;
impl crate::RegisterSpec for PMCRrs {
    type Ux = u32;
}
///`read()` method returns [`pmcr::R`](R) reader structure
impl crate::Readable for PMCRrs {}
///`write(|w| ..)` method takes [`pmcr::W`](W) writer structure
impl crate::Writable for PMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMCR to value 0
impl crate::Resettable for PMCRrs {}
