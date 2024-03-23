#[doc = "Register `APB2SMENR` reader"]
pub type R = crate::R<APB2SMENRrs>;
#[doc = "Register `APB2SMENR` writer"]
pub type W = crate::W<APB2SMENRrs>;
#[doc = "SYSCFG clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGSMEN {
    #[doc = "0: SYSCFG + COMP + VREFBUF clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: SYSCFG + COMP + VREFBUF clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<SYSCFGSMEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGSMEN` reader - SYSCFG clocks enable during Sleep and Stop modes"]
pub type SYSCFGSMEN_R = crate::BitReader<SYSCFGSMEN>;
impl SYSCFGSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGSMEN {
        match self.bits {
            false => SYSCFGSMEN::Disabled,
            true => SYSCFGSMEN::Enabled,
        }
    }
    #[doc = "SYSCFG + COMP + VREFBUF clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGSMEN::Disabled
    }
    #[doc = "SYSCFG + COMP + VREFBUF clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGSMEN::Enabled
    }
}
#[doc = "Field `SYSCFGSMEN` writer - SYSCFG clocks enable during Sleep and Stop modes"]
pub type SYSCFGSMEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGSMEN>;
impl<'a, REG> SYSCFGSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYSCFG + COMP + VREFBUF clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGSMEN::Disabled)
    }
    #[doc = "SYSCFG + COMP + VREFBUF clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGSMEN::Enabled)
    }
}
#[doc = "TIM1 timer clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1SMEN {
    #[doc = "0: TIMx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: TIMx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<TIM1SMEN> for bool {
    #[inline(always)]
    fn from(variant: TIM1SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1SMEN` reader - TIM1 timer clocks enable during Sleep and Stop modes"]
pub type TIM1SMEN_R = crate::BitReader<TIM1SMEN>;
impl TIM1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM1SMEN {
        match self.bits {
            false => TIM1SMEN::Disabled,
            true => TIM1SMEN::Enabled,
        }
    }
    #[doc = "TIMx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM1SMEN::Disabled
    }
    #[doc = "TIMx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM1SMEN::Enabled
    }
}
#[doc = "Field `TIM1SMEN` writer - TIM1 timer clocks enable during Sleep and Stop modes"]
pub type TIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM1SMEN>;
impl<'a, REG> TIM1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1SMEN::Disabled)
    }
    #[doc = "TIMx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1SMEN::Enabled)
    }
}
#[doc = "SPI1 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1SMEN {
    #[doc = "0: SPI1 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: SPI1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<SPI1SMEN> for bool {
    #[inline(always)]
    fn from(variant: SPI1SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1SMEN` reader - SPI1 clocks enable during Sleep and Stop modes"]
pub type SPI1SMEN_R = crate::BitReader<SPI1SMEN>;
impl SPI1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI1SMEN {
        match self.bits {
            false => SPI1SMEN::Disabled,
            true => SPI1SMEN::Enabled,
        }
    }
    #[doc = "SPI1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPI1SMEN::Disabled
    }
    #[doc = "SPI1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPI1SMEN::Enabled
    }
}
#[doc = "Field `SPI1SMEN` writer - SPI1 clocks enable during Sleep and Stop modes"]
pub type SPI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, SPI1SMEN>;
impl<'a, REG> SPI1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1SMEN::Disabled)
    }
    #[doc = "SPI1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1SMEN::Enabled)
    }
}
#[doc = "Field `TIM8SMEN` reader - TIM8 timer clocks enable during Sleep and Stop modes"]
pub use TIM1SMEN_R as TIM8SMEN_R;
#[doc = "Field `TIM8SMEN` writer - TIM8 timer clocks enable during Sleep and Stop modes"]
pub use TIM1SMEN_W as TIM8SMEN_W;
#[doc = "USART1clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1SMEN {
    #[doc = "0: USART1 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: USART1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<USART1SMEN> for bool {
    #[inline(always)]
    fn from(variant: USART1SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART1SMEN` reader - USART1clocks enable during Sleep and Stop modes"]
pub type USART1SMEN_R = crate::BitReader<USART1SMEN>;
impl USART1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART1SMEN {
        match self.bits {
            false => USART1SMEN::Disabled,
            true => USART1SMEN::Enabled,
        }
    }
    #[doc = "USART1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USART1SMEN::Disabled
    }
    #[doc = "USART1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USART1SMEN::Enabled
    }
}
#[doc = "Field `USART1SMEN` writer - USART1clocks enable during Sleep and Stop modes"]
pub type USART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, USART1SMEN>;
impl<'a, REG> USART1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SMEN::Disabled)
    }
    #[doc = "USART1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SMEN::Enabled)
    }
}
#[doc = "Field `TIM15SMEN` reader - TIM15 timer clocks enable during Sleep and Stop modes"]
pub use TIM1SMEN_R as TIM15SMEN_R;
#[doc = "Field `TIM16SMEN` reader - TIM16 timer clocks enable during Sleep and Stop modes"]
pub use TIM1SMEN_R as TIM16SMEN_R;
#[doc = "Field `TIM17SMEN` reader - TIM17 timer clocks enable during Sleep and Stop modes"]
pub use TIM1SMEN_R as TIM17SMEN_R;
#[doc = "Field `TIM15SMEN` writer - TIM15 timer clocks enable during Sleep and Stop modes"]
pub use TIM1SMEN_W as TIM15SMEN_W;
#[doc = "Field `TIM16SMEN` writer - TIM16 timer clocks enable during Sleep and Stop modes"]
pub use TIM1SMEN_W as TIM16SMEN_W;
#[doc = "Field `TIM17SMEN` writer - TIM17 timer clocks enable during Sleep and Stop modes"]
pub use TIM1SMEN_W as TIM17SMEN_W;
#[doc = "SAI1 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI1SMEN {
    #[doc = "0: SAIx clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: SAIx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<SAI1SMEN> for bool {
    #[inline(always)]
    fn from(variant: SAI1SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAI1SMEN` reader - SAI1 clocks enable during Sleep and Stop modes"]
pub type SAI1SMEN_R = crate::BitReader<SAI1SMEN>;
impl SAI1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SAI1SMEN {
        match self.bits {
            false => SAI1SMEN::Disabled,
            true => SAI1SMEN::Enabled,
        }
    }
    #[doc = "SAIx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAI1SMEN::Disabled
    }
    #[doc = "SAIx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAI1SMEN::Enabled
    }
}
#[doc = "Field `SAI1SMEN` writer - SAI1 clocks enable during Sleep and Stop modes"]
pub type SAI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, SAI1SMEN>;
impl<'a, REG> SAI1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SAIx clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SMEN::Disabled)
    }
    #[doc = "SAIx clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1SMEN::Enabled)
    }
}
#[doc = "Field `SAI2SMEN` reader - SAI2 clocks enable during Sleep and Stop modes"]
pub use SAI1SMEN_R as SAI2SMEN_R;
#[doc = "Field `SAI2SMEN` writer - SAI2 clocks enable during Sleep and Stop modes"]
pub use SAI1SMEN_W as SAI2SMEN_W;
#[doc = "DFSDM timer clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM1SMEN {
    #[doc = "0: DFSDM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: DFSDM1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<DFSDM1SMEN> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFSDM1SMEN` reader - DFSDM timer clocks enable during Sleep and Stop modes"]
