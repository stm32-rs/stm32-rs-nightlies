///Register `APBRSTR2` reader
pub type R = crate::R<APBRSTR2rs>;
///Register `APBRSTR2` writer
pub type W = crate::W<APBRSTR2rs>;
///Field `SYSCFGRST` reader - SYSCFG reset Set and cleared by software.
pub type SYSCFGRST_R = crate::BitReader;
///Field `SYSCFGRST` writer - SYSCFG reset Set and cleared by software.
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM1RST` reader - TIM1 timer reset Set and cleared by software.
pub type TIM1RST_R = crate::BitReader;
///Field `TIM1RST` writer - TIM1 timer reset Set and cleared by software.
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1RST` reader - SPI1 reset Set and cleared by software.
pub type SPI1RST_R = crate::BitReader;
///Field `SPI1RST` writer - SPI1 reset Set and cleared by software.
pub type SPI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1RST` reader - USART1 reset Set and cleared by software.
pub type USART1RST_R = crate::BitReader;
///Field `USART1RST` writer - USART1 reset Set and cleared by software.
pub type USART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM14RST` reader - TIM14 timer reset Set and cleared by software.
pub type TIM14RST_R = crate::BitReader;
///Field `TIM14RST` writer - TIM14 timer reset Set and cleared by software.
pub type TIM14RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16RST` reader - TIM16 timer reset Set and cleared by software.
pub type TIM16RST_R = crate::BitReader;
///Field `TIM16RST` writer - TIM16 timer reset Set and cleared by software.
pub type TIM16RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17RST` reader - TIM16 timer reset Set and cleared by software.
pub type TIM17RST_R = crate::BitReader;
///Field `TIM17RST` writer - TIM16 timer reset Set and cleared by software.
pub type TIM17RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCRST` reader - ADC reset Set and cleared by software.
pub type ADCRST_R = crate::BitReader;
///Field `ADCRST` writer - ADC reset Set and cleared by software.
pub type ADCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        f.debug_struct("APBRSTR2")
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
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<APBRSTR2rs> {
        SYSCFGRST_W::new(self, 0)
    }
    ///Bit 11 - TIM1 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W<APBRSTR2rs> {
        TIM1RST_W::new(self, 11)
    }
    ///Bit 12 - SPI1 reset Set and cleared by software.
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<APBRSTR2rs> {
        SPI1RST_W::new(self, 12)
    }
    ///Bit 14 - USART1 reset Set and cleared by software.
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W<APBRSTR2rs> {
        USART1RST_W::new(self, 14)
    }
    ///Bit 15 - TIM14 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim14rst(&mut self) -> TIM14RST_W<APBRSTR2rs> {
        TIM14RST_W::new(self, 15)
    }
    ///Bit 17 - TIM16 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim16rst(&mut self) -> TIM16RST_W<APBRSTR2rs> {
        TIM16RST_W::new(self, 17)
    }
    ///Bit 18 - TIM16 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim17rst(&mut self) -> TIM17RST_W<APBRSTR2rs> {
        TIM17RST_W::new(self, 18)
    }
    ///Bit 20 - ADC reset Set and cleared by software.
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W<APBRSTR2rs> {
        ADCRST_W::new(self, 20)
    }
}
/**RCC APB peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`apbrstr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrstr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#RCC:APBRSTR2)*/
pub struct APBRSTR2rs;
impl crate::RegisterSpec for APBRSTR2rs {
    type Ux = u32;
}
///`read()` method returns [`apbrstr2::R`](R) reader structure
impl crate::Readable for APBRSTR2rs {}
///`write(|w| ..)` method takes [`apbrstr2::W`](W) writer structure
impl crate::Writable for APBRSTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APBRSTR2 to value 0
impl crate::Resettable for APBRSTR2rs {
    const RESET_VALUE: u32 = 0;
}
