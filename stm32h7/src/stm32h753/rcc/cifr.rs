///Register `CIFR` reader
pub type R = crate::R<CIFRrs>;
/**LSI ready Interrupt Flag

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
///Field `LSIRDYF` reader - LSI ready Interrupt Flag
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
///Field `LSERDYF` reader - LSE ready Interrupt Flag
pub use LSIRDYF_R as LSERDYF_R;
///Field `HSIRDYF` reader - HSI ready Interrupt Flag
pub use LSIRDYF_R as HSIRDYF_R;
///Field `HSERDYF` reader - HSE ready Interrupt Flag
pub use LSIRDYF_R as HSERDYF_R;
///Field `CSIRDY` reader - CSI ready Interrupt Flag
pub type CSIRDY_R = crate::BitReader;
///Field `HSI48RDYF` reader - RC48 ready Interrupt Flag
pub use LSIRDYF_R as HSI48RDYF_R;
///Field `PLL1RDYF` reader - PLL1 ready Interrupt Flag
pub use LSIRDYF_R as PLL1RDYF_R;
///Field `PLL2RDYF` reader - PLL2 ready Interrupt Flag
pub use LSIRDYF_R as PLL2RDYF_R;
///Field `PLL3RDYF` reader - PLL3 ready Interrupt Flag
pub use LSIRDYF_R as PLL3RDYF_R;
///Field `LSECSSF` reader - LSE clock security system Interrupt Flag
pub type LSECSSF_R = crate::BitReader;
///Field `HSECSSF` reader - HSE clock security system Interrupt Flag
pub type HSECSSF_R = crate::BitReader;
impl R {
    ///Bit 0 - LSI ready Interrupt Flag
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready Interrupt Flag
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSI ready Interrupt Flag
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSE ready Interrupt Flag
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CSI ready Interrupt Flag
    #[inline(always)]
    pub fn csirdy(&self) -> CSIRDY_R {
        CSIRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RC48 ready Interrupt Flag
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLL1 ready Interrupt Flag
    #[inline(always)]
    pub fn pll1rdyf(&self) -> PLL1RDYF_R {
        PLL1RDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLL2 ready Interrupt Flag
    #[inline(always)]
    pub fn pll2rdyf(&self) -> PLL2RDYF_R {
        PLL2RDYF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PLL3 ready Interrupt Flag
    #[inline(always)]
    pub fn pll3rdyf(&self) -> PLL3RDYF_R {
        PLL3RDYF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LSE clock security system Interrupt Flag
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSE clock security system Interrupt Flag
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
            .field("hsirdyf", &self.hsirdyf())
            .field("hserdyf", &self.hserdyf())
            .field("csirdy", &self.csirdy())
            .field("hsi48rdyf", &self.hsi48rdyf())
            .field("pll1rdyf", &self.pll1rdyf())
            .field("pll2rdyf", &self.pll2rdyf())
            .field("pll3rdyf", &self.pll3rdyf())
            .field("lsecssf", &self.lsecssf())
            .field("hsecssf", &self.hsecssf())
            .finish()
    }
}
/**RCC Clock Source Interrupt Flag Register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#RCC:CIFR)*/
pub struct CIFRrs;
impl crate::RegisterSpec for CIFRrs {
    type Ux = u32;
}
///`read()` method returns [`cifr::R`](R) reader structure
impl crate::Readable for CIFRrs {}
///`reset()` method sets CIFR to value 0
impl crate::Resettable for CIFRrs {}
