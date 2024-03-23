#[doc = "Register `APB2ENR` reader"]
pub type R = crate::R<APB2ENRrs>;
#[doc = "Register `APB2ENR` writer"]
pub type W = crate::W<APB2ENRrs>;
#[doc = "SYSCFG clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGEN {
    #[doc = "0: SYSCFG + COMP + VREFBUF clock disabled"]
    Disabled = 0,
    #[doc = "1: SYSCFG + COMP + VREFBUF clock enabled"]
    Enabled = 1,
}
impl From<SYSCFGEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGEN` reader - SYSCFG clock enable"]
pub type SYSCFGEN_R = crate::BitReader<SYSCFGEN>;
impl SYSCFGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGEN {
        match self.bits {
            false => SYSCFGEN::Disabled,
            true => SYSCFGEN::Enabled,
        }
    }
    #[doc = "SYSCFG + COMP + VREFBUF clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGEN::Disabled
    }
    #[doc = "SYSCFG + COMP + VREFBUF clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGEN::Enabled
    }
}
#[doc = "Field `SYSCFGEN` writer - SYSCFG clock enable"]
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGEN>;
impl<'a, REG> SYSCFGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYSCFG + COMP + VREFBUF clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::Disabled)
    }
    #[doc = "SYSCFG + COMP + VREFBUF clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::Enabled)
    }
}
#[doc = "Firewall clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWEN {
    #[doc = "0: Firewall clock disabled"]
    Disabled = 0,
    #[doc = "1: Firewall clock enabled"]
    Enabled = 1,
}
impl From<FWEN> for bool {
    #[inline(always)]
    fn from(variant: FWEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWEN` reader - Firewall clock enable"]
