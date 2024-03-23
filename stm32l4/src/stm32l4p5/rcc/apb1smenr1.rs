#[doc = "Register `APB1SMENR1` reader"]
pub type R = crate::R<APB1SMENR1rs>;
#[doc = "Register `APB1SMENR1` writer"]
pub type W = crate::W<APB1SMENR1rs>;
#[doc = "TIM2 timer clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2SMEN {
    #[doc = "0: TIMx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: TIMx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<TIM2SMEN> for bool {
    #[inline(always)]
    fn from(variant: TIM2SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2SMEN` reader - TIM2 timer clocks enable during Sleep and Stop modes"]
pub type TIM2SMEN_R = crate::BitReader<TIM2SMEN>;
impl TIM2SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM2SMEN {
        match self.bits {
            false => TIM2SMEN::Disabled,
            true => TIM2SMEN::Enabled,
        }
    }
    #[doc = "TIMx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2SMEN::Disabled
    }
    #[doc = "TIMx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2SMEN::Enabled
    }
}
#[doc = "Field `TIM2SMEN` writer - TIM2 timer clocks enable during Sleep and Stop modes"]
pub type TIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2SMEN>;
impl<'a, REG> TIM2SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2SMEN::Disabled)
    }
    #[doc = "TIMx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2SMEN::Enabled)
    }
}
#[doc = "Field `TIM3SMEN` reader - TIM3 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_R as TIM3SMEN_R;
#[doc = "Field `TIM4SMEN` reader - TIM4 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_R as TIM4SMEN_R;
#[doc = "Field `TIM5SMEN` reader - TIM5 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_R as TIM5SMEN_R;
#[doc = "Field `TIM6SMEN` reader - TIM6 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_R as TIM6SMEN_R;
#[doc = "Field `TIM7SMEN` reader - TIM7 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_R as TIM7SMEN_R;
#[doc = "Field `TIM3SMEN` writer - TIM3 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_W as TIM3SMEN_W;
#[doc = "Field `TIM4SMEN` writer - TIM4 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_W as TIM4SMEN_W;
#[doc = "Field `TIM5SMEN` writer - TIM5 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_W as TIM5SMEN_W;
#[doc = "Field `TIM6SMEN` writer - TIM6 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_W as TIM6SMEN_W;
#[doc = "Field `TIM7SMEN` writer - TIM7 timer clocks enable during Sleep and Stop modes"]
pub use TIM2SMEN_W as TIM7SMEN_W;
#[doc = "RTC APB clock enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCAPBSMEN {
    #[doc = "0: RTC APB clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: RTC APB clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<RTCAPBSMEN> for bool {
    #[inline(always)]
    fn from(variant: RTCAPBSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep and Stop modes"]
pub type RTCAPBSMEN_R = crate::BitReader<RTCAPBSMEN>;
impl RTCAPBSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCAPBSMEN {
        match self.bits {
            false => RTCAPBSMEN::Disabled,
            true => RTCAPBSMEN::Enabled,
        }
    }
    #[doc = "RTC APB clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCAPBSMEN::Disabled
    }
    #[doc = "RTC APB clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCAPBSMEN::Enabled
    }
}
#[doc = "Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep and Stop modes"]
pub type RTCAPBSMEN_W<'a, REG> = crate::BitWriter<'a, REG, RTCAPBSMEN>;
impl<'a, REG> RTCAPBSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC APB clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCAPBSMEN::Disabled)
    }
    #[doc = "RTC APB clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCAPBSMEN::Enabled)
    }
}
#[doc = "Window watchdog clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGSMEN {
    #[doc = "0: Window watchdog clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: Window watchdog clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<WWDGSMEN> for bool {
    #[inline(always)]
    fn from(variant: WWDGSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDGSMEN` reader - Window watchdog clocks enable during Sleep and Stop modes"]
pub type WWDGSMEN_R = crate::BitReader<WWDGSMEN>;
impl WWDGSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WWDGSMEN {
        match self.bits {
            false => WWDGSMEN::Disabled,
            true => WWDGSMEN::Enabled,
        }
    }
    #[doc = "Window watchdog clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WWDGSMEN::Disabled
    }
    #[doc = "Window watchdog clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WWDGSMEN::Enabled
    }
}
#[doc = "Field `WWDGSMEN` writer - Window watchdog clocks enable during Sleep and Stop modes"]
pub type WWDGSMEN_W<'a, REG> = crate::BitWriter<'a, REG, WWDGSMEN>;
impl<'a, REG> WWDGSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Window watchdog clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WWDGSMEN::Disabled)
    }
    #[doc = "Window watchdog clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WWDGSMEN::Enabled)
    }
}
#[doc = "SPI2 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI2SMEN {
    #[doc = "0: SPIx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: SPIx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<SPI2SMEN> for bool {
    #[inline(always)]
    fn from(variant: SPI2SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI2SMEN` reader - SPI2 clocks enable during Sleep and Stop modes"]
pub type SPI2SMEN_R = crate::BitReader<SPI2SMEN>;
impl SPI2SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI2SMEN {
        match self.bits {
            false => SPI2SMEN::Disabled,
            true => SPI2SMEN::Enabled,
        }
    }
    #[doc = "SPIx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPI2SMEN::Disabled
    }
    #[doc = "SPIx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPI2SMEN::Enabled
    }
}
#[doc = "Field `SPI2SMEN` writer - SPI2 clocks enable during Sleep and Stop modes"]
pub type SPI2SMEN_W<'a, REG> = crate::BitWriter<'a, REG, SPI2SMEN>;
impl<'a, REG> SPI2SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPIx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2SMEN::Disabled)
    }
    #[doc = "SPIx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2SMEN::Enabled)
    }
}
#[doc = "Field `SP3SMEN` reader - SPI3 clocks enable during Sleep and Stop modes"]
pub type SP3SMEN_R = crate::BitReader;
#[doc = "Field `SP3SMEN` writer - SPI3 clocks enable during Sleep and Stop modes"]
pub type SP3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USART2 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART2SMEN {
    #[doc = "0: USARTx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: USARTx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<USART2SMEN> for bool {
    #[inline(always)]
    fn from(variant: USART2SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART2SMEN` reader - USART2 clocks enable during Sleep and Stop modes"]
pub type USART2SMEN_R = crate::BitReader<USART2SMEN>;
impl USART2SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART2SMEN {
        match self.bits {
            false => USART2SMEN::Disabled,
            true => USART2SMEN::Enabled,
        }
    }
    #[doc = "USARTx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USART2SMEN::Disabled
    }
    #[doc = "USARTx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USART2SMEN::Enabled
    }
}
#[doc = "Field `USART2SMEN` writer - USART2 clocks enable during Sleep and Stop modes"]
pub type USART2SMEN_W<'a, REG> = crate::BitWriter<'a, REG, USART2SMEN>;
impl<'a, REG> USART2SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USARTx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SMEN::Disabled)
    }
    #[doc = "USARTx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SMEN::Enabled)
    }
}
#[doc = "Field `USART3SMEN` reader - USART3 clocks enable during Sleep and Stop modes"]
pub use USART2SMEN_R as USART3SMEN_R;
#[doc = "Field `USART3SMEN` writer - USART3 clocks enable during Sleep and Stop modes"]
pub use USART2SMEN_W as USART3SMEN_W;
#[doc = "UART4 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART4SMEN {
    #[doc = "0: UARTx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: UARTx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<UART4SMEN> for bool {
    #[inline(always)]
    fn from(variant: UART4SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART4SMEN` reader - UART4 clocks enable during Sleep and Stop modes"]
pub type UART4SMEN_R = crate::BitReader<UART4SMEN>;
impl UART4SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UART4SMEN {
        match self.bits {
            false => UART4SMEN::Disabled,
            true => UART4SMEN::Enabled,
        }
    }
    #[doc = "UARTx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UART4SMEN::Disabled
    }
    #[doc = "UARTx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UART4SMEN::Enabled
    }
}
#[doc = "Field `UART4SMEN` writer - UART4 clocks enable during Sleep and Stop modes"]
pub type UART4SMEN_W<'a, REG> = crate::BitWriter<'a, REG, UART4SMEN>;
impl<'a, REG> UART4SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UARTx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UART4SMEN::Disabled)
    }
    #[doc = "UARTx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UART4SMEN::Enabled)
    }
}
#[doc = "Field `UART5SMEN` reader - UART5 clocks enable during Sleep and Stop modes"]
pub use UART4SMEN_R as UART5SMEN_R;
#[doc = "Field `UART5SMEN` writer - UART5 clocks enable during Sleep and Stop modes"]
pub use UART4SMEN_W as UART5SMEN_W;
#[doc = "I2C1 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1SMEN {
    #[doc = "0: I2Cx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: I2Cx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<I2C1SMEN> for bool {
    #[inline(always)]
    fn from(variant: I2C1SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1SMEN` reader - I2C1 clocks enable during Sleep and Stop modes"]
pub type I2C1SMEN_R = crate::BitReader<I2C1SMEN>;
impl I2C1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C1SMEN {
        match self.bits {
            false => I2C1SMEN::Disabled,
            true => I2C1SMEN::Enabled,
        }
    }
    #[doc = "I2Cx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C1SMEN::Disabled
    }
    #[doc = "I2Cx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C1SMEN::Enabled
    }
}
#[doc = "Field `I2C1SMEN` writer - I2C1 clocks enable during Sleep and Stop modes"]
pub type I2C1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, I2C1SMEN>;
impl<'a, REG> I2C1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2Cx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SMEN::Disabled)
    }
    #[doc = "I2Cx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SMEN::Enabled)
    }
}
#[doc = "Field `I2C2SMEN` reader - I2C2 clocks enable during Sleep and Stop modes"]
pub use I2C1SMEN_R as I2C2SMEN_R;
#[doc = "Field `I2C3SMEN` reader - I2C3 clocks enable during Sleep and Stop modes"]
pub use I2C1SMEN_R as I2C3SMEN_R;
#[doc = "Field `I2C2SMEN` writer - I2C2 clocks enable during Sleep and Stop modes"]
pub use I2C1SMEN_W as I2C2SMEN_W;
#[doc = "Field `I2C3SMEN` writer - I2C3 clocks enable during Sleep and Stop modes"]
pub use I2C1SMEN_W as I2C3SMEN_W;
#[doc = "CRS clock enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSSMEN {
    #[doc = "0: CRS clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: CRS clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<CRSSMEN> for bool {
    #[inline(always)]
    fn from(variant: CRSSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSSMEN` reader - CRS clock enable during Sleep and Stop modes"]
pub type CRSSMEN_R = crate::BitReader<CRSSMEN>;
impl CRSSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRSSMEN {
        match self.bits {
            false => CRSSMEN::Disabled,
            true => CRSSMEN::Enabled,
        }
    }
    #[doc = "CRS clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRSSMEN::Disabled
    }
    #[doc = "CRS clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRSSMEN::Enabled
    }
}
#[doc = "Field `CRSSMEN` writer - CRS clock enable during Sleep and Stop modes"]
pub type CRSSMEN_W<'a, REG> = crate::BitWriter<'a, REG, CRSSMEN>;
impl<'a, REG> CRSSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRS clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRSSMEN::Disabled)
    }
    #[doc = "CRS clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRSSMEN::Enabled)
    }
}
#[doc = "CAN1 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAN1SMEN {
    #[doc = "0: CAN1 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: CAN1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<CAN1SMEN> for bool {
    #[inline(always)]
    fn from(variant: CAN1SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAN1SMEN` reader - CAN1 clocks enable during Sleep and Stop modes"]
pub type CAN1SMEN_R = crate::BitReader<CAN1SMEN>;
impl CAN1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAN1SMEN {
        match self.bits {
            false => CAN1SMEN::Disabled,
            true => CAN1SMEN::Enabled,
        }
    }
    #[doc = "CAN1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAN1SMEN::Disabled
    }
    #[doc = "CAN1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAN1SMEN::Enabled
    }
}
#[doc = "Field `CAN1SMEN` writer - CAN1 clocks enable during Sleep and Stop modes"]
pub type CAN1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, CAN1SMEN>;
impl<'a, REG> CAN1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CAN1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CAN1SMEN::Disabled)
    }
    #[doc = "CAN1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CAN1SMEN::Enabled)
    }
}
#[doc = "Power interface clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWRSMEN {
    #[doc = "0: Power interface clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: Power interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<PWRSMEN> for bool {
    #[inline(always)]
    fn from(variant: PWRSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSMEN` reader - Power interface clocks enable during Sleep and Stop modes"]
pub type PWRSMEN_R = crate::BitReader<PWRSMEN>;
impl PWRSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWRSMEN {
        match self.bits {
            false => PWRSMEN::Disabled,
            true => PWRSMEN::Enabled,
        }
    }
    #[doc = "Power interface clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWRSMEN::Disabled
    }
    #[doc = "Power interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWRSMEN::Enabled
    }
}
#[doc = "Field `PWRSMEN` writer - Power interface clocks enable during Sleep and Stop modes"]
pub type PWRSMEN_W<'a, REG> = crate::BitWriter<'a, REG, PWRSMEN>;
impl<'a, REG> PWRSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power interface clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSMEN::Disabled)
    }
    #[doc = "Power interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSMEN::Enabled)
    }
}
#[doc = "DAC1 interface clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC1SMEN {
    #[doc = "0: DAC1 interface clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: DAC1 interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<DAC1SMEN> for bool {
    #[inline(always)]
    fn from(variant: DAC1SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC1SMEN` reader - DAC1 interface clocks enable during Sleep and Stop modes"]
pub type DAC1SMEN_R = crate::BitReader<DAC1SMEN>;
impl DAC1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC1SMEN {
        match self.bits {
            false => DAC1SMEN::Disabled,
            true => DAC1SMEN::Enabled,
        }
    }
    #[doc = "DAC1 interface clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DAC1SMEN::Disabled
    }
    #[doc = "DAC1 interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DAC1SMEN::Enabled
    }
}
#[doc = "Field `DAC1SMEN` writer - DAC1 interface clocks enable during Sleep and Stop modes"]
pub type DAC1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, DAC1SMEN>;
impl<'a, REG> DAC1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC1 interface clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1SMEN::Disabled)
    }
    #[doc = "DAC1 interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1SMEN::Enabled)
    }
}
#[doc = "OPAMP interface clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAMPSMEN {
    #[doc = "0: OPAMP interface clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: OPAMP interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<OPAMPSMEN> for bool {
    #[inline(always)]
    fn from(variant: OPAMPSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPAMPSMEN` reader - OPAMP interface clocks enable during Sleep and Stop modes"]
pub type OPAMPSMEN_R = crate::BitReader<OPAMPSMEN>;
impl OPAMPSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPAMPSMEN {
        match self.bits {
            false => OPAMPSMEN::Disabled,
            true => OPAMPSMEN::Enabled,
        }
    }
    #[doc = "OPAMP interface clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPAMPSMEN::Disabled
    }
    #[doc = "OPAMP interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPAMPSMEN::Enabled
    }
}
#[doc = "Field `OPAMPSMEN` writer - OPAMP interface clocks enable during Sleep and Stop modes"]
pub type OPAMPSMEN_W<'a, REG> = crate::BitWriter<'a, REG, OPAMPSMEN>;
impl<'a, REG> OPAMPSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OPAMP interface clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMPSMEN::Disabled)
    }
    #[doc = "OPAMP interface clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPAMPSMEN::Enabled)
    }
}
#[doc = "Low power timer 1 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM1SMEN {
    #[doc = "0: LPTIM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: LPTIM1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<LPTIM1SMEN> for bool {
    #[inline(always)]
    fn from(variant: LPTIM1SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM1SMEN` reader - Low power timer 1 clocks enable during Sleep and Stop modes"]
pub type LPTIM1SMEN_R = crate::BitReader<LPTIM1SMEN>;
impl LPTIM1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM1SMEN {
        match self.bits {
            false => LPTIM1SMEN::Disabled,
            true => LPTIM1SMEN::Enabled,
        }
    }
    #[doc = "LPTIM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPTIM1SMEN::Disabled
    }
    #[doc = "LPTIM1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPTIM1SMEN::Enabled
    }
}
#[doc = "Field `LPTIM1SMEN` writer - Low power timer 1 clocks enable during Sleep and Stop modes"]
pub type LPTIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, LPTIM1SMEN>;
impl<'a, REG> LPTIM1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPTIM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SMEN::Disabled)
    }
    #[doc = "LPTIM1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SMEN::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim4smen(&self) -> TIM4SMEN_R {
        TIM4SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim5smen(&self) -> TIM5SMEN_R {
        TIM5SMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim6smen(&self) -> TIM6SMEN_R {
        TIM6SMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim7smen(&self) -> TIM7SMEN_R {
        TIM7SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sp3smen(&self) -> SP3SMEN_R {
        SP3SMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn usart3smen(&self) -> USART3SMEN_R {
        USART3SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn uart4smen(&self) -> UART4SMEN_R {
        UART4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn uart5smen(&self) -> UART5SMEN_R {
        UART5SMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRS clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn crssmen(&self) -> CRSSMEN_R {
        CRSSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn can1smen(&self) -> CAN1SMEN_R {
        CAN1SMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPAMP interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn opampsmen(&self) -> OPAMPSMEN_R {
        OPAMPSMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<APB1SMENR1rs> {
        TIM2SMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<APB1SMENR1rs> {
        TIM3SMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim4smen(&mut self) -> TIM4SMEN_W<APB1SMENR1rs> {
        TIM4SMEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim5smen(&mut self) -> TIM5SMEN_W<APB1SMENR1rs> {
        TIM5SMEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W<APB1SMENR1rs> {
        TIM6SMEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W<APB1SMENR1rs> {
        TIM7SMEN_W::new(self, 5)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<APB1SMENR1rs> {
        RTCAPBSMEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Window watchdog clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<APB1SMENR1rs> {
        WWDGSMEN_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<APB1SMENR1rs> {
        SPI2SMEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sp3smen(&mut self) -> SP3SMEN_W<APB1SMENR1rs> {
        SP3SMEN_W::new(self, 15)
    }
    #[doc = "Bit 17 - USART2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<APB1SMENR1rs> {
        USART2SMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn usart3smen(&mut self) -> USART3SMEN_W<APB1SMENR1rs> {
        USART3SMEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - UART4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn uart4smen(&mut self) -> UART4SMEN_W<APB1SMENR1rs> {
        UART4SMEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - UART5 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn uart5smen(&mut self) -> UART5SMEN_W<APB1SMENR1rs> {
        UART5SMEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<APB1SMENR1rs> {
        I2C1SMEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<APB1SMENR1rs> {
        I2C2SMEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<APB1SMENR1rs> {
        I2C3SMEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - CRS clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn crssmen(&mut self) -> CRSSMEN_W<APB1SMENR1rs> {
        CRSSMEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - CAN1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn can1smen(&mut self) -> CAN1SMEN_W<APB1SMENR1rs> {
        CAN1SMEN_W::new(self, 25)
    }
    #[doc = "Bit 28 - Power interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<APB1SMENR1rs> {
        PWRSMEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<APB1SMENR1rs> {
        DAC1SMEN_W::new(self, 29)
    }
    #[doc = "Bit 30 - OPAMP interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn opampsmen(&mut self) -> OPAMPSMEN_W<APB1SMENR1rs> {
        OPAMPSMEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<APB1SMENR1rs> {
        LPTIM1SMEN_W::new(self, 31)
    }
}
#[doc = "APB1SMENR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1smenr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1smenr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1SMENR1rs;
impl crate::RegisterSpec for APB1SMENR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1smenr1::R`](R) reader structure"]
impl crate::Readable for APB1SMENR1rs {}
#[doc = "`write(|w| ..)` method takes [`apb1smenr1::W`](W) writer structure"]
impl crate::Writable for APB1SMENR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1SMENR1 to value 0xf3fe_cc3f"]
impl crate::Resettable for APB1SMENR1rs {
    const RESET_VALUE: u32 = 0xf3fe_cc3f;
}
