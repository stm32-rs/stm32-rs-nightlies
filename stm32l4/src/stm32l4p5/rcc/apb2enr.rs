///Register `APB2ENR` reader
pub type R = crate::R<APB2ENRrs>;
///Register `APB2ENR` writer
pub type W = crate::W<APB2ENRrs>;
/**SYSCFG clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGEN {
    ///0: SYSCFG + COMP + VREFBUF clock disabled
    Disabled = 0,
    ///1: SYSCFG + COMP + VREFBUF clock enabled
    Enabled = 1,
}
impl From<SYSCFGEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGEN` reader - SYSCFG clock enable
pub type SYSCFGEN_R = crate::BitReader<SYSCFGEN>;
impl SYSCFGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGEN {
        match self.bits {
            false => SYSCFGEN::Disabled,
            true => SYSCFGEN::Enabled,
        }
    }
    ///SYSCFG + COMP + VREFBUF clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGEN::Disabled
    }
    ///SYSCFG + COMP + VREFBUF clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGEN::Enabled
    }
}
///Field `SYSCFGEN` writer - SYSCFG clock enable
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGEN>;
impl<'a, REG> SYSCFGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SYSCFG + COMP + VREFBUF clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::Disabled)
    }
    ///SYSCFG + COMP + VREFBUF clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::Enabled)
    }
}
/**Firewall clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWEN {
    ///0: Firewall clock disabled
    Disabled = 0,
    ///1: Firewall clock enabled
    Enabled = 1,
}
impl From<FWEN> for bool {
    #[inline(always)]
    fn from(variant: FWEN) -> Self {
        variant as u8 != 0
    }
}
///Field `FWEN` reader - Firewall clock enable
pub type FWEN_R = crate::BitReader<FWEN>;
impl FWEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FWEN {
        match self.bits {
            false => FWEN::Disabled,
            true => FWEN::Enabled,
        }
    }
    ///Firewall clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FWEN::Disabled
    }
    ///Firewall clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FWEN::Enabled
    }
}
///Field `FWEN` writer - Firewall clock enable
pub type FWEN_W<'a, REG> = crate::BitWriter<'a, REG, FWEN>;
impl<'a, REG> FWEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Firewall clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FWEN::Disabled)
    }
    ///Firewall clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FWEN::Enabled)
    }
}
/**TIM1 timer clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1EN {
    ///0: TIMx clock disabled
    Disabled = 0,
    ///1: TIMx clock enabled
    Enabled = 1,
}
impl From<TIM1EN> for bool {
    #[inline(always)]
    fn from(variant: TIM1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM1EN` reader - TIM1 timer clock enable
pub type TIM1EN_R = crate::BitReader<TIM1EN>;
impl TIM1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM1EN {
        match self.bits {
            false => TIM1EN::Disabled,
            true => TIM1EN::Enabled,
        }
    }
    ///TIMx clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM1EN::Disabled
    }
    ///TIMx clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM1EN::Enabled
    }
}
///Field `TIM1EN` writer - TIM1 timer clock enable
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM1EN>;
impl<'a, REG> TIM1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TIMx clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1EN::Disabled)
    }
    ///TIMx clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1EN::Enabled)
    }
}
/**SPI1 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1EN {
    ///0: SPI1 clock disabled
    Disabled = 0,
    ///1: SPI1 clock enabled
    Enabled = 1,
}
impl From<SPI1EN> for bool {
    #[inline(always)]
    fn from(variant: SPI1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `SPI1EN` reader - SPI1 clock enable
pub type SPI1EN_R = crate::BitReader<SPI1EN>;
impl SPI1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPI1EN {
        match self.bits {
            false => SPI1EN::Disabled,
            true => SPI1EN::Enabled,
        }
    }
    ///SPI1 clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPI1EN::Disabled
    }
    ///SPI1 clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPI1EN::Enabled
    }
}
///Field `SPI1EN` writer - SPI1 clock enable
pub type SPI1EN_W<'a, REG> = crate::BitWriter<'a, REG, SPI1EN>;
impl<'a, REG> SPI1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SPI1 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1EN::Disabled)
    }
    ///SPI1 clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1EN::Enabled)
    }
}
///Field `TIM8EN` reader - TIM8 timer clock enable
pub use TIM1EN_R as TIM8EN_R;
///Field `TIM8EN` writer - TIM8 timer clock enable
pub use TIM1EN_W as TIM8EN_W;
/**USART1clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1EN {
    ///0: USART1 clock disabled
    Disabled = 0,
    ///1: USART1 clock enabled
    Enabled = 1,
}
impl From<USART1EN> for bool {
    #[inline(always)]
    fn from(variant: USART1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `USART1EN` reader - USART1clock enable
pub type USART1EN_R = crate::BitReader<USART1EN>;
impl USART1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART1EN {
        match self.bits {
            false => USART1EN::Disabled,
            true => USART1EN::Enabled,
        }
    }
    ///USART1 clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USART1EN::Disabled
    }
    ///USART1 clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USART1EN::Enabled
    }
}
///Field `USART1EN` writer - USART1clock enable
pub type USART1EN_W<'a, REG> = crate::BitWriter<'a, REG, USART1EN>;
impl<'a, REG> USART1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///USART1 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(USART1EN::Disabled)
    }
    ///USART1 clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(USART1EN::Enabled)
    }
}
///Field `TIM15EN` reader - TIM15 timer clock enable
pub use TIM1EN_R as TIM15EN_R;
///Field `TIM16EN` reader - TIM16 timer clock enable
pub use TIM1EN_R as TIM16EN_R;
///Field `TIM17EN` reader - TIM17 timer clock enable
pub use TIM1EN_R as TIM17EN_R;
///Field `TIM15EN` writer - TIM15 timer clock enable
pub use TIM1EN_W as TIM15EN_W;
///Field `TIM16EN` writer - TIM16 timer clock enable
pub use TIM1EN_W as TIM16EN_W;
///Field `TIM17EN` writer - TIM17 timer clock enable
pub use TIM1EN_W as TIM17EN_W;
/**SAI1 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI1EN {
    ///0: SAIx clock disabled
    Disabled = 0,
    ///1: SAIx clock enabled
    Enabled = 1,
}
impl From<SAI1EN> for bool {
    #[inline(always)]
    fn from(variant: SAI1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `SAI1EN` reader - SAI1 clock enable
pub type SAI1EN_R = crate::BitReader<SAI1EN>;
impl SAI1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SAI1EN {
        match self.bits {
            false => SAI1EN::Disabled,
            true => SAI1EN::Enabled,
        }
    }
    ///SAIx clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAI1EN::Disabled
    }
    ///SAIx clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAI1EN::Enabled
    }
}
///Field `SAI1EN` writer - SAI1 clock enable
pub type SAI1EN_W<'a, REG> = crate::BitWriter<'a, REG, SAI1EN>;
impl<'a, REG> SAI1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SAIx clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1EN::Disabled)
    }
    ///SAIx clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1EN::Enabled)
    }
}
///Field `SAI2EN` reader - SAI2 clock enable
pub use SAI1EN_R as SAI2EN_R;
///Field `SAI2EN` writer - SAI2 clock enable
pub use SAI1EN_W as SAI2EN_W;
/**DFSDM timer clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM1EN {
    ///0: DFSDM1 clock disabled
    Disabled = 0,
    ///1: DFSDM1 clock enabled
    Enabled = 1,
}
impl From<DFSDM1EN> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `DFSDM1EN` reader - DFSDM timer clock enable
pub type DFSDM1EN_R = crate::BitReader<DFSDM1EN>;
impl DFSDM1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DFSDM1EN {
        match self.bits {
            false => DFSDM1EN::Disabled,
            true => DFSDM1EN::Enabled,
        }
    }
    ///DFSDM1 clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DFSDM1EN::Disabled
    }
    ///DFSDM1 clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DFSDM1EN::Enabled
    }
}
///Field `DFSDM1EN` writer - DFSDM timer clock enable
pub type DFSDM1EN_W<'a, REG> = crate::BitWriter<'a, REG, DFSDM1EN>;
impl<'a, REG> DFSDM1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DFSDM1 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1EN::Disabled)
    }
    ///DFSDM1 clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1EN::Enabled)
    }
}
/**LCD-TFT clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCEN {
    ///0: LTDC clock disabled
    Disabled = 0,
    ///1: LTDC clock enabled
    Enabled = 1,
}
impl From<LTDCEN> for bool {
    #[inline(always)]
    fn from(variant: LTDCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LTDCEN` reader - LCD-TFT clock enable
pub type LTDCEN_R = crate::BitReader<LTDCEN>;
impl LTDCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LTDCEN {
        match self.bits {
            false => LTDCEN::Disabled,
            true => LTDCEN::Enabled,
        }
    }
    ///LTDC clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LTDCEN::Disabled
    }
    ///LTDC clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LTDCEN::Enabled
    }
}
///Field `LTDCEN` writer - LCD-TFT clock enable
pub type LTDCEN_W<'a, REG> = crate::BitWriter<'a, REG, LTDCEN>;
impl<'a, REG> LTDCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LTDC clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCEN::Disabled)
    }
    ///LTDC clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCEN::Enabled)
    }
}
/**DSI clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSIEN {
    ///0: DSI clock disabled
    Disabled = 0,
    ///1: DSI clock enabled
    Enabled = 1,
}
impl From<DSIEN> for bool {
    #[inline(always)]
    fn from(variant: DSIEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DSIEN` reader - DSI clock enable
pub type DSIEN_R = crate::BitReader<DSIEN>;
impl DSIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSIEN {
        match self.bits {
            false => DSIEN::Disabled,
            true => DSIEN::Enabled,
        }
    }
    ///DSI clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DSIEN::Disabled
    }
    ///DSI clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DSIEN::Enabled
    }
}
///Field `DSIEN` writer - DSI clock enable
pub type DSIEN_W<'a, REG> = crate::BitWriter<'a, REG, DSIEN>;
impl<'a, REG> DSIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DSI clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DSIEN::Disabled)
    }
    ///DSI clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DSIEN::Enabled)
    }
}
impl R {
    ///Bit 0 - SYSCFG clock enable
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 7 - Firewall clock enable
    #[inline(always)]
    pub fn fwen(&self) -> FWEN_R {
        FWEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - TIM1 timer clock enable
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 clock enable
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TIM8 timer clock enable
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - USART1clock enable
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM15 timer clock enable
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 timer clock enable
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 timer clock enable
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - SAI1 clock enable
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SAI2 clock enable
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - DFSDM timer clock enable
    #[inline(always)]
    pub fn dfsdm1en(&self) -> DFSDM1EN_R {
        DFSDM1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - LCD-TFT clock enable
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - DSI clock enable
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2ENR")
            .field("syscfgen", &self.syscfgen())
            .field("fwen", &self.fwen())
            .field("tim1en", &self.tim1en())
            .field("spi1en", &self.spi1en())
            .field("tim8en", &self.tim8en())
            .field("usart1en", &self.usart1en())
            .field("tim15en", &self.tim15en())
            .field("tim16en", &self.tim16en())
            .field("tim17en", &self.tim17en())
            .field("sai1en", &self.sai1en())
            .field("sai2en", &self.sai2en())
            .field("dfsdm1en", &self.dfsdm1en())
            .field("ltdcen", &self.ltdcen())
            .field("dsien", &self.dsien())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSCFG clock enable
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<'_, APB2ENRrs> {
        SYSCFGEN_W::new(self, 0)
    }
    ///Bit 7 - Firewall clock enable
    #[inline(always)]
    pub fn fwen(&mut self) -> FWEN_W<'_, APB2ENRrs> {
        FWEN_W::new(self, 7)
    }
    ///Bit 11 - TIM1 timer clock enable
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W<'_, APB2ENRrs> {
        TIM1EN_W::new(self, 11)
    }
    ///Bit 12 - SPI1 clock enable
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<'_, APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    ///Bit 13 - TIM8 timer clock enable
    #[inline(always)]
    pub fn tim8en(&mut self) -> TIM8EN_W<'_, APB2ENRrs> {
        TIM8EN_W::new(self, 13)
    }
    ///Bit 14 - USART1clock enable
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<'_, APB2ENRrs> {
        USART1EN_W::new(self, 14)
    }
    ///Bit 16 - TIM15 timer clock enable
    #[inline(always)]
    pub fn tim15en(&mut self) -> TIM15EN_W<'_, APB2ENRrs> {
        TIM15EN_W::new(self, 16)
    }
    ///Bit 17 - TIM16 timer clock enable
    #[inline(always)]
    pub fn tim16en(&mut self) -> TIM16EN_W<'_, APB2ENRrs> {
        TIM16EN_W::new(self, 17)
    }
    ///Bit 18 - TIM17 timer clock enable
    #[inline(always)]
    pub fn tim17en(&mut self) -> TIM17EN_W<'_, APB2ENRrs> {
        TIM17EN_W::new(self, 18)
    }
    ///Bit 21 - SAI1 clock enable
    #[inline(always)]
    pub fn sai1en(&mut self) -> SAI1EN_W<'_, APB2ENRrs> {
        SAI1EN_W::new(self, 21)
    }
    ///Bit 22 - SAI2 clock enable
    #[inline(always)]
    pub fn sai2en(&mut self) -> SAI2EN_W<'_, APB2ENRrs> {
        SAI2EN_W::new(self, 22)
    }
    ///Bit 24 - DFSDM timer clock enable
    #[inline(always)]
    pub fn dfsdm1en(&mut self) -> DFSDM1EN_W<'_, APB2ENRrs> {
        DFSDM1EN_W::new(self, 24)
    }
    ///Bit 26 - LCD-TFT clock enable
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W<'_, APB2ENRrs> {
        LTDCEN_W::new(self, 26)
    }
    ///Bit 27 - DSI clock enable
    #[inline(always)]
    pub fn dsien(&mut self) -> DSIEN_W<'_, APB2ENRrs> {
        DSIEN_W::new(self, 27)
    }
}
/**APB2ENR

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:APB2ENR)*/
pub struct APB2ENRrs;
impl crate::RegisterSpec for APB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2enr::R`](R) reader structure
impl crate::Readable for APB2ENRrs {}
///`write(|w| ..)` method takes [`apb2enr::W`](W) writer structure
impl crate::Writable for APB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2ENR to value 0
impl crate::Resettable for APB2ENRrs {}
