#[doc = "Reader of register CIFR"]
pub type R = crate::R<u32, super::CIFR>;
#[doc = "Reader of field `LSIRDYF`"]
pub type LSIRDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LSERDYF`"]
pub type LSERDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSIRDYF`"]
pub type HSIRDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSERDYF`"]
pub type HSERDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSIRDY`"]
pub type CSIRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSI48RDYF`"]
pub type HSI48RDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLL1RDYF`"]
pub type PLL1RDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLL2RDYF`"]
pub type PLL2RDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLL3RDYF`"]
pub type PLL3RDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LSECSSF`"]
pub type LSECSSF_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSECSSF`"]
pub type HSECSSF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - LSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSE ready Interrupt Flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSE ready Interrupt Flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn csirdy(&self) -> CSIRDY_R {
        CSIRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RC48 ready Interrupt Flag"]
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll1rdyf(&self) -> PLL1RDYF_R {
        PLL1RDYF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll2rdyf(&self) -> PLL2RDYF_R {
        PLL2RDYF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLL3 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll3rdyf(&self) -> PLL3RDYF_R {
        PLL3RDYF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system Interrupt Flag"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HSE clock security system Interrupt Flag"]
    #[inline(always)]
    pub fn hsecssf(&self) -> HSECSSF_R {
        HSECSSF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
