#[doc = "Register `CIFR` reader"]
pub type R = crate::R<CIFRrs>;
#[doc = "LSI ready Interrupt Flag\n\nValue on reset: 0"]
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
#[doc = "Field `LSIRDYF` reader - LSI ready Interrupt Flag"]
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
#[doc = "Field `LSERDYF` reader - LSE ready Interrupt Flag"]
pub use LSIRDYF_R as LSERDYF_R;
#[doc = "Field `HSIRDYF` reader - HSI ready Interrupt Flag"]
pub use LSIRDYF_R as HSIRDYF_R;
#[doc = "Field `HSERDYF` reader - HSE ready Interrupt Flag"]
pub use LSIRDYF_R as HSERDYF_R;
#[doc = "Field `CSIRDY` reader - CSI ready Interrupt Flag"]
pub type CSIRDY_R = crate::BitReader;
#[doc = "Field `HSI48RDYF` reader - RC48 ready Interrupt Flag"]
pub use LSIRDYF_R as HSI48RDYF_R;
#[doc = "Field `PLL1RDYF` reader - PLL1 ready Interrupt Flag"]
pub use LSIRDYF_R as PLL1RDYF_R;
#[doc = "Field `PLL2RDYF` reader - PLL2 ready Interrupt Flag"]
pub use LSIRDYF_R as PLL2RDYF_R;
#[doc = "Field `PLL3RDYF` reader - PLL3 ready Interrupt Flag"]
pub use LSIRDYF_R as PLL3RDYF_R;
#[doc = "Field `LSECSSF` reader - LSE clock security system Interrupt Flag"]
pub type LSECSSF_R = crate::BitReader;
#[doc = "Field `HSECSSF` reader - HSE clock security system Interrupt Flag"]
pub type HSECSSF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - LSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready Interrupt Flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready Interrupt Flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn csirdy(&self) -> CSIRDY_R {
        CSIRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RC48 ready Interrupt Flag"]
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll1rdyf(&self) -> PLL1RDYF_R {
        PLL1RDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll2rdyf(&self) -> PLL2RDYF_R {
        PLL2RDYF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL3 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll3rdyf(&self) -> PLL3RDYF_R {
        PLL3RDYF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system Interrupt Flag"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSE clock security system Interrupt Flag"]
    #[inline(always)]
    pub fn hsecssf(&self) -> HSECSSF_R {
        HSECSSF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "RCC Clock Source Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cifr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
