#[doc = "Register `SPI2S_CR1` reader"]
pub type R = crate::R<SPI2S_CR1rs>;
#[doc = "Register `SPI2S_CR1` writer"]
pub type W = crate::W<SPI2S_CR1rs>;
#[doc = "Field `SPE` reader - SPE"]
pub type SPE_R = crate::BitReader;
#[doc = "Field `SPE` writer - SPE"]
pub type SPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASRX` reader - MASRX"]
pub type MASRX_R = crate::BitReader;
#[doc = "Field `MASRX` writer - MASRX"]
pub type MASRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTART` reader - CSTART"]
pub type CSTART_R = crate::BitReader;
#[doc = "Field `CSTART` writer - CSTART"]
pub type CSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSUSP` writer - CSUSP"]
pub type CSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDDIR` reader - HDDIR"]
pub type HDDIR_R = crate::BitReader;
#[doc = "Field `HDDIR` writer - HDDIR"]
pub type HDDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSI` reader - SSI"]
pub type SSI_R = crate::BitReader;
#[doc = "Field `SSI` writer - SSI"]
pub type SSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC33_17` reader - CRC33_17"]
pub type CRC33_17_R = crate::BitReader;
#[doc = "Field `CRC33_17` writer - CRC33_17"]
pub type CRC33_17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCRCINI` reader - RCRCINI"]
pub type RCRCINI_R = crate::BitReader;
#[doc = "Field `RCRCINI` writer - RCRCINI"]
pub type RCRCINI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCRCINI` reader - TCRCINI"]
pub type TCRCINI_R = crate::BitReader;
#[doc = "Field `TCRCINI` writer - TCRCINI"]
pub type TCRCINI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOLOCK` reader - IOLOCK"]
pub type IOLOCK_R = crate::BitReader;
#[doc = "Field `IOLOCK` writer - IOLOCK"]
pub type IOLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPE"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - MASRX"]
    #[inline(always)]
    pub fn masrx(&self) -> MASRX_R {
        MASRX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CSTART"]
    #[inline(always)]
    pub fn cstart(&self) -> CSTART_R {
        CSTART_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - HDDIR"]
    #[inline(always)]
    pub fn hddir(&self) -> HDDIR_R {
        HDDIR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SSI"]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CRC33_17"]
    #[inline(always)]
    pub fn crc33_17(&self) -> CRC33_17_R {
        CRC33_17_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RCRCINI"]
    #[inline(always)]
    pub fn rcrcini(&self) -> RCRCINI_R {
        RCRCINI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TCRCINI"]
    #[inline(always)]
    pub fn tcrcini(&self) -> TCRCINI_R {
        TCRCINI_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - IOLOCK"]
    #[inline(always)]
    pub fn iolock(&self) -> IOLOCK_R {
        IOLOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPE"]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SPE_W<SPI2S_CR1rs> {
        SPE_W::new(self, 0)
    }
    #[doc = "Bit 8 - MASRX"]
    #[inline(always)]
    #[must_use]
    pub fn masrx(&mut self) -> MASRX_W<SPI2S_CR1rs> {
        MASRX_W::new(self, 8)
    }
    #[doc = "Bit 9 - CSTART"]
    #[inline(always)]
    #[must_use]
    pub fn cstart(&mut self) -> CSTART_W<SPI2S_CR1rs> {
        CSTART_W::new(self, 9)
    }
    #[doc = "Bit 10 - CSUSP"]
    #[inline(always)]
    #[must_use]
    pub fn csusp(&mut self) -> CSUSP_W<SPI2S_CR1rs> {
        CSUSP_W::new(self, 10)
    }
    #[doc = "Bit 11 - HDDIR"]
    #[inline(always)]
    #[must_use]
    pub fn hddir(&mut self) -> HDDIR_W<SPI2S_CR1rs> {
        HDDIR_W::new(self, 11)
    }
    #[doc = "Bit 12 - SSI"]
    #[inline(always)]
    #[must_use]
    pub fn ssi(&mut self) -> SSI_W<SPI2S_CR1rs> {
        SSI_W::new(self, 12)
    }
    #[doc = "Bit 13 - CRC33_17"]
    #[inline(always)]
    #[must_use]
    pub fn crc33_17(&mut self) -> CRC33_17_W<SPI2S_CR1rs> {
        CRC33_17_W::new(self, 13)
    }
    #[doc = "Bit 14 - RCRCINI"]
    #[inline(always)]
    #[must_use]
    pub fn rcrcini(&mut self) -> RCRCINI_W<SPI2S_CR1rs> {
        RCRCINI_W::new(self, 14)
    }
    #[doc = "Bit 15 - TCRCINI"]
    #[inline(always)]
    #[must_use]
    pub fn tcrcini(&mut self) -> TCRCINI_W<SPI2S_CR1rs> {
        TCRCINI_W::new(self, 15)
    }
    #[doc = "Bit 16 - IOLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn iolock(&mut self) -> IOLOCK_W<SPI2S_CR1rs> {
        IOLOCK_W::new(self, 16)
    }
}
#[doc = "SPI/I2S control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi2s_cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi2s_cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI2S_CR1rs;
impl crate::RegisterSpec for SPI2S_CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi2s_cr1::R`](R) reader structure"]
impl crate::Readable for SPI2S_CR1rs {}
#[doc = "`write(|w| ..)` method takes [`spi2s_cr1::W`](W) writer structure"]
impl crate::Writable for SPI2S_CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI2S_CR1 to value 0"]
impl crate::Resettable for SPI2S_CR1rs {
    const RESET_VALUE: u32 = 0;
}
