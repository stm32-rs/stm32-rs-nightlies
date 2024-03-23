#[doc = "Register `GTZC1_TZIC_FCR2` writer"]
pub type W = crate::W<GTZC1_TZIC_FCR2rs>;
#[doc = "Field `CFDCAN1F` writer - clear the illegal access flag for FDCAN1"]
pub type CFDCAN1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFDCAN2F` writer - clear the illegal access flag for FDCAN2"]
pub type CFDCAN2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUCPDF` writer - clear the illegal access flag for UCPD"]
pub type CUCPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIM1F` writer - clear the illegal access flag for TIM1"]
pub type CTIM1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPI1F` writer - clear the illegal access flag for SPI1"]
pub type CSPI1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIM8F` writer - clear the illegal access flag for TIM8"]
pub type CTIM8F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUSART1F` writer - clear the illegal access flag for USART1"]
pub type CUSART1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIM15F` writer - clear the illegal access flag for TIM15"]
pub type CTIM15F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIM16F` writer - clear the illegal access flag for TIM16"]
pub type CTIM16F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIM17F` writer - clear the illegal access flag for TIM17"]
pub type CTIM17F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPI4F` writer - clear the illegal access flag for SPI4"]
pub type CSPI4F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPI6F` writer - clear the illegal access flag for SPI6"]
pub type CSPI6F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSAI1F` writer - clear the illegal access flag for SAI1"]
pub type CSAI1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSAI2F` writer - clear the illegal access flag for SAI2"]
pub type CSAI2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUSBF` writer - clear the illegal access flag for USB"]
pub type CUSBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPI5F` writer - clear the illegal access flag for SPI5"]
pub type CSPI5F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLPUART1F` writer - clear the illegal access flag for LPUART"]
pub type CLPUART1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CI2C3F` writer - clear the illegal access flag for I2C3"]
pub type CI2C3F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CI2C4F` writer - clear the illegal access flag for I2C4"]
pub type CI2C4F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLPTIM1F` writer - clear the illegal access flag for LPTIM1"]
pub type CLPTIM1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLPTIM3F` writer - clear the illegal access flag for LPTIM3"]
pub type CLPTIM3F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLPTIM4F` writer - clear the illegal access flag for LPTIM4"]
pub type CLPTIM4F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLPTIM5F` writer - clear the illegal access flag for LPTIM5"]
pub type CLPTIM5F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - clear the illegal access flag for FDCAN1"]
    #[inline(always)]
    #[must_use]
    pub fn cfdcan1f(&mut self) -> CFDCAN1F_W<GTZC1_TZIC_FCR2rs> {
        CFDCAN1F_W::new(self, 0)
    }
    #[doc = "Bit 1 - clear the illegal access flag for FDCAN2"]
    #[inline(always)]
    #[must_use]
    pub fn cfdcan2f(&mut self) -> CFDCAN2F_W<GTZC1_TZIC_FCR2rs> {
        CFDCAN2F_W::new(self, 1)
    }
    #[doc = "Bit 2 - clear the illegal access flag for UCPD"]
    #[inline(always)]
    #[must_use]
    pub fn cucpdf(&mut self) -> CUCPDF_W<GTZC1_TZIC_FCR2rs> {
        CUCPDF_W::new(self, 2)
    }
    #[doc = "Bit 8 - clear the illegal access flag for TIM1"]
    #[inline(always)]
    #[must_use]
    pub fn ctim1f(&mut self) -> CTIM1F_W<GTZC1_TZIC_FCR2rs> {
        CTIM1F_W::new(self, 8)
    }
    #[doc = "Bit 9 - clear the illegal access flag for SPI1"]
    #[inline(always)]
    #[must_use]
    pub fn cspi1f(&mut self) -> CSPI1F_W<GTZC1_TZIC_FCR2rs> {
        CSPI1F_W::new(self, 9)
    }
    #[doc = "Bit 10 - clear the illegal access flag for TIM8"]
    #[inline(always)]
    #[must_use]
    pub fn ctim8f(&mut self) -> CTIM8F_W<GTZC1_TZIC_FCR2rs> {
        CTIM8F_W::new(self, 10)
    }
    #[doc = "Bit 11 - clear the illegal access flag for USART1"]
    #[inline(always)]
    #[must_use]
    pub fn cusart1f(&mut self) -> CUSART1F_W<GTZC1_TZIC_FCR2rs> {
        CUSART1F_W::new(self, 11)
    }
    #[doc = "Bit 12 - clear the illegal access flag for TIM15"]
    #[inline(always)]
    #[must_use]
    pub fn ctim15f(&mut self) -> CTIM15F_W<GTZC1_TZIC_FCR2rs> {
        CTIM15F_W::new(self, 12)
    }
    #[doc = "Bit 13 - clear the illegal access flag for TIM16"]
    #[inline(always)]
    #[must_use]
    pub fn ctim16f(&mut self) -> CTIM16F_W<GTZC1_TZIC_FCR2rs> {
        CTIM16F_W::new(self, 13)
    }
    #[doc = "Bit 14 - clear the illegal access flag for TIM17"]
    #[inline(always)]
    #[must_use]
    pub fn ctim17f(&mut self) -> CTIM17F_W<GTZC1_TZIC_FCR2rs> {
        CTIM17F_W::new(self, 14)
    }
    #[doc = "Bit 15 - clear the illegal access flag for SPI4"]
    #[inline(always)]
    #[must_use]
    pub fn cspi4f(&mut self) -> CSPI4F_W<GTZC1_TZIC_FCR2rs> {
        CSPI4F_W::new(self, 15)
    }
    #[doc = "Bit 16 - clear the illegal access flag for SPI6"]
    #[inline(always)]
    #[must_use]
    pub fn cspi6f(&mut self) -> CSPI6F_W<GTZC1_TZIC_FCR2rs> {
        CSPI6F_W::new(self, 16)
    }
    #[doc = "Bit 17 - clear the illegal access flag for SAI1"]
    #[inline(always)]
    #[must_use]
    pub fn csai1f(&mut self) -> CSAI1F_W<GTZC1_TZIC_FCR2rs> {
        CSAI1F_W::new(self, 17)
    }
    #[doc = "Bit 18 - clear the illegal access flag for SAI2"]
    #[inline(always)]
    #[must_use]
    pub fn csai2f(&mut self) -> CSAI2F_W<GTZC1_TZIC_FCR2rs> {
        CSAI2F_W::new(self, 18)
    }
    #[doc = "Bit 19 - clear the illegal access flag for USB"]
    #[inline(always)]
    #[must_use]
    pub fn cusbf(&mut self) -> CUSBF_W<GTZC1_TZIC_FCR2rs> {
        CUSBF_W::new(self, 19)
    }
    #[doc = "Bit 24 - clear the illegal access flag for SPI5"]
    #[inline(always)]
    #[must_use]
    pub fn cspi5f(&mut self) -> CSPI5F_W<GTZC1_TZIC_FCR2rs> {
        CSPI5F_W::new(self, 24)
    }
    #[doc = "Bit 25 - clear the illegal access flag for LPUART"]
    #[inline(always)]
    #[must_use]
    pub fn clpuart1f(&mut self) -> CLPUART1F_W<GTZC1_TZIC_FCR2rs> {
        CLPUART1F_W::new(self, 25)
    }
    #[doc = "Bit 26 - clear the illegal access flag for I2C3"]
    #[inline(always)]
    #[must_use]
    pub fn ci2c3f(&mut self) -> CI2C3F_W<GTZC1_TZIC_FCR2rs> {
        CI2C3F_W::new(self, 26)
    }
    #[doc = "Bit 27 - clear the illegal access flag for I2C4"]
    #[inline(always)]
    #[must_use]
    pub fn ci2c4f(&mut self) -> CI2C4F_W<GTZC1_TZIC_FCR2rs> {
        CI2C4F_W::new(self, 27)
    }
    #[doc = "Bit 28 - clear the illegal access flag for LPTIM1"]
    #[inline(always)]
    #[must_use]
    pub fn clptim1f(&mut self) -> CLPTIM1F_W<GTZC1_TZIC_FCR2rs> {
        CLPTIM1F_W::new(self, 28)
    }
    #[doc = "Bit 29 - clear the illegal access flag for LPTIM3"]
    #[inline(always)]
    #[must_use]
    pub fn clptim3f(&mut self) -> CLPTIM3F_W<GTZC1_TZIC_FCR2rs> {
        CLPTIM3F_W::new(self, 29)
    }
    #[doc = "Bit 30 - clear the illegal access flag for LPTIM4"]
    #[inline(always)]
    #[must_use]
    pub fn clptim4f(&mut self) -> CLPTIM4F_W<GTZC1_TZIC_FCR2rs> {
        CLPTIM4F_W::new(self, 30)
    }
    #[doc = "Bit 31 - clear the illegal access flag for LPTIM5"]
    #[inline(always)]
    #[must_use]
    pub fn clptim5f(&mut self) -> CLPTIM5F_W<GTZC1_TZIC_FCR2rs> {
        CLPTIM5F_W::new(self, 31)
    }
}
#[doc = "TZIC flag clear register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzic_fcr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTZC1_TZIC_FCR2rs;
impl crate::RegisterSpec for GTZC1_TZIC_FCR2rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gtzc1_tzic_fcr2::W`](W) writer structure"]
impl crate::Writable for GTZC1_TZIC_FCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTZC1_TZIC_FCR2 to value 0"]
impl crate::Resettable for GTZC1_TZIC_FCR2rs {
    const RESET_VALUE: u32 = 0;
}
