#[doc = "Register `APB2RSTR` reader"]
pub type R = crate::R<APB2RSTRrs>;
#[doc = "Register `APB2RSTR` writer"]
pub type W = crate::W<APB2RSTRrs>;
#[doc = "System configuration (SYSCFG) reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGRST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset SYSCFG + COMP + VREFBUF"]
    Reset = 1,
}
impl From<SYSCFGRST> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGRST` reader - System configuration (SYSCFG) reset"]
pub type SYSCFGRST_R = crate::BitReader<SYSCFGRST>;
impl SYSCFGRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGRST {
        match self.bits {
            false => SYSCFGRST::NoEffect,
            true => SYSCFGRST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SYSCFGRST::NoEffect
    }
    #[doc = "Reset SYSCFG + COMP + VREFBUF"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYSCFGRST::Reset
    }
}
#[doc = "Field `SYSCFGRST` writer - System configuration (SYSCFG) reset"]
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGRST>;
impl<'a, REG> SYSCFGRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGRST::NoEffect)
    }
    #[doc = "Reset SYSCFG + COMP + VREFBUF"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGRST::Reset)
    }
}
#[doc = "TIM1 timer reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1RST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset TIMx"]
    Reset = 1,
}
impl From<TIM1RST> for bool {
    #[inline(always)]
    fn from(variant: TIM1RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1RST` reader - TIM1 timer reset"]
