///Register `RCC_APBENR1` reader
pub type R = crate::R<RCC_APBENR1rs>;
///Register `RCC_APBENR1` writer
pub type W = crate::W<RCC_APBENR1rs>;
/**TIM2 timer clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2EN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<TIM2EN> for bool {
    #[inline(always)]
    fn from(variant: TIM2EN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2EN` reader - TIM2 timer clock enable Set and cleared by software.
pub type TIM2EN_R = crate::BitReader<TIM2EN>;
impl TIM2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM2EN {
        match self.bits {
            false => TIM2EN::B0x0,
            true => TIM2EN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM2EN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM2EN::B0x1
    }
}
///Field `TIM2EN` writer - TIM2 timer clock enable Set and cleared by software.
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2EN>;
impl<'a, REG> TIM2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::B0x1)
    }
}
/**TIM3 timer clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM3EN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<TIM3EN> for bool {
    #[inline(always)]
    fn from(variant: TIM3EN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM3EN` reader - TIM3 timer clock enable Set and cleared by software.
pub type TIM3EN_R = crate::BitReader<TIM3EN>;
impl TIM3EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM3EN {
        match self.bits {
            false => TIM3EN::B0x0,
            true => TIM3EN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM3EN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM3EN::B0x1
    }
}
///Field `TIM3EN` writer - TIM3 timer clock enable Set and cleared by software.
pub type TIM3EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM3EN>;
impl<'a, REG> TIM3EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3EN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3EN::B0x1)
    }
}
/**RTC APB clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCAPBEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<RTCAPBEN> for bool {
    #[inline(always)]
    fn from(variant: RTCAPBEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCAPBEN` reader - RTC APB clock enable Set and cleared by software.
pub type RTCAPBEN_R = crate::BitReader<RTCAPBEN>;
impl RTCAPBEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTCAPBEN {
        match self.bits {
            false => RTCAPBEN::B0x0,
            true => RTCAPBEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTCAPBEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTCAPBEN::B0x1
    }
}
///Field `RTCAPBEN` writer - RTC APB clock enable Set and cleared by software.
pub type RTCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG, RTCAPBEN>;
impl<'a, REG> RTCAPBEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCAPBEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCAPBEN::B0x1)
    }
}
/**WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<WWDGEN> for bool {
    #[inline(always)]
    fn from(variant: WWDGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `WWDGEN` reader - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0.
pub type WWDGEN_R = crate::BitReader<WWDGEN>;
impl WWDGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WWDGEN {
        match self.bits {
            false => WWDGEN::B0x0,
            true => WWDGEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WWDGEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WWDGEN::B0x1
    }
}
///Field `WWDGEN` writer - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0.
pub type WWDGEN_W<'a, REG> = crate::BitWriter<'a, REG, WWDGEN>;
impl<'a, REG> WWDGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WWDGEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WWDGEN::B0x1)
    }
}
/**USB clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<USBEN> for bool {
    #[inline(always)]
    fn from(variant: USBEN) -> Self {
        variant as u8 != 0
    }
}
///Field `USBEN` reader - USB clock enable Set and cleared by software.
pub type USBEN_R = crate::BitReader<USBEN>;
impl USBEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USBEN {
        match self.bits {
            false => USBEN::B0x0,
            true => USBEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USBEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USBEN::B0x1
    }
}
///Field `USBEN` writer - USB clock enable Set and cleared by software.
pub type USBEN_W<'a, REG> = crate::BitWriter<'a, REG, USBEN>;
impl<'a, REG> USBEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USBEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USBEN::B0x1)
    }
}
/**SPI2 clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI2EN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<SPI2EN> for bool {
    #[inline(always)]
    fn from(variant: SPI2EN) -> Self {
        variant as u8 != 0
    }
}
///Field `SPI2EN` reader - SPI2 clock enable Set and cleared by software.
pub type SPI2EN_R = crate::BitReader<SPI2EN>;
impl SPI2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPI2EN {
        match self.bits {
            false => SPI2EN::B0x0,
            true => SPI2EN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI2EN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI2EN::B0x1
    }
}
///Field `SPI2EN` writer - SPI2 clock enable Set and cleared by software.
pub type SPI2EN_W<'a, REG> = crate::BitWriter<'a, REG, SPI2EN>;
impl<'a, REG> SPI2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2EN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2EN::B0x1)
    }
}
/**CRS clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<CRSEN> for bool {
    #[inline(always)]
    fn from(variant: CRSEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CRSEN` reader - CRS clock enable Set and cleared by software.
pub type CRSEN_R = crate::BitReader<CRSEN>;
impl CRSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRSEN {
        match self.bits {
            false => CRSEN::B0x0,
            true => CRSEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRSEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRSEN::B0x1
    }
}
///Field `CRSEN` writer - CRS clock enable Set and cleared by software.
pub type CRSEN_W<'a, REG> = crate::BitWriter<'a, REG, CRSEN>;
impl<'a, REG> CRSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRSEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRSEN::B0x1)
    }
}
/**USART2 clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART2EN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<USART2EN> for bool {
    #[inline(always)]
    fn from(variant: USART2EN) -> Self {
        variant as u8 != 0
    }
}
///Field `USART2EN` reader - USART2 clock enable Set and cleared by software.
pub type USART2EN_R = crate::BitReader<USART2EN>;
impl USART2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART2EN {
        match self.bits {
            false => USART2EN::B0x0,
            true => USART2EN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART2EN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART2EN::B0x1
    }
}
///Field `USART2EN` writer - USART2 clock enable Set and cleared by software.
pub type USART2EN_W<'a, REG> = crate::BitWriter<'a, REG, USART2EN>;
impl<'a, REG> USART2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART2EN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART2EN::B0x1)
    }
}
/**I2C1 clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1EN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<I2C1EN> for bool {
    #[inline(always)]
    fn from(variant: I2C1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C1EN` reader - I2C1 clock enable Set and cleared by software.
pub type I2C1EN_R = crate::BitReader<I2C1EN>;
impl I2C1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C1EN {
        match self.bits {
            false => I2C1EN::B0x0,
            true => I2C1EN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C1EN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C1EN::B0x1
    }
}
///Field `I2C1EN` writer - I2C1 clock enable Set and cleared by software.
pub type I2C1EN_W<'a, REG> = crate::BitWriter<'a, REG, I2C1EN>;
impl<'a, REG> I2C1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1EN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1EN::B0x1)
    }
}
/**I2C2 clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2EN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<I2C2EN> for bool {
    #[inline(always)]
    fn from(variant: I2C2EN) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C2EN` reader - I2C2 clock enable Set and cleared by software.
pub type I2C2EN_R = crate::BitReader<I2C2EN>;
impl I2C2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C2EN {
        match self.bits {
            false => I2C2EN::B0x0,
            true => I2C2EN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C2EN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C2EN::B0x1
    }
}
///Field `I2C2EN` writer - I2C2 clock enable Set and cleared by software.
pub type I2C2EN_W<'a, REG> = crate::BitWriter<'a, REG, I2C2EN>;
impl<'a, REG> I2C2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2EN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2EN::B0x1)
    }
}
/**Debug support clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<DBGEN> for bool {
    #[inline(always)]
    fn from(variant: DBGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DBGEN` reader - Debug support clock enable Set and cleared by software.
pub type DBGEN_R = crate::BitReader<DBGEN>;
impl DBGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBGEN {
        match self.bits {
            false => DBGEN::B0x0,
            true => DBGEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBGEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBGEN::B0x1
    }
}
///Field `DBGEN` writer - Debug support clock enable Set and cleared by software.
pub type DBGEN_W<'a, REG> = crate::BitWriter<'a, REG, DBGEN>;
impl<'a, REG> DBGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBGEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBGEN::B0x1)
    }
}
/**Power interface clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWREN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<PWREN> for bool {
    #[inline(always)]
    fn from(variant: PWREN) -> Self {
        variant as u8 != 0
    }
}
///Field `PWREN` reader - Power interface clock enable Set and cleared by software.
pub type PWREN_R = crate::BitReader<PWREN>;
impl PWREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PWREN {
        match self.bits {
            false => PWREN::B0x0,
            true => PWREN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PWREN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PWREN::B0x1
    }
}
///Field `PWREN` writer - Power interface clock enable Set and cleared by software.
pub type PWREN_W<'a, REG> = crate::BitWriter<'a, REG, PWREN>;
impl<'a, REG> PWREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PWREN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PWREN::B0x1)
    }
}
impl R {
    ///Bit 0 - TIM2 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 10 - RTC APB clock enable Set and cleared by software.
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0.
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - USB clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - CRS clock enable Set and cleared by software.
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 27 - Debug support clock enable Set and cleared by software.
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable Set and cleared by software.
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APBENR1")
            .field("tim2en", &self.tim2en())
            .field("tim3en", &self.tim3en())
            .field("rtcapben", &self.rtcapben())
            .field("wwdgen", &self.wwdgen())
            .field("usben", &self.usben())
            .field("spi2en", &self.spi2en())
            .field("crsen", &self.crsen())
            .field("usart2en", &self.usart2en())
            .field("i2c1en", &self.i2c1en())
            .field("i2c2en", &self.i2c2en())
            .field("dbgen", &self.dbgen())
            .field("pwren", &self.pwren())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W<'_, RCC_APBENR1rs> {
        TIM2EN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W<'_, RCC_APBENR1rs> {
        TIM3EN_W::new(self, 1)
    }
    ///Bit 10 - RTC APB clock enable Set and cleared by software.
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<'_, RCC_APBENR1rs> {
        RTCAPBEN_W::new(self, 10)
    }
    ///Bit 11 - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0.
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WWDGEN_W<'_, RCC_APBENR1rs> {
        WWDGEN_W::new(self, 11)
    }
    ///Bit 13 - USB clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usben(&mut self) -> USBEN_W<'_, RCC_APBENR1rs> {
        USBEN_W::new(self, 13)
    }
    ///Bit 14 - SPI2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W<'_, RCC_APBENR1rs> {
        SPI2EN_W::new(self, 14)
    }
    ///Bit 16 - CRS clock enable Set and cleared by software.
    #[inline(always)]
    pub fn crsen(&mut self) -> CRSEN_W<'_, RCC_APBENR1rs> {
        CRSEN_W::new(self, 16)
    }
    ///Bit 17 - USART2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W<'_, RCC_APBENR1rs> {
        USART2EN_W::new(self, 17)
    }
    ///Bit 21 - I2C1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<'_, RCC_APBENR1rs> {
        I2C1EN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W<'_, RCC_APBENR1rs> {
        I2C2EN_W::new(self, 22)
    }
    ///Bit 27 - Debug support clock enable Set and cleared by software.
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W<'_, RCC_APBENR1rs> {
        DBGEN_W::new(self, 27)
    }
    ///Bit 28 - Power interface clock enable Set and cleared by software.
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W<'_, RCC_APBENR1rs> {
        PWREN_W::new(self, 28)
    }
}
/**RCC APB peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_apbenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#RCC:RCC_APBENR1)*/
pub struct RCC_APBENR1rs;
impl crate::RegisterSpec for RCC_APBENR1rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_apbenr1::R`](R) reader structure
impl crate::Readable for RCC_APBENR1rs {}
///`write(|w| ..)` method takes [`rcc_apbenr1::W`](W) writer structure
impl crate::Writable for RCC_APBENR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_APBENR1 to value 0
impl crate::Resettable for RCC_APBENR1rs {}
