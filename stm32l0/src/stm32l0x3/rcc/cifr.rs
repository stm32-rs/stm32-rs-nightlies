#[doc = "Register `CIFR` reader"]
pub type R = crate::R<CIFRrs>;
#[doc = "LSI ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYFR {
    #[doc = "0: No clock ready interrupt"]
    NotInterrupted = 0,
    #[doc = "1: Clock ready interrupt"]
    Interrupted = 1,
}
impl From<LSIRDYFR> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag"]
pub type LSIRDYF_R = crate::BitReader<LSIRDYFR>;
impl LSIRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYFR {
        match self.bits {
            false => LSIRDYFR::NotInterrupted,
            true => LSIRDYFR::Interrupted,
        }
    }
    #[doc = "No clock ready interrupt"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSIRDYFR::NotInterrupted
    }
    #[doc = "Clock ready interrupt"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSIRDYFR::Interrupted
    }
}
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag"]
pub use LSIRDYF_R as LSERDYF_R;
#[doc = "Field `HSI16RDYF` reader - HSI16 ready interrupt flag"]
pub use LSIRDYF_R as HSI16RDYF_R;
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag"]
pub use LSIRDYF_R as HSERDYF_R;
#[doc = "Field `PLLRDYF` reader - PLL ready interrupt flag"]
pub use LSIRDYF_R as PLLRDYF_R;
#[doc = "Field `MSIRDYF` reader - MSI ready interrupt flag"]
pub use LSIRDYF_R as MSIRDYF_R;
#[doc = "Field `HSI48RDYF` reader - HSI48 ready interrupt flag"]
pub use LSIRDYF_R as HSI48RDYF_R;
#[doc = "LSE Clock Security System Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSLSEF {
    #[doc = "0: No failure detected on LSE clock failure"]
    NoFailure = 0,
    #[doc = "1: Failure detected on LSE clock failure"]
    Failure = 1,
}
impl From<CSSLSEF> for bool {
    #[inline(always)]
    fn from(variant: CSSLSEF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSLSEF` reader - LSE Clock Security System Interrupt flag"]
pub type CSSLSEF_R = crate::BitReader<CSSLSEF>;
impl CSSLSEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSSLSEF {
        match self.bits {
            false => CSSLSEF::NoFailure,
            true => CSSLSEF::Failure,
        }
    }
    #[doc = "No failure detected on LSE clock failure"]
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == CSSLSEF::NoFailure
    }
    #[doc = "Failure detected on LSE clock failure"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == CSSLSEF::Failure
    }
}
#[doc = "Clock Security System Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSHSEF {
    #[doc = "0: No clock security interrupt caused by HSE clock failure"]
    NoClock = 0,
    #[doc = "1: Clock security interrupt caused by HSE clock failure"]
    Clock = 1,
}
impl From<CSSHSEF> for bool {
    #[inline(always)]
    fn from(variant: CSSHSEF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSHSEF` reader - Clock Security System Interrupt flag"]
pub type CSSHSEF_R = crate::BitReader<CSSHSEF>;
impl CSSHSEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSSHSEF {
        match self.bits {
            false => CSSHSEF::NoClock,
            true => CSSHSEF::Clock,
        }
    }
    #[doc = "No clock security interrupt caused by HSE clock failure"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == CSSHSEF::NoClock
    }
    #[doc = "Clock security interrupt caused by HSE clock failure"]
    #[inline(always)]
    pub fn is_clock(&self) -> bool {
        *self == CSSHSEF::Clock
    }
}
impl R {
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI16 ready interrupt flag"]
    #[inline(always)]
    pub fn hsi16rdyf(&self) -> HSI16RDYF_R {
        HSI16RDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MSI ready interrupt flag"]
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSI48 ready interrupt flag"]
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LSE Clock Security System Interrupt flag"]
    #[inline(always)]
    pub fn csslsef(&self) -> CSSLSEF_R {
        CSSLSEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock Security System Interrupt flag"]
    #[inline(always)]
    pub fn csshsef(&self) -> CSSHSEF_R {
        CSSHSEF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Clock interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cifr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIFRrs;
impl crate::RegisterSpec for CIFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cifr::R`](R) reader structure"]
impl crate::Readable for CIFRrs {}
#[doc = "`reset()` method sets CIFR to value 0"]
impl crate::Resettable for CIFRrs {
    const RESET_VALUE: u32 = 0;
}
