///Register `RCC_APB1RSTR1` reader
pub type R = crate::R<RCC_APB1RSTR1rs>;
///Register `RCC_APB1RSTR1` writer
pub type W = crate::W<RCC_APB1RSTR1rs>;
///Field `TIM2RST` reader - TIM2 reset This bit is set and cleared by software.
pub type TIM2RST_R = crate::BitReader;
///Field `TIM2RST` writer - TIM2 reset This bit is set and cleared by software.
pub type TIM2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3RST` reader - TIM3 reset This bit is set and cleared by software.
pub type TIM3RST_R = crate::BitReader;
///Field `TIM3RST` writer - TIM3 reset This bit is set and cleared by software.
pub type TIM3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM4RST` reader - TIM4 reset This bit is set and cleared by software.
pub type TIM4RST_R = crate::BitReader;
///Field `TIM4RST` writer - TIM4 reset This bit is set and cleared by software.
pub type TIM4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM5RST` reader - TIM5 reset This bit is set and cleared by software.
pub type TIM5RST_R = crate::BitReader;
///Field `TIM5RST` writer - TIM5 reset This bit is set and cleared by software.
pub type TIM5RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6RST` reader - TIM6 reset This bit is set and cleared by software.
pub type TIM6RST_R = crate::BitReader;
///Field `TIM6RST` writer - TIM6 reset This bit is set and cleared by software.
pub type TIM6RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7RST` reader - TIM7 reset This bit is set and cleared by software.
pub type TIM7RST_R = crate::BitReader;
///Field `TIM7RST` writer - TIM7 reset This bit is set and cleared by software.
pub type TIM7RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2RST` reader - SPI2 reset This bit is set and cleared by software.
pub type SPI2RST_R = crate::BitReader;
///Field `SPI2RST` writer - SPI2 reset This bit is set and cleared by software.
pub type SPI2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2RST` reader - USART2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series.Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type USART2RST_R = crate::BitReader;
///Field `USART2RST` writer - USART2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series.Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type USART2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3RST` reader - USART3 reset This bit is set and cleared by software.
pub type USART3RST_R = crate::BitReader;
///Field `USART3RST` writer - USART3 reset This bit is set and cleared by software.
pub type USART3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART4RST` reader - UART4 reset This bit is set and cleared by software.
pub type UART4RST_R = crate::BitReader;
///Field `UART4RST` writer - UART4 reset This bit is set and cleared by software.
pub type UART4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART5RST` reader - UART5 reset This bit is set and cleared by software.
pub type UART5RST_R = crate::BitReader;
///Field `UART5RST` writer - UART5 reset This bit is set and cleared by software.
pub type UART5RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1RST` reader - I2C1 reset This bit is set and cleared by software.
pub type I2C1RST_R = crate::BitReader;
///Field `I2C1RST` writer - I2C1 reset This bit is set and cleared by software.
pub type I2C1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2RST` reader - I2C2 reset This bit is set and cleared by software.
pub type I2C2RST_R = crate::BitReader;
///Field `I2C2RST` writer - I2C2 reset This bit is set and cleared by software.
pub type I2C2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRSRST` reader - CRS reset This bit is set and cleared by software.
pub type CRSRST_R = crate::BitReader;
///Field `CRSRST` writer - CRS reset This bit is set and cleared by software.
pub type CRSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART6RST` reader - USART6 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type USART6RST_R = crate::BitReader;
///Field `USART6RST` writer - USART6 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type USART6RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 14 - SPI2 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - USART2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series.Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn uart5rst(&self) -> UART5RST_R {
        UART5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - CRS reset This bit is set and cleared by software.
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - USART6 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn usart6rst(&self) -> USART6RST_R {
        USART6RST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB1RSTR1")
            .field("tim2rst", &self.tim2rst())
            .field("tim3rst", &self.tim3rst())
            .field("tim4rst", &self.tim4rst())
            .field("tim5rst", &self.tim5rst())
            .field("tim6rst", &self.tim6rst())
            .field("tim7rst", &self.tim7rst())
            .field("spi2rst", &self.spi2rst())
            .field("usart2rst", &self.usart2rst())
            .field("usart3rst", &self.usart3rst())
            .field("uart4rst", &self.uart4rst())
            .field("uart5rst", &self.uart5rst())
            .field("i2c1rst", &self.i2c1rst())
            .field("i2c2rst", &self.i2c2rst())
            .field("crsrst", &self.crsrst())
            .field("usart6rst", &self.usart6rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<RCC_APB1RSTR1rs> {
        TIM2RST_W::new(self, 0)
    }
    ///Bit 1 - TIM3 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> TIM3RST_W<RCC_APB1RSTR1rs> {
        TIM3RST_W::new(self, 1)
    }
    ///Bit 2 - TIM4 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim4rst(&mut self) -> TIM4RST_W<RCC_APB1RSTR1rs> {
        TIM4RST_W::new(self, 2)
    }
    ///Bit 3 - TIM5 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim5rst(&mut self) -> TIM5RST_W<RCC_APB1RSTR1rs> {
        TIM5RST_W::new(self, 3)
    }
    ///Bit 4 - TIM6 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> TIM6RST_W<RCC_APB1RSTR1rs> {
        TIM6RST_W::new(self, 4)
    }
    ///Bit 5 - TIM7 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim7rst(&mut self) -> TIM7RST_W<RCC_APB1RSTR1rs> {
        TIM7RST_W::new(self, 5)
    }
    ///Bit 14 - SPI2 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<RCC_APB1RSTR1rs> {
        SPI2RST_W::new(self, 14)
    }
    ///Bit 17 - USART2 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series.Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<RCC_APB1RSTR1rs> {
        USART2RST_W::new(self, 17)
    }
    ///Bit 18 - USART3 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> USART3RST_W<RCC_APB1RSTR1rs> {
        USART3RST_W::new(self, 18)
    }
    ///Bit 19 - UART4 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn uart4rst(&mut self) -> UART4RST_W<RCC_APB1RSTR1rs> {
        UART4RST_W::new(self, 19)
    }
    ///Bit 20 - UART5 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn uart5rst(&mut self) -> UART5RST_W<RCC_APB1RSTR1rs> {
        UART5RST_W::new(self, 20)
    }
    ///Bit 21 - I2C1 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<RCC_APB1RSTR1rs> {
        I2C1RST_W::new(self, 21)
    }
    ///Bit 22 - I2C2 reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<RCC_APB1RSTR1rs> {
        I2C2RST_W::new(self, 22)
    }
    ///Bit 24 - CRS reset This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn crsrst(&mut self) -> CRSRST_W<RCC_APB1RSTR1rs> {
        CRSRST_W::new(self, 24)
    }
    ///Bit 25 - USART6 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn usart6rst(&mut self) -> USART6RST_W<RCC_APB1RSTR1rs> {
        USART6RST_W::new(self, 25)
    }
}
/**RCC APB1 peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_apb1rstr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1rstr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#RCC:RCC_APB1RSTR1)*/
pub struct RCC_APB1RSTR1rs;
impl crate::RegisterSpec for RCC_APB1RSTR1rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_apb1rstr1::R`](R) reader structure
impl crate::Readable for RCC_APB1RSTR1rs {}
///`write(|w| ..)` method takes [`rcc_apb1rstr1::W`](W) writer structure
impl crate::Writable for RCC_APB1RSTR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_APB1RSTR1 to value 0
impl crate::Resettable for RCC_APB1RSTR1rs {
    const RESET_VALUE: u32 = 0;
}
