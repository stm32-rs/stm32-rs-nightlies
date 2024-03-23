#[doc = "Register `CIFR` reader"]
pub type R = crate::R<CIFRrs>;
#[doc = "LSI ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYF {
    #[doc = "0: Not interrupted"]
    NotInterrupted = 0,
    #[doc = "1: Interrupted"]
    Interrupted = 1,
}
impl From<LSIRDYF> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag"]
pub type LSIRDYF_R = crate::BitReader<LSIRDYF>;
impl LSIRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYF {
        match self.bits {
            false => LSIRDYF::NotInterrupted,
            true => LSIRDYF::Interrupted,
        }
    }
    #[doc = "Not interrupted"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSIRDYF::NotInterrupted
    }
    #[doc = "Interrupted"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSIRDYF::Interrupted
    }
}
#[doc = "LSE ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYF {
    #[doc = "0: Not interrupted"]
    NotInterrupted = 0,
    #[doc = "1: Interrupted"]
    Interrupted = 1,
}
impl From<LSERDYF> for bool {
    #[inline(always)]
    fn from(variant: LSERDYF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag"]
pub type LSERDYF_R = crate::BitReader<LSERDYF>;
impl LSERDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSERDYF {
        match self.bits {
            false => LSERDYF::NotInterrupted,
            true => LSERDYF::Interrupted,
        }
    }
    #[doc = "Not interrupted"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSERDYF::NotInterrupted
    }
    #[doc = "Interrupted"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSERDYF::Interrupted
    }
}
#[doc = "MSI ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYF {
    #[doc = "0: Not interrupted"]
    NotInterrupted = 0,
    #[doc = "1: Interrupted"]
    Interrupted = 1,
}
impl From<MSIRDYF> for bool {
    #[inline(always)]
    fn from(variant: MSIRDYF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSIRDYF` reader - MSI ready interrupt flag"]
pub type MSIRDYF_R = crate::BitReader<MSIRDYF>;
impl MSIRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSIRDYF {
        match self.bits {
            false => MSIRDYF::NotInterrupted,
            true => MSIRDYF::Interrupted,
        }
    }
    #[doc = "Not interrupted"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == MSIRDYF::NotInterrupted
    }
    #[doc = "Interrupted"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == MSIRDYF::Interrupted
    }
}
#[doc = "HSI16 ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYF {
    #[doc = "0: Not interrupted"]
    NotInterrupted = 0,
    #[doc = "1: Interrupted"]
    Interrupted = 1,
}
impl From<HSIRDYF> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDYF` reader - HSI16 ready interrupt flag"]
pub type HSIRDYF_R = crate::BitReader<HSIRDYF>;
impl HSIRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDYF {
        match self.bits {
            false => HSIRDYF::NotInterrupted,
            true => HSIRDYF::Interrupted,
        }
    }
    #[doc = "Not interrupted"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == HSIRDYF::NotInterrupted
    }
    #[doc = "Interrupted"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == HSIRDYF::Interrupted
    }
}
#[doc = "HSE32 ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYF {
    #[doc = "0: Not interrupted"]
    NotInterrupted = 0,
    #[doc = "1: Interrupted"]
    Interrupted = 1,
}
impl From<HSERDYF> for bool {
    #[inline(always)]
    fn from(variant: HSERDYF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDYF` reader - HSE32 ready interrupt flag"]
pub type HSERDYF_R = crate::BitReader<HSERDYF>;
impl HSERDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSERDYF {
        match self.bits {
            false => HSERDYF::NotInterrupted,
            true => HSERDYF::Interrupted,
        }
    }
    #[doc = "Not interrupted"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == HSERDYF::NotInterrupted
    }
    #[doc = "Interrupted"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == HSERDYF::Interrupted
    }
}
#[doc = "PLL ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYF {
    #[doc = "0: Not interrupted"]
    NotInterrupted = 0,
    #[doc = "1: Interrupted"]
    Interrupted = 1,
}
impl From<PLLRDYF> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLRDYF` reader - PLL ready interrupt flag"]
pub type PLLRDYF_R = crate::BitReader<PLLRDYF>;
impl PLLRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLRDYF {
        match self.bits {
            false => PLLRDYF::NotInterrupted,
            true => PLLRDYF::Interrupted,
        }
    }
    #[doc = "Not interrupted"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == PLLRDYF::NotInterrupted
    }
    #[doc = "Interrupted"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == PLLRDYF::Interrupted
    }
}
#[doc = "HSE32 Clock security system interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSF {
    #[doc = "0: Not interrupted"]
    NotInterrupted = 0,
    #[doc = "1: Interrupted"]
    Interrupted = 1,
}
impl From<CSSF> for bool {
    #[inline(always)]
    fn from(variant: CSSF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSF` reader - HSE32 Clock security system interrupt flag"]
pub type CSSF_R = crate::BitReader<CSSF>;
impl CSSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSSF {
        match self.bits {
            false => CSSF::NotInterrupted,
            true => CSSF::Interrupted,
        }
    }
    #[doc = "Not interrupted"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == CSSF::NotInterrupted
    }
    #[doc = "Interrupted"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == CSSF::Interrupted
    }
}
#[doc = "LSE Clock security system interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSF {
    #[doc = "0: Not interrupted"]
    NotInterrupted = 0,
    #[doc = "1: Interrupted"]
    Interrupted = 1,
}
impl From<LSECSSF> for bool {
    #[inline(always)]
    fn from(variant: LSECSSF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSF` reader - LSE Clock security system interrupt flag"]
pub type LSECSSF_R = crate::BitReader<LSECSSF>;
impl LSECSSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSF {
        match self.bits {
            false => LSECSSF::NotInterrupted,
            true => LSECSSF::Interrupted,
        }
    }
    #[doc = "Not interrupted"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSECSSF::NotInterrupted
    }
    #[doc = "Interrupted"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSECSSF::Interrupted
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
    #[doc = "Bit 2 - MSI ready interrupt flag"]
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI16 ready interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE32 ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - HSE32 Clock security system interrupt flag"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE Clock security system interrupt flag"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 1) != 0)
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