pub type TIM1RST_R = crate::BitReader<TIM1RST>;
impl TIM1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM1RST {
        match self.bits {
            false => TIM1RST::NoEffect,
            true => TIM1RST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TIM1RST::NoEffect
    }
    #[doc = "Reset TIMx"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM1RST::Reset
    }
}
#[doc = "Field `TIM1RST` writer - TIM1 timer reset"]
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM1RST>;
impl<'a, REG> TIM1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1RST::NoEffect)
    }
    #[doc = "Reset TIMx"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1RST::Reset)
    }
}
#[doc = "SPI1 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1RST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset SPI1"]
    Reset = 1,
}
impl From<SPI1RST> for bool {
    #[inline(always)]
    fn from(variant: SPI1RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub type SPI1RST_R = crate::BitReader<SPI1RST>;
impl SPI1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI1RST {
        match self.bits {
            false => SPI1RST::NoEffect,
            true => SPI1RST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SPI1RST::NoEffect
    }
    #[doc = "Reset SPI1"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SPI1RST::Reset
    }
}
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub type SPI1RST_W<'a, REG> = crate::BitWriter<'a, REG, SPI1RST>;
impl<'a, REG> SPI1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1RST::NoEffect)
    }
    #[doc = "Reset SPI1"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1RST::Reset)
    }
}
#[doc = "Field `TIM8RST` reader - TIM8 timer reset"]
pub use TIM1RST_R as TIM8RST_R;
#[doc = "Field `TIM8RST` writer - TIM8 timer reset"]
pub use TIM1RST_W as TIM8RST_W;
#[doc = "USART1 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1RST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset UARTx"]
    Reset = 1,
}
impl From<USART1RST> for bool {
    #[inline(always)]
    fn from(variant: USART1RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub type USART1RST_R = crate::BitReader<USART1RST>;
impl USART1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART1RST {
        match self.bits {
            false => USART1RST::NoEffect,
            true => USART1RST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == USART1RST::NoEffect
    }
    #[doc = "Reset UARTx"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == USART1RST::Reset
    }
}
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub type USART1RST_W<'a, REG> = crate::BitWriter<'a, REG, USART1RST>;
impl<'a, REG> USART1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(USART1RST::NoEffect)
    }
    #[doc = "Reset UARTx"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(USART1RST::Reset)
    }
}
#[doc = "Field `TIM15RST` reader - TIM15 timer reset"]
pub use TIM1RST_R as TIM15RST_R;
#[doc = "Field `TIM16RST` reader - TIM16 timer reset"]
pub use TIM1RST_R as TIM16RST_R;
#[doc = "Field `TIM17RST` reader - TIM17 timer reset"]
pub use TIM1RST_R as TIM17RST_R;
#[doc = "Field `TIM15RST` writer - TIM15 timer reset"]
pub use TIM1RST_W as TIM15RST_W;
#[doc = "Field `TIM16RST` writer - TIM16 timer reset"]
pub use TIM1RST_W as TIM16RST_W;
#[doc = "Field `TIM17RST` writer - TIM17 timer reset"]
pub use TIM1RST_W as TIM17RST_W;
#[doc = "Serial audio interface 1 (SAI1) reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI1RST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset SAIx"]
    Reset = 1,
}
impl From<SAI1RST> for bool {
    #[inline(always)]
    fn from(variant: SAI1RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAI1RST` reader - Serial audio interface 1 (SAI1) reset"]
pub type SAI1RST_R = crate::BitReader<SAI1RST>;
impl SAI1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SAI1RST {
        match self.bits {
            false => SAI1RST::NoEffect,
            true => SAI1RST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SAI1RST::NoEffect
    }
    #[doc = "Reset SAIx"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SAI1RST::Reset
    }
}
#[doc = "Field `SAI1RST` writer - Serial audio interface 1 (SAI1) reset"]
pub type SAI1RST_W<'a, REG> = crate::BitWriter<'a, REG, SAI1RST>;
impl<'a, REG> SAI1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1RST::NoEffect)
    }
    #[doc = "Reset SAIx"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1RST::Reset)
    }
}
#[doc = "Field `SAI2RST` reader - Serial audio interface 2 (SAI2) reset"]
pub use SAI1RST_R as SAI2RST_R;
#[doc = "Field `SAI2RST` writer - Serial audio interface 2 (SAI2) reset"]
pub use SAI1RST_W as SAI2RST_W;
#[doc = "Digital filters for sigma-delata modulators (DFSDM) reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM1RST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset DFSDM1"]
    Reset = 1,
}
impl From<DFSDM1RST> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFSDM1RST` reader - Digital filters for sigma-delata modulators (DFSDM) reset"]
pub type DFSDM1RST_R = crate::BitReader<DFSDM1RST>;
impl DFSDM1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DFSDM1RST {
        match self.bits {
            false => DFSDM1RST::NoEffect,
            true => DFSDM1RST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DFSDM1RST::NoEffect
    }
    #[doc = "Reset DFSDM1"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DFSDM1RST::Reset
    }
}
#[doc = "Field `DFSDM1RST` writer - Digital filters for sigma-delata modulators (DFSDM) reset"]
pub type DFSDM1RST_W<'a, REG> = crate::BitWriter<'a, REG, DFSDM1RST>;
impl<'a, REG> DFSDM1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1RST::NoEffect)
    }
    #[doc = "Reset DFSDM1"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1RST::Reset)
    }
}
#[doc = "LCD-TFT reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCRST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset LCD-TFT"]
    Reset = 1,
}
impl From<LTDCRST> for bool {
    #[inline(always)]
    fn from(variant: LTDCRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LTDCRST` reader - LCD-TFT reset"]
pub type LTDCRST_R = crate::BitReader<LTDCRST>;
impl LTDCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LTDCRST {
        match self.bits {
            false => LTDCRST::NoEffect,
            true => LTDCRST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == LTDCRST::NoEffect
    }
    #[doc = "Reset LCD-TFT"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LTDCRST::Reset
    }
}
#[doc = "Field `LTDCRST` writer - LCD-TFT reset"]
pub type LTDCRST_W<'a, REG> = crate::BitWriter<'a, REG, LTDCRST>;
impl<'a, REG> LTDCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCRST::NoEffect)
    }
    #[doc = "Reset LCD-TFT"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCRST::Reset)
    }
}
#[doc = "DSI reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSIRST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset DSI"]
    Reset = 1,
}
impl From<DSIRST> for bool {
    #[inline(always)]
    fn from(variant: DSIRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSIRST` reader - DSI reset"]
pub type DSIRST_R = crate::BitReader<DSIRST>;
impl DSIRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DSIRST {
        match self.bits {
            false => DSIRST::NoEffect,
            true => DSIRST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DSIRST::NoEffect
    }
    #[doc = "Reset DSI"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DSIRST::Reset
    }
}
#[doc = "Field `DSIRST` writer - DSI reset"]
pub type DSIRST_W<'a, REG> = crate::BitWriter<'a, REG, DSIRST>;
impl<'a, REG> DSIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DSIRST::NoEffect)
    }
    #[doc = "Reset DSI"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DSIRST::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - System configuration (SYSCFG) reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer reset"]
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Serial audio interface 1 (SAI1) reset"]
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Serial audio interface 2 (SAI2) reset"]
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Digital filters for sigma-delata modulators (DFSDM) reset"]
    #[inline(always)]
    pub fn dfsdm1rst(&self) -> DFSDM1RST_R {
        DFSDM1RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - LCD-TFT reset"]
    #[inline(always)]
    pub fn ltdcrst(&self) -> LTDCRST_R {
        LTDCRST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DSI reset"]
    #[inline(always)]
    pub fn dsirst(&self) -> DSIRST_R {
        DSIRST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration (SYSCFG) reset"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<APB2RSTRrs> {
        SYSCFGRST_W::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<APB2RSTRrs> {
        TIM1RST_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    #[doc = "Bit 13 - TIM8 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim8rst(&mut self) -> TIM8RST_W<APB2RSTRrs> {
        TIM8RST_W::new(self, 13)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<APB2RSTRrs> {
        USART1RST_W::new(self, 14)
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim15rst(&mut self) -> TIM15RST_W<APB2RSTRrs> {
        TIM15RST_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim16rst(&mut self) -> TIM16RST_W<APB2RSTRrs> {
        TIM16RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim17rst(&mut self) -> TIM17RST_W<APB2RSTRrs> {
        TIM17RST_W::new(self, 18)
    }
    #[doc = "Bit 21 - Serial audio interface 1 (SAI1) reset"]
    #[inline(always)]
    #[must_use]
    pub fn sai1rst(&mut self) -> SAI1RST_W<APB2RSTRrs> {
        SAI1RST_W::new(self, 21)
    }
    #[doc = "Bit 22 - Serial audio interface 2 (SAI2) reset"]
    #[inline(always)]
    #[must_use]
    pub fn sai2rst(&mut self) -> SAI2RST_W<APB2RSTRrs> {
        SAI2RST_W::new(self, 22)
    }
    #[doc = "Bit 24 - Digital filters for sigma-delata modulators (DFSDM) reset"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1rst(&mut self) -> DFSDM1RST_W<APB2RSTRrs> {
        DFSDM1RST_W::new(self, 24)
    }
    #[doc = "Bit 26 - LCD-TFT reset"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcrst(&mut self) -> LTDCRST_W<APB2RSTRrs> {
        LTDCRST_W::new(self, 26)
    }
    #[doc = "Bit 27 - DSI reset"]
    #[inline(always)]
    #[must_use]
    pub fn dsirst(&mut self) -> DSIRST_W<APB2RSTRrs> {
        DSIRST_W::new(self, 27)
    }
}
#[doc = "APB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2RSTRrs;
impl crate::RegisterSpec for APB2RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rstr::R`](R) reader structure"]
impl crate::Readable for APB2RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure"]
impl crate::Writable for APB2RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for APB2RSTRrs {
    const RESET_VALUE: u32 = 0;
}