pub type FWEN_R = crate::BitReader<FWEN>;
impl FWEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FWEN {
        match self.bits {
            false => FWEN::Disabled,
            true => FWEN::Enabled,
        }
    }
    #[doc = "Firewall clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FWEN::Disabled
    }
    #[doc = "Firewall clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FWEN::Enabled
    }
}
#[doc = "Field `FWEN` writer - Firewall clock enable"]
pub type FWEN_W<'a, REG> = crate::BitWriter<'a, REG, FWEN>;
impl<'a, REG> FWEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Firewall clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FWEN::Disabled)
    }
    #[doc = "Firewall clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FWEN::Enabled)
    }
}
#[doc = "TIM1 timer clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1EN {
    #[doc = "0: TIMx clock disabled"]
    Disabled = 0,
    #[doc = "1: TIMx clock enabled"]
    Enabled = 1,
}
impl From<TIM1EN> for bool {
    #[inline(always)]
    fn from(variant: TIM1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1EN` reader - TIM1 timer clock enable"]
pub type TIM1EN_R = crate::BitReader<TIM1EN>;
impl TIM1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM1EN {
        match self.bits {
            false => TIM1EN::Disabled,
            true => TIM1EN::Enabled,
        }
    }
    #[doc = "TIMx clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM1EN::Disabled
    }
    #[doc = "TIMx clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM1EN::Enabled
    }
}
#[doc = "Field `TIM1EN` writer - TIM1 timer clock enable"]
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM1EN>;
impl<'a, REG> TIM1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMx clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1EN::Disabled)
    }
    #[doc = "TIMx clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1EN::Enabled)
    }
}
#[doc = "SPI1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1EN {
    #[doc = "0: SPI1 clock disabled"]
    Disabled = 0,
    #[doc = "1: SPI1 clock enabled"]
    Enabled = 1,
}
impl From<SPI1EN> for bool {
    #[inline(always)]
    fn from(variant: SPI1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub type SPI1EN_R = crate::BitReader<SPI1EN>;
impl SPI1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI1EN {
        match self.bits {
            false => SPI1EN::Disabled,
            true => SPI1EN::Enabled,
        }
    }
    #[doc = "SPI1 clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPI1EN::Disabled
    }
    #[doc = "SPI1 clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPI1EN::Enabled
    }
}
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub type SPI1EN_W<'a, REG> = crate::BitWriter<'a, REG, SPI1EN>;
impl<'a, REG> SPI1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI1 clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1EN::Disabled)
    }
    #[doc = "SPI1 clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1EN::Enabled)
    }
}
#[doc = "Field `TIM8EN` reader - TIM8 timer clock enable"]
pub use TIM1EN_R as TIM8EN_R;
#[doc = "Field `TIM8EN` writer - TIM8 timer clock enable"]
pub use TIM1EN_W as TIM8EN_W;
#[doc = "USART1clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1EN {
    #[doc = "0: USART1 clock disabled"]
    Disabled = 0,
    #[doc = "1: USART1 clock enabled"]
    Enabled = 1,
}
impl From<USART1EN> for bool {
    #[inline(always)]
    fn from(variant: USART1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART1EN` reader - USART1clock enable"]
pub type USART1EN_R = crate::BitReader<USART1EN>;
impl USART1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART1EN {
        match self.bits {
            false => USART1EN::Disabled,
            true => USART1EN::Enabled,
        }
    }
    #[doc = "USART1 clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USART1EN::Disabled
    }
    #[doc = "USART1 clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USART1EN::Enabled
    }
}
#[doc = "Field `USART1EN` writer - USART1clock enable"]
pub type USART1EN_W<'a, REG> = crate::BitWriter<'a, REG, USART1EN>;
impl<'a, REG> USART1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART1 clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(USART1EN::Disabled)
    }
    #[doc = "USART1 clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(USART1EN::Enabled)
    }
}
#[doc = "Field `TIM15EN` reader - TIM15 timer clock enable"]
pub use TIM1EN_R as TIM15EN_R;
#[doc = "Field `TIM16EN` reader - TIM16 timer clock enable"]
pub use TIM1EN_R as TIM16EN_R;
#[doc = "Field `TIM17EN` reader - TIM17 timer clock enable"]
pub use TIM1EN_R as TIM17EN_R;
#[doc = "Field `TIM15EN` writer - TIM15 timer clock enable"]
pub use TIM1EN_W as TIM15EN_W;
#[doc = "Field `TIM16EN` writer - TIM16 timer clock enable"]
pub use TIM1EN_W as TIM16EN_W;
#[doc = "Field `TIM17EN` writer - TIM17 timer clock enable"]
pub use TIM1EN_W as TIM17EN_W;
#[doc = "SAI1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI1EN {
    #[doc = "0: SAIx clock disabled"]
    Disabled = 0,
    #[doc = "1: SAIx clock enabled"]
    Enabled = 1,
}
impl From<SAI1EN> for bool {
    #[inline(always)]
    fn from(variant: SAI1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAI1EN` reader - SAI1 clock enable"]
pub type SAI1EN_R = crate::BitReader<SAI1EN>;
impl SAI1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SAI1EN {
        match self.bits {
            false => SAI1EN::Disabled,
            true => SAI1EN::Enabled,
        }
    }
    #[doc = "SAIx clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAI1EN::Disabled
    }
    #[doc = "SAIx clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAI1EN::Enabled
    }
}
#[doc = "Field `SAI1EN` writer - SAI1 clock enable"]
pub type SAI1EN_W<'a, REG> = crate::BitWriter<'a, REG, SAI1EN>;
impl<'a, REG> SAI1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SAIx clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1EN::Disabled)
    }
    #[doc = "SAIx clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1EN::Enabled)
    }
}
#[doc = "Field `SAI2EN` reader - SAI2 clock enable"]
pub use SAI1EN_R as SAI2EN_R;
#[doc = "Field `SAI2EN` writer - SAI2 clock enable"]
pub use SAI1EN_W as SAI2EN_W;
#[doc = "DFSDM timer clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM1EN {
    #[doc = "0: DFSDM1 clock disabled"]
    Disabled = 0,
    #[doc = "1: DFSDM1 clock enabled"]
    Enabled = 1,
}
impl From<DFSDM1EN> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFSDM1EN` reader - DFSDM timer clock enable"]
pub type DFSDM1EN_R = crate::BitReader<DFSDM1EN>;
impl DFSDM1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DFSDM1EN {
        match self.bits {
            false => DFSDM1EN::Disabled,
            true => DFSDM1EN::Enabled,
        }
    }
    #[doc = "DFSDM1 clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DFSDM1EN::Disabled
    }
    #[doc = "DFSDM1 clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DFSDM1EN::Enabled
    }
}
#[doc = "Field `DFSDM1EN` writer - DFSDM timer clock enable"]
pub type DFSDM1EN_W<'a, REG> = crate::BitWriter<'a, REG, DFSDM1EN>;
impl<'a, REG> DFSDM1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DFSDM1 clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1EN::Disabled)
    }
    #[doc = "DFSDM1 clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1EN::Enabled)
    }
}
#[doc = "LCD-TFT clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCEN {
    #[doc = "0: LTDC clock disabled"]
    Disabled = 0,
    #[doc = "1: LTDC clock enabled"]
    Enabled = 1,
}
impl From<LTDCEN> for bool {
    #[inline(always)]
    fn from(variant: LTDCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LTDCEN` reader - LCD-TFT clock enable"]
pub type LTDCEN_R = crate::BitReader<LTDCEN>;
impl LTDCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LTDCEN {
        match self.bits {
            false => LTDCEN::Disabled,
            true => LTDCEN::Enabled,
        }
    }
    #[doc = "LTDC clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LTDCEN::Disabled
    }
    #[doc = "LTDC clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LTDCEN::Enabled
    }
}
#[doc = "Field `LTDCEN` writer - LCD-TFT clock enable"]
pub type LTDCEN_W<'a, REG> = crate::BitWriter<'a, REG, LTDCEN>;
impl<'a, REG> LTDCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LTDC clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCEN::Disabled)
    }
    #[doc = "LTDC clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCEN::Enabled)
    }
}
#[doc = "DSI clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSIEN {
    #[doc = "0: DSI clock disabled"]
    Disabled = 0,
    #[doc = "1: DSI clock enabled"]
    Enabled = 1,
}
impl From<DSIEN> for bool {
    #[inline(always)]
    fn from(variant: DSIEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSIEN` reader - DSI clock enable"]
pub type DSIEN_R = crate::BitReader<DSIEN>;
impl DSIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DSIEN {
        match self.bits {
            false => DSIEN::Disabled,
            true => DSIEN::Enabled,
        }
    }
    #[doc = "DSI clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DSIEN::Disabled
    }
    #[doc = "DSI clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DSIEN::Enabled
    }
}
#[doc = "Field `DSIEN` writer - DSI clock enable"]
pub type DSIEN_W<'a, REG> = crate::BitWriter<'a, REG, DSIEN>;
impl<'a, REG> DSIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DSI clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DSIEN::Disabled)
    }
    #[doc = "DSI clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DSIEN::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - SYSCFG clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Firewall clock enable"]
    #[inline(always)]
    pub fn fwen(&self) -> FWEN_R {
        FWEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer clock enable"]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable"]
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 clock enable"]
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI2 clock enable"]
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - DFSDM timer clock enable"]
    #[inline(always)]
    pub fn dfsdm1en(&self) -> DFSDM1EN_R {
        DFSDM1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - LCD-TFT clock enable"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DSI clock enable"]
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<APB2ENRrs> {
        SYSCFGEN_W::new(self, 0)
    }
    #[doc = "Bit 7 - Firewall clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn fwen(&mut self) -> FWEN_W<APB2ENRrs> {
        FWEN_W::new(self, 7)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<APB2ENRrs> {
        TIM1EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - TIM8 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim8en(&mut self) -> TIM8EN_W<APB2ENRrs> {
        TIM8EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - USART1clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<APB2ENRrs> {
        USART1EN_W::new(self, 14)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim15en(&mut self) -> TIM15EN_W<APB2ENRrs> {
        TIM15EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<APB2ENRrs> {
        TIM16EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<APB2ENRrs> {
        TIM17EN_W::new(self, 18)
    }
    #[doc = "Bit 21 - SAI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sai1en(&mut self) -> SAI1EN_W<APB2ENRrs> {
        SAI1EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - SAI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sai2en(&mut self) -> SAI2EN_W<APB2ENRrs> {
        SAI2EN_W::new(self, 22)
    }
    #[doc = "Bit 24 - DFSDM timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1en(&mut self) -> DFSDM1EN_W<APB2ENRrs> {
        DFSDM1EN_W::new(self, 24)
    }
    #[doc = "Bit 26 - LCD-TFT clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<APB2ENRrs> {
        LTDCEN_W::new(self, 26)
    }
    #[doc = "Bit 27 - DSI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsien(&mut self) -> DSIEN_W<APB2ENRrs> {
        DSIEN_W::new(self, 27)
    }
}
#[doc = "APB2ENR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2ENRrs;
impl crate::RegisterSpec for APB2ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2enr::R`](R) reader structure"]
impl crate::Readable for APB2ENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb2enr::W`](W) writer structure"]
impl crate::Writable for APB2ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2ENR to value 0"]
impl crate::Resettable for APB2ENRrs {
    const RESET_VALUE: u32 = 0;
}
