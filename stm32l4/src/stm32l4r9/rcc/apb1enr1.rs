///Register `APB1ENR1` reader
pub type R = crate::R<APB1ENR1rs>;
///Register `APB1ENR1` writer
pub type W = crate::W<APB1ENR1rs>;
///Field `TIM2EN` reader - TIM2 timer clock enable
pub type TIM2EN_R = crate::BitReader;
///Field `TIM2EN` writer - TIM2 timer clock enable
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3EN` reader - TIM3 timer clock enable
pub type TIM3EN_R = crate::BitReader;
///Field `TIM3EN` writer - TIM3 timer clock enable
pub type TIM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM4EN` reader - TIM4 timer clock enable
pub type TIM4EN_R = crate::BitReader;
///Field `TIM4EN` writer - TIM4 timer clock enable
pub type TIM4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM5EN` reader - TIM5 timer clock enable
pub type TIM5EN_R = crate::BitReader;
///Field `TIM5EN` writer - TIM5 timer clock enable
pub type TIM5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6EN` reader - TIM6 timer clock enable
pub type TIM6EN_R = crate::BitReader;
///Field `TIM6EN` writer - TIM6 timer clock enable
pub type TIM6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7EN` reader - TIM7 timer clock enable
pub type TIM7EN_R = crate::BitReader;
///Field `TIM7EN` writer - TIM7 timer clock enable
pub type TIM7EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBEN` reader - RTC APB clock enable
pub type RTCAPBEN_R = crate::BitReader;
///Field `RTCAPBEN` writer - RTC APB clock enable
pub type RTCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGEN` reader - Window watchdog clock enable
pub type WWDGEN_R = crate::BitReader;
///Field `WWDGEN` writer - Window watchdog clock enable
pub type WWDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2EN` reader - SPI2 clock enable
pub type SPI2EN_R = crate::BitReader;
///Field `SPI2EN` writer - SPI2 clock enable
pub type SPI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3EN` reader - SPI peripheral 3 clock enable
pub type SPI3EN_R = crate::BitReader;
///Field `SPI3EN` writer - SPI peripheral 3 clock enable
pub type SPI3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2EN` reader - USART2 clock enable
pub type USART2EN_R = crate::BitReader;
///Field `USART2EN` writer - USART2 clock enable
pub type USART2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3EN` reader - USART3 clock enable
pub type USART3EN_R = crate::BitReader;
///Field `USART3EN` writer - USART3 clock enable
pub type USART3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART4EN` reader - UART4 clock enable
pub type UART4EN_R = crate::BitReader;
///Field `UART4EN` writer - UART4 clock enable
pub type UART4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART5EN` reader - UART5 clock enable
pub type UART5EN_R = crate::BitReader;
///Field `UART5EN` writer - UART5 clock enable
pub type UART5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**I2C1 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1EN {
    ///0: I2C1 clock disabled
    Disabled = 0,
    ///1: I2C1 clock enabled
    Enabled = 1,
}
impl From<I2C1EN> for bool {
    #[inline(always)]
    fn from(variant: I2C1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C1EN` reader - I2C1 clock enable
