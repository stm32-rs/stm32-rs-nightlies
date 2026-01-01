///Register `CIFR` reader
pub type R = crate::R<CIFRrs>;
///Field `LSIRDYF` reader - LSI ready interrupt flag
pub type LSIRDYF_R = crate::BitReader;
///Field `LSERDYF` reader - LSE ready interrupt flag
pub type LSERDYF_R = crate::BitReader;
///Field `MSIRDYF` reader - MSI ready interrupt flag
pub type MSIRDYF_R = crate::BitReader;
///Field `HSIRDYF` reader - HSI ready interrupt flag
pub type HSIRDYF_R = crate::BitReader;
///Field `HSERDYF` reader - HSE ready interrupt flag
pub type HSERDYF_R = crate::BitReader;
///Field `PLL1RDYF` reader - PLL1 ready interrupt flag
pub type PLL1RDYF_R = crate::BitReader;
///Field `PLL2RDYF` reader - PLL2 ready interrupt flag
pub type PLL2RDYF_R = crate::BitReader;
///Field `PLL3RDYF` reader - PLL3 ready interrupt flag
pub type PLL3RDYF_R = crate::BitReader;
///Field `PLL4RDYF` reader - PLL4 ready interrupt flag
pub type PLL4RDYF_R = crate::BitReader;
///Field `LSECSSF` reader - LSE ready interrupt flag
pub type LSECSSF_R = crate::BitReader;
///Field `HSECSSF` reader - HSE ready interrupt flag
pub type HSECSSF_R = crate::BitReader;
///Field `WKUPF` reader - CPU wakeup from Stop interrupt flag
pub type WKUPF_R = crate::BitReader;
impl R {
    ///Bit 0 - LSI ready interrupt flag
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt flag
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI ready interrupt flag
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI ready interrupt flag
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE ready interrupt flag
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - PLL1 ready interrupt flag
    #[inline(always)]
    pub fn pll1rdyf(&self) -> PLL1RDYF_R {
        PLL1RDYF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLL2 ready interrupt flag
    #[inline(always)]
    pub fn pll2rdyf(&self) -> PLL2RDYF_R {
        PLL2RDYF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PLL3 ready interrupt flag
    #[inline(always)]
    pub fn pll3rdyf(&self) -> PLL3RDYF_R {
        PLL3RDYF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - PLL4 ready interrupt flag
    #[inline(always)]
    pub fn pll4rdyf(&self) -> PLL4RDYF_R {
        PLL4RDYF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - LSE ready interrupt flag
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE ready interrupt flag
    #[inline(always)]
    pub fn hsecssf(&self) -> HSECSSF_R {
        HSECSSF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 24 - CPU wakeup from Stop interrupt flag
    #[inline(always)]
    pub fn wkupf(&self) -> WKUPF_R {
        WKUPF_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIFR")
            .field("lsirdyf", &self.lsirdyf())
            .field("lserdyf", &self.lserdyf())
            .field("msirdyf", &self.msirdyf())
            .field("hsirdyf", &self.hsirdyf())
            .field("hserdyf", &self.hserdyf())
            .field("pll1rdyf", &self.pll1rdyf())
            .field("pll2rdyf", &self.pll2rdyf())
            .field("pll3rdyf", &self.pll3rdyf())
            .field("pll4rdyf", &self.pll4rdyf())
            .field("lsecssf", &self.lsecssf())
            .field("hsecssf", &self.hsecssf())
            .field("wkupf", &self.wkupf())
            .finish()
    }
}
/**RCC clock-source interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:CIFR)*/
pub struct CIFRrs;
impl crate::RegisterSpec for CIFRrs {
    type Ux = u32;
}
///`read()` method returns [`cifr::R`](R) reader structure
impl crate::Readable for CIFRrs {}
///`reset()` method sets CIFR to value 0
impl crate::Resettable for CIFRrs {}
