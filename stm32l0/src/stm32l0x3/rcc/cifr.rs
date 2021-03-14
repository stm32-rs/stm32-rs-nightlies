#[doc = "Reader of register CIFR"]
pub type R = crate::R<u32, super::CIFR>;
#[doc = "Clock Security System Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSHSEF_A {
    #[doc = "0: No clock security interrupt caused by HSE clock failure"]
    NOCLOCK = 0,
    #[doc = "1: Clock security interrupt caused by HSE clock failure"]
    CLOCK = 1,
}
impl From<CSSHSEF_A> for bool {
    #[inline(always)]
    fn from(variant: CSSHSEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSSHSEF`"]
pub type CSSHSEF_R = crate::R<bool, CSSHSEF_A>;
impl CSSHSEF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSHSEF_A {
        match self.bits {
            false => CSSHSEF_A::NOCLOCK,
            true => CSSHSEF_A::CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `NOCLOCK`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == CSSHSEF_A::NOCLOCK
    }
    #[doc = "Checks if the value of the field is `CLOCK`"]
    #[inline(always)]
    pub fn is_clock(&self) -> bool {
        *self == CSSHSEF_A::CLOCK
    }
}
#[doc = "LSE Clock Security System Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSLSEF_A {
    #[doc = "0: No failure detected on LSE clock failure"]
    NOFAILURE = 0,
    #[doc = "1: Failure detected on LSE clock failure"]
    FAILURE = 1,
}
impl From<CSSLSEF_A> for bool {
    #[inline(always)]
    fn from(variant: CSSLSEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSSLSEF`"]
pub type CSSLSEF_R = crate::R<bool, CSSLSEF_A>;
impl CSSLSEF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSLSEF_A {
        match self.bits {
            false => CSSLSEF_A::NOFAILURE,
            true => CSSLSEF_A::FAILURE,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAILURE`"]
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == CSSLSEF_A::NOFAILURE
    }
    #[doc = "Checks if the value of the field is `FAILURE`"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == CSSLSEF_A::FAILURE
    }
}
#[doc = "HSI48 ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSI48RDYF_A {
    #[doc = "0: No clock ready interrupt"]
    NOTINTERRUPTED = 0,
    #[doc = "1: Clock ready interrupt"]
    INTERRUPTED = 1,
}
impl From<HSI48RDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSI48RDYF`"]
pub type HSI48RDYF_R = crate::R<bool, HSI48RDYF_A>;
impl HSI48RDYF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI48RDYF_A {
        match self.bits {
            false => HSI48RDYF_A::NOTINTERRUPTED,
            true => HSI48RDYF_A::INTERRUPTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINTERRUPTED`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == HSI48RDYF_A::NOTINTERRUPTED
    }
    #[doc = "Checks if the value of the field is `INTERRUPTED`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == HSI48RDYF_A::INTERRUPTED
    }
}
#[doc = "MSI ready interrupt flag"]
pub type MSIRDYF_A = HSI48RDYF_A;
#[doc = "Reader of field `MSIRDYF`"]
pub type MSIRDYF_R = crate::R<bool, HSI48RDYF_A>;
#[doc = "PLL ready interrupt flag"]
pub type PLLRDYF_A = HSI48RDYF_A;
#[doc = "Reader of field `PLLRDYF`"]
pub type PLLRDYF_R = crate::R<bool, HSI48RDYF_A>;
#[doc = "HSE ready interrupt flag"]
pub type HSERDYF_A = HSI48RDYF_A;
#[doc = "Reader of field `HSERDYF`"]
pub type HSERDYF_R = crate::R<bool, HSI48RDYF_A>;
#[doc = "HSI16 ready interrupt flag"]
pub type HSI16RDYF_A = HSI48RDYF_A;
#[doc = "Reader of field `HSI16RDYF`"]
pub type HSI16RDYF_R = crate::R<bool, HSI48RDYF_A>;
#[doc = "LSE ready interrupt flag"]
pub type LSERDYF_A = HSI48RDYF_A;
#[doc = "Reader of field `LSERDYF`"]
pub type LSERDYF_R = crate::R<bool, HSI48RDYF_A>;
#[doc = "LSI ready interrupt flag"]
pub type LSIRDYF_A = HSI48RDYF_A;
#[doc = "Reader of field `LSIRDYF`"]
pub type LSIRDYF_R = crate::R<bool, HSI48RDYF_A>;
impl R {
    #[doc = "Bit 8 - Clock Security System Interrupt flag"]
    #[inline(always)]
    pub fn csshsef(&self) -> CSSHSEF_R {
        CSSHSEF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LSE Clock Security System Interrupt flag"]
    #[inline(always)]
    pub fn csslsef(&self) -> CSSLSEF_R {
        CSSLSEF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HSI48 ready interrupt flag"]
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MSI ready interrupt flag"]
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSI16 ready interrupt flag"]
    #[inline(always)]
    pub fn hsi16rdyf(&self) -> HSI16RDYF_R {
        HSI16RDYF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 0x01) != 0)
    }
}
