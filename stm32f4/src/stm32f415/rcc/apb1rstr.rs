///Register `APB1RSTR` reader
pub type R = crate::R<APB1RSTRrs>;
///Register `APB1RSTR` writer
pub type W = crate::W<APB1RSTRrs>;
///Field `TIM2RST` reader - TIM2 reset
pub type TIM2RST_R = crate::BitReader;
///Field `TIM2RST` writer - TIM2 reset
pub type TIM2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3RST` reader - TIM3 reset
pub type TIM3RST_R = crate::BitReader;
///Field `TIM3RST` writer - TIM3 reset
pub type TIM3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM4RST` reader - TIM4 reset
pub type TIM4RST_R = crate::BitReader;
///Field `TIM4RST` writer - TIM4 reset
pub type TIM4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM5RST` reader - TIM5 reset
pub type TIM5RST_R = crate::BitReader;
///Field `TIM5RST` writer - TIM5 reset
pub type TIM5RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6RST` reader - TIM6 reset
pub type TIM6RST_R = crate::BitReader;
///Field `TIM6RST` writer - TIM6 reset
pub type TIM6RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7RST` reader - TIM7 reset
pub type TIM7RST_R = crate::BitReader;
///Field `TIM7RST` writer - TIM7 reset
pub type TIM7RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM12RST` reader - TIM12 reset
pub type TIM12RST_R = crate::BitReader;
///Field `TIM12RST` writer - TIM12 reset
pub type TIM12RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM13RST` reader - TIM13 reset
pub type TIM13RST_R = crate::BitReader;
///Field `TIM13RST` writer - TIM13 reset
pub type TIM13RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM14RST` reader - TIM14 reset
pub type TIM14RST_R = crate::BitReader;
///Field `TIM14RST` writer - TIM14 reset
pub type TIM14RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGRST` reader - Window watchdog reset
pub type WWDGRST_R = crate::BitReader;
///Field `WWDGRST` writer - Window watchdog reset
pub type WWDGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2RST` reader - SPI 2 reset
pub type SPI2RST_R = crate::BitReader;
///Field `SPI2RST` writer - SPI 2 reset
pub type SPI2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3RST` reader - SPI 3 reset
pub type SPI3RST_R = crate::BitReader;
///Field `SPI3RST` writer - SPI 3 reset
pub type SPI3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART2RST` reader - USART 2 reset
pub type UART2RST_R = crate::BitReader;
///Field `UART2RST` writer - USART 2 reset
pub type UART2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART3RST` reader - USART 3 reset
pub type UART3RST_R = crate::BitReader;
///Field `UART3RST` writer - USART 3 reset
pub type UART3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART4RST` reader - USART 4 reset
pub type UART4RST_R = crate::BitReader;
///Field `UART4RST` writer - USART 4 reset
pub type UART4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART5RST` reader - USART 5 reset
pub type UART5RST_R = crate::BitReader;
///Field `UART5RST` writer - USART 5 reset
pub type UART5RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1RST` reader - I2C 1 reset
pub type I2C1RST_R = crate::BitReader;
///Field `I2C1RST` writer - I2C 1 reset
pub type I2C1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2RST` reader - I2C 2 reset
pub type I2C2RST_R = crate::BitReader;
///Field `I2C2RST` writer - I2C 2 reset
pub type I2C2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3RST` reader - I2C3 reset
pub type I2C3RST_R = crate::BitReader;
///Field `I2C3RST` writer - I2C3 reset
pub type I2C3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAN1RST` reader - CAN1 reset
pub type CAN1RST_R = crate::BitReader;
///Field `CAN1RST` writer - CAN1 reset
pub type CAN1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAN2RST` reader - CAN2 reset
pub type CAN2RST_R = crate::BitReader;
///Field `CAN2RST` writer - CAN2 reset
pub type CAN2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRRST` reader - Power interface reset
pub type PWRRST_R = crate::BitReader;
///Field `PWRRST` writer - Power interface reset
pub type PWRRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DACRST` reader - DAC reset
pub type DACRST_R = crate::BitReader;
///Field `DACRST` writer - DAC reset
pub type DACRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART7RST` reader - UART7 reset
pub type UART7RST_R = crate::BitReader;
///Field `UART7RST` writer - UART7 reset
pub type UART7RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART8RST` reader - UART8 reset
pub type UART8RST_R = crate::BitReader;
///Field `UART8RST` writer - UART8 reset
pub type UART8RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2 reset
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 reset
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 reset
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 reset
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 reset
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 reset
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TIM12 reset
    #[inline(always)]
    pub fn tim12rst(&self) -> TIM12RST_R {
        TIM12RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TIM13 reset
    #[inline(always)]
    pub fn tim13rst(&self) -> TIM13RST_R {
        TIM13RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TIM14 reset
    #[inline(always)]
    pub fn tim14rst(&self) -> TIM14RST_R {
        TIM14RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Window watchdog reset
    #[inline(always)]
    pub fn wwdgrst(&self) -> WWDGRST_R {
        WWDGRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI 2 reset
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI 3 reset
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - USART 2 reset
    #[inline(always)]
    pub fn uart2rst(&self) -> UART2RST_R {
        UART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART 3 reset
    #[inline(always)]
    pub fn uart3rst(&self) -> UART3RST_R {
        UART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - USART 4 reset
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - USART 5 reset
    #[inline(always)]
    pub fn uart5rst(&self) -> UART5RST_R {
        UART5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C 1 reset
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C 2 reset
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3 reset
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - CAN1 reset
    #[inline(always)]
    pub fn can1rst(&self) -> CAN1RST_R {
        CAN1RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - CAN2 reset
    #[inline(always)]
    pub fn can2rst(&self) -> CAN2RST_R {
        CAN2RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC reset
    #[inline(always)]
    pub fn dacrst(&self) -> DACRST_R {
        DACRST_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - UART7 reset
    #[inline(always)]
    pub fn uart7rst(&self) -> UART7RST_R {
        UART7RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - UART8 reset
    #[inline(always)]
    pub fn uart8rst(&self) -> UART8RST_R {
        UART8RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RSTR")
            .field("tim2rst", &self.tim2rst())
            .field("tim3rst", &self.tim3rst())
            .field("tim4rst", &self.tim4rst())
            .field("tim5rst", &self.tim5rst())
            .field("tim6rst", &self.tim6rst())
            .field("tim7rst", &self.tim7rst())
            .field("tim12rst", &self.tim12rst())
            .field("tim13rst", &self.tim13rst())
            .field("tim14rst", &self.tim14rst())
            .field("wwdgrst", &self.wwdgrst())
            .field("spi2rst", &self.spi2rst())
            .field("spi3rst", &self.spi3rst())
            .field("uart2rst", &self.uart2rst())
            .field("uart3rst", &self.uart3rst())
            .field("uart4rst", &self.uart4rst())
            .field("uart5rst", &self.uart5rst())
            .field("i2c1rst", &self.i2c1rst())
            .field("i2c2rst", &self.i2c2rst())
            .field("i2c3rst", &self.i2c3rst())
            .field("can1rst", &self.can1rst())
            .field("can2rst", &self.can2rst())
            .field("pwrrst", &self.pwrrst())
            .field("dacrst", &self.dacrst())
            .field("uart7rst", &self.uart7rst())
            .field("uart8rst", &self.uart8rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 reset
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W<'_, APB1RSTRrs> {
        TIM2RST_W::new(self, 0)
    }
    ///Bit 1 - TIM3 reset
    #[inline(always)]
    pub fn tim3rst(&mut self) -> TIM3RST_W<'_, APB1RSTRrs> {
        TIM3RST_W::new(self, 1)
    }
    ///Bit 2 - TIM4 reset
    #[inline(always)]
    pub fn tim4rst(&mut self) -> TIM4RST_W<'_, APB1RSTRrs> {
        TIM4RST_W::new(self, 2)
    }
    ///Bit 3 - TIM5 reset
    #[inline(always)]
    pub fn tim5rst(&mut self) -> TIM5RST_W<'_, APB1RSTRrs> {
        TIM5RST_W::new(self, 3)
    }
    ///Bit 4 - TIM6 reset
    #[inline(always)]
    pub fn tim6rst(&mut self) -> TIM6RST_W<'_, APB1RSTRrs> {
        TIM6RST_W::new(self, 4)
    }
    ///Bit 5 - TIM7 reset
    #[inline(always)]
    pub fn tim7rst(&mut self) -> TIM7RST_W<'_, APB1RSTRrs> {
        TIM7RST_W::new(self, 5)
    }
    ///Bit 6 - TIM12 reset
    #[inline(always)]
    pub fn tim12rst(&mut self) -> TIM12RST_W<'_, APB1RSTRrs> {
        TIM12RST_W::new(self, 6)
    }
    ///Bit 7 - TIM13 reset
    #[inline(always)]
    pub fn tim13rst(&mut self) -> TIM13RST_W<'_, APB1RSTRrs> {
        TIM13RST_W::new(self, 7)
    }
    ///Bit 8 - TIM14 reset
    #[inline(always)]
    pub fn tim14rst(&mut self) -> TIM14RST_W<'_, APB1RSTRrs> {
        TIM14RST_W::new(self, 8)
    }
    ///Bit 11 - Window watchdog reset
    #[inline(always)]
    pub fn wwdgrst(&mut self) -> WWDGRST_W<'_, APB1RSTRrs> {
        WWDGRST_W::new(self, 11)
    }
    ///Bit 14 - SPI 2 reset
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W<'_, APB1RSTRrs> {
        SPI2RST_W::new(self, 14)
    }
    ///Bit 15 - SPI 3 reset
    #[inline(always)]
    pub fn spi3rst(&mut self) -> SPI3RST_W<'_, APB1RSTRrs> {
        SPI3RST_W::new(self, 15)
    }
    ///Bit 17 - USART 2 reset
    #[inline(always)]
    pub fn uart2rst(&mut self) -> UART2RST_W<'_, APB1RSTRrs> {
        UART2RST_W::new(self, 17)
    }
    ///Bit 18 - USART 3 reset
    #[inline(always)]
    pub fn uart3rst(&mut self) -> UART3RST_W<'_, APB1RSTRrs> {
        UART3RST_W::new(self, 18)
    }
    ///Bit 19 - USART 4 reset
    #[inline(always)]
    pub fn uart4rst(&mut self) -> UART4RST_W<'_, APB1RSTRrs> {
        UART4RST_W::new(self, 19)
    }
    ///Bit 20 - USART 5 reset
    #[inline(always)]
    pub fn uart5rst(&mut self) -> UART5RST_W<'_, APB1RSTRrs> {
        UART5RST_W::new(self, 20)
    }
    ///Bit 21 - I2C 1 reset
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<'_, APB1RSTRrs> {
        I2C1RST_W::new(self, 21)
    }
    ///Bit 22 - I2C 2 reset
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<'_, APB1RSTRrs> {
        I2C2RST_W::new(self, 22)
    }
    ///Bit 23 - I2C3 reset
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<'_, APB1RSTRrs> {
        I2C3RST_W::new(self, 23)
    }
    ///Bit 25 - CAN1 reset
    #[inline(always)]
    pub fn can1rst(&mut self) -> CAN1RST_W<'_, APB1RSTRrs> {
        CAN1RST_W::new(self, 25)
    }
    ///Bit 26 - CAN2 reset
    #[inline(always)]
    pub fn can2rst(&mut self) -> CAN2RST_W<'_, APB1RSTRrs> {
        CAN2RST_W::new(self, 26)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PWRRST_W<'_, APB1RSTRrs> {
        PWRRST_W::new(self, 28)
    }
    ///Bit 29 - DAC reset
    #[inline(always)]
    pub fn dacrst(&mut self) -> DACRST_W<'_, APB1RSTRrs> {
        DACRST_W::new(self, 29)
    }
    ///Bit 30 - UART7 reset
    #[inline(always)]
    pub fn uart7rst(&mut self) -> UART7RST_W<'_, APB1RSTRrs> {
        UART7RST_W::new(self, 30)
    }
    ///Bit 31 - UART8 reset
    #[inline(always)]
    pub fn uart8rst(&mut self) -> UART8RST_W<'_, APB1RSTRrs> {
        UART8RST_W::new(self, 31)
    }
}
/**APB1 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#RCC:APB1RSTR)*/
pub struct APB1RSTRrs;
impl crate::RegisterSpec for APB1RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1rstr::R`](R) reader structure
impl crate::Readable for APB1RSTRrs {}
///`write(|w| ..)` method takes [`apb1rstr::W`](W) writer structure
impl crate::Writable for APB1RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1RSTR to value 0
impl crate::Resettable for APB1RSTRrs {}
