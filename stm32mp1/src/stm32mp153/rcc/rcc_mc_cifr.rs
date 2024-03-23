#[doc = "Register `RCC_MC_CIFR` reader"]
pub type R = crate::R<RCC_MC_CIFRrs>;
#[doc = "Register `RCC_MC_CIFR` writer"]
pub type W = crate::W<RCC_MC_CIFRrs>;
#[doc = "Field `LSIRDYF` reader - LSIRDYF"]
pub type LSIRDYF_R = crate::BitReader;
#[doc = "Field `LSIRDYF` writer - LSIRDYF"]
pub type LSIRDYF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYF` reader - LSERDYF"]
pub type LSERDYF_R = crate::BitReader;
#[doc = "Field `LSERDYF` writer - LSERDYF"]
pub type LSERDYF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYF` reader - HSIRDYF"]
pub type HSIRDYF_R = crate::BitReader;
#[doc = "Field `HSIRDYF` writer - HSIRDYF"]
pub type HSIRDYF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYF` reader - HSERDYF"]
pub type HSERDYF_R = crate::BitReader;
#[doc = "Field `HSERDYF` writer - HSERDYF"]
pub type HSERDYF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSIRDYF` reader - CSIRDYF"]
pub type CSIRDYF_R = crate::BitReader;
#[doc = "Field `CSIRDYF` writer - CSIRDYF"]
pub type CSIRDYF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL1DYF` reader - PLL1DYF"]
pub type PLL1DYF_R = crate::BitReader;
#[doc = "Field `PLL1DYF` writer - PLL1DYF"]
pub type PLL1DYF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL2DYF` reader - PLL2DYF"]
pub type PLL2DYF_R = crate::BitReader;
#[doc = "Field `PLL2DYF` writer - PLL2DYF"]
pub type PLL2DYF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL3DYF` reader - PLL3DYF"]
pub type PLL3DYF_R = crate::BitReader;
#[doc = "Field `PLL3DYF` writer - PLL3DYF"]
pub type PLL3DYF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL4DYF` reader - PLL4DYF"]
pub type PLL4DYF_R = crate::BitReader;
#[doc = "Field `PLL4DYF` writer - PLL4DYF"]
pub type PLL4DYF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSF` reader - LSECSSF"]
pub type LSECSSF_R = crate::BitReader;
#[doc = "Field `LSECSSF` writer - LSECSSF"]
pub type LSECSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPF` reader - WKUPF"]
pub type WKUPF_R = crate::BitReader;
#[doc = "Field `WKUPF` writer - WKUPF"]
pub type WKUPF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSIRDYF"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSERDYF"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSIRDYF"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSERDYF"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CSIRDYF"]
    #[inline(always)]
    pub fn csirdyf(&self) -> CSIRDYF_R {
        CSIRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL1DYF"]
    #[inline(always)]
    pub fn pll1dyf(&self) -> PLL1DYF_R {
        PLL1DYF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL2DYF"]
    #[inline(always)]
    pub fn pll2dyf(&self) -> PLL2DYF_R {
        PLL2DYF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PLL3DYF"]
    #[inline(always)]
    pub fn pll3dyf(&self) -> PLL3DYF_R {
        PLL3DYF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PLL4DYF"]
    #[inline(always)]
    pub fn pll4dyf(&self) -> PLL4DYF_R {
        PLL4DYF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - LSECSSF"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - WKUPF"]
    #[inline(always)]
    pub fn wkupf(&self) -> WKUPF_R {
        WKUPF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSIRDYF"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyf(&mut self) -> LSIRDYF_W<RCC_MC_CIFRrs> {
        LSIRDYF_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSERDYF"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyf(&mut self) -> LSERDYF_W<RCC_MC_CIFRrs> {
        LSERDYF_W::new(self, 1)
    }
    #[doc = "Bit 2 - HSIRDYF"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyf(&mut self) -> HSIRDYF_W<RCC_MC_CIFRrs> {
        HSIRDYF_W::new(self, 2)
    }
    #[doc = "Bit 3 - HSERDYF"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyf(&mut self) -> HSERDYF_W<RCC_MC_CIFRrs> {
        HSERDYF_W::new(self, 3)
    }
    #[doc = "Bit 4 - CSIRDYF"]
    #[inline(always)]
    #[must_use]
    pub fn csirdyf(&mut self) -> CSIRDYF_W<RCC_MC_CIFRrs> {
        CSIRDYF_W::new(self, 4)
    }
    #[doc = "Bit 8 - PLL1DYF"]
    #[inline(always)]
    #[must_use]
    pub fn pll1dyf(&mut self) -> PLL1DYF_W<RCC_MC_CIFRrs> {
        PLL1DYF_W::new(self, 8)
    }
    #[doc = "Bit 9 - PLL2DYF"]
    #[inline(always)]
    #[must_use]
    pub fn pll2dyf(&mut self) -> PLL2DYF_W<RCC_MC_CIFRrs> {
        PLL2DYF_W::new(self, 9)
    }
    #[doc = "Bit 10 - PLL3DYF"]
    #[inline(always)]
    #[must_use]
    pub fn pll3dyf(&mut self) -> PLL3DYF_W<RCC_MC_CIFRrs> {
        PLL3DYF_W::new(self, 10)
    }
    #[doc = "Bit 11 - PLL4DYF"]
    #[inline(always)]
    #[must_use]
    pub fn pll4dyf(&mut self) -> PLL4DYF_W<RCC_MC_CIFRrs> {
        PLL4DYF_W::new(self, 11)
    }
    #[doc = "Bit 16 - LSECSSF"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssf(&mut self) -> LSECSSF_W<RCC_MC_CIFRrs> {
        LSECSSF_W::new(self, 16)
    }
    #[doc = "Bit 20 - WKUPF"]
    #[inline(always)]
    #[must_use]
    pub fn wkupf(&mut self) -> WKUPF_W<RCC_MC_CIFRrs> {
        WKUPF_W::new(self, 20)
    }
}
#[doc = "This register shall be used by the MCU in order to read and clear the interrupt flags.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_cifr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_cifr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MC_CIFRrs;
impl crate::RegisterSpec for RCC_MC_CIFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mc_cifr::R`](R) reader structure"]
impl crate::Readable for RCC_MC_CIFRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mc_cifr::W`](W) writer structure"]
impl crate::Writable for RCC_MC_CIFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MC_CIFR to value 0"]
impl crate::Resettable for RCC_MC_CIFRrs {
    const RESET_VALUE: u32 = 0;
}