pub type DFSDM1SMEN_R = crate::BitReader<DFSDM1SMEN>;
impl DFSDM1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DFSDM1SMEN {
        match self.bits {
            false => DFSDM1SMEN::Disabled,
            true => DFSDM1SMEN::Enabled,
        }
    }
    #[doc = "DFSDM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DFSDM1SMEN::Disabled
    }
    #[doc = "DFSDM1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DFSDM1SMEN::Enabled
    }
}
#[doc = "Field `DFSDM1SMEN` writer - DFSDM timer clocks enable during Sleep and Stop modes"]
pub type DFSDM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, DFSDM1SMEN>;
impl<'a, REG> DFSDM1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DFSDM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1SMEN::Disabled)
    }
    #[doc = "DFSDM1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1SMEN::Enabled)
    }
}
#[doc = "LCD-TFT timer clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCSMEN {
    #[doc = "0: LCD-TFT clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: LCD-TFT clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<LTDCSMEN> for bool {
    #[inline(always)]
    fn from(variant: LTDCSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LTDCSMEN` reader - LCD-TFT timer clocks enable during Sleep and Stop modes"]
pub type LTDCSMEN_R = crate::BitReader<LTDCSMEN>;
impl LTDCSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LTDCSMEN {
        match self.bits {
            false => LTDCSMEN::Disabled,
            true => LTDCSMEN::Enabled,
        }
    }
    #[doc = "LCD-TFT clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LTDCSMEN::Disabled
    }
    #[doc = "LCD-TFT clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LTDCSMEN::Enabled
    }
}
#[doc = "Field `LTDCSMEN` writer - LCD-TFT timer clocks enable during Sleep and Stop modes"]
pub type LTDCSMEN_W<'a, REG> = crate::BitWriter<'a, REG, LTDCSMEN>;
impl<'a, REG> LTDCSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD-TFT clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCSMEN::Disabled)
    }
    #[doc = "LCD-TFT clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCSMEN::Enabled)
    }
}
#[doc = "DSI clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSISMEN {
    #[doc = "0: DSI clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: DSI clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<DSISMEN> for bool {
    #[inline(always)]
    fn from(variant: DSISMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSISMEN` reader - DSI clocks enable during Sleep and Stop modes"]
pub type DSISMEN_R = crate::BitReader<DSISMEN>;
impl DSISMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DSISMEN {
        match self.bits {
            false => DSISMEN::Disabled,
            true => DSISMEN::Enabled,
        }
    }
    #[doc = "DSI clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DSISMEN::Disabled
    }
    #[doc = "DSI clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DSISMEN::Enabled
    }
}
#[doc = "Field `DSISMEN` writer - DSI clocks enable during Sleep and Stop modes"]
pub type DSISMEN_W<'a, REG> = crate::BitWriter<'a, REG, DSISMEN>;
impl<'a, REG> DSISMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DSI clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DSISMEN::Disabled)
    }
    #[doc = "DSI clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DSISMEN::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - SYSCFG clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim8smen(&self) -> TIM8SMEN_R {
        TIM8SMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim15smen(&self) -> TIM15SMEN_R {
        TIM15SMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sai1smen(&self) -> SAI1SMEN_R {
        SAI1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sai2smen(&self) -> SAI2SMEN_R {
        SAI2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - DFSDM timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dfsdm1smen(&self) -> DFSDM1SMEN_R {
        DFSDM1SMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - LCD-TFT timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn ltdcsmen(&self) -> LTDCSMEN_R {
        LTDCSMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DSI clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dsismen(&self) -> DSISMEN_R {
        DSISMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W<APB2SMENRrs> {
        SYSCFGSMEN_W::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W<APB2SMENRrs> {
        TIM1SMEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<APB2SMENRrs> {
        SPI1SMEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim8smen(&mut self) -> TIM8SMEN_W<APB2SMENRrs> {
        TIM8SMEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - USART1clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<APB2SMENRrs> {
        USART1SMEN_W::new(self, 14)
    }
    #[doc = "Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim15smen(&mut self) -> TIM15SMEN_W<APB2SMENRrs> {
        TIM15SMEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W<APB2SMENRrs> {
        TIM16SMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W<APB2SMENRrs> {
        TIM17SMEN_W::new(self, 18)
    }
    #[doc = "Bit 21 - SAI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sai1smen(&mut self) -> SAI1SMEN_W<APB2SMENRrs> {
        SAI1SMEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - SAI2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sai2smen(&mut self) -> SAI2SMEN_W<APB2SMENRrs> {
        SAI2SMEN_W::new(self, 22)
    }
    #[doc = "Bit 24 - DFSDM timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1smen(&mut self) -> DFSDM1SMEN_W<APB2SMENRrs> {
        DFSDM1SMEN_W::new(self, 24)
    }
    #[doc = "Bit 26 - LCD-TFT timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcsmen(&mut self) -> LTDCSMEN_W<APB2SMENRrs> {
        LTDCSMEN_W::new(self, 26)
    }
    #[doc = "Bit 27 - DSI clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dsismen(&mut self) -> DSISMEN_W<APB2SMENRrs> {
        DSISMEN_W::new(self, 27)
    }
}
#[doc = "APB2SMENR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2SMENRrs;
impl crate::RegisterSpec for APB2SMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2smenr::R`](R) reader structure"]
impl crate::Readable for APB2SMENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb2smenr::W`](W) writer structure"]
impl crate::Writable for APB2SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2SMENR to value 0x0d67_7801"]
impl crate::Resettable for APB2SMENRrs {
    const RESET_VALUE: u32 = 0x0d67_7801;
}
