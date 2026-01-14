///Register `RCC_APBSMENR2` reader
pub type R = crate::R<RCC_APBSMENR2rs>;
///Register `RCC_APBSMENR2` writer
pub type W = crate::W<RCC_APBSMENR2rs>;
/**SYSCFG clock enable during Sleep and Stop modes Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGSMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<SYSCFGSMEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGSMEN` reader - SYSCFG clock enable during Sleep and Stop modes Set and cleared by software.
pub type SYSCFGSMEN_R = crate::BitReader<SYSCFGSMEN>;
impl SYSCFGSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGSMEN {
        match self.bits {
            false => SYSCFGSMEN::B0x0,
            true => SYSCFGSMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SYSCFGSMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SYSCFGSMEN::B0x1
    }
}
///Field `SYSCFGSMEN` writer - SYSCFG clock enable during Sleep and Stop modes Set and cleared by software.
pub type SYSCFGSMEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGSMEN>;
impl<'a, REG> SYSCFGSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGSMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGSMEN::B0x1)
    }
}
/**TIM1 timer clock enable during Sleep mode Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1SMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<TIM1SMEN> for bool {
    #[inline(always)]
    fn from(variant: TIM1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM1SMEN` reader - TIM1 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM1SMEN_R = crate::BitReader<TIM1SMEN>;
impl TIM1SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM1SMEN {
        match self.bits {
            false => TIM1SMEN::B0x0,
            true => TIM1SMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM1SMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM1SMEN::B0x1
    }
}
///Field `TIM1SMEN` writer - TIM1 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM1SMEN>;
impl<'a, REG> TIM1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1SMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1SMEN::B0x1)
    }
}
/**SPI1 clock enable during Sleep mode Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1SMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<SPI1SMEN> for bool {
    #[inline(always)]
    fn from(variant: SPI1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SPI1SMEN` reader - SPI1 clock enable during Sleep mode Set and cleared by software.
pub type SPI1SMEN_R = crate::BitReader<SPI1SMEN>;
impl SPI1SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPI1SMEN {
        match self.bits {
            false => SPI1SMEN::B0x0,
            true => SPI1SMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI1SMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI1SMEN::B0x1
    }
}
///Field `SPI1SMEN` writer - SPI1 clock enable during Sleep mode Set and cleared by software.
pub type SPI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, SPI1SMEN>;
impl<'a, REG> SPI1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1SMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1SMEN::B0x1)
    }
}
/**USART1 clock enable during Sleep and Stop modes Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1SMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<USART1SMEN> for bool {
    #[inline(always)]
    fn from(variant: USART1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `USART1SMEN` reader - USART1 clock enable during Sleep and Stop modes Set and cleared by software.
pub type USART1SMEN_R = crate::BitReader<USART1SMEN>;
impl USART1SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART1SMEN {
        match self.bits {
            false => USART1SMEN::B0x0,
            true => USART1SMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART1SMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART1SMEN::B0x1
    }
}
///Field `USART1SMEN` writer - USART1 clock enable during Sleep and Stop modes Set and cleared by software.
pub type USART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, USART1SMEN>;
impl<'a, REG> USART1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SMEN::B0x1)
    }
}
/**TIM14 timer clock enable during Sleep mode Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM14SMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<TIM14SMEN> for bool {
    #[inline(always)]
    fn from(variant: TIM14SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM14SMEN` reader - TIM14 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM14SMEN_R = crate::BitReader<TIM14SMEN>;
impl TIM14SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM14SMEN {
        match self.bits {
            false => TIM14SMEN::B0x0,
            true => TIM14SMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM14SMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM14SMEN::B0x1
    }
}
///Field `TIM14SMEN` writer - TIM14 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM14SMEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM14SMEN>;
impl<'a, REG> TIM14SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM14SMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM14SMEN::B0x1)
    }
}
/**TIM16 timer clock enable during Sleep mode Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM16SMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<TIM16SMEN> for bool {
    #[inline(always)]
    fn from(variant: TIM16SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM16SMEN` reader - TIM16 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM16SMEN_R = crate::BitReader<TIM16SMEN>;
impl TIM16SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM16SMEN {
        match self.bits {
            false => TIM16SMEN::B0x0,
            true => TIM16SMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM16SMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM16SMEN::B0x1
    }
}
///Field `TIM16SMEN` writer - TIM16 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM16SMEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM16SMEN>;
impl<'a, REG> TIM16SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM16SMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM16SMEN::B0x1)
    }
}
/**TIM16 timer clock enable during Sleep mode Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM17SMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<TIM17SMEN> for bool {
    #[inline(always)]
    fn from(variant: TIM17SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM17SMEN` reader - TIM16 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM17SMEN_R = crate::BitReader<TIM17SMEN>;
impl TIM17SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM17SMEN {
        match self.bits {
            false => TIM17SMEN::B0x0,
            true => TIM17SMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM17SMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM17SMEN::B0x1
    }
}
///Field `TIM17SMEN` writer - TIM16 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM17SMEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM17SMEN>;
impl<'a, REG> TIM17SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM17SMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM17SMEN::B0x1)
    }
}
/**ADC clock enable during Sleep mode Set and cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCSMEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<ADCSMEN> for bool {
    #[inline(always)]
    fn from(variant: ADCSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCSMEN` reader - ADC clock enable during Sleep mode Set and cleared by software.
pub type ADCSMEN_R = crate::BitReader<ADCSMEN>;
impl ADCSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCSMEN {
        match self.bits {
            false => ADCSMEN::B0x0,
            true => ADCSMEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADCSMEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADCSMEN::B0x1
    }
}
///Field `ADCSMEN` writer - ADC clock enable during Sleep mode Set and cleared by software.
pub type ADCSMEN_W<'a, REG> = crate::BitWriter<'a, REG, ADCSMEN>;
impl<'a, REG> ADCSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSMEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSMEN::B0x1)
    }
}
impl R {
    ///Bit 0 - SYSCFG clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 11 - TIM1 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TIM14 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim14smen(&self) -> TIM14SMEN_R {
        TIM14SMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - TIM16 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM16 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - ADC clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn adcsmen(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APBSMENR2")
            .field("syscfgsmen", &self.syscfgsmen())
            .field("tim1smen", &self.tim1smen())
            .field("spi1smen", &self.spi1smen())
            .field("usart1smen", &self.usart1smen())
            .field("tim14smen", &self.tim14smen())
            .field("tim16smen", &self.tim16smen())
            .field("tim17smen", &self.tim17smen())
            .field("adcsmen", &self.adcsmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSCFG clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W<'_, RCC_APBSMENR2rs> {
        SYSCFGSMEN_W::new(self, 0)
    }
    ///Bit 11 - TIM1 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W<'_, RCC_APBSMENR2rs> {
        TIM1SMEN_W::new(self, 11)
    }
    ///Bit 12 - SPI1 clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<'_, RCC_APBSMENR2rs> {
        SPI1SMEN_W::new(self, 12)
    }
    ///Bit 14 - USART1 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<'_, RCC_APBSMENR2rs> {
        USART1SMEN_W::new(self, 14)
    }
    ///Bit 15 - TIM14 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim14smen(&mut self) -> TIM14SMEN_W<'_, RCC_APBSMENR2rs> {
        TIM14SMEN_W::new(self, 15)
    }
    ///Bit 17 - TIM16 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W<'_, RCC_APBSMENR2rs> {
        TIM16SMEN_W::new(self, 17)
    }
    ///Bit 18 - TIM16 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W<'_, RCC_APBSMENR2rs> {
        TIM17SMEN_W::new(self, 18)
    }
    ///Bit 20 - ADC clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn adcsmen(&mut self) -> ADCSMEN_W<'_, RCC_APBSMENR2rs> {
        ADCSMEN_W::new(self, 20)
    }
}
/**RCC APB peripheral clock enable in Sleep/Stop mode register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_apbsmenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbsmenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#RCC:RCC_APBSMENR2)*/
pub struct RCC_APBSMENR2rs;
impl crate::RegisterSpec for RCC_APBSMENR2rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_apbsmenr2::R`](R) reader structure
impl crate::Readable for RCC_APBSMENR2rs {}
///`write(|w| ..)` method takes [`rcc_apbsmenr2::W`](W) writer structure
impl crate::Writable for RCC_APBSMENR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_APBSMENR2 to value 0x0016_d801
impl crate::Resettable for RCC_APBSMENR2rs {
    const RESET_VALUE: u32 = 0x0016_d801;
}
