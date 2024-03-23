#[doc = "Register `FCR2` writer"]
pub type W = crate::W<FCR2rs>;
#[doc = "Field `CTIM1F` writer - clear the illegal access flag for TIM1"]
pub type CTIM1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPI1F` writer - clear the illegal access flag for SPI1"]
pub type CSPI1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIM8F` writer - clear the illegal access flag for TIM8"]
pub type CTIM8F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUSART1F` writer - clear the illegal access flag for USART1"]
pub type CUSART1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIM15F` writer - clear the illegal access flag for TIM5"]
pub type CTIM15F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIM16F` writer - clear the illegal access flag for TIM6"]
pub type CTIM16F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIM17F` writer - clear the illegal access flag for TIM7"]
pub type CTIM17F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSAI1F` writer - clear the illegal access flag for SAI1"]
pub type CSAI1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSAI2F` writer - clear the illegal access flag for SAI2"]
pub type CSAI2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTDCF` writer - clear the illegal access flag for LTDC"]
pub type CLTDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDSIF` writer - clear the illegal access flag for DSI"]
pub type CDSIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - clear the illegal access flag for TIM1"]
    #[inline(always)]
    #[must_use]
    pub fn ctim1f(&mut self) -> CTIM1F_W<FCR2rs> {
        CTIM1F_W::new(self, 0)
    }
    #[doc = "Bit 1 - clear the illegal access flag for SPI1"]
    #[inline(always)]
    #[must_use]
    pub fn cspi1f(&mut self) -> CSPI1F_W<FCR2rs> {
        CSPI1F_W::new(self, 1)
    }
    #[doc = "Bit 2 - clear the illegal access flag for TIM8"]
    #[inline(always)]
    #[must_use]
    pub fn ctim8f(&mut self) -> CTIM8F_W<FCR2rs> {
        CTIM8F_W::new(self, 2)
    }
    #[doc = "Bit 3 - clear the illegal access flag for USART1"]
    #[inline(always)]
    #[must_use]
    pub fn cusart1f(&mut self) -> CUSART1F_W<FCR2rs> {
        CUSART1F_W::new(self, 3)
    }
    #[doc = "Bit 4 - clear the illegal access flag for TIM5"]
    #[inline(always)]
    #[must_use]
    pub fn ctim15f(&mut self) -> CTIM15F_W<FCR2rs> {
        CTIM15F_W::new(self, 4)
    }
    #[doc = "Bit 5 - clear the illegal access flag for TIM6"]
    #[inline(always)]
    #[must_use]
    pub fn ctim16f(&mut self) -> CTIM16F_W<FCR2rs> {
        CTIM16F_W::new(self, 5)
    }
    #[doc = "Bit 6 - clear the illegal access flag for TIM7"]
    #[inline(always)]
    #[must_use]
    pub fn ctim17f(&mut self) -> CTIM17F_W<FCR2rs> {
        CTIM17F_W::new(self, 6)
    }
    #[doc = "Bit 7 - clear the illegal access flag for SAI1"]
    #[inline(always)]
    #[must_use]
    pub fn csai1f(&mut self) -> CSAI1F_W<FCR2rs> {
        CSAI1F_W::new(self, 7)
    }
    #[doc = "Bit 8 - clear the illegal access flag for SAI2"]
    #[inline(always)]
    #[must_use]
    pub fn csai2f(&mut self) -> CSAI2F_W<FCR2rs> {
        CSAI2F_W::new(self, 8)
    }
    #[doc = "Bit 9 - clear the illegal access flag for LTDC"]
    #[inline(always)]
    #[must_use]
    pub fn cltdcf(&mut self) -> CLTDCF_W<FCR2rs> {
        CLTDCF_W::new(self, 9)
    }
    #[doc = "Bit 10 - clear the illegal access flag for DSI"]
    #[inline(always)]
    #[must_use]
    pub fn cdsif(&mut self) -> CDSIF_W<FCR2rs> {
        CDSIF_W::new(self, 10)
    }
}
#[doc = "TZIC flag clear register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCR2rs;
impl crate::RegisterSpec for FCR2rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcr2::W`](W) writer structure"]
impl crate::Writable for FCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR2 to value 0"]
impl crate::Resettable for FCR2rs {
    const RESET_VALUE: u32 = 0;
}
