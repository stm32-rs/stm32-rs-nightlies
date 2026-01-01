///Register `RCC_APBRSTR2` reader
pub type R = crate::R<RCC_APBRSTR2rs>;
///Register `RCC_APBRSTR2` writer
pub type W = crate::W<RCC_APBRSTR2rs>;
/**SYSCFG reset Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGRST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset SYSCFG
    B0x1 = 1,
}
impl From<SYSCFGRST> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRST) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGRST` reader - SYSCFG reset Set and cleared by software.
pub type SYSCFGRST_R = crate::BitReader<SYSCFGRST>;
impl SYSCFGRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGRST {
        match self.bits {
            false => SYSCFGRST::B0x0,
            true => SYSCFGRST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SYSCFGRST::B0x0
    }
    ///Reset SYSCFG
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SYSCFGRST::B0x1
    }
}
///Field `SYSCFGRST` writer - SYSCFG reset Set and cleared by software.
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGRST>;
impl<'a, REG> SYSCFGRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGRST::B0x0)
    }
    ///Reset SYSCFG
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGRST::B0x1)
    }
}
/**TIM1 timer reset Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1RST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset TIM1 timer
    B0x1 = 1,
}
impl From<TIM1RST> for bool {
    #[inline(always)]
    fn from(variant: TIM1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM1RST` reader - TIM1 timer reset Set and cleared by software.
pub type TIM1RST_R = crate::BitReader<TIM1RST>;
impl TIM1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM1RST {
        match self.bits {
            false => TIM1RST::B0x0,
            true => TIM1RST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM1RST::B0x0
    }
    ///Reset TIM1 timer
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM1RST::B0x1
    }
}
///Field `TIM1RST` writer - TIM1 timer reset Set and cleared by software.
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM1RST>;
impl<'a, REG> TIM1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1RST::B0x0)
    }
    ///Reset TIM1 timer
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1RST::B0x1)
    }
}
/**SPI1 reset Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1RST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset SPI1
    B0x1 = 1,
}
impl From<SPI1RST> for bool {
    #[inline(always)]
    fn from(variant: SPI1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `SPI1RST` reader - SPI1 reset Set and cleared by software.
pub type SPI1RST_R = crate::BitReader<SPI1RST>;
impl SPI1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPI1RST {
        match self.bits {
            false => SPI1RST::B0x0,
            true => SPI1RST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI1RST::B0x0
    }
    ///Reset SPI1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI1RST::B0x1
    }
}
///Field `SPI1RST` writer - SPI1 reset Set and cleared by software.
pub type SPI1RST_W<'a, REG> = crate::BitWriter<'a, REG, SPI1RST>;
impl<'a, REG> SPI1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1RST::B0x0)
    }
    ///Reset SPI1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1RST::B0x1)
    }
}
/**USART1 reset Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1RST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset USART1
    B0x1 = 1,
}
impl From<USART1RST> for bool {
    #[inline(always)]
    fn from(variant: USART1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `USART1RST` reader - USART1 reset Set and cleared by software.
pub type USART1RST_R = crate::BitReader<USART1RST>;
impl USART1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART1RST {
        match self.bits {
            false => USART1RST::B0x0,
            true => USART1RST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART1RST::B0x0
    }
    ///Reset USART1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART1RST::B0x1
    }
}
///Field `USART1RST` writer - USART1 reset Set and cleared by software.
pub type USART1RST_W<'a, REG> = crate::BitWriter<'a, REG, USART1RST>;
impl<'a, REG> USART1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART1RST::B0x0)
    }
    ///Reset USART1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART1RST::B0x1)
    }
}
/**TIM14 timer reset Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM14RST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset TIM14 timer
    B0x1 = 1,
}
impl From<TIM14RST> for bool {
    #[inline(always)]
    fn from(variant: TIM14RST) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM14RST` reader - TIM14 timer reset Set and cleared by software.
pub type TIM14RST_R = crate::BitReader<TIM14RST>;
impl TIM14RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM14RST {
        match self.bits {
            false => TIM14RST::B0x0,
            true => TIM14RST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM14RST::B0x0
    }
    ///Reset TIM14 timer
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM14RST::B0x1
    }
}
///Field `TIM14RST` writer - TIM14 timer reset Set and cleared by software.
pub type TIM14RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM14RST>;
impl<'a, REG> TIM14RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM14RST::B0x0)
    }
    ///Reset TIM14 timer
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM14RST::B0x1)
    }
}
/**TIM16 timer reset Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM16RST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset TIM16 timer
    B0x1 = 1,
}
impl From<TIM16RST> for bool {
    #[inline(always)]
    fn from(variant: TIM16RST) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM16RST` reader - TIM16 timer reset Set and cleared by software.
pub type TIM16RST_R = crate::BitReader<TIM16RST>;
impl TIM16RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM16RST {
        match self.bits {
            false => TIM16RST::B0x0,
            true => TIM16RST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM16RST::B0x0
    }
    ///Reset TIM16 timer
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM16RST::B0x1
    }
}
///Field `TIM16RST` writer - TIM16 timer reset Set and cleared by software.
pub type TIM16RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM16RST>;
impl<'a, REG> TIM16RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM16RST::B0x0)
    }
    ///Reset TIM16 timer
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM16RST::B0x1)
    }
}
/**TIM16 timer reset Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM17RST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset TIM17 timer
    B0x1 = 1,
}
impl From<TIM17RST> for bool {
    #[inline(always)]
    fn from(variant: TIM17RST) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM17RST` reader - TIM16 timer reset Set and cleared by software.
pub type TIM17RST_R = crate::BitReader<TIM17RST>;
impl TIM17RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM17RST {
        match self.bits {
            false => TIM17RST::B0x0,
            true => TIM17RST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM17RST::B0x0
    }
    ///Reset TIM17 timer
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM17RST::B0x1
    }
}
///Field `TIM17RST` writer - TIM16 timer reset Set and cleared by software.
pub type TIM17RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM17RST>;
impl<'a, REG> TIM17RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM17RST::B0x0)
    }
    ///Reset TIM17 timer
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM17RST::B0x1)
    }
}
/**ADC reset Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCRST {
    ///0: No effect
    B0x0 = 0,
    ///1: Reset ADC
    B0x1 = 1,
}
impl From<ADCRST> for bool {
    #[inline(always)]
    fn from(variant: ADCRST) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCRST` reader - ADC reset Set and cleared by software.
pub type ADCRST_R = crate::BitReader<ADCRST>;
impl ADCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCRST {
        match self.bits {
            false => ADCRST::B0x0,
            true => ADCRST::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADCRST::B0x0
    }
    ///Reset ADC
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADCRST::B0x1
    }
}
///Field `ADCRST` writer - ADC reset Set and cleared by software.
pub type ADCRST_W<'a, REG> = crate::BitWriter<'a, REG, ADCRST>;
impl<'a, REG> ADCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADCRST::B0x0)
    }
    ///Reset ADC
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADCRST::B0x1)
    }
}
impl R {
    ///Bit 0 - SYSCFG reset Set and cleared by software.
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 11 - TIM1 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 reset Set and cleared by software.
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1 reset Set and cleared by software.
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TIM14 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim14rst(&self) -> TIM14RST_R {
        TIM14RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - TIM16 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM16 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - ADC reset Set and cleared by software.
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APBRSTR2")
            .field("syscfgrst", &self.syscfgrst())
            .field("tim1rst", &self.tim1rst())
            .field("spi1rst", &self.spi1rst())
            .field("usart1rst", &self.usart1rst())
            .field("tim14rst", &self.tim14rst())
            .field("tim16rst", &self.tim16rst())
            .field("tim17rst", &self.tim17rst())
            .field("adcrst", &self.adcrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSCFG reset Set and cleared by software.
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<'_, RCC_APBRSTR2rs> {
        SYSCFGRST_W::new(self, 0)
    }
    ///Bit 11 - TIM1 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W<'_, RCC_APBRSTR2rs> {
        TIM1RST_W::new(self, 11)
    }
    ///Bit 12 - SPI1 reset Set and cleared by software.
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<'_, RCC_APBRSTR2rs> {
        SPI1RST_W::new(self, 12)
    }
    ///Bit 14 - USART1 reset Set and cleared by software.
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W<'_, RCC_APBRSTR2rs> {
        USART1RST_W::new(self, 14)
    }
    ///Bit 15 - TIM14 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim14rst(&mut self) -> TIM14RST_W<'_, RCC_APBRSTR2rs> {
        TIM14RST_W::new(self, 15)
    }
    ///Bit 17 - TIM16 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim16rst(&mut self) -> TIM16RST_W<'_, RCC_APBRSTR2rs> {
        TIM16RST_W::new(self, 17)
    }
    ///Bit 18 - TIM16 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim17rst(&mut self) -> TIM17RST_W<'_, RCC_APBRSTR2rs> {
        TIM17RST_W::new(self, 18)
    }
    ///Bit 20 - ADC reset Set and cleared by software.
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W<'_, RCC_APBRSTR2rs> {
        ADCRST_W::new(self, 20)
    }
}
/**RCC APB peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_apbrstr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbrstr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#RCC:RCC_APBRSTR2)*/
pub struct RCC_APBRSTR2rs;
impl crate::RegisterSpec for RCC_APBRSTR2rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_apbrstr2::R`](R) reader structure
impl crate::Readable for RCC_APBRSTR2rs {}
///`write(|w| ..)` method takes [`rcc_apbrstr2::W`](W) writer structure
impl crate::Writable for RCC_APBRSTR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_APBRSTR2 to value 0
impl crate::Resettable for RCC_APBRSTR2rs {}
