#[doc = "Register `CIFR` reader"]
pub type R = crate::R<CIFRrs>;
#[doc = "LSI ready interrupt flag Reset by software by writing LSIRDYC bit. Set by hardware when the LSI clock becomes stable and LSIRDYIE is set.\n\nValue on reset: 0"]
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
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag Reset by software by writing LSIRDYC bit. Set by hardware when the LSI clock becomes stable and LSIRDYIE is set."]
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
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag Reset by software by writing LSERDYC bit. Set by hardware when the LSE clock becomes stable and LSERDYIE is set."]
pub use LSIRDYF_R as LSERDYF_R;
#[doc = "Field `CSIRDYF` reader - CSI ready interrupt flag Reset by software by writing CSIRDYC bit. Set by hardware when the CSI clock becomes stable and CSIRDYIE is set."]
pub use LSIRDYF_R as CSIRDYF_R;
#[doc = "Field `HSIRDYF` reader - HSI ready interrupt flag Reset by software by writing HSIRDYC bit. Set by hardware when the HSI clock becomes stable and HSIRDYIE is set."]
pub use LSIRDYF_R as HSIRDYF_R;
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag Reset by software by writing HSERDYC bit. Set by hardware when the HSE clock becomes stable and HSERDYIE is set."]
pub use LSIRDYF_R as HSERDYF_R;
#[doc = "Field `HSI48RDYF` reader - HSI48 ready interrupt flag Reset by software by writing HSI48RDYC bit. Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set."]
pub use LSIRDYF_R as HSI48RDYF_R;
#[doc = "Field `PLL1RDYF` reader - PLL1 ready interrupt flag Reset by software by writing PLL1RDYC bit. Set by hardware when the PLL1 locks and PLL1RDYIE is set."]
pub use LSIRDYF_R as PLL1RDYF_R;
#[doc = "Field `PLL2RDYF` reader - PLL2 ready interrupt flag Reset by software by writing PLL2RDYC bit. Set by hardware when the PLL2 locks and PLL2RDYIE is set."]
pub use LSIRDYF_R as PLL2RDYF_R;
#[doc = "Field `PLL3RDYF` reader - PLL3 ready interrupt flag Reset by software by writing PLL3RDYC bit. Set by hardware when the PLL3 locks and PLL3RDYIE is set."]
pub use LSIRDYF_R as PLL3RDYF_R;
#[doc = "HSE clock security system interrupt flag Reset by software by writing HSECSSC bit. Set by hardware in case of HSE clock failure.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSECSSFR {
    #[doc = "0: No clock security interrupt caused by HSE clock failure"]
    NoInterrupt = 0,
    #[doc = "1: Clock security interrupt caused by HSE clock failure"]
    Interrupt = 1,
}
impl From<HSECSSFR> for bool {
    #[inline(always)]
    fn from(variant: HSECSSFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSECSSF` reader - HSE clock security system interrupt flag Reset by software by writing HSECSSC bit. Set by hardware in case of HSE clock failure."]
pub type HSECSSF_R = crate::BitReader<HSECSSFR>;
impl HSECSSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSECSSFR {
        match self.bits {
            false => HSECSSFR::NoInterrupt,
            true => HSECSSFR::Interrupt,
        }
    }
    #[doc = "No clock security interrupt caused by HSE clock failure"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == HSECSSFR::NoInterrupt
    }
    #[doc = "Clock security interrupt caused by HSE clock failure"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == HSECSSFR::Interrupt
    }
}
impl R {
    #[doc = "Bit 0 - LSI ready interrupt flag Reset by software by writing LSIRDYC bit. Set by hardware when the LSI clock becomes stable and LSIRDYIE is set."]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag Reset by software by writing LSERDYC bit. Set by hardware when the LSE clock becomes stable and LSERDYIE is set."]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CSI ready interrupt flag Reset by software by writing CSIRDYC bit. Set by hardware when the CSI clock becomes stable and CSIRDYIE is set."]
    #[inline(always)]
    pub fn csirdyf(&self) -> CSIRDYF_R {
        CSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI ready interrupt flag Reset by software by writing HSIRDYC bit. Set by hardware when the HSI clock becomes stable and HSIRDYIE is set."]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt flag Reset by software by writing HSERDYC bit. Set by hardware when the HSE clock becomes stable and HSERDYIE is set."]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSI48 ready interrupt flag Reset by software by writing HSI48RDYC bit. Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set."]
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready interrupt flag Reset by software by writing PLL1RDYC bit. Set by hardware when the PLL1 locks and PLL1RDYIE is set."]
    #[inline(always)]
    pub fn pll1rdyf(&self) -> PLL1RDYF_R {
        PLL1RDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready interrupt flag Reset by software by writing PLL2RDYC bit. Set by hardware when the PLL2 locks and PLL2RDYIE is set."]
    #[inline(always)]
    pub fn pll2rdyf(&self) -> PLL2RDYF_R {
        PLL2RDYF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL3 ready interrupt flag Reset by software by writing PLL3RDYC bit. Set by hardware when the PLL3 locks and PLL3RDYIE is set."]
    #[inline(always)]
    pub fn pll3rdyf(&self) -> PLL3RDYF_R {
        PLL3RDYF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - HSE clock security system interrupt flag Reset by software by writing HSECSSC bit. Set by hardware in case of HSE clock failure."]
    #[inline(always)]
    pub fn hsecssf(&self) -> HSECSSF_R {
        HSECSSF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "RCC clock source interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cifr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
