#[doc = "Register `CIFR` reader"]
pub type R = crate::R<CIFRrs>;
#[doc = "LSI ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYF {
    #[doc = "0: No clock ready interrupt caused by the LSI oscillator"]
    NoInterrupt = 0,
    #[doc = "1: Clock ready interrupt caused by the LSI oscillator"]
    Interrupt = 1,
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
            false => LSIRDYF::NoInterrupt,
            true => LSIRDYF::Interrupt,
        }
    }
    #[doc = "No clock ready interrupt caused by the LSI oscillator"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == LSIRDYF::NoInterrupt
    }
    #[doc = "Clock ready interrupt caused by the LSI oscillator"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == LSIRDYF::Interrupt
    }
}
#[doc = "LSE ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYF {
    #[doc = "0: No clock ready interrupt caused by the LSE oscillator"]
    NoInterrupt = 0,
    #[doc = "1: Clock ready interrupt caused by the LSE oscillator"]
    Interrupt = 1,
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
            false => LSERDYF::NoInterrupt,
            true => LSERDYF::Interrupt,
        }
    }
    #[doc = "No clock ready interrupt caused by the LSE oscillator"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == LSERDYF::NoInterrupt
    }
    #[doc = "Clock ready interrupt caused by the LSE oscillator"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == LSERDYF::Interrupt
    }
}
#[doc = "MSI ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYF {
    #[doc = "0: No clock ready interrupt caused by the MSI oscillator"]
    NoInterrupt = 0,
    #[doc = "1: Clock ready interrupt caused by the MSI oscillator"]
    Interrupt = 1,
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
            false => MSIRDYF::NoInterrupt,
            true => MSIRDYF::Interrupt,
        }
    }
    #[doc = "No clock ready interrupt caused by the MSI oscillator"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == MSIRDYF::NoInterrupt
    }
    #[doc = "Clock ready interrupt caused by the MSI oscillator"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == MSIRDYF::Interrupt
    }
}
#[doc = "HSI ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYF {
    #[doc = "0: No clock ready interrupt caused by the HSI16 oscillator"]
    NoInterrupt = 0,
    #[doc = "1: Clock ready interrupt caused by the HSI16 oscillator"]
    Interrupt = 1,
}
impl From<HSIRDYF> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDYF` reader - HSI ready interrupt flag"]
pub type HSIRDYF_R = crate::BitReader<HSIRDYF>;
impl HSIRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDYF {
        match self.bits {
            false => HSIRDYF::NoInterrupt,
            true => HSIRDYF::Interrupt,
        }
    }
    #[doc = "No clock ready interrupt caused by the HSI16 oscillator"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == HSIRDYF::NoInterrupt
    }
    #[doc = "Clock ready interrupt caused by the HSI16 oscillator"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == HSIRDYF::Interrupt
    }
}
#[doc = "HSE ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYF {
    #[doc = "0: No clock ready interrupt caused by the HSE oscillator"]
    NoInterrupt = 0,
    #[doc = "1: Clock ready interrupt caused by the HSE oscillator"]
    Interrupt = 1,
}
impl From<HSERDYF> for bool {
    #[inline(always)]
    fn from(variant: HSERDYF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag"]
pub type HSERDYF_R = crate::BitReader<HSERDYF>;
impl HSERDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSERDYF {
        match self.bits {
            false => HSERDYF::NoInterrupt,
            true => HSERDYF::Interrupt,
        }
    }
    #[doc = "No clock ready interrupt caused by the HSE oscillator"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == HSERDYF::NoInterrupt
    }
    #[doc = "Clock ready interrupt caused by the HSE oscillator"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == HSERDYF::Interrupt
    }
}
#[doc = "PLL ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYF {
    #[doc = "0: No clock ready interrupt caused by PLL lock"]
    NoInterrupt = 0,
    #[doc = "1: Clock ready interrupt caused by PLL lock"]
    Interrupt = 1,
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
            false => PLLRDYF::NoInterrupt,
            true => PLLRDYF::Interrupt,
        }
    }
    #[doc = "No clock ready interrupt caused by PLL lock"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PLLRDYF::NoInterrupt
    }
    #[doc = "Clock ready interrupt caused by PLL lock"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == PLLRDYF::Interrupt
    }
}
#[doc = "PLLSAI1 ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1RDYF {
    #[doc = "0: No clock ready interrupt caused by PLLSAI1 lock"]
    NoInterrupt = 0,
    #[doc = "1: Clock ready interrupt caused by PLLSAI1 lock"]
    Interrupt = 1,
}
impl From<PLLSAI1RDYF> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1RDYF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSAI1RDYF` reader - PLLSAI1 ready interrupt flag"]
pub type PLLSAI1RDYF_R = crate::BitReader<PLLSAI1RDYF>;
impl PLLSAI1RDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI1RDYF {
        match self.bits {
            false => PLLSAI1RDYF::NoInterrupt,
            true => PLLSAI1RDYF::Interrupt,
        }
    }
    #[doc = "No clock ready interrupt caused by PLLSAI1 lock"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PLLSAI1RDYF::NoInterrupt
    }
    #[doc = "Clock ready interrupt caused by PLLSAI1 lock"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == PLLSAI1RDYF::Interrupt
    }
}
#[doc = "PLLSAI2 ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2RDYF {
    #[doc = "0: No clock ready interrupt caused by PLLSAI2 lock"]
    NoInterrupt = 0,
    #[doc = "1: Clock ready interrupt caused by PLLSAI2 lock"]
    Interrupt = 1,
}
impl From<PLLSAI2RDYF> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2RDYF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSAI2RDYF` reader - PLLSAI2 ready interrupt flag"]
pub type PLLSAI2RDYF_R = crate::BitReader<PLLSAI2RDYF>;
impl PLLSAI2RDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI2RDYF {
        match self.bits {
            false => PLLSAI2RDYF::NoInterrupt,
            true => PLLSAI2RDYF::Interrupt,
        }
    }
    #[doc = "No clock ready interrupt caused by PLLSAI2 lock"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PLLSAI2RDYF::NoInterrupt
    }
    #[doc = "Clock ready interrupt caused by PLLSAI2 lock"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == PLLSAI2RDYF::Interrupt
    }
}
#[doc = "Clock security system interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSF {
    #[doc = "0: No clock security interrupt caused by HSE clock failure"]
    NoInterrupt = 0,
    #[doc = "1: Clock security interrupt caused by HSE clock failure"]
    Interrupt = 1,
}
impl From<CSSF> for bool {
    #[inline(always)]
    fn from(variant: CSSF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSF` reader - Clock security system interrupt flag"]
