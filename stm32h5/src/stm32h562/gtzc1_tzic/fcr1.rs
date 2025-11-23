///Register `FCR1` writer
pub type W = crate::W<FCR1rs>;
///Field `CTIM2F` writer - clear the illegal access flag for TIM2
pub type CTIM2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM3F` writer - clear the illegal access flag for TIM3
pub type CTIM3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM4F` writer - clear the illegal access flag for TIM4
pub type CTIM4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM5F` writer - clear the illegal access flag for TIM5
pub type CTIM5F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM6F` writer - clear the illegal access flag for TIM6
pub type CTIM6F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM7F` writer - clear the illegal access flag for TIM7
pub type CTIM7F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM12F` writer - clear the illegal access flag for TIM12
pub type CTIM12F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM13F` writer - clear the illegal access flag for TIM13
pub type CTIM13F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM14F` writer - clear the illegal access flag for TIM14
pub type CTIM14F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWWDGF` writer - clear the illegal access flag for WWDG
pub type CWWDGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIWDGF` writer - clear the illegal access flag for IWDG
pub type CIWDGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSPI2F` writer - clear the illegal access flag for SPI2
pub type CSPI2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSPI3F` writer - clear the illegal access flag for SPI3
pub type CSPI3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUSART2F` writer - clear the illegal access flag for USART2
pub type CUSART2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUSART3F` writer - clear the illegal access flag for USART3
pub type CUSART3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUART4F` writer - clear the illegal access flag for UART4
pub type CUART4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUART5F` writer - clear the illegal access flag for UART5
pub type CUART5F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CI2C1F` writer - clear the illegal access flag for I2C1
pub type CI2C1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CI2C2F` writer - clear the illegal access flag for I2C2
pub type CI2C2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CI3C1F` writer - clear the illegal access flag for I3C1
pub type CI3C1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCRSF` writer - clear the illegal access flag for CRS
pub type CCRSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUSART6F` writer - clear the illegal access flag for USART6
pub type CUSART6F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUSART10F` writer - clear the illegal access flag for USART10
pub type CUSART10F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUSART11F` writer - clear the illegal access flag for USART11
pub type CUSART11F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHDMICECF` writer - clear the illegal access flag for HDMICEC
pub type CHDMICECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDAC1F` writer - clear the illegal access flag for DAC1
pub type CDAC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUART7F` writer - clear the illegal access flag for UART7
pub type CUART7F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUART8F` writer - clear the illegal access flag for UART8
pub type CUART8F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUART9F` writer - clear the illegal access flag for UART9
pub type CUART9F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUART12F` writer - clear the illegal access flag for UART12
pub type CUART12F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDTSF` writer - clear the illegal access flag for DTS
pub type CDTSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLPTIM2F` writer - clear the illegal access flag for LPTIM2
pub type CLPTIM2F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clear the illegal access flag for TIM2
    #[inline(always)]
    pub fn ctim2f(&mut self) -> CTIM2F_W<'_, FCR1rs> {
        CTIM2F_W::new(self, 0)
    }
    ///Bit 1 - clear the illegal access flag for TIM3
    #[inline(always)]
    pub fn ctim3f(&mut self) -> CTIM3F_W<'_, FCR1rs> {
        CTIM3F_W::new(self, 1)
    }
    ///Bit 2 - clear the illegal access flag for TIM4
    #[inline(always)]
    pub fn ctim4f(&mut self) -> CTIM4F_W<'_, FCR1rs> {
        CTIM4F_W::new(self, 2)
    }
    ///Bit 3 - clear the illegal access flag for TIM5
    #[inline(always)]
    pub fn ctim5f(&mut self) -> CTIM5F_W<'_, FCR1rs> {
        CTIM5F_W::new(self, 3)
    }
    ///Bit 4 - clear the illegal access flag for TIM6
    #[inline(always)]
    pub fn ctim6f(&mut self) -> CTIM6F_W<'_, FCR1rs> {
        CTIM6F_W::new(self, 4)
    }
    ///Bit 5 - clear the illegal access flag for TIM7
    #[inline(always)]
    pub fn ctim7f(&mut self) -> CTIM7F_W<'_, FCR1rs> {
        CTIM7F_W::new(self, 5)
    }
    ///Bit 6 - clear the illegal access flag for TIM12
    #[inline(always)]
    pub fn ctim12f(&mut self) -> CTIM12F_W<'_, FCR1rs> {
        CTIM12F_W::new(self, 6)
    }
    ///Bit 7 - clear the illegal access flag for TIM13
    #[inline(always)]
    pub fn ctim13f(&mut self) -> CTIM13F_W<'_, FCR1rs> {
        CTIM13F_W::new(self, 7)
    }
    ///Bit 8 - clear the illegal access flag for TIM14
    #[inline(always)]
    pub fn ctim14f(&mut self) -> CTIM14F_W<'_, FCR1rs> {
        CTIM14F_W::new(self, 8)
    }
    ///Bit 9 - clear the illegal access flag for WWDG
    #[inline(always)]
    pub fn cwwdgf(&mut self) -> CWWDGF_W<'_, FCR1rs> {
        CWWDGF_W::new(self, 9)
    }
    ///Bit 10 - clear the illegal access flag for IWDG
    #[inline(always)]
    pub fn ciwdgf(&mut self) -> CIWDGF_W<'_, FCR1rs> {
        CIWDGF_W::new(self, 10)
    }
    ///Bit 11 - clear the illegal access flag for SPI2
    #[inline(always)]
    pub fn cspi2f(&mut self) -> CSPI2F_W<'_, FCR1rs> {
        CSPI2F_W::new(self, 11)
    }
    ///Bit 12 - clear the illegal access flag for SPI3
    #[inline(always)]
    pub fn cspi3f(&mut self) -> CSPI3F_W<'_, FCR1rs> {
        CSPI3F_W::new(self, 12)
    }
    ///Bit 13 - clear the illegal access flag for USART2
    #[inline(always)]
    pub fn cusart2f(&mut self) -> CUSART2F_W<'_, FCR1rs> {
        CUSART2F_W::new(self, 13)
    }
    ///Bit 14 - clear the illegal access flag for USART3
    #[inline(always)]
    pub fn cusart3f(&mut self) -> CUSART3F_W<'_, FCR1rs> {
        CUSART3F_W::new(self, 14)
    }
    ///Bit 15 - clear the illegal access flag for UART4
    #[inline(always)]
    pub fn cuart4f(&mut self) -> CUART4F_W<'_, FCR1rs> {
        CUART4F_W::new(self, 15)
    }
    ///Bit 16 - clear the illegal access flag for UART5
    #[inline(always)]
    pub fn cuart5f(&mut self) -> CUART5F_W<'_, FCR1rs> {
        CUART5F_W::new(self, 16)
    }
    ///Bit 17 - clear the illegal access flag for I2C1
    #[inline(always)]
    pub fn ci2c1f(&mut self) -> CI2C1F_W<'_, FCR1rs> {
        CI2C1F_W::new(self, 17)
    }
    ///Bit 18 - clear the illegal access flag for I2C2
    #[inline(always)]
    pub fn ci2c2f(&mut self) -> CI2C2F_W<'_, FCR1rs> {
        CI2C2F_W::new(self, 18)
    }
    ///Bit 19 - clear the illegal access flag for I3C1
    #[inline(always)]
    pub fn ci3c1f(&mut self) -> CI3C1F_W<'_, FCR1rs> {
        CI3C1F_W::new(self, 19)
    }
    ///Bit 20 - clear the illegal access flag for CRS
    #[inline(always)]
    pub fn ccrsf(&mut self) -> CCRSF_W<'_, FCR1rs> {
        CCRSF_W::new(self, 20)
    }
    ///Bit 21 - clear the illegal access flag for USART6
    #[inline(always)]
    pub fn cusart6f(&mut self) -> CUSART6F_W<'_, FCR1rs> {
        CUSART6F_W::new(self, 21)
    }
    ///Bit 22 - clear the illegal access flag for USART10
    #[inline(always)]
    pub fn cusart10f(&mut self) -> CUSART10F_W<'_, FCR1rs> {
        CUSART10F_W::new(self, 22)
    }
    ///Bit 23 - clear the illegal access flag for USART11
    #[inline(always)]
    pub fn cusart11f(&mut self) -> CUSART11F_W<'_, FCR1rs> {
        CUSART11F_W::new(self, 23)
    }
    ///Bit 24 - clear the illegal access flag for HDMICEC
    #[inline(always)]
    pub fn chdmicecf(&mut self) -> CHDMICECF_W<'_, FCR1rs> {
        CHDMICECF_W::new(self, 24)
    }
    ///Bit 25 - clear the illegal access flag for DAC1
    #[inline(always)]
    pub fn cdac1f(&mut self) -> CDAC1F_W<'_, FCR1rs> {
        CDAC1F_W::new(self, 25)
    }
    ///Bit 26 - clear the illegal access flag for UART7
    #[inline(always)]
    pub fn cuart7f(&mut self) -> CUART7F_W<'_, FCR1rs> {
        CUART7F_W::new(self, 26)
    }
    ///Bit 27 - clear the illegal access flag for UART8
    #[inline(always)]
    pub fn cuart8f(&mut self) -> CUART8F_W<'_, FCR1rs> {
        CUART8F_W::new(self, 27)
    }
    ///Bit 28 - clear the illegal access flag for UART9
    #[inline(always)]
    pub fn cuart9f(&mut self) -> CUART9F_W<'_, FCR1rs> {
        CUART9F_W::new(self, 28)
    }
    ///Bit 29 - clear the illegal access flag for UART12
    #[inline(always)]
    pub fn cuart12f(&mut self) -> CUART12F_W<'_, FCR1rs> {
        CUART12F_W::new(self, 29)
    }
    ///Bit 30 - clear the illegal access flag for DTS
    #[inline(always)]
    pub fn cdtsf(&mut self) -> CDTSF_W<'_, FCR1rs> {
        CDTSF_W::new(self, 30)
    }
    ///Bit 31 - clear the illegal access flag for LPTIM2
    #[inline(always)]
    pub fn clptim2f(&mut self) -> CLPTIM2F_W<'_, FCR1rs> {
        CLPTIM2F_W::new(self, 31)
    }
}
/**TZIC flag clear register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#GTZC1_TZIC:FCR1)*/
pub struct FCR1rs;
impl crate::RegisterSpec for FCR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr1::W`](W) writer structure
impl crate::Writable for FCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR1 to value 0
impl crate::Resettable for FCR1rs {}
