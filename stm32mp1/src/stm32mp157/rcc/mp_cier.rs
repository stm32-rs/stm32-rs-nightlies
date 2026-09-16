///Register `MP_CIER` reader
pub type R = crate::R<MP_CIERrs>;
///Register `MP_CIER` writer
pub type W = crate::W<MP_CIERrs>;
///Field `LSIRDYIE` reader - LSIRDYIE
pub type LSIRDYIE_R = crate::BitReader;
///Field `LSIRDYIE` writer - LSIRDYIE
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDYIE` reader - LSERDYIE
pub type LSERDYIE_R = crate::BitReader;
///Field `LSERDYIE` writer - LSERDYIE
pub type LSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDYIE` reader - HSIRDYIE
pub type HSIRDYIE_R = crate::BitReader;
///Field `HSIRDYIE` writer - HSIRDYIE
pub type HSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDYIE` reader - HSERDYIE
pub type HSERDYIE_R = crate::BitReader;
///Field `HSERDYIE` writer - HSERDYIE
pub type HSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSIRDYIE` reader - CSIRDYIE
pub type CSIRDYIE_R = crate::BitReader;
///Field `CSIRDYIE` writer - CSIRDYIE
pub type CSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1DYIE` reader - PLL1DYIE
pub type PLL1DYIE_R = crate::BitReader;
///Field `PLL1DYIE` writer - PLL1DYIE
pub type PLL1DYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2DYIE` reader - PLL2DYIE
pub type PLL2DYIE_R = crate::BitReader;
///Field `PLL2DYIE` writer - PLL2DYIE
pub type PLL2DYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3DYIE` reader - PLL3DYIE
pub type PLL3DYIE_R = crate::BitReader;
///Field `PLL3DYIE` writer - PLL3DYIE
pub type PLL3DYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4DYIE` reader - PLL4DYIE
pub type PLL4DYIE_R = crate::BitReader;
///Field `PLL4DYIE` writer - PLL4DYIE
pub type PLL4DYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSECSSIE` reader - LSECSSIE
pub type LSECSSIE_R = crate::BitReader;
///Field `LSECSSIE` writer - LSECSSIE
pub type LSECSSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPIE` reader - WKUPIE
pub type WKUPIE_R = crate::BitReader;
///Field `WKUPIE` writer - WKUPIE
pub type WKUPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LSIRDYIE
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSERDYIE
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSIRDYIE
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSERDYIE
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CSIRDYIE
    #[inline(always)]
    pub fn csirdyie(&self) -> CSIRDYIE_R {
        CSIRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - PLL1DYIE
    #[inline(always)]
    pub fn pll1dyie(&self) -> PLL1DYIE_R {
        PLL1DYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLL2DYIE
    #[inline(always)]
    pub fn pll2dyie(&self) -> PLL2DYIE_R {
        PLL2DYIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PLL3DYIE
    #[inline(always)]
    pub fn pll3dyie(&self) -> PLL3DYIE_R {
        PLL3DYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - PLL4DYIE
    #[inline(always)]
    pub fn pll4dyie(&self) -> PLL4DYIE_R {
        PLL4DYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - LSECSSIE
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - WKUPIE
    #[inline(always)]
    pub fn wkupie(&self) -> WKUPIE_R {
        WKUPIE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_CIER")
            .field("lsirdyie", &self.lsirdyie())
            .field("lserdyie", &self.lserdyie())
            .field("hsirdyie", &self.hsirdyie())
            .field("hserdyie", &self.hserdyie())
            .field("csirdyie", &self.csirdyie())
            .field("pll1dyie", &self.pll1dyie())
            .field("pll2dyie", &self.pll2dyie())
            .field("pll3dyie", &self.pll3dyie())
            .field("pll4dyie", &self.pll4dyie())
            .field("lsecssie", &self.lsecssie())
            .field("wkupie", &self.wkupie())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSIRDYIE
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<'_, MP_CIERrs> {
        LSIRDYIE_W::new(self, 0)
    }
    ///Bit 1 - LSERDYIE
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<'_, MP_CIERrs> {
        LSERDYIE_W::new(self, 1)
    }
    ///Bit 2 - HSIRDYIE
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<'_, MP_CIERrs> {
        HSIRDYIE_W::new(self, 2)
    }
    ///Bit 3 - HSERDYIE
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<'_, MP_CIERrs> {
        HSERDYIE_W::new(self, 3)
    }
    ///Bit 4 - CSIRDYIE
    #[inline(always)]
    pub fn csirdyie(&mut self) -> CSIRDYIE_W<'_, MP_CIERrs> {
        CSIRDYIE_W::new(self, 4)
    }
    ///Bit 8 - PLL1DYIE
    #[inline(always)]
    pub fn pll1dyie(&mut self) -> PLL1DYIE_W<'_, MP_CIERrs> {
        PLL1DYIE_W::new(self, 8)
    }
    ///Bit 9 - PLL2DYIE
    #[inline(always)]
    pub fn pll2dyie(&mut self) -> PLL2DYIE_W<'_, MP_CIERrs> {
        PLL2DYIE_W::new(self, 9)
    }
    ///Bit 10 - PLL3DYIE
    #[inline(always)]
    pub fn pll3dyie(&mut self) -> PLL3DYIE_W<'_, MP_CIERrs> {
        PLL3DYIE_W::new(self, 10)
    }
    ///Bit 11 - PLL4DYIE
    #[inline(always)]
    pub fn pll4dyie(&mut self) -> PLL4DYIE_W<'_, MP_CIERrs> {
        PLL4DYIE_W::new(self, 11)
    }
    ///Bit 16 - LSECSSIE
    #[inline(always)]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<'_, MP_CIERrs> {
        LSECSSIE_W::new(self, 16)
    }
    ///Bit 20 - WKUPIE
    #[inline(always)]
    pub fn wkupie(&mut self) -> WKUPIE_W<'_, MP_CIERrs> {
        WKUPIE_W::new(self, 20)
    }
}
/**This register shall be used by the MPU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MP_CIER)*/
pub struct MP_CIERrs;
impl crate::RegisterSpec for MP_CIERrs {
    type Ux = u32;
}
///`read()` method returns [`mp_cier::R`](R) reader structure
impl crate::Readable for MP_CIERrs {}
///`write(|w| ..)` method takes [`mp_cier::W`](W) writer structure
impl crate::Writable for MP_CIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_CIER to value 0
impl crate::Resettable for MP_CIERrs {}
