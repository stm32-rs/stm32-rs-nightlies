#[doc = "Register `GTZC1_TZIC_FCR1` writer"]
pub type W = crate::W<GTZC1_TZIC_FCR1rs>;
#[doc = "Field `CTIM2F` writer - clear the illegal access flag for TIM2"]
pub type CTIM2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIM3F` writer - clear the illegal access flag for TIM3"]
pub type CTIM3F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIM4F` writer - clear the illegal access flag for TIM4"]
pub type CTIM4F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIM5F` writer - clear the illegal access flag for TIM5"]
pub type CTIM5F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIM6F` writer - clear the illegal access flag for TIM6"]
pub type CTIM6F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIM7F` writer - clear the illegal access flag for TIM7"]
pub type CTIM7F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIM12F` writer - clear the illegal access flag for TIM12"]
pub type CTIM12F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIM13F` writer - clear the illegal access flag for TIM13"]
pub type CTIM13F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIM14F` writer - clear the illegal access flag for TIM14"]
pub type CTIM14F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWWDGF` writer - clear the illegal access flag for WWDG"]
pub type CWWDGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIWDGF` writer - clear the illegal access flag for IWDG"]
pub type CIWDGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPI2F` writer - clear the illegal access flag for SPI2"]
pub type CSPI2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPI3F` writer - clear the illegal access flag for SPI3"]
pub type CSPI3F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUSART2F` writer - clear the illegal access flag for USART2"]
pub type CUSART2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUSART3F` writer - clear the illegal access flag for USART3"]
pub type CUSART3F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUART4F` writer - clear the illegal access flag for UART4"]
pub type CUART4F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUART5F` writer - clear the illegal access flag for UART5"]
pub type CUART5F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CI2C1F` writer - clear the illegal access flag for I2C1"]
pub type CI2C1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CI2C2F` writer - clear the illegal access flag for I2C2"]
pub type CI2C2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CI3C1F` writer - clear the illegal access flag for I3C1"]
pub type CI3C1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCRSF` writer - clear the illegal access flag for CRS"]
pub type CCRSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUSART6F` writer - clear the illegal access flag for USART6"]
pub type CUSART6F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUSART10F` writer - clear the illegal access flag for USART10"]
pub type CUSART10F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUSART11F` writer - clear the illegal access flag for USART11"]
pub type CUSART11F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDMICECF` writer - clear the illegal access flag for HDMICEC"]
pub type CHDMICECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDAC1F` writer - clear the illegal access flag for DAC1"]
pub type CDAC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUART7F` writer - clear the illegal access flag for UART7"]
pub type CUART7F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUART8F` writer - clear the illegal access flag for UART8"]
pub type CUART8F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUART9F` writer - clear the illegal access flag for UART9"]
pub type CUART9F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUART12F` writer - clear the illegal access flag for UART12"]
pub type CUART12F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDTSF` writer - clear the illegal access flag for DTS"]
pub type CDTSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLPTIM2F` writer - clear the illegal access flag for LPTIM2"]
pub type CLPTIM2F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - clear the illegal access flag for TIM2"]
    #[inline(always)]
    #[must_use]
    pub fn ctim2f(&mut self) -> CTIM2F_W<GTZC1_TZIC_FCR1rs> {
        CTIM2F_W::new(self, 0)
    }
    #[doc = "Bit 1 - clear the illegal access flag for TIM3"]
    #[inline(always)]
    #[must_use]
    pub fn ctim3f(&mut self) -> CTIM3F_W<GTZC1_TZIC_FCR1rs> {
        CTIM3F_W::new(self, 1)
    }
    #[doc = "Bit 2 - clear the illegal access flag for TIM4"]
    #[inline(always)]
    #[must_use]
    pub fn ctim4f(&mut self) -> CTIM4F_W<GTZC1_TZIC_FCR1rs> {
        CTIM4F_W::new(self, 2)
    }
    #[doc = "Bit 3 - clear the illegal access flag for TIM5"]
    #[inline(always)]
    #[must_use]
    pub fn ctim5f(&mut self) -> CTIM5F_W<GTZC1_TZIC_FCR1rs> {
        CTIM5F_W::new(self, 3)
    }
    #[doc = "Bit 4 - clear the illegal access flag for TIM6"]
    #[inline(always)]
    #[must_use]
    pub fn ctim6f(&mut self) -> CTIM6F_W<GTZC1_TZIC_FCR1rs> {
        CTIM6F_W::new(self, 4)
    }
    #[doc = "Bit 5 - clear the illegal access flag for TIM7"]
    #[inline(always)]
    #[must_use]
    pub fn ctim7f(&mut self) -> CTIM7F_W<GTZC1_TZIC_FCR1rs> {
        CTIM7F_W::new(self, 5)
    }
    #[doc = "Bit 6 - clear the illegal access flag for TIM12"]
    #[inline(always)]
    #[must_use]
    pub fn ctim12f(&mut self) -> CTIM12F_W<GTZC1_TZIC_FCR1rs> {
        CTIM12F_W::new(self, 6)
    }
    #[doc = "Bit 7 - clear the illegal access flag for TIM13"]
    #[inline(always)]
    #[must_use]
    pub fn ctim13f(&mut self) -> CTIM13F_W<GTZC1_TZIC_FCR1rs> {
        CTIM13F_W::new(self, 7)
    }
    #[doc = "Bit 8 - clear the illegal access flag for TIM14"]
    #[inline(always)]
    #[must_use]
    pub fn ctim14f(&mut self) -> CTIM14F_W<GTZC1_TZIC_FCR1rs> {
        CTIM14F_W::new(self, 8)
    }
    #[doc = "Bit 9 - clear the illegal access flag for WWDG"]
    #[inline(always)]
    #[must_use]
    pub fn cwwdgf(&mut self) -> CWWDGF_W<GTZC1_TZIC_FCR1rs> {
        CWWDGF_W::new(self, 9)
    }
    #[doc = "Bit 10 - clear the illegal access flag for IWDG"]
    #[inline(always)]
    #[must_use]
    pub fn ciwdgf(&mut self) -> CIWDGF_W<GTZC1_TZIC_FCR1rs> {
        CIWDGF_W::new(self, 10)
    }
    #[doc = "Bit 11 - clear the illegal access flag for SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn cspi2f(&mut self) -> CSPI2F_W<GTZC1_TZIC_FCR1rs> {
        CSPI2F_W::new(self, 11)
    }
    #[doc = "Bit 12 - clear the illegal access flag for SPI3"]
    #[inline(always)]
    #[must_use]
    pub fn cspi3f(&mut self) -> CSPI3F_W<GTZC1_TZIC_FCR1rs> {
        CSPI3F_W::new(self, 12)
    }
    #[doc = "Bit 13 - clear the illegal access flag for USART2"]
    #[inline(always)]
    #[must_use]
    pub fn cusart2f(&mut self) -> CUSART2F_W<GTZC1_TZIC_FCR1rs> {
        CUSART2F_W::new(self, 13)
    }
    #[doc = "Bit 14 - clear the illegal access flag for USART3"]
    #[inline(always)]
    #[must_use]
    pub fn cusart3f(&mut self) -> CUSART3F_W<GTZC1_TZIC_FCR1rs> {
        CUSART3F_W::new(self, 14)
    }
    #[doc = "Bit 15 - clear the illegal access flag for UART4"]
    #[inline(always)]
    #[must_use]
    pub fn cuart4f(&mut self) -> CUART4F_W<GTZC1_TZIC_FCR1rs> {
        CUART4F_W::new(self, 15)
    }
    #[doc = "Bit 16 - clear the illegal access flag for UART5"]
    #[inline(always)]
    #[must_use]
    pub fn cuart5f(&mut self) -> CUART5F_W<GTZC1_TZIC_FCR1rs> {
        CUART5F_W::new(self, 16)
    }
    #[doc = "Bit 17 - clear the illegal access flag for I2C1"]
    #[inline(always)]
    #[must_use]
    pub fn ci2c1f(&mut self) -> CI2C1F_W<GTZC1_TZIC_FCR1rs> {
        CI2C1F_W::new(self, 17)
    }
    #[doc = "Bit 18 - clear the illegal access flag for I2C2"]
    #[inline(always)]
    #[must_use]
    pub fn ci2c2f(&mut self) -> CI2C2F_W<GTZC1_TZIC_FCR1rs> {
        CI2C2F_W::new(self, 18)
    }
    #[doc = "Bit 19 - clear the illegal access flag for I3C1"]
    #[inline(always)]
    #[must_use]
    pub fn ci3c1f(&mut self) -> CI3C1F_W<GTZC1_TZIC_FCR1rs> {
        CI3C1F_W::new(self, 19)
    }
    #[doc = "Bit 20 - clear the illegal access flag for CRS"]
    #[inline(always)]
    #[must_use]
    pub fn ccrsf(&mut self) -> CCRSF_W<GTZC1_TZIC_FCR1rs> {
        CCRSF_W::new(self, 20)
    }
    #[doc = "Bit 21 - clear the illegal access flag for USART6"]
    #[inline(always)]
    #[must_use]
    pub fn cusart6f(&mut self) -> CUSART6F_W<GTZC1_TZIC_FCR1rs> {
        CUSART6F_W::new(self, 21)
    }
    #[doc = "Bit 22 - clear the illegal access flag for USART10"]
    #[inline(always)]
    #[must_use]
    pub fn cusart10f(&mut self) -> CUSART10F_W<GTZC1_TZIC_FCR1rs> {
        CUSART10F_W::new(self, 22)
    }
    #[doc = "Bit 23 - clear the illegal access flag for USART11"]
    #[inline(always)]
    #[must_use]
    pub fn cusart11f(&mut self) -> CUSART11F_W<GTZC1_TZIC_FCR1rs> {
        CUSART11F_W::new(self, 23)
    }
    #[doc = "Bit 24 - clear the illegal access flag for HDMICEC"]
    #[inline(always)]
    #[must_use]
    pub fn chdmicecf(&mut self) -> CHDMICECF_W<GTZC1_TZIC_FCR1rs> {
        CHDMICECF_W::new(self, 24)
    }
    #[doc = "Bit 25 - clear the illegal access flag for DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn cdac1f(&mut self) -> CDAC1F_W<GTZC1_TZIC_FCR1rs> {
        CDAC1F_W::new(self, 25)
    }
    #[doc = "Bit 26 - clear the illegal access flag for UART7"]
    #[inline(always)]
    #[must_use]
    pub fn cuart7f(&mut self) -> CUART7F_W<GTZC1_TZIC_FCR1rs> {
        CUART7F_W::new(self, 26)
    }
    #[doc = "Bit 27 - clear the illegal access flag for UART8"]
    #[inline(always)]
    #[must_use]
    pub fn cuart8f(&mut self) -> CUART8F_W<GTZC1_TZIC_FCR1rs> {
        CUART8F_W::new(self, 27)
    }
    #[doc = "Bit 28 - clear the illegal access flag for UART9"]
    #[inline(always)]
    #[must_use]
    pub fn cuart9f(&mut self) -> CUART9F_W<GTZC1_TZIC_FCR1rs> {
        CUART9F_W::new(self, 28)
    }
    #[doc = "Bit 29 - clear the illegal access flag for UART12"]
    #[inline(always)]
    #[must_use]
    pub fn cuart12f(&mut self) -> CUART12F_W<GTZC1_TZIC_FCR1rs> {
        CUART12F_W::new(self, 29)
    }
    #[doc = "Bit 30 - clear the illegal access flag for DTS"]
    #[inline(always)]
    #[must_use]
    pub fn cdtsf(&mut self) -> CDTSF_W<GTZC1_TZIC_FCR1rs> {
        CDTSF_W::new(self, 30)
    }
    #[doc = "Bit 31 - clear the illegal access flag for LPTIM2"]
    #[inline(always)]
    #[must_use]
    pub fn clptim2f(&mut self) -> CLPTIM2F_W<GTZC1_TZIC_FCR1rs> {
        CLPTIM2F_W::new(self, 31)
    }
}
#[doc = "TZIC flag clear register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzic_fcr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTZC1_TZIC_FCR1rs;
impl crate::RegisterSpec for GTZC1_TZIC_FCR1rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gtzc1_tzic_fcr1::W`](W) writer structure"]
impl crate::Writable for GTZC1_TZIC_FCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTZC1_TZIC_FCR1 to value 0"]
impl crate::Resettable for GTZC1_TZIC_FCR1rs {
    const RESET_VALUE: u32 = 0;
}
