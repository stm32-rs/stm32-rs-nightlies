///Register `CIFR` reader
pub type R = crate::R<CIFRrs>;
/**LSI ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYF {
    ///0: No clock ready interrupt caused by the LSI oscillator
    NoInterrupt = 0,
    ///1: Clock ready interrupt caused by the LSI oscillator
    Interrupt = 1,
}
impl From<LSIRDYF> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYF` reader - LSI ready interrupt flag
pub type LSIRDYF_R = crate::BitReader<LSIRDYF>;
impl LSIRDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYF {
        match self.bits {
            false => LSIRDYF::NoInterrupt,
            true => LSIRDYF::Interrupt,
        }
    }
    ///No clock ready interrupt caused by the LSI oscillator
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == LSIRDYF::NoInterrupt
    }
    ///Clock ready interrupt caused by the LSI oscillator
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == LSIRDYF::Interrupt
    }
}
/**LSE ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYF {
    ///0: No clock ready interrupt caused by the LSE oscillator
    NoInterrupt = 0,
    ///1: Clock ready interrupt caused by the LSE oscillator
    Interrupt = 1,
}
impl From<LSERDYF> for bool {
    #[inline(always)]
    fn from(variant: LSERDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `LSERDYF` reader - LSE ready interrupt flag
pub type LSERDYF_R = crate::BitReader<LSERDYF>;
impl LSERDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSERDYF {
        match self.bits {
            false => LSERDYF::NoInterrupt,
            true => LSERDYF::Interrupt,
        }
    }
    ///No clock ready interrupt caused by the LSE oscillator
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == LSERDYF::NoInterrupt
    }
    ///Clock ready interrupt caused by the LSE oscillator
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == LSERDYF::Interrupt
    }
}
/**MSI ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYF {
    ///0: No clock ready interrupt caused by the MSI oscillator
    NoInterrupt = 0,
    ///1: Clock ready interrupt caused by the MSI oscillator
    Interrupt = 1,
}
impl From<MSIRDYF> for bool {
    #[inline(always)]
    fn from(variant: MSIRDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIRDYF` reader - MSI ready interrupt flag
pub type MSIRDYF_R = crate::BitReader<MSIRDYF>;
impl MSIRDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSIRDYF {
        match self.bits {
            false => MSIRDYF::NoInterrupt,
            true => MSIRDYF::Interrupt,
        }
    }
    ///No clock ready interrupt caused by the MSI oscillator
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == MSIRDYF::NoInterrupt
    }
    ///Clock ready interrupt caused by the MSI oscillator
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == MSIRDYF::Interrupt
    }
}
/**HSI ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYF {
    ///0: No clock ready interrupt caused by the HSI16 oscillator
    NoInterrupt = 0,
    ///1: Clock ready interrupt caused by the HSI16 oscillator
    Interrupt = 1,
}
impl From<HSIRDYF> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIRDYF` reader - HSI ready interrupt flag
pub type HSIRDYF_R = crate::BitReader<HSIRDYF>;
impl HSIRDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDYF {
        match self.bits {
            false => HSIRDYF::NoInterrupt,
            true => HSIRDYF::Interrupt,
        }
    }
    ///No clock ready interrupt caused by the HSI16 oscillator
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == HSIRDYF::NoInterrupt
    }
    ///Clock ready interrupt caused by the HSI16 oscillator
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == HSIRDYF::Interrupt
    }
}
/**HSE ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYF {
    ///0: No clock ready interrupt caused by the HSE oscillator
    NoInterrupt = 0,
    ///1: Clock ready interrupt caused by the HSE oscillator
    Interrupt = 1,
}
impl From<HSERDYF> for bool {
    #[inline(always)]
    fn from(variant: HSERDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `HSERDYF` reader - HSE ready interrupt flag
pub type HSERDYF_R = crate::BitReader<HSERDYF>;
impl HSERDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSERDYF {
        match self.bits {
            false => HSERDYF::NoInterrupt,
            true => HSERDYF::Interrupt,
        }
    }
    ///No clock ready interrupt caused by the HSE oscillator
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == HSERDYF::NoInterrupt
    }
    ///Clock ready interrupt caused by the HSE oscillator
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == HSERDYF::Interrupt
    }
}
/**PLL ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYF {
    ///0: No clock ready interrupt caused by PLL lock
    NoInterrupt = 0,
    ///1: Clock ready interrupt caused by PLL lock
    Interrupt = 1,
}
impl From<PLLRDYF> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLRDYF` reader - PLL ready interrupt flag
pub type PLLRDYF_R = crate::BitReader<PLLRDYF>;
impl PLLRDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLRDYF {
        match self.bits {
            false => PLLRDYF::NoInterrupt,
            true => PLLRDYF::Interrupt,
        }
    }
    ///No clock ready interrupt caused by PLL lock
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PLLRDYF::NoInterrupt
    }
    ///Clock ready interrupt caused by PLL lock
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == PLLRDYF::Interrupt
    }
}
/**PLLSAI1 ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1RDYF {
    ///0: No clock ready interrupt caused by PLLSAI1 lock
    NoInterrupt = 0,
    ///1: Clock ready interrupt caused by PLLSAI1 lock
    Interrupt = 1,
}
impl From<PLLSAI1RDYF> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1RDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSAI1RDYF` reader - PLLSAI1 ready interrupt flag
pub type PLLSAI1RDYF_R = crate::BitReader<PLLSAI1RDYF>;
impl PLLSAI1RDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI1RDYF {
        match self.bits {
            false => PLLSAI1RDYF::NoInterrupt,
            true => PLLSAI1RDYF::Interrupt,
        }
    }
    ///No clock ready interrupt caused by PLLSAI1 lock
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PLLSAI1RDYF::NoInterrupt
    }
    ///Clock ready interrupt caused by PLLSAI1 lock
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == PLLSAI1RDYF::Interrupt
    }
}
/**PLLSAI2 ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2RDYF {
    ///0: No clock ready interrupt caused by PLLSAI2 lock
    NoInterrupt = 0,
    ///1: Clock ready interrupt caused by PLLSAI2 lock
    Interrupt = 1,
}
impl From<PLLSAI2RDYF> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2RDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSAI2RDYF` reader - PLLSAI2 ready interrupt flag
pub type PLLSAI2RDYF_R = crate::BitReader<PLLSAI2RDYF>;
impl PLLSAI2RDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAI2RDYF {
        match self.bits {
            false => PLLSAI2RDYF::NoInterrupt,
            true => PLLSAI2RDYF::Interrupt,
        }
    }
    ///No clock ready interrupt caused by PLLSAI2 lock
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == PLLSAI2RDYF::NoInterrupt
    }
    ///Clock ready interrupt caused by PLLSAI2 lock
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == PLLSAI2RDYF::Interrupt
    }
}
/**Clock security system interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSF {
    ///0: No clock security interrupt caused by HSE clock failure
    NoInterrupt = 0,
    ///1: Clock security interrupt caused by HSE clock failure
    Interrupt = 1,
}
impl From<CSSF> for bool {
    #[inline(always)]
    fn from(variant: CSSF) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSF` reader - Clock security system interrupt flag
pub type CSSF_R = crate::BitReader<CSSF>;
impl CSSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSSF {
        match self.bits {
            false => CSSF::NoInterrupt,
            true => CSSF::Interrupt,
        }
    }
    ///No clock security interrupt caused by HSE clock failure
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == CSSF::NoInterrupt
    }
    ///Clock security interrupt caused by HSE clock failure
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == CSSF::Interrupt
    }
}
/**LSE Clock security system interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSF {
    ///0: No clock security interrupt caused by LSE clock failure
    NoInterrupt = 0,
    ///1: Clock security interrupt caused by LSE clock failure
    Interrupt = 1,
}
impl From<LSECSSF> for bool {
    #[inline(always)]
    fn from(variant: LSECSSF) -> Self {
        variant as u8 != 0
    }
}
///Field `LSECSSF` reader - LSE Clock security system interrupt flag
pub type LSECSSF_R = crate::BitReader<LSECSSF>;
impl LSECSSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSF {
        match self.bits {
            false => LSECSSF::NoInterrupt,
            true => LSECSSF::Interrupt,
        }
    }
    ///No clock security interrupt caused by LSE clock failure
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == LSECSSF::NoInterrupt
    }
    ///Clock security interrupt caused by LSE clock failure
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == LSECSSF::Interrupt
    }
}
/**HSI48 ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYF {
    ///0: No clock ready interrupt caused by the HSI48 oscillator
    NoInterrupt = 0,
    ///1: Clock ready interrupt caused by the HSI48 oscillator
    Interrupt = 1,
}
impl From<HSI48RDYF> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `HSI48RDYF` reader - HSI48 ready interrupt flag
pub type HSI48RDYF_R = crate::BitReader<HSI48RDYF>;
impl HSI48RDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSI48RDYF {
        match self.bits {
            false => HSI48RDYF::NoInterrupt,
            true => HSI48RDYF::Interrupt,
        }
    }
    ///No clock ready interrupt caused by the HSI48 oscillator
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == HSI48RDYF::NoInterrupt
    }
    ///Clock ready interrupt caused by the HSI48 oscillator
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == HSI48RDYF::Interrupt
    }
}
impl R {
    ///Bit 0 - LSI ready interrupt flag
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt flag
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI ready interrupt flag
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI ready interrupt flag
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE ready interrupt flag
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PLL ready interrupt flag
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLLSAI1 ready interrupt flag
    #[inline(always)]
    pub fn pllsai1rdyf(&self) -> PLLSAI1RDYF_R {
        PLLSAI1RDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLLSAI2 ready interrupt flag
    #[inline(always)]
    pub fn pllsai2rdyf(&self) -> PLLSAI2RDYF_R {
        PLLSAI2RDYF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Clock security system interrupt flag
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LSE Clock security system interrupt flag
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI48 ready interrupt flag
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIFR")
            .field("lsirdyf", &self.lsirdyf())
            .field("lserdyf", &self.lserdyf())
            .field("msirdyf", &self.msirdyf())
            .field("hsirdyf", &self.hsirdyf())
            .field("hserdyf", &self.hserdyf())
            .field("pllrdyf", &self.pllrdyf())
            .field("pllsai1rdyf", &self.pllsai1rdyf())
            .field("pllsai2rdyf", &self.pllsai2rdyf())
            .field("cssf", &self.cssf())
            .field("lsecssf", &self.lsecssf())
            .field("hsi48rdyf", &self.hsi48rdyf())
            .finish()
    }
}
/**Clock interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:CIFR)*/
pub struct CIFRrs;
impl crate::RegisterSpec for CIFRrs {
    type Ux = u32;
}
///`read()` method returns [`cifr::R`](R) reader structure
impl crate::Readable for CIFRrs {}
///`reset()` method sets CIFR to value 0
impl crate::Resettable for CIFRrs {}
