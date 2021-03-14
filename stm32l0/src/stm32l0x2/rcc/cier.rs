#[doc = "Reader of register CIER"]
pub type R = crate::R<u32, super::CIER>;
#[doc = "LSE CSS interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSLSE_A {
    #[doc = "0: LSE CSS interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: LSE CSS interrupt enabled"]
    ENABLED = 1,
}
impl From<CSSLSE_A> for bool {
    #[inline(always)]
    fn from(variant: CSSLSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSSLSE`"]
pub type CSSLSE_R = crate::R<bool, CSSLSE_A>;
impl CSSLSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSLSE_A {
        match self.bits {
            false => CSSLSE_A::DISABLED,
            true => CSSLSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSSLSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSSLSE_A::ENABLED
    }
}
#[doc = "HSI48 ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSI48RDYIE_A {
    #[doc = "0: Ready interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Ready interrupt enabled"]
    ENABLED = 1,
}
impl From<HSI48RDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSI48RDYIE`"]
pub type HSI48RDYIE_R = crate::R<bool, HSI48RDYIE_A>;
impl HSI48RDYIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI48RDYIE_A {
        match self.bits {
            false => HSI48RDYIE_A::DISABLED,
            true => HSI48RDYIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSI48RDYIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSI48RDYIE_A::ENABLED
    }
}
#[doc = "MSI ready interrupt flag"]
pub type MSIRDYIE_A = HSI48RDYIE_A;
#[doc = "Reader of field `MSIRDYIE`"]
pub type MSIRDYIE_R = crate::R<bool, HSI48RDYIE_A>;
#[doc = "PLL ready interrupt flag"]
pub type PLLRDYIE_A = HSI48RDYIE_A;
#[doc = "Reader of field `PLLRDYIE`"]
pub type PLLRDYIE_R = crate::R<bool, HSI48RDYIE_A>;
#[doc = "HSE ready interrupt flag"]
pub type HSERDYIE_A = HSI48RDYIE_A;
#[doc = "Reader of field `HSERDYIE`"]
pub type HSERDYIE_R = crate::R<bool, HSI48RDYIE_A>;
#[doc = "HSI16 ready interrupt flag"]
pub type HSI16RDYIE_A = HSI48RDYIE_A;
#[doc = "Reader of field `HSI16RDYIE`"]
pub type HSI16RDYIE_R = crate::R<bool, HSI48RDYIE_A>;
#[doc = "LSE ready interrupt flag"]
pub type LSERDYIE_A = HSI48RDYIE_A;
#[doc = "Reader of field `LSERDYIE`"]
pub type LSERDYIE_R = crate::R<bool, HSI48RDYIE_A>;
#[doc = "LSI ready interrupt flag"]
pub type LSIRDYIE_A = HSI48RDYIE_A;
#[doc = "Reader of field `LSIRDYIE`"]
pub type LSIRDYIE_R = crate::R<bool, HSI48RDYIE_A>;
impl R {
    #[doc = "Bit 7 - LSE CSS interrupt flag"]
    #[inline(always)]
    pub fn csslse(&self) -> CSSLSE_R {
        CSSLSE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HSI48 ready interrupt flag"]
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MSI ready interrupt flag"]
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSI16 ready interrupt flag"]
    #[inline(always)]
    pub fn hsi16rdyie(&self) -> HSI16RDYIE_R {
        HSI16RDYIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 0x01) != 0)
    }
}
