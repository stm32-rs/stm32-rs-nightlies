///Register `MP_CIFR` reader
pub type R = crate::R<MP_CIFRrs>;
///Register `MP_CIFR` writer
pub type W = crate::W<MP_CIFRrs>;
///Field `LSIRDYF` reader - LSIRDYF
pub type LSIRDYF_R = crate::BitReader;
///Field `LSIRDYF` writer - LSIRDYF
pub type LSIRDYF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDYF` reader - LSERDYF
pub type LSERDYF_R = crate::BitReader;
///Field `LSERDYF` writer - LSERDYF
pub type LSERDYF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDYF` reader - HSIRDYF
pub type HSIRDYF_R = crate::BitReader;
///Field `HSIRDYF` writer - HSIRDYF
pub type HSIRDYF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDYF` reader - HSERDYF
pub type HSERDYF_R = crate::BitReader;
///Field `HSERDYF` writer - HSERDYF
pub type HSERDYF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSIRDYF` reader - CSIRDYF
pub type CSIRDYF_R = crate::BitReader;
///Field `CSIRDYF` writer - CSIRDYF
pub type CSIRDYF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1DYF` reader - PLL1DYF
pub type PLL1DYF_R = crate::BitReader;
///Field `PLL1DYF` writer - PLL1DYF
pub type PLL1DYF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2DYF` reader - PLL2DYF
pub type PLL2DYF_R = crate::BitReader;
///Field `PLL2DYF` writer - PLL2DYF
pub type PLL2DYF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3DYF` reader - PLL3DYF
pub type PLL3DYF_R = crate::BitReader;
///Field `PLL3DYF` writer - PLL3DYF
pub type PLL3DYF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4DYF` reader - PLL4DYF
pub type PLL4DYF_R = crate::BitReader;
///Field `PLL4DYF` writer - PLL4DYF
pub type PLL4DYF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSECSSF` reader - LSECSSF
pub type LSECSSF_R = crate::BitReader;
///Field `LSECSSF` writer - LSECSSF
pub type LSECSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPF` reader - WKUPF
pub type WKUPF_R = crate::BitReader;
///Field `WKUPF` writer - WKUPF
pub type WKUPF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LSIRDYF
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSERDYF
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSIRDYF
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSERDYF
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CSIRDYF
    #[inline(always)]
    pub fn csirdyf(&self) -> CSIRDYF_R {
        CSIRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - PLL1DYF
    #[inline(always)]
    pub fn pll1dyf(&self) -> PLL1DYF_R {
        PLL1DYF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLL2DYF
    #[inline(always)]
    pub fn pll2dyf(&self) -> PLL2DYF_R {
        PLL2DYF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PLL3DYF
    #[inline(always)]
    pub fn pll3dyf(&self) -> PLL3DYF_R {
        PLL3DYF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - PLL4DYF
    #[inline(always)]
    pub fn pll4dyf(&self) -> PLL4DYF_R {
        PLL4DYF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - LSECSSF
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - WKUPF
    #[inline(always)]
    pub fn wkupf(&self) -> WKUPF_R {
        WKUPF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_CIFR")
            .field("lsirdyf", &self.lsirdyf())
            .field("lserdyf", &self.lserdyf())
            .field("hsirdyf", &self.hsirdyf())
            .field("hserdyf", &self.hserdyf())
            .field("csirdyf", &self.csirdyf())
            .field("pll1dyf", &self.pll1dyf())
            .field("pll2dyf", &self.pll2dyf())
            .field("pll3dyf", &self.pll3dyf())
            .field("pll4dyf", &self.pll4dyf())
            .field("lsecssf", &self.lsecssf())
            .field("wkupf", &self.wkupf())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSIRDYF
    #[inline(always)]
    pub fn lsirdyf(&mut self) -> LSIRDYF_W<'_, MP_CIFRrs> {
        LSIRDYF_W::new(self, 0)
    }
    ///Bit 1 - LSERDYF
    #[inline(always)]
    pub fn lserdyf(&mut self) -> LSERDYF_W<'_, MP_CIFRrs> {
        LSERDYF_W::new(self, 1)
    }
    ///Bit 2 - HSIRDYF
    #[inline(always)]
    pub fn hsirdyf(&mut self) -> HSIRDYF_W<'_, MP_CIFRrs> {
        HSIRDYF_W::new(self, 2)
    }
    ///Bit 3 - HSERDYF
    #[inline(always)]
    pub fn hserdyf(&mut self) -> HSERDYF_W<'_, MP_CIFRrs> {
        HSERDYF_W::new(self, 3)
    }
    ///Bit 4 - CSIRDYF
    #[inline(always)]
    pub fn csirdyf(&mut self) -> CSIRDYF_W<'_, MP_CIFRrs> {
        CSIRDYF_W::new(self, 4)
    }
    ///Bit 8 - PLL1DYF
    #[inline(always)]
    pub fn pll1dyf(&mut self) -> PLL1DYF_W<'_, MP_CIFRrs> {
        PLL1DYF_W::new(self, 8)
    }
    ///Bit 9 - PLL2DYF
    #[inline(always)]
    pub fn pll2dyf(&mut self) -> PLL2DYF_W<'_, MP_CIFRrs> {
        PLL2DYF_W::new(self, 9)
    }
    ///Bit 10 - PLL3DYF
    #[inline(always)]
    pub fn pll3dyf(&mut self) -> PLL3DYF_W<'_, MP_CIFRrs> {
        PLL3DYF_W::new(self, 10)
    }
    ///Bit 11 - PLL4DYF
    #[inline(always)]
    pub fn pll4dyf(&mut self) -> PLL4DYF_W<'_, MP_CIFRrs> {
        PLL4DYF_W::new(self, 11)
    }
    ///Bit 16 - LSECSSF
    #[inline(always)]
    pub fn lsecssf(&mut self) -> LSECSSF_W<'_, MP_CIFRrs> {
        LSECSSF_W::new(self, 16)
    }
    ///Bit 20 - WKUPF
    #[inline(always)]
    pub fn wkupf(&mut self) -> WKUPF_W<'_, MP_CIFRrs> {
        WKUPF_W::new(self, 20)
    }
}
/**This register shall be used by the MPU in order to read and clear the interrupt flags.Writing has no effect, writing will clear the corresponding flag.Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_cifr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_cifr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_CIFR)*/
pub struct MP_CIFRrs;
impl crate::RegisterSpec for MP_CIFRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_cifr::R`](R) reader structure
impl crate::Readable for MP_CIFRrs {}
///`write(|w| ..)` method takes [`mp_cifr::W`](W) writer structure
impl crate::Writable for MP_CIFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_CIFR to value 0
impl crate::Resettable for MP_CIFRrs {}
