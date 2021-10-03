#[doc = "Register `CIER` reader"]
pub struct R(crate::R<CIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIER_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `CSSLSE` reader - LSE CSS interrupt flag"]
pub struct CSSLSE_R(crate::FieldReader<bool, CSSLSE_A>);
impl CSSLSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSLSE_R(crate::FieldReader::new(bits))
    }
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
        **self == CSSLSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CSSLSE_A::ENABLED
    }
}
impl core::ops::Deref for CSSLSE_R {
    type Target = crate::FieldReader<bool, CSSLSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "MSI ready interrupt flag"]
pub type MSIRDYIE_A = LSIRDYIE_A;
#[doc = "Field `MSIRDYIE` reader - MSI ready interrupt flag"]
pub type MSIRDYIE_R = LSIRDYIE_R;
#[doc = "PLL ready interrupt flag"]
pub type PLLRDYIE_A = LSIRDYIE_A;
#[doc = "Field `PLLRDYIE` reader - PLL ready interrupt flag"]
pub type PLLRDYIE_R = LSIRDYIE_R;
#[doc = "HSE ready interrupt flag"]
pub type HSERDYIE_A = LSIRDYIE_A;
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt flag"]
pub type HSERDYIE_R = LSIRDYIE_R;
#[doc = "HSI16 ready interrupt flag"]
pub type HSI16RDYIE_A = LSIRDYIE_A;
#[doc = "Field `HSI16RDYIE` reader - HSI16 ready interrupt flag"]
pub type HSI16RDYIE_R = LSIRDYIE_R;
#[doc = "LSE ready interrupt flag"]
pub type LSERDYIE_A = LSIRDYIE_A;
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt flag"]
pub type LSERDYIE_R = LSIRDYIE_R;
#[doc = "LSI ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYIE_A {
    #[doc = "0: Ready interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Ready interrupt enabled"]
    ENABLED = 1,
}
impl From<LSIRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt flag"]
pub struct LSIRDYIE_R(crate::FieldReader<bool, LSIRDYIE_A>);
impl LSIRDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSIRDYIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYIE_A {
        match self.bits {
            false => LSIRDYIE_A::DISABLED,
            true => LSIRDYIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LSIRDYIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LSIRDYIE_A::ENABLED
    }
}
impl core::ops::Deref for LSIRDYIE_R {
    type Target = crate::FieldReader<bool, LSIRDYIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 7 - LSE CSS interrupt flag"]
    #[inline(always)]
    pub fn csslse(&self) -> CSSLSE_R {
        CSSLSE_R::new(((self.bits >> 7) & 0x01) != 0)
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
#[doc = "Clock interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cier](index.html) module"]
pub struct CIER_SPEC;
impl crate::RegisterSpec for CIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cier::R](R) reader structure"]
impl crate::Readable for CIER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIER to value 0"]
impl crate::Resettable for CIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