pub type CSSF_R = crate::BitReader<CSSF>;
impl CSSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSSF {
        match self.bits {
            false => CSSF::NoInterrupt,
            true => CSSF::Interrupt,
        }
    }
    #[doc = "No clock security interrupt caused by HSE clock failure"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == CSSF::NoInterrupt
    }
    #[doc = "Clock security interrupt caused by HSE clock failure"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == CSSF::Interrupt
    }
}
#[doc = "LSE Clock security system interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSF {
    #[doc = "0: No clock security interrupt caused by LSE clock failure"]
    NoInterrupt = 0,
    #[doc = "1: Clock security interrupt caused by LSE clock failure"]
    Interrupt = 1,
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
            false => LSECSSF::NoInterrupt,
            true => LSECSSF::Interrupt,
        }
    }
    #[doc = "No clock security interrupt caused by LSE clock failure"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == LSECSSF::NoInterrupt
    }
    #[doc = "Clock security interrupt caused by LSE clock failure"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == LSECSSF::Interrupt
    }
}
#[doc = "HSI48 ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYF {
    #[doc = "0: No clock ready interrupt caused by the HSI48 oscillator"]
    NoInterrupt = 0,
    #[doc = "1: Clock ready interrupt caused by the HSI48 oscillator"]
    Interrupt = 1,
}
impl From<HSI48RDYF> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48RDYF` reader - HSI48 ready interrupt flag"]
pub type HSI48RDYF_R = crate::BitReader<HSI48RDYF>;
impl HSI48RDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSI48RDYF {
        match self.bits {
            false => HSI48RDYF::NoInterrupt,
            true => HSI48RDYF::Interrupt,
        }
    }
    #[doc = "No clock ready interrupt caused by the HSI48 oscillator"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == HSI48RDYF::NoInterrupt
    }
    #[doc = "Clock ready interrupt caused by the HSI48 oscillator"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == HSI48RDYF::Interrupt
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
    #[doc = "Bit 3 - HSI ready interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt flag"]
    #[inline(always)]
    pub fn pllsai1rdyf(&self) -> PLLSAI1RDYF_R {
        PLLSAI1RDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLLSAI2 ready interrupt flag"]
    #[inline(always)]
    pub fn pllsai2rdyf(&self) -> PLLSAI2RDYF_R {
        PLLSAI2RDYF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock security system interrupt flag"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE Clock security system interrupt flag"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt flag"]
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 10) & 1) != 0)
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
