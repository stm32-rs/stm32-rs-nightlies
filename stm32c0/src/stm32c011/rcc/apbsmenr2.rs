///Register `APBSMENR2` reader
pub type R = crate::R<APBSMENR2rs>;
///Register `APBSMENR2` writer
pub type W = crate::W<APBSMENR2rs>;
///Field `SYSCFGSMEN` reader - SYSCFG clock enable during Sleep and Stop modes Set and cleared by software.
pub type SYSCFGSMEN_R = crate::BitReader;
///Field `SYSCFGSMEN` writer - SYSCFG clock enable during Sleep and Stop modes Set and cleared by software.
pub type SYSCFGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM1SMEN` reader - TIM1 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM1SMEN_R = crate::BitReader;
///Field `TIM1SMEN` writer - TIM1 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1SMEN` reader - SPI1 clock enable during Sleep mode Set and cleared by software.
pub type SPI1SMEN_R = crate::BitReader;
///Field `SPI1SMEN` writer - SPI1 clock enable during Sleep mode Set and cleared by software.
pub type SPI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1SMEN` reader - USART1 clock enable during Sleep and Stop modes Set and cleared by software.
pub type USART1SMEN_R = crate::BitReader;
///Field `USART1SMEN` writer - USART1 clock enable during Sleep and Stop modes Set and cleared by software.
pub type USART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM14SMEN` reader - TIM14 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM14SMEN_R = crate::BitReader;
///Field `TIM14SMEN` writer - TIM14 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM14SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16SMEN` reader - TIM16 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM16SMEN_R = crate::BitReader;
///Field `TIM16SMEN` writer - TIM16 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM16SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17SMEN` reader - TIM16 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM17SMEN_R = crate::BitReader;
///Field `TIM17SMEN` writer - TIM16 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM17SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCSMEN` reader - ADC clock enable during Sleep mode Set and cleared by software.
pub type ADCSMEN_R = crate::BitReader;
///Field `ADCSMEN` writer - ADC clock enable during Sleep mode Set and cleared by software.
pub type ADCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        f.debug_struct("APBSMENR2")
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
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W<APBSMENR2rs> {
        SYSCFGSMEN_W::new(self, 0)
    }
    ///Bit 11 - TIM1 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W<APBSMENR2rs> {
        TIM1SMEN_W::new(self, 11)
    }
    ///Bit 12 - SPI1 clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<APBSMENR2rs> {
        SPI1SMEN_W::new(self, 12)
    }
    ///Bit 14 - USART1 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<APBSMENR2rs> {
        USART1SMEN_W::new(self, 14)
    }
    ///Bit 15 - TIM14 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim14smen(&mut self) -> TIM14SMEN_W<APBSMENR2rs> {
        TIM14SMEN_W::new(self, 15)
    }
    ///Bit 17 - TIM16 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W<APBSMENR2rs> {
        TIM16SMEN_W::new(self, 17)
    }
    ///Bit 18 - TIM16 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W<APBSMENR2rs> {
        TIM17SMEN_W::new(self, 18)
    }
    ///Bit 20 - ADC clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn adcsmen(&mut self) -> ADCSMEN_W<APBSMENR2rs> {
        ADCSMEN_W::new(self, 20)
    }
}
/**RCC APB peripheral clock enable in Sleep/Stop mode register 2

You can [`read`](crate::Reg::read) this register and get [`apbsmenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbsmenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#RCC:APBSMENR2)*/
pub struct APBSMENR2rs;
impl crate::RegisterSpec for APBSMENR2rs {
    type Ux = u32;
}
///`read()` method returns [`apbsmenr2::R`](R) reader structure
impl crate::Readable for APBSMENR2rs {}
///`write(|w| ..)` method takes [`apbsmenr2::W`](W) writer structure
impl crate::Writable for APBSMENR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APBSMENR2 to value 0x0017_d801
impl crate::Resettable for APBSMENR2rs {
    const RESET_VALUE: u32 = 0x0017_d801;
}
