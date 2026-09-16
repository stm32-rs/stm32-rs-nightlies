///Register `FCR2` writer
pub type W = crate::W<FCR2rs>;
///Field `CFDCAN1F` writer - clear the illegal access flag for FDCAN1
pub type CFDCAN1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFDCAN2F` writer - clear the illegal access flag for FDCAN2
pub type CFDCAN2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUCPDF` writer - clear the illegal access flag for UCPD
pub type CUCPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM1F` writer - clear the illegal access flag for TIM1
pub type CTIM1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSPI1F` writer - clear the illegal access flag for SPI1
pub type CSPI1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM8F` writer - clear the illegal access flag for TIM8
pub type CTIM8F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUSART1F` writer - clear the illegal access flag for USART1
pub type CUSART1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM15F` writer - clear the illegal access flag for TIM15
pub type CTIM15F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM16F` writer - clear the illegal access flag for TIM16
pub type CTIM16F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM17F` writer - clear the illegal access flag for TIM17
pub type CTIM17F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSPI4F` writer - clear the illegal access flag for SPI4
pub type CSPI4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSPI6F` writer - clear the illegal access flag for SPI6
pub type CSPI6F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSAI1F` writer - clear the illegal access flag for SAI1
pub type CSAI1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSAI2F` writer - clear the illegal access flag for SAI2
pub type CSAI2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUSBF` writer - clear the illegal access flag for USB
pub type CUSBF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSPI5F` writer - clear the illegal access flag for SPI5
pub type CSPI5F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLPUART1F` writer - clear the illegal access flag for LPUART
pub type CLPUART1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CI2C3F` writer - clear the illegal access flag for I2C3
pub type CI2C3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CI2C4F` writer - clear the illegal access flag for I2C4
pub type CI2C4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLPTIM1F` writer - clear the illegal access flag for LPTIM1
pub type CLPTIM1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLPTIM3F` writer - clear the illegal access flag for LPTIM3
pub type CLPTIM3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLPTIM4F` writer - clear the illegal access flag for LPTIM4
pub type CLPTIM4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLPTIM5F` writer - clear the illegal access flag for LPTIM5
pub type CLPTIM5F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clear the illegal access flag for FDCAN1
    #[inline(always)]
    pub fn cfdcan1f(&mut self) -> CFDCAN1F_W<'_, FCR2rs> {
        CFDCAN1F_W::new(self, 0)
    }
    ///Bit 1 - clear the illegal access flag for FDCAN2
    #[inline(always)]
    pub fn cfdcan2f(&mut self) -> CFDCAN2F_W<'_, FCR2rs> {
        CFDCAN2F_W::new(self, 1)
    }
    ///Bit 2 - clear the illegal access flag for UCPD
    #[inline(always)]
    pub fn cucpdf(&mut self) -> CUCPDF_W<'_, FCR2rs> {
        CUCPDF_W::new(self, 2)
    }
    ///Bit 8 - clear the illegal access flag for TIM1
    #[inline(always)]
    pub fn ctim1f(&mut self) -> CTIM1F_W<'_, FCR2rs> {
        CTIM1F_W::new(self, 8)
    }
    ///Bit 9 - clear the illegal access flag for SPI1
    #[inline(always)]
    pub fn cspi1f(&mut self) -> CSPI1F_W<'_, FCR2rs> {
        CSPI1F_W::new(self, 9)
    }
    ///Bit 10 - clear the illegal access flag for TIM8
    #[inline(always)]
    pub fn ctim8f(&mut self) -> CTIM8F_W<'_, FCR2rs> {
        CTIM8F_W::new(self, 10)
    }
    ///Bit 11 - clear the illegal access flag for USART1
    #[inline(always)]
    pub fn cusart1f(&mut self) -> CUSART1F_W<'_, FCR2rs> {
        CUSART1F_W::new(self, 11)
    }
    ///Bit 12 - clear the illegal access flag for TIM15
    #[inline(always)]
    pub fn ctim15f(&mut self) -> CTIM15F_W<'_, FCR2rs> {
        CTIM15F_W::new(self, 12)
    }
    ///Bit 13 - clear the illegal access flag for TIM16
    #[inline(always)]
    pub fn ctim16f(&mut self) -> CTIM16F_W<'_, FCR2rs> {
        CTIM16F_W::new(self, 13)
    }
    ///Bit 14 - clear the illegal access flag for TIM17
    #[inline(always)]
    pub fn ctim17f(&mut self) -> CTIM17F_W<'_, FCR2rs> {
        CTIM17F_W::new(self, 14)
    }
    ///Bit 15 - clear the illegal access flag for SPI4
    #[inline(always)]
    pub fn cspi4f(&mut self) -> CSPI4F_W<'_, FCR2rs> {
        CSPI4F_W::new(self, 15)
    }
    ///Bit 16 - clear the illegal access flag for SPI6
    #[inline(always)]
    pub fn cspi6f(&mut self) -> CSPI6F_W<'_, FCR2rs> {
        CSPI6F_W::new(self, 16)
    }
    ///Bit 17 - clear the illegal access flag for SAI1
    #[inline(always)]
    pub fn csai1f(&mut self) -> CSAI1F_W<'_, FCR2rs> {
        CSAI1F_W::new(self, 17)
    }
    ///Bit 18 - clear the illegal access flag for SAI2
    #[inline(always)]
    pub fn csai2f(&mut self) -> CSAI2F_W<'_, FCR2rs> {
        CSAI2F_W::new(self, 18)
    }
    ///Bit 19 - clear the illegal access flag for USB
    #[inline(always)]
    pub fn cusbf(&mut self) -> CUSBF_W<'_, FCR2rs> {
        CUSBF_W::new(self, 19)
    }
    ///Bit 24 - clear the illegal access flag for SPI5
    #[inline(always)]
    pub fn cspi5f(&mut self) -> CSPI5F_W<'_, FCR2rs> {
        CSPI5F_W::new(self, 24)
    }
    ///Bit 25 - clear the illegal access flag for LPUART
    #[inline(always)]
    pub fn clpuart1f(&mut self) -> CLPUART1F_W<'_, FCR2rs> {
        CLPUART1F_W::new(self, 25)
    }
    ///Bit 26 - clear the illegal access flag for I2C3
    #[inline(always)]
    pub fn ci2c3f(&mut self) -> CI2C3F_W<'_, FCR2rs> {
        CI2C3F_W::new(self, 26)
    }
    ///Bit 27 - clear the illegal access flag for I2C4
    #[inline(always)]
    pub fn ci2c4f(&mut self) -> CI2C4F_W<'_, FCR2rs> {
        CI2C4F_W::new(self, 27)
    }
    ///Bit 28 - clear the illegal access flag for LPTIM1
    #[inline(always)]
    pub fn clptim1f(&mut self) -> CLPTIM1F_W<'_, FCR2rs> {
        CLPTIM1F_W::new(self, 28)
    }
    ///Bit 29 - clear the illegal access flag for LPTIM3
    #[inline(always)]
    pub fn clptim3f(&mut self) -> CLPTIM3F_W<'_, FCR2rs> {
        CLPTIM3F_W::new(self, 29)
    }
    ///Bit 30 - clear the illegal access flag for LPTIM4
    #[inline(always)]
    pub fn clptim4f(&mut self) -> CLPTIM4F_W<'_, FCR2rs> {
        CLPTIM4F_W::new(self, 30)
    }
    ///Bit 31 - clear the illegal access flag for LPTIM5
    #[inline(always)]
    pub fn clptim5f(&mut self) -> CLPTIM5F_W<'_, FCR2rs> {
        CLPTIM5F_W::new(self, 31)
    }
}
/**GTZC1 TZIC flag clear register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#GTZC1_TZIC:FCR2)*/
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
