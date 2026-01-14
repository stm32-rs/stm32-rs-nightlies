///Register `APB1SMENR1` reader
pub type R = crate::R<APB1SMENR1rs>;
///Register `APB1SMENR1` writer
pub type W = crate::W<APB1SMENR1rs>;
/**TIM2 timer clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2SMEN {
    ///0: TIMx clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: TIMx clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<TIM2SMEN> for bool {
    #[inline(always)]
    fn from(variant: TIM2SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2SMEN` reader - TIM2 timer clocks enable during Sleep and Stop modes
pub type TIM2SMEN_R = crate::BitReader<TIM2SMEN>;
impl TIM2SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM2SMEN {
        match self.bits {
            false => TIM2SMEN::Disabled,
            true => TIM2SMEN::Enabled,
        }
    }
    ///TIMx clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2SMEN::Disabled
    }
    ///TIMx clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2SMEN::Enabled
    }
}
///Field `TIM2SMEN` writer - TIM2 timer clocks enable during Sleep and Stop modes
pub type TIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2SMEN>;
impl<'a, REG> TIM2SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TIMx clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2SMEN::Disabled)
    }
    ///TIMx clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2SMEN::Enabled)
    }
}
///Field `TIM3SMEN` reader - TIM3 timer clocks enable during Sleep and Stop modes
pub use TIM2SMEN_R as TIM3SMEN_R;
///Field `TIM4SMEN` reader - TIM4 timer clocks enable during Sleep and Stop modes
pub use TIM2SMEN_R as TIM4SMEN_R;
///Field `TIM5SMEN` reader - TIM5 timer clocks enable during Sleep and Stop modes
pub use TIM2SMEN_R as TIM5SMEN_R;
///Field `TIM6SMEN` reader - TIM6 timer clocks enable during Sleep and Stop modes
pub use TIM2SMEN_R as TIM6SMEN_R;
///Field `TIM7SMEN` reader - TIM7 timer clocks enable during Sleep and Stop modes
pub use TIM2SMEN_R as TIM7SMEN_R;
///Field `TIM3SMEN` writer - TIM3 timer clocks enable during Sleep and Stop modes
pub use TIM2SMEN_W as TIM3SMEN_W;
///Field `TIM4SMEN` writer - TIM4 timer clocks enable during Sleep and Stop modes
pub use TIM2SMEN_W as TIM4SMEN_W;
///Field `TIM5SMEN` writer - TIM5 timer clocks enable during Sleep and Stop modes
pub use TIM2SMEN_W as TIM5SMEN_W;
///Field `TIM6SMEN` writer - TIM6 timer clocks enable during Sleep and Stop modes
pub use TIM2SMEN_W as TIM6SMEN_W;
///Field `TIM7SMEN` writer - TIM7 timer clocks enable during Sleep and Stop modes
pub use TIM2SMEN_W as TIM7SMEN_W;
/**RTC APB clock enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCAPBSMEN {
    ///0: RTC APB clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: RTC APB clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<RTCAPBSMEN> for bool {
    #[inline(always)]
    fn from(variant: RTCAPBSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep and Stop modes
pub type RTCAPBSMEN_R = crate::BitReader<RTCAPBSMEN>;
impl RTCAPBSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTCAPBSMEN {
        match self.bits {
            false => RTCAPBSMEN::Disabled,
            true => RTCAPBSMEN::Enabled,
        }
    }
    ///RTC APB clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCAPBSMEN::Disabled
    }
    ///RTC APB clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCAPBSMEN::Enabled
    }
}
///Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep and Stop modes
pub type RTCAPBSMEN_W<'a, REG> = crate::BitWriter<'a, REG, RTCAPBSMEN>;
impl<'a, REG> RTCAPBSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTC APB clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCAPBSMEN::Disabled)
    }
    ///RTC APB clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCAPBSMEN::Enabled)
    }
}
/**Window watchdog clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGSMEN {
    ///0: Window watchdog clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: Window watchdog clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<WWDGSMEN> for bool {
    #[inline(always)]
    fn from(variant: WWDGSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `WWDGSMEN` reader - Window watchdog clocks enable during Sleep and Stop modes
pub type WWDGSMEN_R = crate::BitReader<WWDGSMEN>;
impl WWDGSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WWDGSMEN {
        match self.bits {
            false => WWDGSMEN::Disabled,
            true => WWDGSMEN::Enabled,
        }
    }
    ///Window watchdog clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WWDGSMEN::Disabled
    }
    ///Window watchdog clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WWDGSMEN::Enabled
    }
}
///Field `WWDGSMEN` writer - Window watchdog clocks enable during Sleep and Stop modes
pub type WWDGSMEN_W<'a, REG> = crate::BitWriter<'a, REG, WWDGSMEN>;
impl<'a, REG> WWDGSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Window watchdog clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WWDGSMEN::Disabled)
    }
    ///Window watchdog clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WWDGSMEN::Enabled)
    }
}
/**SPI2 clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI2SMEN {
    ///0: SPIx clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: SPIx clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<SPI2SMEN> for bool {
    #[inline(always)]
    fn from(variant: SPI2SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SPI2SMEN` reader - SPI2 clocks enable during Sleep and Stop modes
pub type SPI2SMEN_R = crate::BitReader<SPI2SMEN>;
impl SPI2SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPI2SMEN {
        match self.bits {
            false => SPI2SMEN::Disabled,
            true => SPI2SMEN::Enabled,
        }
    }
    ///SPIx clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPI2SMEN::Disabled
    }
    ///SPIx clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPI2SMEN::Enabled
    }
}
///Field `SPI2SMEN` writer - SPI2 clocks enable during Sleep and Stop modes
pub type SPI2SMEN_W<'a, REG> = crate::BitWriter<'a, REG, SPI2SMEN>;
impl<'a, REG> SPI2SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SPIx clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2SMEN::Disabled)
    }
    ///SPIx clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2SMEN::Enabled)
    }
}
///Field `SP3SMEN` reader - SPI3 clocks enable during Sleep and Stop modes
pub type SP3SMEN_R = crate::BitReader;
///Field `SP3SMEN` writer - SPI3 clocks enable during Sleep and Stop modes
pub type SP3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**USART2 clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART2SMEN {
    ///0: USARTx clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: USARTx clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<USART2SMEN> for bool {
    #[inline(always)]
    fn from(variant: USART2SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `USART2SMEN` reader - USART2 clocks enable during Sleep and Stop modes
pub type USART2SMEN_R = crate::BitReader<USART2SMEN>;
impl USART2SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART2SMEN {
        match self.bits {
            false => USART2SMEN::Disabled,
            true => USART2SMEN::Enabled,
        }
    }
    ///USARTx clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USART2SMEN::Disabled
    }
    ///USARTx clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USART2SMEN::Enabled
    }
}
///Field `USART2SMEN` writer - USART2 clocks enable during Sleep and Stop modes
pub type USART2SMEN_W<'a, REG> = crate::BitWriter<'a, REG, USART2SMEN>;
impl<'a, REG> USART2SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///USARTx clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SMEN::Disabled)
    }
    ///USARTx clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SMEN::Enabled)
    }
}
///Field `USART3SMEN` reader - USART3 clocks enable during Sleep and Stop modes
pub use USART2SMEN_R as USART3SMEN_R;
///Field `USART3SMEN` writer - USART3 clocks enable during Sleep and Stop modes
pub use USART2SMEN_W as USART3SMEN_W;
/**UART4 clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART4SMEN {
    ///0: UARTx clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: UARTx clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<UART4SMEN> for bool {
    #[inline(always)]
    fn from(variant: UART4SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `UART4SMEN` reader - UART4 clocks enable during Sleep and Stop modes
pub type UART4SMEN_R = crate::BitReader<UART4SMEN>;
impl UART4SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UART4SMEN {
        match self.bits {
            false => UART4SMEN::Disabled,
            true => UART4SMEN::Enabled,
        }
    }
    ///UARTx clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UART4SMEN::Disabled
    }
    ///UARTx clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UART4SMEN::Enabled
    }
}
///Field `UART4SMEN` writer - UART4 clocks enable during Sleep and Stop modes
pub type UART4SMEN_W<'a, REG> = crate::BitWriter<'a, REG, UART4SMEN>;
impl<'a, REG> UART4SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///UARTx clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UART4SMEN::Disabled)
    }
    ///UARTx clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UART4SMEN::Enabled)
    }
}
///Field `UART5SMEN` reader - UART5 clocks enable during Sleep and Stop modes
pub use UART4SMEN_R as UART5SMEN_R;
///Field `UART5SMEN` writer - UART5 clocks enable during Sleep and Stop modes
pub use UART4SMEN_W as UART5SMEN_W;
/**I2C1 clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1SMEN {
    ///0: I2Cx clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: I2Cx clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<I2C1SMEN> for bool {
    #[inline(always)]
    fn from(variant: I2C1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C1SMEN` reader - I2C1 clocks enable during Sleep and Stop modes
pub type I2C1SMEN_R = crate::BitReader<I2C1SMEN>;
impl I2C1SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C1SMEN {
        match self.bits {
            false => I2C1SMEN::Disabled,
            true => I2C1SMEN::Enabled,
        }
    }
    ///I2Cx clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C1SMEN::Disabled
    }
    ///I2Cx clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C1SMEN::Enabled
    }
}
///Field `I2C1SMEN` writer - I2C1 clocks enable during Sleep and Stop modes
pub type I2C1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, I2C1SMEN>;
impl<'a, REG> I2C1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///I2Cx clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SMEN::Disabled)
    }
    ///I2Cx clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SMEN::Enabled)
    }
}
///Field `I2C2SMEN` reader - I2C2 clocks enable during Sleep and Stop modes
pub use I2C1SMEN_R as I2C2SMEN_R;
///Field `I2C3SMEN` reader - I2C3 clocks enable during Sleep and Stop modes
pub use I2C1SMEN_R as I2C3SMEN_R;
///Field `I2C2SMEN` writer - I2C2 clocks enable during Sleep and Stop modes
pub use I2C1SMEN_W as I2C2SMEN_W;
///Field `I2C3SMEN` writer - I2C3 clocks enable during Sleep and Stop modes
pub use I2C1SMEN_W as I2C3SMEN_W;
/**CRS clock enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSSMEN {
    ///0: CRS clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: CRS clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<CRSSMEN> for bool {
    #[inline(always)]
    fn from(variant: CRSSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CRSSMEN` reader - CRS clock enable during Sleep and Stop modes
pub type CRSSMEN_R = crate::BitReader<CRSSMEN>;
impl CRSSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRSSMEN {
        match self.bits {
            false => CRSSMEN::Disabled,
            true => CRSSMEN::Enabled,
        }
    }
    ///CRS clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRSSMEN::Disabled
    }
    ///CRS clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRSSMEN::Enabled
    }
}
///Field `CRSSMEN` writer - CRS clock enable during Sleep and Stop modes
pub type CRSSMEN_W<'a, REG> = crate::BitWriter<'a, REG, CRSSMEN>;
impl<'a, REG> CRSSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CRS clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRSSMEN::Disabled)
    }
    ///CRS clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRSSMEN::Enabled)
    }
}
/**CAN1 clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAN1SMEN {
    ///0: CAN1 clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: CAN1 clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<CAN1SMEN> for bool {
    #[inline(always)]
    fn from(variant: CAN1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CAN1SMEN` reader - CAN1 clocks enable during Sleep and Stop modes
pub type CAN1SMEN_R = crate::BitReader<CAN1SMEN>;
impl CAN1SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CAN1SMEN {
        match self.bits {
            false => CAN1SMEN::Disabled,
            true => CAN1SMEN::Enabled,
        }
    }
    ///CAN1 clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAN1SMEN::Disabled
    }
    ///CAN1 clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAN1SMEN::Enabled
    }
}
///Field `CAN1SMEN` writer - CAN1 clocks enable during Sleep and Stop modes
pub type CAN1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, CAN1SMEN>;
impl<'a, REG> CAN1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CAN1 clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CAN1SMEN::Disabled)
    }
    ///CAN1 clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CAN1SMEN::Enabled)
    }
}
/**Power interface clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWRSMEN {
    ///0: Power interface clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: Power interface clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<PWRSMEN> for bool {
    #[inline(always)]
    fn from(variant: PWRSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PWRSMEN` reader - Power interface clocks enable during Sleep and Stop modes
pub type PWRSMEN_R = crate::BitReader<PWRSMEN>;
impl PWRSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PWRSMEN {
        match self.bits {
            false => PWRSMEN::Disabled,
            true => PWRSMEN::Enabled,
        }
    }
    ///Power interface clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWRSMEN::Disabled
    }
    ///Power interface clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWRSMEN::Enabled
    }
}
///Field `PWRSMEN` writer - Power interface clocks enable during Sleep and Stop modes
pub type PWRSMEN_W<'a, REG> = crate::BitWriter<'a, REG, PWRSMEN>;
impl<'a, REG> PWRSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Power interface clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSMEN::Disabled)
    }
    ///Power interface clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSMEN::Enabled)
    }
}
/**DAC1 interface clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC1SMEN {
    ///0: DAC1 interface clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: DAC1 interface clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<DAC1SMEN> for bool {
    #[inline(always)]
    fn from(variant: DAC1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DAC1SMEN` reader - DAC1 interface clocks enable during Sleep and Stop modes
pub type DAC1SMEN_R = crate::BitReader<DAC1SMEN>;
impl DAC1SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DAC1SMEN {
        match self.bits {
            false => DAC1SMEN::Disabled,
            true => DAC1SMEN::Enabled,
        }
    }
    ///DAC1 interface clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DAC1SMEN::Disabled
    }
    ///DAC1 interface clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DAC1SMEN::Enabled
    }
}
///Field `DAC1SMEN` writer - DAC1 interface clocks enable during Sleep and Stop modes
pub type DAC1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, DAC1SMEN>;
impl<'a, REG> DAC1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DAC1 interface clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1SMEN::Disabled)
    }
    ///DAC1 interface clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1SMEN::Enabled)
    }
}
/**OPAMP interface clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAMPSMEN {
    ///0: OPAMP interface clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: OPAMP interface clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<OPAMPSMEN> for bool {
    #[inline(always)]
    fn from(variant: OPAMPSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `OPAMPSMEN` reader - OPAMP interface clocks enable during Sleep and Stop modes
pub type OPAMPSMEN_R = crate::BitReader<OPAMPSMEN>;
impl OPAMPSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OPAMPSMEN {
        match self.bits {
            false => OPAMPSMEN::Disabled,
            true => OPAMPSMEN::Enabled,
        }
    }
    ///OPAMP interface clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPAMPSMEN::Disabled
    }
    ///OPAMP interface clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPAMPSMEN::Enabled
    }
}
///Field `OPAMPSMEN` writer - OPAMP interface clocks enable during Sleep and Stop modes
pub type OPAMPSMEN_W<'a, REG> = crate::BitWriter<'a, REG, OPAMPSMEN>;
impl<'a, REG> OPAMPSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OPAMP interface clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMPSMEN::Disabled)
    }
    ///OPAMP interface clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMPSMEN::Enabled)
    }
}
/**Low power timer 1 clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM1SMEN {
    ///0: LPTIM1 clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: LPTIM1 clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<LPTIM1SMEN> for bool {
    #[inline(always)]
    fn from(variant: LPTIM1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LPTIM1SMEN` reader - Low power timer 1 clocks enable during Sleep and Stop modes
pub type LPTIM1SMEN_R = crate::BitReader<LPTIM1SMEN>;
impl LPTIM1SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM1SMEN {
        match self.bits {
            false => LPTIM1SMEN::Disabled,
            true => LPTIM1SMEN::Enabled,
        }
    }
    ///LPTIM1 clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPTIM1SMEN::Disabled
    }
    ///LPTIM1 clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPTIM1SMEN::Enabled
    }
}
///Field `LPTIM1SMEN` writer - Low power timer 1 clocks enable during Sleep and Stop modes
pub type LPTIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, LPTIM1SMEN>;
impl<'a, REG> LPTIM1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LPTIM1 clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SMEN::Disabled)
    }
    ///LPTIM1 clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SMEN::Enabled)
    }
}
impl R {
    ///Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim4smen(&self) -> TIM4SMEN_R {
        TIM4SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim5smen(&self) -> TIM5SMEN_R {
        TIM5SMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim6smen(&self) -> TIM6SMEN_R {
        TIM6SMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim7smen(&self) -> TIM7SMEN_R {
        TIM7SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 10 - RTC APB clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sp3smen(&self) -> SP3SMEN_R {
        SP3SMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - USART2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usart3smen(&self) -> USART3SMEN_R {
        USART3SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn uart4smen(&self) -> UART4SMEN_R {
        UART4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn uart5smen(&self) -> UART5SMEN_R {
        UART5SMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CRS clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn crssmen(&self) -> CRSSMEN_R {
        CRSSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - CAN1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn can1smen(&self) -> CAN1SMEN_R {
        CAN1SMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 28 - Power interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC1 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - OPAMP interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn opampsmen(&self) -> OPAMPSMEN_R {
        OPAMPSMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1SMENR1")
            .field("tim2smen", &self.tim2smen())
            .field("tim3smen", &self.tim3smen())
            .field("tim4smen", &self.tim4smen())
            .field("tim5smen", &self.tim5smen())
            .field("tim6smen", &self.tim6smen())
            .field("tim7smen", &self.tim7smen())
            .field("rtcapbsmen", &self.rtcapbsmen())
            .field("wwdgsmen", &self.wwdgsmen())
            .field("spi2smen", &self.spi2smen())
            .field("sp3smen", &self.sp3smen())
            .field("usart2smen", &self.usart2smen())
            .field("usart3smen", &self.usart3smen())
            .field("uart4smen", &self.uart4smen())
            .field("uart5smen", &self.uart5smen())
            .field("i2c1smen", &self.i2c1smen())
            .field("i2c2smen", &self.i2c2smen())
            .field("i2c3smen", &self.i2c3smen())
            .field("crssmen", &self.crssmen())
            .field("can1smen", &self.can1smen())
            .field("pwrsmen", &self.pwrsmen())
            .field("dac1smen", &self.dac1smen())
            .field("opampsmen", &self.opampsmen())
            .field("lptim1smen", &self.lptim1smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<'_, APB1SMENR1rs> {
        TIM2SMEN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<'_, APB1SMENR1rs> {
        TIM3SMEN_W::new(self, 1)
    }
    ///Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim4smen(&mut self) -> TIM4SMEN_W<'_, APB1SMENR1rs> {
        TIM4SMEN_W::new(self, 2)
    }
    ///Bit 3 - TIM5 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim5smen(&mut self) -> TIM5SMEN_W<'_, APB1SMENR1rs> {
        TIM5SMEN_W::new(self, 3)
    }
    ///Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W<'_, APB1SMENR1rs> {
        TIM6SMEN_W::new(self, 4)
    }
    ///Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W<'_, APB1SMENR1rs> {
        TIM7SMEN_W::new(self, 5)
    }
    ///Bit 10 - RTC APB clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<'_, APB1SMENR1rs> {
        RTCAPBSMEN_W::new(self, 10)
    }
    ///Bit 11 - Window watchdog clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<'_, APB1SMENR1rs> {
        WWDGSMEN_W::new(self, 11)
    }
    ///Bit 14 - SPI2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<'_, APB1SMENR1rs> {
        SPI2SMEN_W::new(self, 14)
    }
    ///Bit 15 - SPI3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sp3smen(&mut self) -> SP3SMEN_W<'_, APB1SMENR1rs> {
        SP3SMEN_W::new(self, 15)
    }
    ///Bit 17 - USART2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<'_, APB1SMENR1rs> {
        USART2SMEN_W::new(self, 17)
    }
    ///Bit 18 - USART3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usart3smen(&mut self) -> USART3SMEN_W<'_, APB1SMENR1rs> {
        USART3SMEN_W::new(self, 18)
    }
    ///Bit 19 - UART4 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn uart4smen(&mut self) -> UART4SMEN_W<'_, APB1SMENR1rs> {
        UART4SMEN_W::new(self, 19)
    }
    ///Bit 20 - UART5 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn uart5smen(&mut self) -> UART5SMEN_W<'_, APB1SMENR1rs> {
        UART5SMEN_W::new(self, 20)
    }
    ///Bit 21 - I2C1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<'_, APB1SMENR1rs> {
        I2C1SMEN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<'_, APB1SMENR1rs> {
        I2C2SMEN_W::new(self, 22)
    }
    ///Bit 23 - I2C3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<'_, APB1SMENR1rs> {
        I2C3SMEN_W::new(self, 23)
    }
    ///Bit 24 - CRS clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn crssmen(&mut self) -> CRSSMEN_W<'_, APB1SMENR1rs> {
        CRSSMEN_W::new(self, 24)
    }
    ///Bit 25 - CAN1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn can1smen(&mut self) -> CAN1SMEN_W<'_, APB1SMENR1rs> {
        CAN1SMEN_W::new(self, 25)
    }
    ///Bit 28 - Power interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<'_, APB1SMENR1rs> {
        PWRSMEN_W::new(self, 28)
    }
    ///Bit 29 - DAC1 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<'_, APB1SMENR1rs> {
        DAC1SMEN_W::new(self, 29)
    }
    ///Bit 30 - OPAMP interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn opampsmen(&mut self) -> OPAMPSMEN_W<'_, APB1SMENR1rs> {
        OPAMPSMEN_W::new(self, 30)
    }
    ///Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<'_, APB1SMENR1rs> {
        LPTIM1SMEN_W::new(self, 31)
    }
}
/**APB1SMENR1

You can [`read`](crate::Reg::read) this register and get [`apb1smenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:APB1SMENR1)*/
pub struct APB1SMENR1rs;
impl crate::RegisterSpec for APB1SMENR1rs {
    type Ux = u32;
}
///`read()` method returns [`apb1smenr1::R`](R) reader structure
impl crate::Readable for APB1SMENR1rs {}
///`write(|w| ..)` method takes [`apb1smenr1::W`](W) writer structure
impl crate::Writable for APB1SMENR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1SMENR1 to value 0xf3fe_cc3f
impl crate::Resettable for APB1SMENR1rs {
    const RESET_VALUE: u32 = 0xf3fe_cc3f;
}
