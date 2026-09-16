///Register `RCC_APBENR2` reader
pub type R = crate::R<RCC_APBENR2rs>;
///Register `RCC_APBENR2` writer
pub type W = crate::W<RCC_APBENR2rs>;
/**SYSCFG clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<SYSCFGEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGEN` reader - SYSCFG clock enable Set and cleared by software.
pub type SYSCFGEN_R = crate::BitReader<SYSCFGEN>;
impl SYSCFGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGEN {
        match self.bits {
            false => SYSCFGEN::B0x0,
            true => SYSCFGEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SYSCFGEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SYSCFGEN::B0x1
    }
}
///Field `SYSCFGEN` writer - SYSCFG clock enable Set and cleared by software.
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGEN>;
impl<'a, REG> SYSCFGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::B0x1)
    }
}
/**TIM1 timer clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1EN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<TIM1EN> for bool {
    #[inline(always)]
    fn from(variant: TIM1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM1EN` reader - TIM1 timer clock enable Set and cleared by software.
pub type TIM1EN_R = crate::BitReader<TIM1EN>;
impl TIM1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM1EN {
        match self.bits {
            false => TIM1EN::B0x0,
            true => TIM1EN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM1EN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM1EN::B0x1
    }
}
///Field `TIM1EN` writer - TIM1 timer clock enable Set and cleared by software.
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM1EN>;
impl<'a, REG> TIM1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1EN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1EN::B0x1)
    }
}
/**SPI1 clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1EN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<SPI1EN> for bool {
    #[inline(always)]
    fn from(variant: SPI1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `SPI1EN` reader - SPI1 clock enable Set and cleared by software.
pub type SPI1EN_R = crate::BitReader<SPI1EN>;
impl SPI1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPI1EN {
        match self.bits {
            false => SPI1EN::B0x0,
            true => SPI1EN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI1EN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI1EN::B0x1
    }
}
///Field `SPI1EN` writer - SPI1 clock enable Set and cleared by software.
pub type SPI1EN_W<'a, REG> = crate::BitWriter<'a, REG, SPI1EN>;
impl<'a, REG> SPI1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1EN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1EN::B0x1)
    }
}
/**USART1 clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1EN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<USART1EN> for bool {
    #[inline(always)]
    fn from(variant: USART1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `USART1EN` reader - USART1 clock enable Set and cleared by software.
pub type USART1EN_R = crate::BitReader<USART1EN>;
impl USART1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART1EN {
        match self.bits {
            false => USART1EN::B0x0,
            true => USART1EN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART1EN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART1EN::B0x1
    }
}
///Field `USART1EN` writer - USART1 clock enable Set and cleared by software.
pub type USART1EN_W<'a, REG> = crate::BitWriter<'a, REG, USART1EN>;
impl<'a, REG> USART1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART1EN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART1EN::B0x1)
    }
}
/**TIM14 timer clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM14EN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<TIM14EN> for bool {
    #[inline(always)]
    fn from(variant: TIM14EN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM14EN` reader - TIM14 timer clock enable Set and cleared by software.
pub type TIM14EN_R = crate::BitReader<TIM14EN>;
impl TIM14EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM14EN {
        match self.bits {
            false => TIM14EN::B0x0,
            true => TIM14EN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM14EN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM14EN::B0x1
    }
}
///Field `TIM14EN` writer - TIM14 timer clock enable Set and cleared by software.
pub type TIM14EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM14EN>;
impl<'a, REG> TIM14EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM14EN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM14EN::B0x1)
    }
}
/**TIM16 timer clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM16EN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<TIM16EN> for bool {
    #[inline(always)]
    fn from(variant: TIM16EN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM16EN` reader - TIM16 timer clock enable Set and cleared by software.
pub type TIM16EN_R = crate::BitReader<TIM16EN>;
impl TIM16EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM16EN {
        match self.bits {
            false => TIM16EN::B0x0,
            true => TIM16EN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM16EN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM16EN::B0x1
    }
}
///Field `TIM16EN` writer - TIM16 timer clock enable Set and cleared by software.
pub type TIM16EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM16EN>;
impl<'a, REG> TIM16EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM16EN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM16EN::B0x1)
    }
}
/**TIM16 timer clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM17EN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<TIM17EN> for bool {
    #[inline(always)]
    fn from(variant: TIM17EN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM17EN` reader - TIM16 timer clock enable Set and cleared by software.
pub type TIM17EN_R = crate::BitReader<TIM17EN>;
impl TIM17EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM17EN {
        match self.bits {
            false => TIM17EN::B0x0,
            true => TIM17EN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM17EN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM17EN::B0x1
    }
}
///Field `TIM17EN` writer - TIM16 timer clock enable Set and cleared by software.
pub type TIM17EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM17EN>;
impl<'a, REG> TIM17EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM17EN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM17EN::B0x1)
    }
}
/**ADC clock enable Set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCEN {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<ADCEN> for bool {
    #[inline(always)]
    fn from(variant: ADCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCEN` reader - ADC clock enable Set and cleared by software.
pub type ADCEN_R = crate::BitReader<ADCEN>;
impl ADCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCEN {
        match self.bits {
            false => ADCEN::B0x0,
            true => ADCEN::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADCEN::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADCEN::B0x1
    }
}
///Field `ADCEN` writer - ADC clock enable Set and cleared by software.
pub type ADCEN_W<'a, REG> = crate::BitWriter<'a, REG, ADCEN>;
impl<'a, REG> ADCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADCEN::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADCEN::B0x1)
    }
}
impl R {
    ///Bit 0 - SYSCFG clock enable Set and cleared by software.
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 11 - TIM1 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TIM14 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim14en(&self) -> TIM14EN_R {
        TIM14EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - TIM16 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM16 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - ADC clock enable Set and cleared by software.
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APBENR2")
            .field("syscfgen", &self.syscfgen())
            .field("tim1en", &self.tim1en())
            .field("spi1en", &self.spi1en())
            .field("usart1en", &self.usart1en())
            .field("tim14en", &self.tim14en())
            .field("tim16en", &self.tim16en())
            .field("tim17en", &self.tim17en())
            .field("adcen", &self.adcen())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSCFG clock enable Set and cleared by software.
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<'_, RCC_APBENR2rs> {
        SYSCFGEN_W::new(self, 0)
    }
    ///Bit 11 - TIM1 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W<'_, RCC_APBENR2rs> {
        TIM1EN_W::new(self, 11)
    }
    ///Bit 12 - SPI1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<'_, RCC_APBENR2rs> {
        SPI1EN_W::new(self, 12)
    }
    ///Bit 14 - USART1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<'_, RCC_APBENR2rs> {
        USART1EN_W::new(self, 14)
    }
    ///Bit 15 - TIM14 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim14en(&mut self) -> TIM14EN_W<'_, RCC_APBENR2rs> {
        TIM14EN_W::new(self, 15)
    }
    ///Bit 17 - TIM16 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim16en(&mut self) -> TIM16EN_W<'_, RCC_APBENR2rs> {
        TIM16EN_W::new(self, 17)
    }
    ///Bit 18 - TIM16 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim17en(&mut self) -> TIM17EN_W<'_, RCC_APBENR2rs> {
        TIM17EN_W::new(self, 18)
    }
    ///Bit 20 - ADC clock enable Set and cleared by software.
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W<'_, RCC_APBENR2rs> {
        ADCEN_W::new(self, 20)
    }
}
/**RCC APB peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_apbenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#RCC:RCC_APBENR2)*/
pub struct RCC_APBENR2rs;
impl crate::RegisterSpec for RCC_APBENR2rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_apbenr2::R`](R) reader structure
impl crate::Readable for RCC_APBENR2rs {}
///`write(|w| ..)` method takes [`rcc_apbenr2::W`](W) writer structure
impl crate::Writable for RCC_APBENR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_APBENR2 to value 0
impl crate::Resettable for RCC_APBENR2rs {}
