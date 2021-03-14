#[doc = "Reader of register CIFR"]
pub type R = crate::R<u32, super::CIFR>;
#[doc = "Reader of field `LSI2RDYF`"]
pub type LSI2RDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSI48RDYF`"]
pub type HSI48RDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LSECSSF`"]
pub type LSECSSF_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSECSSF`"]
pub type HSECSSF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLLSAI1RDYF`"]
pub type PLLSAI1RDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLLRDYF`"]
pub type PLLRDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSERDYF`"]
pub type HSERDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSIRDYF`"]
pub type HSIRDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `MSIRDYF`"]
pub type MSIRDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LSERDYF`"]
pub type LSERDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LSI1RDYF`"]
pub type LSI1RDYF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 11 - LSI2 ready interrupt flag"]
    #[inline(always)]
    pub fn lsi2rdyf(&self) -> LSI2RDYF_R {
        LSI2RDYF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt flag"]
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LSE Clock security system interrupt flag"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HSE Clock security system interrupt flag"]
    #[inline(always)]
    pub fn hsecssf(&self) -> HSECSSF_R {
        HSECSSF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt flag"]
    #[inline(always)]
    pub fn pllsai1rdyf(&self) -> PLLSAI1RDYF_R {
        PLLSAI1RDYF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSI ready interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MSI ready interrupt flag"]
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LSI1 ready interrupt flag"]
    #[inline(always)]
    pub fn lsi1rdyf(&self) -> LSI1RDYF_R {
        LSI1RDYF_R::new((self.bits & 0x01) != 0)
    }
}
