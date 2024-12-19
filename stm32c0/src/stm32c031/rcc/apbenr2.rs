///Register `APBENR2` reader
pub type R = crate::R<APBENR2rs>;
///Register `APBENR2` writer
pub type W = crate::W<APBENR2rs>;
///Field `SYSCFGEN` reader - SYSCFG clock enable Set and cleared by software.
pub type SYSCFGEN_R = crate::BitReader;
///Field `SYSCFGEN` writer - SYSCFG clock enable Set and cleared by software.
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
///Field `TIM14EN` reader - TIM14 timer clock enable Set and cleared by software.
pub type TIM14EN_R = crate::BitReader;
///Field `TIM14EN` writer - TIM14 timer clock enable Set and cleared by software.
pub type TIM14EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16EN` reader - TIM16 timer clock enable Set and cleared by software.
pub type TIM16EN_R = crate::BitReader;
///Field `TIM16EN` writer - TIM16 timer clock enable Set and cleared by software.
pub type TIM16EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17EN` reader - TIM16 timer clock enable Set and cleared by software.
pub type TIM17EN_R = crate::BitReader;
///Field `TIM17EN` writer - TIM16 timer clock enable Set and cleared by software.
pub type TIM17EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCEN` reader - ADC clock enable Set and cleared by software.
pub type ADCEN_R = crate::BitReader;
///Field `ADCEN` writer - ADC clock enable Set and cleared by software.
pub type ADCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        f.debug_struct("APBENR2")
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
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<APBENR2rs> {
        SYSCFGEN_W::new(self, 0)
    }
    ///Bit 11 - TIM1 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W<APBENR2rs> {
        TIM1EN_W::new(self, 11)
    }
    ///Bit 12 - SPI1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<APBENR2rs> {
        SPI1EN_W::new(self, 12)
    }
    ///Bit 14 - USART1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<APBENR2rs> {
        USART1EN_W::new(self, 14)
    }
    ///Bit 15 - TIM14 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim14en(&mut self) -> TIM14EN_W<APBENR2rs> {
        TIM14EN_W::new(self, 15)
    }
    ///Bit 17 - TIM16 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim16en(&mut self) -> TIM16EN_W<APBENR2rs> {
        TIM16EN_W::new(self, 17)
    }
    ///Bit 18 - TIM16 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim17en(&mut self) -> TIM17EN_W<APBENR2rs> {
        TIM17EN_W::new(self, 18)
    }
    ///Bit 20 - ADC clock enable Set and cleared by software.
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W<APBENR2rs> {
        ADCEN_W::new(self, 20)
    }
}
/**RCC APB peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apbenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#RCC:APBENR2)*/
pub struct APBENR2rs;
impl crate::RegisterSpec for APBENR2rs {
    type Ux = u32;
}
///`read()` method returns [`apbenr2::R`](R) reader structure
impl crate::Readable for APBENR2rs {}
///`write(|w| ..)` method takes [`apbenr2::W`](W) writer structure
impl crate::Writable for APBENR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APBENR2 to value 0
impl crate::Resettable for APBENR2rs {
    const RESET_VALUE: u32 = 0;
}
