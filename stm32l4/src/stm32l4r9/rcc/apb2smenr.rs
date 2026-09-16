///Register `APB2SMENR` reader
pub type R = crate::R<APB2SMENRrs>;
///Register `APB2SMENR` writer
pub type W = crate::W<APB2SMENRrs>;
///Field `SYSCFGSMEN` reader - SYSCFG clocks enable during Sleep and Stop modes
pub type SYSCFGSMEN_R = crate::BitReader;
///Field `SYSCFGSMEN` writer - SYSCFG clocks enable during Sleep and Stop modes
pub type SYSCFGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM1SMEN` reader - TIM1 timer clocks enable during Sleep and Stop modes
pub type TIM1SMEN_R = crate::BitReader;
///Field `TIM1SMEN` writer - TIM1 timer clocks enable during Sleep and Stop modes
pub type TIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1SMEN` reader - SPI1 clocks enable during Sleep and Stop modes
pub type SPI1SMEN_R = crate::BitReader;
///Field `SPI1SMEN` writer - SPI1 clocks enable during Sleep and Stop modes
pub type SPI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8SMEN` reader - TIM8 timer clocks enable during Sleep and Stop modes
pub type TIM8SMEN_R = crate::BitReader;
///Field `TIM8SMEN` writer - TIM8 timer clocks enable during Sleep and Stop modes
pub type TIM8SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1SMEN` reader - USART1clocks enable during Sleep and Stop modes
pub type USART1SMEN_R = crate::BitReader;
///Field `USART1SMEN` writer - USART1clocks enable during Sleep and Stop modes
pub type USART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15SMEN` reader - TIM15 timer clocks enable during Sleep and Stop modes
pub type TIM15SMEN_R = crate::BitReader;
///Field `TIM15SMEN` writer - TIM15 timer clocks enable during Sleep and Stop modes
pub type TIM15SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16SMEN` reader - TIM16 timer clocks enable during Sleep and Stop modes
pub type TIM16SMEN_R = crate::BitReader;
///Field `TIM16SMEN` writer - TIM16 timer clocks enable during Sleep and Stop modes
pub type TIM16SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17SMEN` reader - TIM17 timer clocks enable during Sleep and Stop modes
pub type TIM17SMEN_R = crate::BitReader;
///Field `TIM17SMEN` writer - TIM17 timer clocks enable during Sleep and Stop modes
pub type TIM17SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1SMEN` reader - SAI1 clocks enable during Sleep and Stop modes
pub type SAI1SMEN_R = crate::BitReader;
///Field `SAI1SMEN` writer - SAI1 clocks enable during Sleep and Stop modes
pub type SAI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI2SMEN` reader - SAI2 clocks enable during Sleep and Stop modes
pub type SAI2SMEN_R = crate::BitReader;
///Field `SAI2SMEN` writer - SAI2 clocks enable during Sleep and Stop modes
pub type SAI2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFSDM1SMEN` reader - DFSDM timer clocks enable during Sleep and Stop modes
pub type DFSDM1SMEN_R = crate::BitReader;
///Field `DFSDM1SMEN` writer - DFSDM timer clocks enable during Sleep and Stop modes
pub type DFSDM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LTDCSMEN` reader - LCD-TFT timer clocks enable during Sleep and Stop modes
pub type LTDCSMEN_R = crate::BitReader;
///Field `LTDCSMEN` writer - LCD-TFT timer clocks enable during Sleep and Stop modes
pub type LTDCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSISMEN` reader - DSI clocks enable during Sleep and Stop modes
pub type DSISMEN_R = crate::BitReader;
///Field `DSISMEN` writer - DSI clocks enable during Sleep and Stop modes
pub type DSISMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SYSCFG clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim8smen(&self) -> TIM8SMEN_R {
        TIM8SMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - USART1clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim15smen(&self) -> TIM15SMEN_R {
        TIM15SMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - SAI1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sai1smen(&self) -> SAI1SMEN_R {
        SAI1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SAI2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sai2smen(&self) -> SAI2SMEN_R {
        SAI2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - DFSDM timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dfsdm1smen(&self) -> DFSDM1SMEN_R {
        DFSDM1SMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - LCD-TFT timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn ltdcsmen(&self) -> LTDCSMEN_R {
        LTDCSMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - DSI clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dsismen(&self) -> DSISMEN_R {
        DSISMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2SMENR")
            .field("syscfgsmen", &self.syscfgsmen())
            .field("tim1smen", &self.tim1smen())
            .field("spi1smen", &self.spi1smen())
            .field("tim8smen", &self.tim8smen())
            .field("usart1smen", &self.usart1smen())
            .field("tim15smen", &self.tim15smen())
            .field("tim16smen", &self.tim16smen())
            .field("tim17smen", &self.tim17smen())
            .field("sai1smen", &self.sai1smen())
            .field("sai2smen", &self.sai2smen())
            .field("dfsdm1smen", &self.dfsdm1smen())
            .field("ltdcsmen", &self.ltdcsmen())
            .field("dsismen", &self.dsismen())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSCFG clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W<'_, APB2SMENRrs> {
        SYSCFGSMEN_W::new(self, 0)
    }
    ///Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W<'_, APB2SMENRrs> {
        TIM1SMEN_W::new(self, 11)
    }
    ///Bit 12 - SPI1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<'_, APB2SMENRrs> {
        SPI1SMEN_W::new(self, 12)
    }
    ///Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim8smen(&mut self) -> TIM8SMEN_W<'_, APB2SMENRrs> {
        TIM8SMEN_W::new(self, 13)
    }
    ///Bit 14 - USART1clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<'_, APB2SMENRrs> {
        USART1SMEN_W::new(self, 14)
    }
    ///Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim15smen(&mut self) -> TIM15SMEN_W<'_, APB2SMENRrs> {
        TIM15SMEN_W::new(self, 16)
    }
    ///Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W<'_, APB2SMENRrs> {
        TIM16SMEN_W::new(self, 17)
    }
    ///Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W<'_, APB2SMENRrs> {
        TIM17SMEN_W::new(self, 18)
    }
    ///Bit 21 - SAI1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sai1smen(&mut self) -> SAI1SMEN_W<'_, APB2SMENRrs> {
        SAI1SMEN_W::new(self, 21)
    }
    ///Bit 22 - SAI2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sai2smen(&mut self) -> SAI2SMEN_W<'_, APB2SMENRrs> {
        SAI2SMEN_W::new(self, 22)
    }
    ///Bit 24 - DFSDM timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dfsdm1smen(&mut self) -> DFSDM1SMEN_W<'_, APB2SMENRrs> {
        DFSDM1SMEN_W::new(self, 24)
    }
    ///Bit 26 - LCD-TFT timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn ltdcsmen(&mut self) -> LTDCSMEN_W<'_, APB2SMENRrs> {
        LTDCSMEN_W::new(self, 26)
    }
    ///Bit 27 - DSI clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dsismen(&mut self) -> DSISMEN_W<'_, APB2SMENRrs> {
        DSISMEN_W::new(self, 27)
    }
}
/**APB2SMENR

You can [`read`](crate::Reg::read) this register and get [`apb2smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#RCC:APB2SMENR)*/
pub struct APB2SMENRrs;
impl crate::RegisterSpec for APB2SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2smenr::R`](R) reader structure
impl crate::Readable for APB2SMENRrs {}
///`write(|w| ..)` method takes [`apb2smenr::W`](W) writer structure
impl crate::Writable for APB2SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2SMENR to value 0x0167_7c01
impl crate::Resettable for APB2SMENRrs {
    const RESET_VALUE: u32 = 0x0167_7c01;
}
