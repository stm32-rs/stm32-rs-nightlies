///Register `CIFR` reader
pub type R = crate::R<CIFRrs>;
/**LSI ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYF {
    ///0: Not interrupted
    NotInterrupted = 0,
    ///1: Interrupted
    Interrupted = 1,
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
            false => LSIRDYF::NotInterrupted,
            true => LSIRDYF::Interrupted,
        }
    }
    ///Not interrupted
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSIRDYF::NotInterrupted
    }
    ///Interrupted
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSIRDYF::Interrupted
    }
}
/**LSE ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYF {
    ///0: Not interrupted
    NotInterrupted = 0,
    ///1: Interrupted
    Interrupted = 1,
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
            false => LSERDYF::NotInterrupted,
            true => LSERDYF::Interrupted,
        }
    }
    ///Not interrupted
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSERDYF::NotInterrupted
    }
    ///Interrupted
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSERDYF::Interrupted
    }
}
/**MSI ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYF {
    ///0: Not interrupted
    NotInterrupted = 0,
    ///1: Interrupted
    Interrupted = 1,
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
            false => MSIRDYF::NotInterrupted,
            true => MSIRDYF::Interrupted,
        }
    }
    ///Not interrupted
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == MSIRDYF::NotInterrupted
    }
    ///Interrupted
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == MSIRDYF::Interrupted
    }
}
/**HSI16 ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYF {
    ///0: Not interrupted
    NotInterrupted = 0,
    ///1: Interrupted
    Interrupted = 1,
}
impl From<HSIRDYF> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIRDYF` reader - HSI16 ready interrupt flag
pub type HSIRDYF_R = crate::BitReader<HSIRDYF>;
impl HSIRDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDYF {
        match self.bits {
            false => HSIRDYF::NotInterrupted,
            true => HSIRDYF::Interrupted,
        }
    }
    ///Not interrupted
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == HSIRDYF::NotInterrupted
    }
    ///Interrupted
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == HSIRDYF::Interrupted
    }
}
/**HSE32 ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYF {
    ///0: Not interrupted
    NotInterrupted = 0,
    ///1: Interrupted
    Interrupted = 1,
}
impl From<HSERDYF> for bool {
    #[inline(always)]
    fn from(variant: HSERDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `HSERDYF` reader - HSE32 ready interrupt flag
pub type HSERDYF_R = crate::BitReader<HSERDYF>;
impl HSERDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSERDYF {
        match self.bits {
            false => HSERDYF::NotInterrupted,
            true => HSERDYF::Interrupted,
        }
    }
    ///Not interrupted
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == HSERDYF::NotInterrupted
    }
    ///Interrupted
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == HSERDYF::Interrupted
    }
}
/**PLL ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYF {
    ///0: Not interrupted
    NotInterrupted = 0,
    ///1: Interrupted
    Interrupted = 1,
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
            false => PLLRDYF::NotInterrupted,
            true => PLLRDYF::Interrupted,
        }
    }
    ///Not interrupted
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == PLLRDYF::NotInterrupted
    }
    ///Interrupted
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == PLLRDYF::Interrupted
    }
}
/**HSE32 Clock security system interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSF {
    ///0: Not interrupted
    NotInterrupted = 0,
    ///1: Interrupted
    Interrupted = 1,
}
impl From<CSSF> for bool {
    #[inline(always)]
    fn from(variant: CSSF) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSF` reader - HSE32 Clock security system interrupt flag
pub type CSSF_R = crate::BitReader<CSSF>;
impl CSSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSSF {
        match self.bits {
            false => CSSF::NotInterrupted,
            true => CSSF::Interrupted,
        }
    }
    ///Not interrupted
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == CSSF::NotInterrupted
    }
    ///Interrupted
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == CSSF::Interrupted
    }
}
/**LSE Clock security system interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSF {
    ///0: Not interrupted
    NotInterrupted = 0,
    ///1: Interrupted
    Interrupted = 1,
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
            false => LSECSSF::NotInterrupted,
            true => LSECSSF::Interrupted,
        }
    }
    ///Not interrupted
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSECSSF::NotInterrupted
    }
    ///Interrupted
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSECSSF::Interrupted
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
    ///Bit 3 - HSI16 ready interrupt flag
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE32 ready interrupt flag
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PLL ready interrupt flag
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - HSE32 Clock security system interrupt flag
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LSE Clock security system interrupt flag
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIFR")
            .field("lsecssf", &self.lsecssf())
            .field("cssf", &self.cssf())
            .field("pllrdyf", &self.pllrdyf())
            .field("hserdyf", &self.hserdyf())
            .field("hsirdyf", &self.hsirdyf())
            .field("msirdyf", &self.msirdyf())
            .field("lserdyf", &self.lserdyf())
            .field("lsirdyf", &self.lsirdyf())
            .finish()
    }
}
/**Clock interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:CIFR)*/
pub struct CIFRrs;
impl crate::RegisterSpec for CIFRrs {
    type Ux = u32;
}
///`read()` method returns [`cifr::R`](R) reader structure
impl crate::Readable for CIFRrs {}
///`reset()` method sets CIFR to value 0
impl crate::Resettable for CIFRrs {
    const RESET_VALUE: u32 = 0;
}
