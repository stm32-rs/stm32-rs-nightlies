#[doc = "Register `FCR1` writer"]
pub type W = crate::W<FCR1rs>;
#[doc = "Field `CSPI3F` writer - clear the illegal access flag for SPI3"]
pub type CSPI3F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLPUART1F` writer - clear the illegal access flag for LPUART1"]
pub type CLPUART1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CI2C3F` writer - clear the illegal access flag for I2C3"]
pub type CI2C3F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLPTIM1F` writer - clear the illegal access flag for LPTIM1"]
pub type CLPTIM1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLPTIM3F` writer - clear the illegal access flag for LPTIM3"]
pub type CLPTIM3F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLPTIM4F` writer - clear the illegal access flag for LPTIM4"]
pub type CLPTIM4F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COPAMPF` writer - clear the illegal access flag for OPAMP"]
pub type COPAMPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCOMPF` writer - clear the illegal access flag for COMP"]
pub type CCOMPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CADC2F` writer - clear the illegal access flag for ADC2"]
pub type CADC2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CVREFBUFF` writer - clear the illegal access flag for VREFBUF"]
pub type CVREFBUFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDAC1F` writer - clear the illegal access flag for DAC1"]
pub type CDAC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CADF1F` writer - clear the illegal access flag for ADF1"]
pub type CADF1F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - clear the illegal access flag for SPI3"]
    #[inline(always)]
    #[must_use]
    pub fn cspi3f(&mut self) -> CSPI3F_W<FCR1rs> {
        CSPI3F_W::new(self, 0)
    }
    #[doc = "Bit 1 - clear the illegal access flag for LPUART1"]
    #[inline(always)]
    #[must_use]
    pub fn clpuart1f(&mut self) -> CLPUART1F_W<FCR1rs> {
        CLPUART1F_W::new(self, 1)
    }
    #[doc = "Bit 2 - clear the illegal access flag for I2C3"]
    #[inline(always)]
    #[must_use]
    pub fn ci2c3f(&mut self) -> CI2C3F_W<FCR1rs> {
        CI2C3F_W::new(self, 2)
    }
    #[doc = "Bit 3 - clear the illegal access flag for LPTIM1"]
    #[inline(always)]
    #[must_use]
    pub fn clptim1f(&mut self) -> CLPTIM1F_W<FCR1rs> {
        CLPTIM1F_W::new(self, 3)
    }
    #[doc = "Bit 4 - clear the illegal access flag for LPTIM3"]
    #[inline(always)]
    #[must_use]
    pub fn clptim3f(&mut self) -> CLPTIM3F_W<FCR1rs> {
        CLPTIM3F_W::new(self, 4)
    }
    #[doc = "Bit 5 - clear the illegal access flag for LPTIM4"]
    #[inline(always)]
    #[must_use]
    pub fn clptim4f(&mut self) -> CLPTIM4F_W<FCR1rs> {
        CLPTIM4F_W::new(self, 5)
    }
    #[doc = "Bit 6 - clear the illegal access flag for OPAMP"]
    #[inline(always)]
    #[must_use]
    pub fn copampf(&mut self) -> COPAMPF_W<FCR1rs> {
        COPAMPF_W::new(self, 6)
    }
    #[doc = "Bit 7 - clear the illegal access flag for COMP"]
    #[inline(always)]
    #[must_use]
    pub fn ccompf(&mut self) -> CCOMPF_W<FCR1rs> {
        CCOMPF_W::new(self, 7)
    }
    #[doc = "Bit 8 - clear the illegal access flag for ADC2"]
    #[inline(always)]
    #[must_use]
    pub fn cadc2f(&mut self) -> CADC2F_W<FCR1rs> {
        CADC2F_W::new(self, 8)
    }
    #[doc = "Bit 9 - clear the illegal access flag for VREFBUF"]
    #[inline(always)]
    #[must_use]
    pub fn cvrefbuff(&mut self) -> CVREFBUFF_W<FCR1rs> {
        CVREFBUFF_W::new(self, 9)
    }
    #[doc = "Bit 11 - clear the illegal access flag for DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn cdac1f(&mut self) -> CDAC1F_W<FCR1rs> {
        CDAC1F_W::new(self, 11)
    }
    #[doc = "Bit 12 - clear the illegal access flag for ADF1"]
    #[inline(always)]
    #[must_use]
    pub fn cadf1f(&mut self) -> CADF1F_W<FCR1rs> {
        CADF1F_W::new(self, 12)
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
