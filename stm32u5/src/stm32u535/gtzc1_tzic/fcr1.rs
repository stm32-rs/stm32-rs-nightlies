#[doc = "Register `FCR1` writer"]
pub type W = crate::W<FCR1rs>;
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
#[doc = "Field `CWWDGF` writer - clear the illegal access flag for WWDG"]
pub type CWWDGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIWDGF` writer - clear the illegal access flag for IWDG"]
pub type CIWDGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPI2F` writer - clear the illegal access flag for SPI2"]
pub type CSPI2F_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `CCRSF` writer - clear the illegal access flag for CRS"]
pub type CCRSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CI2C4F` writer - clear the illegal access flag for I2C4"]
pub type CI2C4F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLPTIM2F` writer - clear the illegal access flag for LPTIM2"]
pub type CLPTIM2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFDCAN1F` writer - clear the illegal access flag for FDCAN1"]
pub type CFDCAN1F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - clear the illegal access flag for TIM2"]
    #[inline(always)]
    #[must_use]
    pub fn ctim2f(&mut self) -> CTIM2F_W<FCR1rs> {
        CTIM2F_W::new(self, 0)
    }
    #[doc = "Bit 1 - clear the illegal access flag for TIM3"]
    #[inline(always)]
    #[must_use]
    pub fn ctim3f(&mut self) -> CTIM3F_W<FCR1rs> {
        CTIM3F_W::new(self, 1)
    }
    #[doc = "Bit 2 - clear the illegal access flag for TIM4"]
    #[inline(always)]
    #[must_use]
    pub fn ctim4f(&mut self) -> CTIM4F_W<FCR1rs> {
        CTIM4F_W::new(self, 2)
    }
    #[doc = "Bit 3 - clear the illegal access flag for TIM5"]
    #[inline(always)]
    #[must_use]
    pub fn ctim5f(&mut self) -> CTIM5F_W<FCR1rs> {
        CTIM5F_W::new(self, 3)
    }
    #[doc = "Bit 4 - clear the illegal access flag for TIM6"]
    #[inline(always)]
    #[must_use]
    pub fn ctim6f(&mut self) -> CTIM6F_W<FCR1rs> {
        CTIM6F_W::new(self, 4)
    }
    #[doc = "Bit 5 - clear the illegal access flag for TIM7"]
    #[inline(always)]
    #[must_use]
    pub fn ctim7f(&mut self) -> CTIM7F_W<FCR1rs> {
        CTIM7F_W::new(self, 5)
    }
    #[doc = "Bit 6 - clear the illegal access flag for WWDG"]
    #[inline(always)]
    #[must_use]
    pub fn cwwdgf(&mut self) -> CWWDGF_W<FCR1rs> {
        CWWDGF_W::new(self, 6)
    }
    #[doc = "Bit 7 - clear the illegal access flag for IWDG"]
    #[inline(always)]
    #[must_use]
    pub fn ciwdgf(&mut self) -> CIWDGF_W<FCR1rs> {
        CIWDGF_W::new(self, 7)
    }
    #[doc = "Bit 8 - clear the illegal access flag for SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn cspi2f(&mut self) -> CSPI2F_W<FCR1rs> {
        CSPI2F_W::new(self, 8)
    }
    #[doc = "Bit 10 - clear the illegal access flag for USART3"]
    #[inline(always)]
    #[must_use]
    pub fn cusart3f(&mut self) -> CUSART3F_W<FCR1rs> {
        CUSART3F_W::new(self, 10)
    }
    #[doc = "Bit 11 - clear the illegal access flag for UART4"]
    #[inline(always)]
    #[must_use]
    pub fn cuart4f(&mut self) -> CUART4F_W<FCR1rs> {
        CUART4F_W::new(self, 11)
    }
    #[doc = "Bit 12 - clear the illegal access flag for UART5"]
    #[inline(always)]
    #[must_use]
    pub fn cuart5f(&mut self) -> CUART5F_W<FCR1rs> {
        CUART5F_W::new(self, 12)
    }
    #[doc = "Bit 13 - clear the illegal access flag for I2C1"]
    #[inline(always)]
    #[must_use]
    pub fn ci2c1f(&mut self) -> CI2C1F_W<FCR1rs> {
        CI2C1F_W::new(self, 13)
    }
    #[doc = "Bit 14 - clear the illegal access flag for I2C2"]
    #[inline(always)]
    #[must_use]
    pub fn ci2c2f(&mut self) -> CI2C2F_W<FCR1rs> {
        CI2C2F_W::new(self, 14)
    }
    #[doc = "Bit 15 - clear the illegal access flag for CRS"]
    #[inline(always)]
    #[must_use]
    pub fn ccrsf(&mut self) -> CCRSF_W<FCR1rs> {
        CCRSF_W::new(self, 15)
    }
    #[doc = "Bit 16 - clear the illegal access flag for I2C4"]
    #[inline(always)]
    #[must_use]
    pub fn ci2c4f(&mut self) -> CI2C4F_W<FCR1rs> {
        CI2C4F_W::new(self, 16)
    }
    #[doc = "Bit 17 - clear the illegal access flag for LPTIM2"]
    #[inline(always)]
    #[must_use]
    pub fn clptim2f(&mut self) -> CLPTIM2F_W<FCR1rs> {
        CLPTIM2F_W::new(self, 17)
    }
    #[doc = "Bit 18 - clear the illegal access flag for FDCAN1"]
    #[inline(always)]
    #[must_use]
    pub fn cfdcan1f(&mut self) -> CFDCAN1F_W<FCR1rs> {
        CFDCAN1F_W::new(self, 18)
    }
}
#[doc = "TZIC flag clear register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCR1rs;
impl crate::RegisterSpec for FCR1rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcr1::W`](W) writer structure"]
impl crate::Writable for FCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR1 to value 0"]
impl crate::Resettable for FCR1rs {
    const RESET_VALUE: u32 = 0;
}
