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
///Field `HSI16RDYF` reader - HSI16 ready interrupt flag
pub use LSIRDYF_R as HSI16RDYF_R;
///Field `HSERDYF` reader - HSE ready interrupt flag
pub use LSIRDYF_R as HSERDYF_R;
///Field `PLLRDYF` reader - PLL ready interrupt flag
pub use LSIRDYF_R as PLLRDYF_R;
///Field `MSIRDYF` reader - MSI ready interrupt flag
pub use LSIRDYF_R as MSIRDYF_R;
///Field `HSI48RDYF` reader - HSI48 ready interrupt flag
pub use LSIRDYF_R as HSI48RDYF_R;
/**LSE Clock Security System Interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSLSEF {
    ///0: No failure detected on LSE clock failure
    NoFailure = 0,
    ///1: Failure detected on LSE clock failure
    Failure = 1,
}
impl From<CSSLSEF> for bool {
    #[inline(always)]
    fn from(variant: CSSLSEF) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSLSEF` reader - LSE Clock Security System Interrupt flag
pub type CSSLSEF_R = crate::BitReader<CSSLSEF>;
impl CSSLSEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSSLSEF {
        match self.bits {
            false => CSSLSEF::NoFailure,
            true => CSSLSEF::Failure,
        }
    }
    ///No failure detected on LSE clock failure
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == CSSLSEF::NoFailure
    }
    ///Failure detected on LSE clock failure
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == CSSLSEF::Failure
    }
}
/**Clock Security System Interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSHSEF {
    ///0: No clock security interrupt caused by HSE clock failure
    NoClock = 0,
    ///1: Clock security interrupt caused by HSE clock failure
    Clock = 1,
}
impl From<CSSHSEF> for bool {
    #[inline(always)]
    fn from(variant: CSSHSEF) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSHSEF` reader - Clock Security System Interrupt flag
pub type CSSHSEF_R = crate::BitReader<CSSHSEF>;
impl CSSHSEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSSHSEF {
        match self.bits {
            false => CSSHSEF::NoClock,
            true => CSSHSEF::Clock,
        }
    }
    ///No clock security interrupt caused by HSE clock failure
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == CSSHSEF::NoClock
    }
    ///Clock security interrupt caused by HSE clock failure
    #[inline(always)]
    pub fn is_clock(&self) -> bool {
        *self == CSSHSEF::Clock
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
    ///Bit 2 - HSI16 ready interrupt flag
    #[inline(always)]
    pub fn hsi16rdyf(&self) -> HSI16RDYF_R {
        HSI16RDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSE ready interrupt flag
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PLL ready interrupt flag
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MSI ready interrupt flag
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - HSI48 ready interrupt flag
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LSE Clock Security System Interrupt flag
    #[inline(always)]
    pub fn csslsef(&self) -> CSSLSEF_R {
        CSSLSEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Clock Security System Interrupt flag
    #[inline(always)]
    pub fn csshsef(&self) -> CSSHSEF_R {
        CSSHSEF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIFR")
            .field("csshsef", &self.csshsef())
            .field("csslsef", &self.csslsef())
            .field("lsirdyf", &self.lsirdyf())
            .field("hsi48rdyf", &self.hsi48rdyf())
            .field("msirdyf", &self.msirdyf())
            .field("pllrdyf", &self.pllrdyf())
            .field("hserdyf", &self.hserdyf())
            .field("hsi16rdyf", &self.hsi16rdyf())
            .field("lserdyf", &self.lserdyf())
            .finish()
    }
}
/**Clock interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#RCC:CIFR)*/
pub struct CIFRrs;
impl crate::RegisterSpec for CIFRrs {
    type Ux = u32;
}
///`read()` method returns [`cifr::R`](R) reader structure
impl crate::Readable for CIFRrs {}
///`reset()` method sets CIFR to value 0
impl crate::Resettable for CIFRrs {}
