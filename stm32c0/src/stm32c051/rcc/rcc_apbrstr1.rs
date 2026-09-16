///Register `RCC_APBRSTR1` reader
pub type R = crate::R<RCC_APBRSTR1rs>;
///Register `RCC_APBRSTR1` writer
pub type W = crate::W<RCC_APBRSTR1rs>;
/**TIM2 timer reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2RST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset TIM2
    B0x1 = 1,
}
impl From<TIM2RST> for bool {
    #[inline(always)]
    fn from(variant: TIM2RST) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2RST` reader - TIM2 timer reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type TIM2RST_R = crate::BitReader<TIM2RST>;
impl TIM2RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM2RST {
        match self.bits {
            false => TIM2RST::B0x0,
            true => TIM2RST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM2RST::B0x0
    }
    ///Reset TIM2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM2RST::B0x1
    }
}
///Field `TIM2RST` writer - TIM2 timer reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type TIM2RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM2RST>;
impl<'a, REG> TIM2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2RST::B0x0)
    }
    ///Reset TIM2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2RST::B0x1)
    }
}
/**TIM3 timer reset Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM3RST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset TIM3
    B0x1 = 1,
}
impl From<TIM3RST> for bool {
    #[inline(always)]
    fn from(variant: TIM3RST) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM3RST` reader - TIM3 timer reset Set and cleared by software.
pub type TIM3RST_R = crate::BitReader<TIM3RST>;
impl TIM3RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM3RST {
        match self.bits {
            false => TIM3RST::B0x0,
            true => TIM3RST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM3RST::B0x0
    }
    ///Reset TIM3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM3RST::B0x1
    }
}
///Field `TIM3RST` writer - TIM3 timer reset Set and cleared by software.
pub type TIM3RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM3RST>;
impl<'a, REG> TIM3RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3RST::B0x0)
    }
    ///Reset TIM3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3RST::B0x1)
    }
}
/**USB reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBRST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset USB
    B0x1 = 1,
}
impl From<USBRST> for bool {
    #[inline(always)]
    fn from(variant: USBRST) -> Self {
        variant as u8 != 0
    }
}
///Field `USBRST` reader - USB reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type USBRST_R = crate::BitReader<USBRST>;
impl USBRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USBRST {
        match self.bits {
            false => USBRST::B0x0,
            true => USBRST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USBRST::B0x0
    }
    ///Reset USB
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USBRST::B0x1
    }
}
///Field `USBRST` writer - USB reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG, USBRST>;
impl<'a, REG> USBRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USBRST::B0x0)
    }
    ///Reset USB
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USBRST::B0x1)
    }
}
/**SPI2 reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI2RST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset SPI2
    B0x1 = 1,
}
impl From<SPI2RST> for bool {
    #[inline(always)]
    fn from(variant: SPI2RST) -> Self {
        variant as u8 != 0
    }
}
///Field `SPI2RST` reader - SPI2 reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type SPI2RST_R = crate::BitReader<SPI2RST>;
impl SPI2RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPI2RST {
        match self.bits {
            false => SPI2RST::B0x0,
            true => SPI2RST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI2RST::B0x0
    }
    ///Reset SPI2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI2RST::B0x1
    }
}
///Field `SPI2RST` writer - SPI2 reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type SPI2RST_W<'a, REG> = crate::BitWriter<'a, REG, SPI2RST>;
impl<'a, REG> SPI2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2RST::B0x0)
    }
    ///Reset SPI2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2RST::B0x1)
    }
}
/**CRS reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSRST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset CRS
    B0x1 = 1,
}
impl From<CRSRST> for bool {
    #[inline(always)]
    fn from(variant: CRSRST) -> Self {
        variant as u8 != 0
    }
}
///Field `CRSRST` reader - CRS reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type CRSRST_R = crate::BitReader<CRSRST>;
impl CRSRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRSRST {
        match self.bits {
            false => CRSRST::B0x0,
            true => CRSRST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRSRST::B0x0
    }
    ///Reset CRS
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRSRST::B0x1
    }
}
///Field `CRSRST` writer - CRS reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type CRSRST_W<'a, REG> = crate::BitWriter<'a, REG, CRSRST>;
impl<'a, REG> CRSRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRSRST::B0x0)
    }
    ///Reset CRS
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRSRST::B0x1)
    }
}
/**USART2 reset Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART2RST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset USART2
    B0x1 = 1,
}
impl From<USART2RST> for bool {
    #[inline(always)]
    fn from(variant: USART2RST) -> Self {
        variant as u8 != 0
    }
}
///Field `USART2RST` reader - USART2 reset Set and cleared by software.
pub type USART2RST_R = crate::BitReader<USART2RST>;
impl USART2RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART2RST {
        match self.bits {
            false => USART2RST::B0x0,
            true => USART2RST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART2RST::B0x0
    }
    ///Reset USART2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART2RST::B0x1
    }
}
///Field `USART2RST` writer - USART2 reset Set and cleared by software.
pub type USART2RST_W<'a, REG> = crate::BitWriter<'a, REG, USART2RST>;
impl<'a, REG> USART2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART2RST::B0x0)
    }
    ///Reset USART2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART2RST::B0x1)
    }
}
/**I2C1 reset Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1RST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset I2C1
    B0x1 = 1,
}
impl From<I2C1RST> for bool {
    #[inline(always)]
    fn from(variant: I2C1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C1RST` reader - I2C1 reset Set and cleared by software.
pub type I2C1RST_R = crate::BitReader<I2C1RST>;
impl I2C1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C1RST {
        match self.bits {
            false => I2C1RST::B0x0,
            true => I2C1RST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C1RST::B0x0
    }
    ///Reset I2C1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C1RST::B0x1
    }
}
///Field `I2C1RST` writer - I2C1 reset Set and cleared by software.
pub type I2C1RST_W<'a, REG> = crate::BitWriter<'a, REG, I2C1RST>;
impl<'a, REG> I2C1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1RST::B0x0)
    }
    ///Reset I2C1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1RST::B0x1)
    }
}
/**I2C2 reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2RST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset I2C2
    B0x1 = 1,
}
impl From<I2C2RST> for bool {
    #[inline(always)]
    fn from(variant: I2C2RST) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C2RST` reader - I2C2 reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type I2C2RST_R = crate::BitReader<I2C2RST>;
impl I2C2RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C2RST {
        match self.bits {
            false => I2C2RST::B0x0,
            true => I2C2RST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C2RST::B0x0
    }
    ///Reset I2C2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C2RST::B0x1
    }
}
///Field `I2C2RST` writer - I2C2 reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
pub type I2C2RST_W<'a, REG> = crate::BitWriter<'a, REG, I2C2RST>;
impl<'a, REG> I2C2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2RST::B0x0)
    }
    ///Reset I2C2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2RST::B0x1)
    }
}
/**Debug support reset Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGRST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset DBG
    B0x1 = 1,
}
impl From<DBGRST> for bool {
    #[inline(always)]
    fn from(variant: DBGRST) -> Self {
        variant as u8 != 0
    }
}
///Field `DBGRST` reader - Debug support reset Set and cleared by software.
pub type DBGRST_R = crate::BitReader<DBGRST>;
impl DBGRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBGRST {
        match self.bits {
            false => DBGRST::B0x0,
            true => DBGRST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBGRST::B0x0
    }
    ///Reset DBG
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBGRST::B0x1
    }
}
///Field `DBGRST` writer - Debug support reset Set and cleared by software.
pub type DBGRST_W<'a, REG> = crate::BitWriter<'a, REG, DBGRST>;
impl<'a, REG> DBGRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBGRST::B0x0)
    }
    ///Reset DBG
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBGRST::B0x1)
    }
}
/**Power interface reset Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWRRST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset PWR
    B0x1 = 1,
}
impl From<PWRRST> for bool {
    #[inline(always)]
    fn from(variant: PWRRST) -> Self {
        variant as u8 != 0
    }
}
///Field `PWRRST` reader - Power interface reset Set and cleared by software.
pub type PWRRST_R = crate::BitReader<PWRRST>;
impl PWRRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PWRRST {
        match self.bits {
            false => PWRRST::B0x0,
            true => PWRRST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PWRRST::B0x0
    }
    ///Reset PWR
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PWRRST::B0x1
    }
}
///Field `PWRRST` writer - Power interface reset Set and cleared by software.
pub type PWRRST_W<'a, REG> = crate::BitWriter<'a, REG, PWRRST>;
impl<'a, REG> PWRRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PWRRST::B0x0)
    }
    ///Reset PWR
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PWRRST::B0x1)
    }
}
impl R {
    ///Bit 0 - TIM2 timer reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 13 - USB reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SPI2 reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - CRS reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - USART2 reset Set and cleared by software.
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - I2C1 reset Set and cleared by software.
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 27 - Debug support reset Set and cleared by software.
    #[inline(always)]
    pub fn dbgrst(&self) -> DBGRST_R {
        DBGRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Power interface reset Set and cleared by software.
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APBRSTR1")
            .field("tim2rst", &self.tim2rst())
            .field("tim3rst", &self.tim3rst())
            .field("usbrst", &self.usbrst())
            .field("spi2rst", &self.spi2rst())
            .field("crsrst", &self.crsrst())
            .field("usart2rst", &self.usart2rst())
            .field("i2c1rst", &self.i2c1rst())
            .field("i2c2rst", &self.i2c2rst())
            .field("dbgrst", &self.dbgrst())
            .field("pwrrst", &self.pwrrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 timer reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W<'_, RCC_APBRSTR1rs> {
        TIM2RST_W::new(self, 0)
    }
    ///Bit 1 - TIM3 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim3rst(&mut self) -> TIM3RST_W<'_, RCC_APBRSTR1rs> {
        TIM3RST_W::new(self, 1)
    }
    ///Bit 13 - USB reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<'_, RCC_APBRSTR1rs> {
        USBRST_W::new(self, 13)
    }
    ///Bit 14 - SPI2 reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W<'_, RCC_APBRSTR1rs> {
        SPI2RST_W::new(self, 14)
    }
    ///Bit 16 - CRS reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn crsrst(&mut self) -> CRSRST_W<'_, RCC_APBRSTR1rs> {
        CRSRST_W::new(self, 16)
    }
    ///Bit 17 - USART2 reset Set and cleared by software.
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W<'_, RCC_APBRSTR1rs> {
        USART2RST_W::new(self, 17)
    }
    ///Bit 21 - I2C1 reset Set and cleared by software.
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<'_, RCC_APBRSTR1rs> {
        I2C1RST_W::new(self, 21)
    }
    ///Bit 22 - I2C2 reset Set and cleared by software. Note: Only applicable on STM32C071xx, reserved on other devices.
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<'_, RCC_APBRSTR1rs> {
        I2C2RST_W::new(self, 22)
    }
    ///Bit 27 - Debug support reset Set and cleared by software.
    #[inline(always)]
    pub fn dbgrst(&mut self) -> DBGRST_W<'_, RCC_APBRSTR1rs> {
        DBGRST_W::new(self, 27)
    }
    ///Bit 28 - Power interface reset Set and cleared by software.
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PWRRST_W<'_, RCC_APBRSTR1rs> {
        PWRRST_W::new(self, 28)
    }
}
/**RCC APB peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_apbrstr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbrstr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_APBRSTR1)*/
pub struct RCC_APBRSTR1rs;
impl crate::RegisterSpec for RCC_APBRSTR1rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_apbrstr1::R`](R) reader structure
impl crate::Readable for RCC_APBRSTR1rs {}
///`write(|w| ..)` method takes [`rcc_apbrstr1::W`](W) writer structure
impl crate::Writable for RCC_APBRSTR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_APBRSTR1 to value 0
impl crate::Resettable for RCC_APBRSTR1rs {}
