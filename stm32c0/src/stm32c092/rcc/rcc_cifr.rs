///Register `RCC_CIFR` reader
pub type R = crate::R<RCC_CIFRrs>;
/**LSI ready interrupt flag This flag indicates a pending interrupt upon LSE clock getting ready. Set by hardware when the LSI clock becomes stable and LSIRDYDIE is set. Cleared by software setting the LSIRDYC bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYF {
    ///0: Interrupt not pending
    B0x0 = 0,
    ///1: Interrupt pending
    B0x1 = 1,
}
impl From<LSIRDYF> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYF` reader - LSI ready interrupt flag This flag indicates a pending interrupt upon LSE clock getting ready. Set by hardware when the LSI clock becomes stable and LSIRDYDIE is set. Cleared by software setting the LSIRDYC bit.
pub type LSIRDYF_R = crate::BitReader<LSIRDYF>;
impl LSIRDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYF {
        match self.bits {
            false => LSIRDYF::B0x0,
            true => LSIRDYF::B0x1,
        }
    }
    ///Interrupt not pending
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSIRDYF::B0x0
    }
    ///Interrupt pending
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSIRDYF::B0x1
    }
}
/**LSE ready interrupt flag This flag indicates a pending interrupt upon LSE clock getting ready. Set by hardware when the LSE clock becomes stable and LSERDYDIE is set. Cleared by software setting the LSERDYC bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYF {
    ///0: Interrupt not pending
    B0x0 = 0,
    ///1: Interrupt pending
    B0x1 = 1,
}
impl From<LSERDYF> for bool {
    #[inline(always)]
    fn from(variant: LSERDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `LSERDYF` reader - LSE ready interrupt flag This flag indicates a pending interrupt upon LSE clock getting ready. Set by hardware when the LSE clock becomes stable and LSERDYDIE is set. Cleared by software setting the LSERDYC bit.
pub type LSERDYF_R = crate::BitReader<LSERDYF>;
impl LSERDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSERDYF {
        match self.bits {
            false => LSERDYF::B0x0,
            true => LSERDYF::B0x1,
        }
    }
    ///Interrupt not pending
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSERDYF::B0x0
    }
    ///Interrupt pending
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSERDYF::B0x1
    }
}
/**HSIUSB48 ready interrupt flag Set by hardware when the HSIUSB48 clock becomes stable and HSIUSB48RDYIE is set as a response to setting HSIUSB48ON (refer to RCC clock control register (RCC_CR)). When HSIUSB48ON is not set but the HSIUSB48 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIUSB48RDYC bit. Note: Only applicable on STM32C071xx, reserved on other devices.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIUSB48RDYF {
    ///0: Interrupt not pending
    B0x0 = 0,
    ///1: Interrupt pending
    B0x1 = 1,
}
impl From<HSIUSB48RDYF> for bool {
    #[inline(always)]
    fn from(variant: HSIUSB48RDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIUSB48RDYF` reader - HSIUSB48 ready interrupt flag Set by hardware when the HSIUSB48 clock becomes stable and HSIUSB48RDYIE is set as a response to setting HSIUSB48ON (refer to RCC clock control register (RCC_CR)). When HSIUSB48ON is not set but the HSIUSB48 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIUSB48RDYC bit. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type HSIUSB48RDYF_R = crate::BitReader<HSIUSB48RDYF>;
impl HSIUSB48RDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIUSB48RDYF {
        match self.bits {
            false => HSIUSB48RDYF::B0x0,
            true => HSIUSB48RDYF::B0x1,
        }
    }
    ///Interrupt not pending
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIUSB48RDYF::B0x0
    }
    ///Interrupt pending
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIUSB48RDYF::B0x1
    }
}
/**HSI48 ready interrupt flag This flag indicates a pending interrupt upon HSI48 clock getting ready. Set by hardware when the HSI48 clock becomes stable and HSIRDYIE is set in response to setting the HSION (refer to RCC clock control register (RCC_CR)). When HSION is not set but the HSI48 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYF {
    ///0: Interrupt not pending
    B0x0 = 0,
    ///1: Interrupt pending
    B0x1 = 1,
}
impl From<HSIRDYF> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIRDYF` reader - HSI48 ready interrupt flag This flag indicates a pending interrupt upon HSI48 clock getting ready. Set by hardware when the HSI48 clock becomes stable and HSIRDYIE is set in response to setting the HSION (refer to RCC clock control register (RCC_CR)). When HSION is not set but the HSI48 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit.
pub type HSIRDYF_R = crate::BitReader<HSIRDYF>;
impl HSIRDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDYF {
        match self.bits {
            false => HSIRDYF::B0x0,
            true => HSIRDYF::B0x1,
        }
    }
    ///Interrupt not pending
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIRDYF::B0x0
    }
    ///Interrupt pending
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIRDYF::B0x1
    }
}
/**HSE ready interrupt flag This flag indicates a pending interrupt upon HSE clock getting ready. Set by hardware when the HSE clock becomes stable and HSERDYIE is set. Cleared by software setting the HSERDYC bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYF {
    ///0: Interrupt not pending
    B0x0 = 0,
    ///1: Interrupt pending
    B0x1 = 1,
}
impl From<HSERDYF> for bool {
    #[inline(always)]
    fn from(variant: HSERDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `HSERDYF` reader - HSE ready interrupt flag This flag indicates a pending interrupt upon HSE clock getting ready. Set by hardware when the HSE clock becomes stable and HSERDYIE is set. Cleared by software setting the HSERDYC bit.
pub type HSERDYF_R = crate::BitReader<HSERDYF>;
impl HSERDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSERDYF {
        match self.bits {
            false => HSERDYF::B0x0,
            true => HSERDYF::B0x1,
        }
    }
    ///Interrupt not pending
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSERDYF::B0x0
    }
    ///Interrupt pending
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSERDYF::B0x1
    }
}
/**HSE clock security system interrupt flag This flag indicates a pending interrupt upon HSE clock failure. Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSF {
    ///0: Interrupt not pending
    B0x0 = 0,
    ///1: Interrupt pending
    B0x1 = 1,
}
impl From<CSSF> for bool {
    #[inline(always)]
    fn from(variant: CSSF) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSF` reader - HSE clock security system interrupt flag This flag indicates a pending interrupt upon HSE clock failure. Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit.
pub type CSSF_R = crate::BitReader<CSSF>;
impl CSSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSSF {
        match self.bits {
            false => CSSF::B0x0,
            true => CSSF::B0x1,
        }
    }
    ///Interrupt not pending
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CSSF::B0x0
    }
    ///Interrupt pending
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CSSF::B0x1
    }
}
/**LSE clock security system interrupt flag This flag indicates a pending interrupt upon LSE clock failure. Set by hardware when a failure is detected in the LSE oscillator. Cleared by software by setting the LSECSSC bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSF {
    ///0: Interrupt not pending
    B0x0 = 0,
    ///1: Interrupt pending
    B0x1 = 1,
}
impl From<LSECSSF> for bool {
    #[inline(always)]
    fn from(variant: LSECSSF) -> Self {
        variant as u8 != 0
    }
}
///Field `LSECSSF` reader - LSE clock security system interrupt flag This flag indicates a pending interrupt upon LSE clock failure. Set by hardware when a failure is detected in the LSE oscillator. Cleared by software by setting the LSECSSC bit.
pub type LSECSSF_R = crate::BitReader<LSECSSF>;
impl LSECSSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSF {
        match self.bits {
            false => LSECSSF::B0x0,
            true => LSECSSF::B0x1,
        }
    }
    ///Interrupt not pending
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSECSSF::B0x0
    }
    ///Interrupt pending
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSECSSF::B0x1
    }
}
impl R {
    ///Bit 0 - LSI ready interrupt flag This flag indicates a pending interrupt upon LSE clock getting ready. Set by hardware when the LSI clock becomes stable and LSIRDYDIE is set. Cleared by software setting the LSIRDYC bit.
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt flag This flag indicates a pending interrupt upon LSE clock getting ready. Set by hardware when the LSE clock becomes stable and LSERDYDIE is set. Cleared by software setting the LSERDYC bit.
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSIUSB48 ready interrupt flag Set by hardware when the HSIUSB48 clock becomes stable and HSIUSB48RDYIE is set as a response to setting HSIUSB48ON (refer to RCC clock control register (RCC_CR)). When HSIUSB48ON is not set but the HSIUSB48 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIUSB48RDYC bit. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn hsiusb48rdyf(&self) -> HSIUSB48RDYF_R {
        HSIUSB48RDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI48 ready interrupt flag This flag indicates a pending interrupt upon HSI48 clock getting ready. Set by hardware when the HSI48 clock becomes stable and HSIRDYIE is set in response to setting the HSION (refer to RCC clock control register (RCC_CR)). When HSION is not set but the HSI48 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit.
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE ready interrupt flag This flag indicates a pending interrupt upon HSE clock getting ready. Set by hardware when the HSE clock becomes stable and HSERDYIE is set. Cleared by software setting the HSERDYC bit.
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - HSE clock security system interrupt flag This flag indicates a pending interrupt upon HSE clock failure. Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit.
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LSE clock security system interrupt flag This flag indicates a pending interrupt upon LSE clock failure. Set by hardware when a failure is detected in the LSE oscillator. Cleared by software by setting the LSECSSC bit.
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CIFR")
            .field("lsirdyf", &self.lsirdyf())
            .field("lserdyf", &self.lserdyf())
            .field("hsiusb48rdyf", &self.hsiusb48rdyf())
            .field("hsirdyf", &self.hsirdyf())
            .field("hserdyf", &self.hserdyf())
            .field("cssf", &self.cssf())
            .field("lsecssf", &self.lsecssf())
            .finish()
    }
}
/**RCC clock interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`rcc_cifr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#RCC:RCC_CIFR)*/
pub struct RCC_CIFRrs;
impl crate::RegisterSpec for RCC_CIFRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_cifr::R`](R) reader structure
impl crate::Readable for RCC_CIFRrs {}
///`reset()` method sets RCC_CIFR to value 0
impl crate::Resettable for RCC_CIFRrs {}
