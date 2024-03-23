#[doc = "Register `CIER` reader"]
pub type R = crate::R<CIERrs>;
#[doc = "LSI ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYIE {
    #[doc = "0: Ready interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Ready interrupt enabled"]
    Enabled = 1,
}
impl From<LSIRDYIE> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt flag"]
pub type LSIRDYIE_R = crate::BitReader<LSIRDYIE>;
impl LSIRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYIE {
        match self.bits {
            false => LSIRDYIE::Disabled,
            true => LSIRDYIE::Enabled,
        }
    }
    #[doc = "Ready interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSIRDYIE::Disabled
    }
    #[doc = "Ready interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSIRDYIE::Enabled
    }
}
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt flag"]
pub use LSIRDYIE_R as LSERDYIE_R;
#[doc = "Field `HSI16RDYIE` reader - HSI16 ready interrupt flag"]
pub use LSIRDYIE_R as HSI16RDYIE_R;
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt flag"]
pub use LSIRDYIE_R as HSERDYIE_R;
#[doc = "Field `PLLRDYIE` reader - PLL ready interrupt flag"]
pub use LSIRDYIE_R as PLLRDYIE_R;
#[doc = "Field `MSIRDYIE` reader - MSI ready interrupt flag"]
pub use LSIRDYIE_R as MSIRDYIE_R;
#[doc = "Field `HSI48RDYIE` reader - HSI48 ready interrupt flag"]
pub use LSIRDYIE_R as HSI48RDYIE_R;
#[doc = "LSE CSS interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSLSE {
    #[doc = "0: LSE CSS interrupt disabled"]
    Disabled = 0,
    #[doc = "1: LSE CSS interrupt enabled"]
    Enabled = 1,
}
impl From<CSSLSE> for bool {
    #[inline(always)]
    fn from(variant: CSSLSE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSLSE` reader - LSE CSS interrupt flag"]
pub type CSSLSE_R = crate::BitReader<CSSLSE>;
impl CSSLSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSSLSE {
        match self.bits {
            false => CSSLSE::Disabled,
            true => CSSLSE::Enabled,
        }
    }
    #[doc = "LSE CSS interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSSLSE::Disabled
    }
    #[doc = "LSE CSS interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSSLSE::Enabled
    }
}
impl R {
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI16 ready interrupt flag"]
    #[inline(always)]
    pub fn hsi16rdyie(&self) -> HSI16RDYIE_R {
        HSI16RDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MSI ready interrupt flag"]
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSI48 ready interrupt flag"]
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LSE CSS interrupt flag"]
    #[inline(always)]
    pub fn csslse(&self) -> CSSLSE_R {
        CSSLSE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Clock interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cier::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIERrs;
impl crate::RegisterSpec for CIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cier::R`](R) reader structure"]
impl crate::Readable for CIERrs {}
#[doc = "`reset()` method sets CIER to value 0"]
impl crate::Resettable for CIERrs {
    const RESET_VALUE: u32 = 0;
}
