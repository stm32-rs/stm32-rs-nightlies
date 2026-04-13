///Register `APB2RSTR` reader
pub type R = crate::R<APB2RSTRrs>;
///Register `APB2RSTR` writer
pub type W = crate::W<APB2RSTRrs>;
/**System configuration (SYSCFG) reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGRST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset SYSCFG + COMP + VREFBUF
    Reset = 1,
}
impl From<SYSCFGRST> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRST) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGRST` reader - System configuration (SYSCFG) reset
pub type SYSCFGRST_R = crate::BitReader<SYSCFGRST>;
impl SYSCFGRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGRST {
        match self.bits {
            false => SYSCFGRST::NoEffect,
            true => SYSCFGRST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SYSCFGRST::NoEffect
    }
    ///Reset SYSCFG + COMP + VREFBUF
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYSCFGRST::Reset
    }
}
///Field `SYSCFGRST` writer - System configuration (SYSCFG) reset
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGRST>;
impl<'a, REG> SYSCFGRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGRST::NoEffect)
    }
    ///Reset SYSCFG + COMP + VREFBUF
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGRST::Reset)
    }
}
/**TIM1 timer reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1RST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset TIMx
    Reset = 1,
}
impl From<TIM1RST> for bool {
    #[inline(always)]
    fn from(variant: TIM1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM1RST` reader - TIM1 timer reset
pub type TIM1RST_R = crate::BitReader<TIM1RST>;
impl TIM1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM1RST {
        match self.bits {
            false => TIM1RST::NoEffect,
            true => TIM1RST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TIM1RST::NoEffect
    }
    ///Reset TIMx
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM1RST::Reset
    }
}
///Field `TIM1RST` writer - TIM1 timer reset
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM1RST>;
impl<'a, REG> TIM1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1RST::NoEffect)
    }
    ///Reset TIMx
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1RST::Reset)
    }
}
/**SPI1 reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1RST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset SPI1
    Reset = 1,
}
impl From<SPI1RST> for bool {
    #[inline(always)]
    fn from(variant: SPI1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `SPI1RST` reader - SPI1 reset
pub type SPI1RST_R = crate::BitReader<SPI1RST>;
impl SPI1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPI1RST {
        match self.bits {
            false => SPI1RST::NoEffect,
            true => SPI1RST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SPI1RST::NoEffect
    }
    ///Reset SPI1
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SPI1RST::Reset
    }
}
///Field `SPI1RST` writer - SPI1 reset
pub type SPI1RST_W<'a, REG> = crate::BitWriter<'a, REG, SPI1RST>;
impl<'a, REG> SPI1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1RST::NoEffect)
    }
    ///Reset SPI1
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1RST::Reset)
    }
}
///Field `TIM8RST` reader - TIM8 timer reset
pub use TIM1RST_R as TIM8RST_R;
///Field `TIM8RST` writer - TIM8 timer reset
pub use TIM1RST_W as TIM8RST_W;
/**USART1 reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1RST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset UARTx
    Reset = 1,
}
impl From<USART1RST> for bool {
    #[inline(always)]
    fn from(variant: USART1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `USART1RST` reader - USART1 reset
pub type USART1RST_R = crate::BitReader<USART1RST>;
impl USART1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART1RST {
        match self.bits {
            false => USART1RST::NoEffect,
            true => USART1RST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == USART1RST::NoEffect
    }
    ///Reset UARTx
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == USART1RST::Reset
    }
}
///Field `USART1RST` writer - USART1 reset
pub type USART1RST_W<'a, REG> = crate::BitWriter<'a, REG, USART1RST>;
impl<'a, REG> USART1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(USART1RST::NoEffect)
    }
    ///Reset UARTx
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(USART1RST::Reset)
    }
}
///Field `TIM15RST` reader - TIM15 timer reset
pub use TIM1RST_R as TIM15RST_R;
///Field `TIM16RST` reader - TIM16 timer reset
pub use TIM1RST_R as TIM16RST_R;
///Field `TIM17RST` reader - TIM17 timer reset
pub use TIM1RST_R as TIM17RST_R;
///Field `TIM15RST` writer - TIM15 timer reset
pub use TIM1RST_W as TIM15RST_W;
///Field `TIM16RST` writer - TIM16 timer reset
pub use TIM1RST_W as TIM16RST_W;
///Field `TIM17RST` writer - TIM17 timer reset
pub use TIM1RST_W as TIM17RST_W;
/**Serial audio interface 1 (SAI1) reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI1RST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset SAIx
    Reset = 1,
}
impl From<SAI1RST> for bool {
    #[inline(always)]
    fn from(variant: SAI1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `SAI1RST` reader - Serial audio interface 1 (SAI1) reset
pub type SAI1RST_R = crate::BitReader<SAI1RST>;
impl SAI1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SAI1RST {
        match self.bits {
            false => SAI1RST::NoEffect,
            true => SAI1RST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SAI1RST::NoEffect
    }
    ///Reset SAIx
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SAI1RST::Reset
    }
}
///Field `SAI1RST` writer - Serial audio interface 1 (SAI1) reset
pub type SAI1RST_W<'a, REG> = crate::BitWriter<'a, REG, SAI1RST>;
impl<'a, REG> SAI1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1RST::NoEffect)
    }
    ///Reset SAIx
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SAI1RST::Reset)
    }
}
///Field `SAI2RST` reader - Serial audio interface 2 (SAI2) reset
pub use SAI1RST_R as SAI2RST_R;
///Field `SAI2RST` writer - Serial audio interface 2 (SAI2) reset
pub use SAI1RST_W as SAI2RST_W;
/**Digital filters for sigma-delata modulators (DFSDM) reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDM1RST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset DFSDM1
    Reset = 1,
}
impl From<DFSDM1RST> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `DFSDM1RST` reader - Digital filters for sigma-delata modulators (DFSDM) reset
pub type DFSDM1RST_R = crate::BitReader<DFSDM1RST>;
impl DFSDM1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DFSDM1RST {
        match self.bits {
            false => DFSDM1RST::NoEffect,
            true => DFSDM1RST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DFSDM1RST::NoEffect
    }
    ///Reset DFSDM1
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DFSDM1RST::Reset
    }
}
///Field `DFSDM1RST` writer - Digital filters for sigma-delata modulators (DFSDM) reset
pub type DFSDM1RST_W<'a, REG> = crate::BitWriter<'a, REG, DFSDM1RST>;
impl<'a, REG> DFSDM1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1RST::NoEffect)
    }
    ///Reset DFSDM1
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDM1RST::Reset)
    }
}
/**LCD-TFT reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCRST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset LCD-TFT
    Reset = 1,
}
impl From<LTDCRST> for bool {
    #[inline(always)]
    fn from(variant: LTDCRST) -> Self {
        variant as u8 != 0
    }
}
///Field `LTDCRST` reader - LCD-TFT reset
pub type LTDCRST_R = crate::BitReader<LTDCRST>;
impl LTDCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LTDCRST {
        match self.bits {
            false => LTDCRST::NoEffect,
            true => LTDCRST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == LTDCRST::NoEffect
    }
    ///Reset LCD-TFT
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LTDCRST::Reset
    }
}
///Field `LTDCRST` writer - LCD-TFT reset
pub type LTDCRST_W<'a, REG> = crate::BitWriter<'a, REG, LTDCRST>;
impl<'a, REG> LTDCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCRST::NoEffect)
    }
    ///Reset LCD-TFT
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCRST::Reset)
    }
}
/**DSI reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSIRST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset DSI
    Reset = 1,
}
impl From<DSIRST> for bool {
    #[inline(always)]
    fn from(variant: DSIRST) -> Self {
        variant as u8 != 0
    }
}
///Field `DSIRST` reader - DSI reset
pub type DSIRST_R = crate::BitReader<DSIRST>;
impl DSIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSIRST {
        match self.bits {
            false => DSIRST::NoEffect,
            true => DSIRST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DSIRST::NoEffect
    }
    ///Reset DSI
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DSIRST::Reset
    }
}
///Field `DSIRST` writer - DSI reset
pub type DSIRST_W<'a, REG> = crate::BitWriter<'a, REG, DSIRST>;
impl<'a, REG> DSIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(DSIRST::NoEffect)
    }
    ///Reset DSI
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DSIRST::Reset)
    }
}
impl R {
    ///Bit 0 - System configuration (SYSCFG) reset
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 11 - TIM1 timer reset
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 reset
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TIM8 timer reset
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - USART1 reset
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM15 timer reset
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 timer reset
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 timer reset
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - Serial audio interface 1 (SAI1) reset
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Serial audio interface 2 (SAI2) reset
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Digital filters for sigma-delata modulators (DFSDM) reset
    #[inline(always)]
    pub fn dfsdm1rst(&self) -> DFSDM1RST_R {
        DFSDM1RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - LCD-TFT reset
    #[inline(always)]
    pub fn ltdcrst(&self) -> LTDCRST_R {
        LTDCRST_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - DSI reset
    #[inline(always)]
    pub fn dsirst(&self) -> DSIRST_R {
        DSIRST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RSTR")
            .field("syscfgrst", &self.syscfgrst())
            .field("tim1rst", &self.tim1rst())
            .field("spi1rst", &self.spi1rst())
            .field("tim8rst", &self.tim8rst())
            .field("usart1rst", &self.usart1rst())
            .field("tim15rst", &self.tim15rst())
            .field("tim16rst", &self.tim16rst())
            .field("tim17rst", &self.tim17rst())
            .field("sai1rst", &self.sai1rst())
            .field("sai2rst", &self.sai2rst())
            .field("dfsdm1rst", &self.dfsdm1rst())
            .field("ltdcrst", &self.ltdcrst())
            .field("dsirst", &self.dsirst())
            .finish()
    }
}
impl W {
    ///Bit 0 - System configuration (SYSCFG) reset
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<'_, APB2RSTRrs> {
        SYSCFGRST_W::new(self, 0)
    }
    ///Bit 11 - TIM1 timer reset
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W<'_, APB2RSTRrs> {
        TIM1RST_W::new(self, 11)
    }
    ///Bit 12 - SPI1 reset
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<'_, APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    ///Bit 13 - TIM8 timer reset
    #[inline(always)]
    pub fn tim8rst(&mut self) -> TIM8RST_W<'_, APB2RSTRrs> {
        TIM8RST_W::new(self, 13)
    }
    ///Bit 14 - USART1 reset
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W<'_, APB2RSTRrs> {
        USART1RST_W::new(self, 14)
    }
    ///Bit 16 - TIM15 timer reset
    #[inline(always)]
    pub fn tim15rst(&mut self) -> TIM15RST_W<'_, APB2RSTRrs> {
        TIM15RST_W::new(self, 16)
    }
    ///Bit 17 - TIM16 timer reset
    #[inline(always)]
    pub fn tim16rst(&mut self) -> TIM16RST_W<'_, APB2RSTRrs> {
        TIM16RST_W::new(self, 17)
    }
    ///Bit 18 - TIM17 timer reset
    #[inline(always)]
    pub fn tim17rst(&mut self) -> TIM17RST_W<'_, APB2RSTRrs> {
        TIM17RST_W::new(self, 18)
    }
    ///Bit 21 - Serial audio interface 1 (SAI1) reset
    #[inline(always)]
    pub fn sai1rst(&mut self) -> SAI1RST_W<'_, APB2RSTRrs> {
        SAI1RST_W::new(self, 21)
    }
    ///Bit 22 - Serial audio interface 2 (SAI2) reset
    #[inline(always)]
    pub fn sai2rst(&mut self) -> SAI2RST_W<'_, APB2RSTRrs> {
        SAI2RST_W::new(self, 22)
    }
    ///Bit 24 - Digital filters for sigma-delata modulators (DFSDM) reset
    #[inline(always)]
    pub fn dfsdm1rst(&mut self) -> DFSDM1RST_W<'_, APB2RSTRrs> {
        DFSDM1RST_W::new(self, 24)
    }
    ///Bit 26 - LCD-TFT reset
    #[inline(always)]
    pub fn ltdcrst(&mut self) -> LTDCRST_W<'_, APB2RSTRrs> {
        LTDCRST_W::new(self, 26)
    }
    ///Bit 27 - DSI reset
    #[inline(always)]
    pub fn dsirst(&mut self) -> DSIRST_W<'_, APB2RSTRrs> {
        DSIRST_W::new(self, 27)
    }
}
/**APB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:APB2RSTR)*/
pub struct APB2RSTRrs;
impl crate::RegisterSpec for APB2RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2rstr::R`](R) reader structure
impl crate::Readable for APB2RSTRrs {}
///`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure
impl crate::Writable for APB2RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2RSTR to value 0
impl crate::Resettable for APB2RSTRrs {}