pub type I2C1EN_R = crate::BitReader<I2C1EN>;
impl I2C1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C1EN {
        match self.bits {
            false => I2C1EN::Disabled,
            true => I2C1EN::Enabled,
        }
    }
    ///I2C1 clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C1EN::Disabled
    }
    ///I2C1 clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C1EN::Enabled
    }
}
///Field `I2C1EN` writer - I2C1 clock enable
pub type I2C1EN_W<'a, REG> = crate::BitWriter<'a, REG, I2C1EN>;
impl<'a, REG> I2C1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///I2C1 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1EN::Disabled)
    }
    ///I2C1 clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1EN::Enabled)
    }
}
/**I2C2 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2EN {
    ///0: I2C2 clock disabled
    Disabled = 0,
    ///1: I2C2 clock enabled
    Enabled = 1,
}
impl From<I2C2EN> for bool {
    #[inline(always)]
    fn from(variant: I2C2EN) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C2EN` reader - I2C2 clock enable
pub type I2C2EN_R = crate::BitReader<I2C2EN>;
impl I2C2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C2EN {
        match self.bits {
            false => I2C2EN::Disabled,
            true => I2C2EN::Enabled,
        }
    }
    ///I2C2 clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C2EN::Disabled
    }
    ///I2C2 clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C2EN::Enabled
    }
}
///Field `I2C2EN` writer - I2C2 clock enable
pub type I2C2EN_W<'a, REG> = crate::BitWriter<'a, REG, I2C2EN>;
impl<'a, REG> I2C2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///I2C2 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2EN::Disabled)
    }
    ///I2C2 clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2EN::Enabled)
    }
}
/**I2C3 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C3EN {
    ///0: I2C3 clock disabled
    Disabled = 0,
    ///1: I2C3 clock enabled
    Enabled = 1,
}
impl From<I2C3EN> for bool {
    #[inline(always)]
    fn from(variant: I2C3EN) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C3EN` reader - I2C3 clock enable
pub type I2C3EN_R = crate::BitReader<I2C3EN>;
impl I2C3EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C3EN {
        match self.bits {
            false => I2C3EN::Disabled,
            true => I2C3EN::Enabled,
        }
    }
    ///I2C3 clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C3EN::Disabled
    }
    ///I2C3 clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C3EN::Enabled
    }
}
///Field `I2C3EN` writer - I2C3 clock enable
pub type I2C3EN_W<'a, REG> = crate::BitWriter<'a, REG, I2C3EN>;
impl<'a, REG> I2C3EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///I2C3 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C3EN::Disabled)
    }
    ///I2C3 clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C3EN::Enabled)
    }
}
///Field `CRSEN` reader - Clock Recovery System clock enable
pub type CRSEN_R = crate::BitReader;
///Field `CRSEN` writer - Clock Recovery System clock enable
pub type CRSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAN1EN` reader - CAN1 clock enable
pub type CAN1EN_R = crate::BitReader;
///Field `CAN1EN` writer - CAN1 clock enable
pub type CAN1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWREN` reader - Power interface clock enable
pub type PWREN_R = crate::BitReader;
///Field `PWREN` writer - Power interface clock enable
pub type PWREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC1EN` reader - DAC1 interface clock enable
pub type DAC1EN_R = crate::BitReader;
///Field `DAC1EN` writer - DAC1 interface clock enable
pub type DAC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPAMPEN` reader - OPAMP interface clock enable
pub type OPAMPEN_R = crate::BitReader;
///Field `OPAMPEN` writer - OPAMP interface clock enable
pub type OPAMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Low power timer 1 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM1EN {
    ///0: LPTIM1 clock disabled
    Disabled = 0,
    ///1: LPTIM1 clock enabled
    Enabled = 1,
}
impl From<LPTIM1EN> for bool {
    #[inline(always)]
    fn from(variant: LPTIM1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `LPTIM1EN` reader - Low power timer 1 clock enable
pub type LPTIM1EN_R = crate::BitReader<LPTIM1EN>;
impl LPTIM1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM1EN {
        match self.bits {
            false => LPTIM1EN::Disabled,
            true => LPTIM1EN::Enabled,
        }
    }
    ///LPTIM1 clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPTIM1EN::Disabled
    }
    ///LPTIM1 clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPTIM1EN::Enabled
    }
}
///Field `LPTIM1EN` writer - Low power timer 1 clock enable
pub type LPTIM1EN_W<'a, REG> = crate::BitWriter<'a, REG, LPTIM1EN>;
impl<'a, REG> LPTIM1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LPTIM1 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1EN::Disabled)
    }
    ///LPTIM1 clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1EN::Enabled)
    }
}
impl R {
    ///Bit 0 - TIM2 timer clock enable
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 timer clock enable
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 timer clock enable
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 timer clock enable
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 timer clock enable
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 timer clock enable
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 10 - RTC APB clock enable
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clock enable
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI peripheral 3 clock enable
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 clock enable
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 clock enable
    #[inline(always)]
    pub fn uart4en(&self) -> UART4EN_R {
        UART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 clock enable
    #[inline(always)]
    pub fn uart5en(&self) -> UART5EN_R {
        UART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3 clock enable
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Clock Recovery System clock enable
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - CAN1 clock enable
    #[inline(always)]
    pub fn can1en(&self) -> CAN1EN_R {
        CAN1EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC1 interface clock enable
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - OPAMP interface clock enable
    #[inline(always)]
    pub fn opampen(&self) -> OPAMPEN_R {
        OPAMPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low power timer 1 clock enable
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1ENR1")
            .field("tim2en", &self.tim2en())
            .field("tim3en", &self.tim3en())
            .field("tim4en", &self.tim4en())
            .field("tim5en", &self.tim5en())
            .field("tim6en", &self.tim6en())
            .field("tim7en", &self.tim7en())
            .field("rtcapben", &self.rtcapben())
            .field("wwdgen", &self.wwdgen())
            .field("spi2en", &self.spi2en())
            .field("spi3en", &self.spi3en())
            .field("usart2en", &self.usart2en())
            .field("usart3en", &self.usart3en())
            .field("uart4en", &self.uart4en())
            .field("uart5en", &self.uart5en())
            .field("i2c1en", &self.i2c1en())
            .field("i2c2en", &self.i2c2en())
            .field("i2c3en", &self.i2c3en())
            .field("crsen", &self.crsen())
            .field("can1en", &self.can1en())
            .field("pwren", &self.pwren())
            .field("dac1en", &self.dac1en())
            .field("opampen", &self.opampen())
            .field("lptim1en", &self.lptim1en())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 timer clock enable
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W<'_, APB1ENR1rs> {
        TIM2EN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 timer clock enable
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W<'_, APB1ENR1rs> {
        TIM3EN_W::new(self, 1)
    }
    ///Bit 2 - TIM4 timer clock enable
    #[inline(always)]
    pub fn tim4en(&mut self) -> TIM4EN_W<'_, APB1ENR1rs> {
        TIM4EN_W::new(self, 2)
    }
    ///Bit 3 - TIM5 timer clock enable
    #[inline(always)]
    pub fn tim5en(&mut self) -> TIM5EN_W<'_, APB1ENR1rs> {
        TIM5EN_W::new(self, 3)
    }
    ///Bit 4 - TIM6 timer clock enable
    #[inline(always)]
    pub fn tim6en(&mut self) -> TIM6EN_W<'_, APB1ENR1rs> {
        TIM6EN_W::new(self, 4)
    }
    ///Bit 5 - TIM7 timer clock enable
    #[inline(always)]
    pub fn tim7en(&mut self) -> TIM7EN_W<'_, APB1ENR1rs> {
        TIM7EN_W::new(self, 5)
    }
    ///Bit 10 - RTC APB clock enable
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<'_, APB1ENR1rs> {
        RTCAPBEN_W::new(self, 10)
    }
    ///Bit 11 - Window watchdog clock enable
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WWDGEN_W<'_, APB1ENR1rs> {
        WWDGEN_W::new(self, 11)
    }
    ///Bit 14 - SPI2 clock enable
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W<'_, APB1ENR1rs> {
        SPI2EN_W::new(self, 14)
    }
    ///Bit 15 - SPI peripheral 3 clock enable
    #[inline(always)]
    pub fn spi3en(&mut self) -> SPI3EN_W<'_, APB1ENR1rs> {
        SPI3EN_W::new(self, 15)
    }
    ///Bit 17 - USART2 clock enable
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W<'_, APB1ENR1rs> {
        USART2EN_W::new(self, 17)
    }
    ///Bit 18 - USART3 clock enable
    #[inline(always)]
    pub fn usart3en(&mut self) -> USART3EN_W<'_, APB1ENR1rs> {
        USART3EN_W::new(self, 18)
    }
    ///Bit 19 - UART4 clock enable
    #[inline(always)]
    pub fn uart4en(&mut self) -> UART4EN_W<'_, APB1ENR1rs> {
        UART4EN_W::new(self, 19)
    }
    ///Bit 20 - UART5 clock enable
    #[inline(always)]
    pub fn uart5en(&mut self) -> UART5EN_W<'_, APB1ENR1rs> {
        UART5EN_W::new(self, 20)
    }
    ///Bit 21 - I2C1 clock enable
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<'_, APB1ENR1rs> {
        I2C1EN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W<'_, APB1ENR1rs> {
        I2C2EN_W::new(self, 22)
    }
    ///Bit 23 - I2C3 clock enable
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2C3EN_W<'_, APB1ENR1rs> {
        I2C3EN_W::new(self, 23)
    }
    ///Bit 24 - Clock Recovery System clock enable
    #[inline(always)]
    pub fn crsen(&mut self) -> CRSEN_W<'_, APB1ENR1rs> {
        CRSEN_W::new(self, 24)
    }
    ///Bit 25 - CAN1 clock enable
    #[inline(always)]
    pub fn can1en(&mut self) -> CAN1EN_W<'_, APB1ENR1rs> {
        CAN1EN_W::new(self, 25)
    }
    ///Bit 28 - Power interface clock enable
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W<'_, APB1ENR1rs> {
        PWREN_W::new(self, 28)
    }
    ///Bit 29 - DAC1 interface clock enable
    #[inline(always)]
    pub fn dac1en(&mut self) -> DAC1EN_W<'_, APB1ENR1rs> {
        DAC1EN_W::new(self, 29)
    }
    ///Bit 30 - OPAMP interface clock enable
    #[inline(always)]
    pub fn opampen(&mut self) -> OPAMPEN_W<'_, APB1ENR1rs> {
        OPAMPEN_W::new(self, 30)
    }
    ///Bit 31 - Low power timer 1 clock enable
    #[inline(always)]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<'_, APB1ENR1rs> {
        LPTIM1EN_W::new(self, 31)
    }
}
/**APB1ENR1

You can [`read`](crate::Reg::read) this register and get [`apb1enr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#RCC:APB1ENR1)*/
pub struct APB1ENR1rs;
impl crate::RegisterSpec for APB1ENR1rs {
    type Ux = u32;
}
///`read()` method returns [`apb1enr1::R`](R) reader structure
impl crate::Readable for APB1ENR1rs {}
///`write(|w| ..)` method takes [`apb1enr1::W`](W) writer structure
impl crate::Writable for APB1ENR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1ENR1 to value 0
impl crate::Resettable for APB1ENR1rs {}
