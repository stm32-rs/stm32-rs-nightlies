///Register `RCC_APBENR2` reader
pub type R = crate::R<RCC_APBENR2rs>;
///Register `RCC_APBENR2` writer
pub type W = crate::W<RCC_APBENR2rs>;
///Field `SYSCFGEN` reader - SYSCFG, COMP and VREFBUF clock enable Set and cleared by software.
pub type SYSCFGEN_R = crate::BitReader;
///Field `SYSCFGEN` writer - SYSCFG, COMP and VREFBUF clock enable Set and cleared by software.
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM1EN` reader - TIM1 timer clock enable Set and cleared by software.
pub type TIM1EN_R = crate::BitReader;
///Field `TIM1EN` writer - TIM1 timer clock enable Set and cleared by software.
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1EN` reader - SPI1 clock enable Set and cleared by software.
pub type SPI1EN_R = crate::BitReader;
///Field `SPI1EN` writer - SPI1 clock enable Set and cleared by software.
pub type SPI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1EN` reader - USART1 clock enable Set and cleared by software.
pub type USART1EN_R = crate::BitReader;
///Field `USART1EN` writer - USART1 clock enable Set and cleared by software.
pub type USART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15EN` reader - TIM15 timer clock enable Set and cleared by software.
pub type TIM15EN_R = crate::BitReader;
///Field `TIM15EN` writer - TIM15 timer clock enable Set and cleared by software.
pub type TIM15EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16EN` reader - TIM16 timer clock enable Set and cleared by software.
pub type TIM16EN_R = crate::BitReader;
///Field `TIM16EN` writer - TIM16 timer clock enable Set and cleared by software.
pub type TIM16EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCEN` reader - ADC clock enable Set and cleared by software.
pub type ADCEN_R = crate::BitReader;
///Field `ADCEN` writer - ADC clock enable Set and cleared by software.
pub type ADCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SYSCFG, COMP and VREFBUF clock enable Set and cleared by software.
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
    ///Bit 16 - TIM15 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
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
            .field("tim15en", &self.tim15en())
            .field("tim16en", &self.tim16en())
            .field("adcen", &self.adcen())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSCFG, COMP and VREFBUF clock enable Set and cleared by software.
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<RCC_APBENR2rs> {
        SYSCFGEN_W::new(self, 0)
    }
    ///Bit 11 - TIM1 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W<RCC_APBENR2rs> {
        TIM1EN_W::new(self, 11)
    }
    ///Bit 12 - SPI1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<RCC_APBENR2rs> {
        SPI1EN_W::new(self, 12)
    }
    ///Bit 14 - USART1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<RCC_APBENR2rs> {
        USART1EN_W::new(self, 14)
    }
    ///Bit 16 - TIM15 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim15en(&mut self) -> TIM15EN_W<RCC_APBENR2rs> {
        TIM15EN_W::new(self, 16)
    }
    ///Bit 17 - TIM16 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim16en(&mut self) -> TIM16EN_W<RCC_APBENR2rs> {
        TIM16EN_W::new(self, 17)
    }
    ///Bit 20 - ADC clock enable Set and cleared by software.
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W<RCC_APBENR2rs> {
        ADCEN_W::new(self, 20)
    }
}
/**APB peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_apbenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apbenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#RCC:RCC_APBENR2)*/
pub struct RCC_APBENR2rs;
impl crate::RegisterSpec for RCC_APBENR2rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_apbenr2::R`](R) reader structure
impl crate::Readable for RCC_APBENR2rs {}
///`write(|w| ..)` method takes [`rcc_apbenr2::W`](W) writer structure
impl crate::Writable for RCC_APBENR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_APBENR2 to value 0
impl crate::Resettable for RCC_APBENR2rs {
    const RESET_VALUE: u32 = 0;
}