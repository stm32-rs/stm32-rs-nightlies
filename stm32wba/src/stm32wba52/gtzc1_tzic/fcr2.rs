///Register `FCR2` writer
pub type W = crate::W<FCR2rs>;
///Field `CTIM1F` writer - clear the illegal access flag for TIM1
pub type CTIM1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSPI1F` writer - clear the illegal access flag for SPI1
pub type CSPI1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUSART1F` writer - clear the illegal access flag for USART1
pub type CUSART1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM16F` writer - clear the illegal access flag for TIM6
pub type CTIM16F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM17F` writer - clear the illegal access flag for TIM7
pub type CTIM17F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSPI3F` writer - clear the illegal access flag for SPI3
pub type CSPI3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLPUART1F` writer - clear the illegal access flag for LPUART1
pub type CLPUART1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CI2C3F` writer - clear the illegal access flag for I2C3
pub type CI2C3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLPTIM1F` writer - clear the illegal access flag for LPTIM1
pub type CLPTIM1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCOMPF` writer - iclear the illegal access flag for COMP
pub type CCOMPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CADC4F` writer - clear the illegal access flag for ADC4
pub type CADC4F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clear the illegal access flag for TIM1
    #[inline(always)]
    pub fn ctim1f(&mut self) -> CTIM1F_W<'_, FCR2rs> {
        CTIM1F_W::new(self, 0)
    }
    ///Bit 1 - clear the illegal access flag for SPI1
    #[inline(always)]
    pub fn cspi1f(&mut self) -> CSPI1F_W<'_, FCR2rs> {
        CSPI1F_W::new(self, 1)
    }
    ///Bit 3 - clear the illegal access flag for USART1
    #[inline(always)]
    pub fn cusart1f(&mut self) -> CUSART1F_W<'_, FCR2rs> {
        CUSART1F_W::new(self, 3)
    }
    ///Bit 5 - clear the illegal access flag for TIM6
    #[inline(always)]
    pub fn ctim16f(&mut self) -> CTIM16F_W<'_, FCR2rs> {
        CTIM16F_W::new(self, 5)
    }
    ///Bit 6 - clear the illegal access flag for TIM7
    #[inline(always)]
    pub fn ctim17f(&mut self) -> CTIM17F_W<'_, FCR2rs> {
        CTIM17F_W::new(self, 6)
    }
    ///Bit 16 - clear the illegal access flag for SPI3
    #[inline(always)]
    pub fn cspi3f(&mut self) -> CSPI3F_W<'_, FCR2rs> {
        CSPI3F_W::new(self, 16)
    }
    ///Bit 17 - clear the illegal access flag for LPUART1
    #[inline(always)]
    pub fn clpuart1f(&mut self) -> CLPUART1F_W<'_, FCR2rs> {
        CLPUART1F_W::new(self, 17)
    }
    ///Bit 18 - clear the illegal access flag for I2C3
    #[inline(always)]
    pub fn ci2c3f(&mut self) -> CI2C3F_W<'_, FCR2rs> {
        CI2C3F_W::new(self, 18)
    }
    ///Bit 19 - clear the illegal access flag for LPTIM1
    #[inline(always)]
    pub fn clptim1f(&mut self) -> CLPTIM1F_W<'_, FCR2rs> {
        CLPTIM1F_W::new(self, 19)
    }
    ///Bit 23 - iclear the illegal access flag for COMP
    #[inline(always)]
    pub fn ccompf(&mut self) -> CCOMPF_W<'_, FCR2rs> {
        CCOMPF_W::new(self, 23)
    }
    ///Bit 24 - clear the illegal access flag for ADC4
    #[inline(always)]
    pub fn cadc4f(&mut self) -> CADC4F_W<'_, FCR2rs> {
        CADC4F_W::new(self, 24)
    }
}
/**TZIC flag clear register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GTZC1_TZIC:FCR2)*/
pub struct FCR2rs;
impl crate::RegisterSpec for FCR2rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr2::W`](W) writer structure
impl crate::Writable for FCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR2 to value 0
impl crate::Resettable for FCR2rs {}
