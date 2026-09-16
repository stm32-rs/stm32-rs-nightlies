///Register `RCC_APBSMENR1` reader
pub type R = crate::R<RCC_APBSMENR1rs>;
///Register `RCC_APBSMENR1` writer
pub type W = crate::W<RCC_APBSMENR1rs>;
/**TIM2 timer clock enable during Sleep mode Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2SMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<TIM2SMEN> for bool {
    #[inline(always)]
    fn from(variant: TIM2SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2SMEN` reader - TIM2 timer clock enable during Sleep mode Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type TIM2SMEN_R = crate::BitReader<TIM2SMEN>;
impl TIM2SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM2SMEN {
        match self.bits {
            false => TIM2SMEN::B0x0,
            true => TIM2SMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM2SMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM2SMEN::B0x1
    }
}
///Field `TIM2SMEN` writer - TIM2 timer clock enable during Sleep mode Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type TIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2SMEN>;
impl<'a, REG> TIM2SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2SMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2SMEN::B0x1)
    }
}
/**TIM3 timer clock enable during Sleep mode Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM3SMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<TIM3SMEN> for bool {
    #[inline(always)]
    fn from(variant: TIM3SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM3SMEN` reader - TIM3 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM3SMEN_R = crate::BitReader<TIM3SMEN>;
impl TIM3SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM3SMEN {
        match self.bits {
            false => TIM3SMEN::B0x0,
            true => TIM3SMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM3SMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM3SMEN::B0x1
    }
}
///Field `TIM3SMEN` writer - TIM3 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM3SMEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM3SMEN>;
impl<'a, REG> TIM3SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3SMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3SMEN::B0x1)
    }
}
/**RTC APB clock enable during Sleep mode Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCAPBSMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RTCAPBSMEN> for bool {
    #[inline(always)]
    fn from(variant: RTCAPBSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep mode Set and cleared by software.
pub type RTCAPBSMEN_R = crate::BitReader<RTCAPBSMEN>;
impl RTCAPBSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTCAPBSMEN {
        match self.bits {
            false => RTCAPBSMEN::B0x0,
            true => RTCAPBSMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTCAPBSMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTCAPBSMEN::B0x1
    }
}
///Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep mode Set and cleared by software.
pub type RTCAPBSMEN_W<'a, REG> = crate::BitWriter<'a, REG, RTCAPBSMEN>;
impl<'a, REG> RTCAPBSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCAPBSMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCAPBSMEN::B0x1)
    }
}
/**WWDG clock enable during Sleep and Stop modes Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGSMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<WWDGSMEN> for bool {
    #[inline(always)]
    fn from(variant: WWDGSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `WWDGSMEN` reader - WWDG clock enable during Sleep and Stop modes Set and cleared by software.
pub type WWDGSMEN_R = crate::BitReader<WWDGSMEN>;
impl WWDGSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WWDGSMEN {
        match self.bits {
            false => WWDGSMEN::B0x0,
            true => WWDGSMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WWDGSMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WWDGSMEN::B0x1
    }
}
///Field `WWDGSMEN` writer - WWDG clock enable during Sleep and Stop modes Set and cleared by software.
pub type WWDGSMEN_W<'a, REG> = crate::BitWriter<'a, REG, WWDGSMEN>;
impl<'a, REG> WWDGSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WWDGSMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WWDGSMEN::B0x1)
    }
}
/**USB clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBSMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<USBSMEN> for bool {
    #[inline(always)]
    fn from(variant: USBSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `USBSMEN` reader - USB clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type USBSMEN_R = crate::BitReader<USBSMEN>;
impl USBSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USBSMEN {
        match self.bits {
            false => USBSMEN::B0x0,
            true => USBSMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USBSMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USBSMEN::B0x1
    }
}
///Field `USBSMEN` writer - USB clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type USBSMEN_W<'a, REG> = crate::BitWriter<'a, REG, USBSMEN>;
impl<'a, REG> USBSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USBSMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USBSMEN::B0x1)
    }
}
/**SPI2 clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI2SMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<SPI2SMEN> for bool {
    #[inline(always)]
    fn from(variant: SPI2SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SPI2SMEN` reader - SPI2 clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type SPI2SMEN_R = crate::BitReader<SPI2SMEN>;
impl SPI2SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPI2SMEN {
        match self.bits {
            false => SPI2SMEN::B0x0,
            true => SPI2SMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI2SMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI2SMEN::B0x1
    }
}
///Field `SPI2SMEN` writer - SPI2 clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type SPI2SMEN_W<'a, REG> = crate::BitWriter<'a, REG, SPI2SMEN>;
impl<'a, REG> SPI2SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2SMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2SMEN::B0x1)
    }
}
/**CRS clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSSMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<CRSSMEN> for bool {
    #[inline(always)]
    fn from(variant: CRSSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CRSSMEN` reader - CRS clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type CRSSMEN_R = crate::BitReader<CRSSMEN>;
impl CRSSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRSSMEN {
        match self.bits {
            false => CRSSMEN::B0x0,
            true => CRSSMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRSSMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRSSMEN::B0x1
    }
}
///Field `CRSSMEN` writer - CRS clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type CRSSMEN_W<'a, REG> = crate::BitWriter<'a, REG, CRSSMEN>;
impl<'a, REG> CRSSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRSSMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRSSMEN::B0x1)
    }
}
/**USART2 clock enable during Sleep and Stop modes Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART2SMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<USART2SMEN> for bool {
    #[inline(always)]
    fn from(variant: USART2SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `USART2SMEN` reader - USART2 clock enable during Sleep and Stop modes Set and cleared by software.
pub type USART2SMEN_R = crate::BitReader<USART2SMEN>;
impl USART2SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART2SMEN {
        match self.bits {
            false => USART2SMEN::B0x0,
            true => USART2SMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART2SMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART2SMEN::B0x1
    }
}
///Field `USART2SMEN` writer - USART2 clock enable during Sleep and Stop modes Set and cleared by software.
pub type USART2SMEN_W<'a, REG> = crate::BitWriter<'a, REG, USART2SMEN>;
impl<'a, REG> USART2SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SMEN::B0x1)
    }
}
/**I2C1 clock enable during Sleep and Stop modes Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1SMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<I2C1SMEN> for bool {
    #[inline(always)]
    fn from(variant: I2C1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C1SMEN` reader - I2C1 clock enable during Sleep and Stop modes Set and cleared by software.
pub type I2C1SMEN_R = crate::BitReader<I2C1SMEN>;
impl I2C1SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C1SMEN {
        match self.bits {
            false => I2C1SMEN::B0x0,
            true => I2C1SMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C1SMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C1SMEN::B0x1
    }
}
///Field `I2C1SMEN` writer - I2C1 clock enable during Sleep and Stop modes Set and cleared by software.
pub type I2C1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, I2C1SMEN>;
impl<'a, REG> I2C1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SMEN::B0x1)
    }
}
/**I2C2 clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2SMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<I2C2SMEN> for bool {
    #[inline(always)]
    fn from(variant: I2C2SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C2SMEN` reader - I2C2 clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type I2C2SMEN_R = crate::BitReader<I2C2SMEN>;
impl I2C2SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C2SMEN {
        match self.bits {
            false => I2C2SMEN::B0x0,
            true => I2C2SMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C2SMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C2SMEN::B0x1
    }
}
///Field `I2C2SMEN` writer - I2C2 clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type I2C2SMEN_W<'a, REG> = crate::BitWriter<'a, REG, I2C2SMEN>;
impl<'a, REG> I2C2SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2SMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2SMEN::B0x1)
    }
}
/**Debug support clock enable during Sleep mode Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGSMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<DBGSMEN> for bool {
    #[inline(always)]
    fn from(variant: DBGSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DBGSMEN` reader - Debug support clock enable during Sleep mode Set and cleared by software.
pub type DBGSMEN_R = crate::BitReader<DBGSMEN>;
impl DBGSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBGSMEN {
        match self.bits {
            false => DBGSMEN::B0x0,
            true => DBGSMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBGSMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBGSMEN::B0x1
    }
}
///Field `DBGSMEN` writer - Debug support clock enable during Sleep mode Set and cleared by software.
pub type DBGSMEN_W<'a, REG> = crate::BitWriter<'a, REG, DBGSMEN>;
impl<'a, REG> DBGSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBGSMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBGSMEN::B0x1)
    }
}
/**Power interface clock enable during Sleep mode Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWRSMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<PWRSMEN> for bool {
    #[inline(always)]
    fn from(variant: PWRSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PWRSMEN` reader - Power interface clock enable during Sleep mode Set and cleared by software.
pub type PWRSMEN_R = crate::BitReader<PWRSMEN>;
impl PWRSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PWRSMEN {
        match self.bits {
            false => PWRSMEN::B0x0,
            true => PWRSMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PWRSMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PWRSMEN::B0x1
    }
}
///Field `PWRSMEN` writer - Power interface clock enable during Sleep mode Set and cleared by software.
pub type PWRSMEN_W<'a, REG> = crate::BitWriter<'a, REG, PWRSMEN>;
impl<'a, REG> PWRSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSMEN::B0x1)
    }
}
impl R {
    ///Bit 0 - TIM2 timer clock enable during Sleep mode Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 10 - RTC APB clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - WWDG clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - USB clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn usbsmen(&self) -> USBSMEN_R {
        USBSMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - CRS clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn crssmen(&self) -> CRSSMEN_R {
        CRSSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 27 - Debug support clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn dbgsmen(&self) -> DBGSMEN_R {
        DBGSMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APBSMENR1")
            .field("tim2smen", &self.tim2smen())
            .field("tim3smen", &self.tim3smen())
            .field("rtcapbsmen", &self.rtcapbsmen())
            .field("wwdgsmen", &self.wwdgsmen())
            .field("usbsmen", &self.usbsmen())
            .field("spi2smen", &self.spi2smen())
            .field("crssmen", &self.crssmen())
            .field("usart2smen", &self.usart2smen())
            .field("i2c1smen", &self.i2c1smen())
            .field("i2c2smen", &self.i2c2smen())
            .field("dbgsmen", &self.dbgsmen())
            .field("pwrsmen", &self.pwrsmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 timer clock enable during Sleep mode Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<'_, RCC_APBSMENR1rs> {
        TIM2SMEN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<'_, RCC_APBSMENR1rs> {
        TIM3SMEN_W::new(self, 1)
    }
    ///Bit 10 - RTC APB clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<'_, RCC_APBSMENR1rs> {
        RTCAPBSMEN_W::new(self, 10)
    }
    ///Bit 11 - WWDG clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<'_, RCC_APBSMENR1rs> {
        WWDGSMEN_W::new(self, 11)
    }
    ///Bit 13 - USB clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn usbsmen(&mut self) -> USBSMEN_W<'_, RCC_APBSMENR1rs> {
        USBSMEN_W::new(self, 13)
    }
    ///Bit 14 - SPI2 clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<'_, RCC_APBSMENR1rs> {
        SPI2SMEN_W::new(self, 14)
    }
    ///Bit 16 - CRS clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn crssmen(&mut self) -> CRSSMEN_W<'_, RCC_APBSMENR1rs> {
        CRSSMEN_W::new(self, 16)
    }
    ///Bit 17 - USART2 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<'_, RCC_APBSMENR1rs> {
        USART2SMEN_W::new(self, 17)
    }
    ///Bit 21 - I2C1 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<'_, RCC_APBSMENR1rs> {
        I2C1SMEN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable during Sleep and Stop modes Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<'_, RCC_APBSMENR1rs> {
        I2C2SMEN_W::new(self, 22)
    }
    ///Bit 27 - Debug support clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn dbgsmen(&mut self) -> DBGSMEN_W<'_, RCC_APBSMENR1rs> {
        DBGSMEN_W::new(self, 27)
    }
    ///Bit 28 - Power interface clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<'_, RCC_APBSMENR1rs> {
        PWRSMEN_W::new(self, 28)
    }
}
/**RCC APB peripheral clock enable in Sleep/Stop mode register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_apbsmenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbsmenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#RCC:RCC_APBSMENR1)*/
pub struct RCC_APBSMENR1rs;
impl crate::RegisterSpec for RCC_APBSMENR1rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_apbsmenr1::R`](R) reader structure
impl crate::Readable for RCC_APBSMENR1rs {}
///`write(|w| ..)` method takes [`rcc_apbsmenr1::W`](W) writer structure
impl crate::Writable for RCC_APBSMENR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_APBSMENR1 to value 0x1863_6c03
impl crate::Resettable for RCC_APBSMENR1rs {
    const RESET_VALUE: u32 = 0x1863_6c03;
}
