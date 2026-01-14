///Register `CIFR` reader
pub type R = crate::R<CIFRrs>;
/**LSI ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYFR {
    ///0: No clock ready interrupt
    NotInterrupted = 0,
    ///1: Clock ready interrupt
    Interrupted = 1,
}
impl From<LSIRDYFR> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYFR) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYF` reader - LSI ready interrupt flag
pub type LSIRDYF_R = crate::BitReader<LSIRDYFR>;
impl LSIRDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYFR {
        match self.bits {
            false => LSIRDYFR::NotInterrupted,
            true => LSIRDYFR::Interrupted,
        }
    }
    ///No clock ready interrupt
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSIRDYFR::NotInterrupted
    }
    ///Clock ready interrupt
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSIRDYFR::Interrupted
    }
}
///Field `LSERDYF` reader - LSE ready interrupt flag
pub use LSIRDYF_R as LSERDYF_R;
///Field `CSIRDYF` reader - CSI ready interrupt flag
pub use LSIRDYF_R as CSIRDYF_R;
///Field `HSIRDYF` reader - HSI ready interrupt flag
pub use LSIRDYF_R as HSIRDYF_R;
///Field `HSERDYF` reader - HSE ready interrupt flag
pub use LSIRDYF_R as HSERDYF_R;
///Field `HSI48RDYF` reader - HSI48 ready interrupt flag
pub use LSIRDYF_R as HSI48RDYF_R;
///Field `PLL1RDYF` reader - PLL1 ready interrupt flag
pub use LSIRDYF_R as PLL1RDYF_R;
///Field `PLL2RDYF` reader - PLL2 ready interrupt flag
pub use LSIRDYF_R as PLL2RDYF_R;
///Field `PLL3RDYF` reader - PLL3 ready interrupt flag
pub use LSIRDYF_R as PLL3RDYF_R;
/**HSE clock security system interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSECSSFR {
    ///0: No clock security interrupt caused by HSE clock failure
    NoInterrupt = 0,
    ///1: Clock security interrupt caused by HSE clock failure
    Interrupt = 1,
}
impl From<HSECSSFR> for bool {
    #[inline(always)]
    fn from(variant: HSECSSFR) -> Self {
        variant as u8 != 0
    }
}
///Field `HSECSSF` reader - HSE clock security system interrupt flag
pub type HSECSSF_R = crate::BitReader<HSECSSFR>;
impl HSECSSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSECSSFR {
        match self.bits {
            false => HSECSSFR::NoInterrupt,
            true => HSECSSFR::Interrupt,
        }
    }
    ///No clock security interrupt caused by HSE clock failure
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == HSECSSFR::NoInterrupt
    }
    ///Clock security interrupt caused by HSE clock failure
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == HSECSSFR::Interrupt
    }
}
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
    ///Bit 2 - CSI ready interrupt flag
    #[inline(always)]
    pub fn csirdyf(&self) -> CSIRDYF_R {
        CSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
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
    ///Bit 5 - HSI48 ready interrupt flag
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLL1 ready interrupt flag
    #[inline(always)]
    pub fn pll1rdyf(&self) -> PLL1RDYF_R {
        PLL1RDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLL2 ready interrupt flag
    #[inline(always)]
    pub fn pll2rdyf(&self) -> PLL2RDYF_R {
        PLL2RDYF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PLL3 ready interrupt flag
    #[inline(always)]
    pub fn pll3rdyf(&self) -> PLL3RDYF_R {
        PLL3RDYF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - HSE clock security system interrupt flag
    #[inline(always)]
    pub fn hsecssf(&self) -> HSECSSF_R {
        HSECSSF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIFR")
            .field("lsirdyf", &self.lsirdyf())
            .field("lserdyf", &self.lserdyf())
            .field("csirdyf", &self.csirdyf())
            .field("hsirdyf", &self.hsirdyf())
            .field("hserdyf", &self.hserdyf())
            .field("hsi48rdyf", &self.hsi48rdyf())
            .field("pll1rdyf", &self.pll1rdyf())
            .field("pll2rdyf", &self.pll2rdyf())
            .field("pll3rdyf", &self.pll3rdyf())
            .field("hsecssf", &self.hsecssf())
            .finish()
    }
}
/**RCC clock source interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#RCC:CIFR)*/
pub struct CIFRrs;
impl crate::RegisterSpec for CIFRrs {
    type Ux = u32;
}
///`read()` method returns [`cifr::R`](R) reader structure
impl crate::Readable for CIFRrs {}
///`reset()` method sets CIFR to value 0
impl crate::Resettable for CIFRrs {}
