#[doc = "Register `RCC_MC_CIER` reader"]
pub type R = crate::R<RCC_MC_CIERrs>;
#[doc = "Register `RCC_MC_CIER` writer"]
pub type W = crate::W<RCC_MC_CIERrs>;
#[doc = "Field `LSIRDYIE` reader - LSIRDYIE"]
pub type LSIRDYIE_R = crate::BitReader;
#[doc = "Field `LSIRDYIE` writer - LSIRDYIE"]
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYIE` reader - LSERDYIE"]
pub type LSERDYIE_R = crate::BitReader;
#[doc = "Field `LSERDYIE` writer - LSERDYIE"]
pub type LSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYIE` reader - HSIRDYIE"]
pub type HSIRDYIE_R = crate::BitReader;
#[doc = "Field `HSIRDYIE` writer - HSIRDYIE"]
pub type HSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYIE` reader - HSERDYIE"]
pub type HSERDYIE_R = crate::BitReader;
#[doc = "Field `HSERDYIE` writer - HSERDYIE"]
pub type HSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSIRDYIE` reader - CSIRDYIE"]
pub type CSIRDYIE_R = crate::BitReader;
#[doc = "Field `CSIRDYIE` writer - CSIRDYIE"]
pub type CSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL1DYIE` reader - PLL1DYIE"]
pub type PLL1DYIE_R = crate::BitReader;
#[doc = "Field `PLL1DYIE` writer - PLL1DYIE"]
pub type PLL1DYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL2DYIE` reader - PLL2DYIE"]
pub type PLL2DYIE_R = crate::BitReader;
#[doc = "Field `PLL2DYIE` writer - PLL2DYIE"]
pub type PLL2DYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL3DYIE` reader - PLL3DYIE"]
pub type PLL3DYIE_R = crate::BitReader;
#[doc = "Field `PLL3DYIE` writer - PLL3DYIE"]
pub type PLL3DYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL4DYIE` reader - PLL4DYIE"]
pub type PLL4DYIE_R = crate::BitReader;
#[doc = "Field `PLL4DYIE` writer - PLL4DYIE"]
pub type PLL4DYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSIE` reader - LSECSSIE"]
pub type LSECSSIE_R = crate::BitReader;
#[doc = "Field `LSECSSIE` writer - LSECSSIE"]
pub type LSECSSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPIE` reader - WKUPIE"]
pub type WKUPIE_R = crate::BitReader;
#[doc = "Field `WKUPIE` writer - WKUPIE"]
pub type WKUPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSIRDYIE"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSERDYIE"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSIRDYIE"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSERDYIE"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CSIRDYIE"]
    #[inline(always)]
    pub fn csirdyie(&self) -> CSIRDYIE_R {
        CSIRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL1DYIE"]
    #[inline(always)]
    pub fn pll1dyie(&self) -> PLL1DYIE_R {
        PLL1DYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL2DYIE"]
    #[inline(always)]
    pub fn pll2dyie(&self) -> PLL2DYIE_R {
        PLL2DYIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PLL3DYIE"]
    #[inline(always)]
    pub fn pll3dyie(&self) -> PLL3DYIE_R {
        PLL3DYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PLL4DYIE"]
    #[inline(always)]
    pub fn pll4dyie(&self) -> PLL4DYIE_R {
        PLL4DYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - LSECSSIE"]
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - WKUPIE"]
    #[inline(always)]
    pub fn wkupie(&self) -> WKUPIE_R {
        WKUPIE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSIRDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<RCC_MC_CIERrs> {
        LSIRDYIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSERDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<RCC_MC_CIERrs> {
        LSERDYIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - HSIRDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<RCC_MC_CIERrs> {
        HSIRDYIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - HSERDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<RCC_MC_CIERrs> {
        HSERDYIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - CSIRDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn csirdyie(&mut self) -> CSIRDYIE_W<RCC_MC_CIERrs> {
        CSIRDYIE_W::new(self, 4)
    }
    #[doc = "Bit 8 - PLL1DYIE"]
    #[inline(always)]
    #[must_use]
    pub fn pll1dyie(&mut self) -> PLL1DYIE_W<RCC_MC_CIERrs> {
        PLL1DYIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - PLL2DYIE"]
    #[inline(always)]
    #[must_use]
    pub fn pll2dyie(&mut self) -> PLL2DYIE_W<RCC_MC_CIERrs> {
        PLL2DYIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - PLL3DYIE"]
    #[inline(always)]
    #[must_use]
    pub fn pll3dyie(&mut self) -> PLL3DYIE_W<RCC_MC_CIERrs> {
        PLL3DYIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - PLL4DYIE"]
    #[inline(always)]
    #[must_use]
    pub fn pll4dyie(&mut self) -> PLL4DYIE_W<RCC_MC_CIERrs> {
        PLL4DYIE_W::new(self, 11)
    }
    #[doc = "Bit 16 - LSECSSIE"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<RCC_MC_CIERrs> {
        LSECSSIE_W::new(self, 16)
    }
    #[doc = "Bit 20 - WKUPIE"]
    #[inline(always)]
    #[must_use]
    pub fn wkupie(&mut self) -> WKUPIE_W<RCC_MC_CIERrs> {
        WKUPIE_W::new(self, 20)
    }
}
#[doc = "This register shall be used by the MCU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_cier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_cier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MC_CIERrs;
impl crate::RegisterSpec for RCC_MC_CIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mc_cier::R`](R) reader structure"]
impl crate::Readable for RCC_MC_CIERrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mc_cier::W`](W) writer structure"]
impl crate::Writable for RCC_MC_CIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MC_CIER to value 0"]
impl crate::Resettable for RCC_MC_CIERrs {
    const RESET_VALUE: u32 = 0;
}
