///Register `SPI2S_CR1` reader
pub type R = crate::R<SPI2S_CR1rs>;
///Register `SPI2S_CR1` writer
pub type W = crate::W<SPI2S_CR1rs>;
///Field `SPE` reader - SPE
pub type SPE_R = crate::BitReader;
///Field `SPE` writer - SPE
pub type SPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASRX` reader - MASRX
pub type MASRX_R = crate::BitReader;
///Field `MASRX` writer - MASRX
pub type MASRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSTART` reader - CSTART
pub type CSTART_R = crate::BitReader;
///Field `CSTART` writer - CSTART
pub type CSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSUSP` writer - CSUSP
pub type CSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HDDIR` reader - HDDIR
pub type HDDIR_R = crate::BitReader;
///Field `HDDIR` writer - HDDIR
pub type HDDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSI` reader - SSI
pub type SSI_R = crate::BitReader;
///Field `SSI` writer - SSI
pub type SSI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC33_17` reader - CRC33_17
pub type CRC33_17_R = crate::BitReader;
///Field `CRC33_17` writer - CRC33_17
pub type CRC33_17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCRCINI` reader - RCRCINI
pub type RCRCINI_R = crate::BitReader;
///Field `RCRCINI` writer - RCRCINI
pub type RCRCINI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCRCINI` reader - TCRCINI
pub type TCRCINI_R = crate::BitReader;
///Field `TCRCINI` writer - TCRCINI
pub type TCRCINI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IOLOCK` reader - IOLOCK
pub type IOLOCK_R = crate::BitReader;
///Field `IOLOCK` writer - IOLOCK
pub type IOLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SPE
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - MASRX
    #[inline(always)]
    pub fn masrx(&self) -> MASRX_R {
        MASRX_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CSTART
    #[inline(always)]
    pub fn cstart(&self) -> CSTART_R {
        CSTART_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - HDDIR
    #[inline(always)]
    pub fn hddir(&self) -> HDDIR_R {
        HDDIR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SSI
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CRC33_17
    #[inline(always)]
    pub fn crc33_17(&self) -> CRC33_17_R {
        CRC33_17_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - RCRCINI
    #[inline(always)]
    pub fn rcrcini(&self) -> RCRCINI_R {
        RCRCINI_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TCRCINI
    #[inline(always)]
    pub fn tcrcini(&self) -> TCRCINI_R {
        TCRCINI_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - IOLOCK
    #[inline(always)]
    pub fn iolock(&self) -> IOLOCK_R {
        IOLOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI2S_CR1")
            .field("spe", &self.spe())
            .field("masrx", &self.masrx())
            .field("cstart", &self.cstart())
            .field("hddir", &self.hddir())
            .field("ssi", &self.ssi())
            .field("crc33_17", &self.crc33_17())
            .field("rcrcini", &self.rcrcini())
            .field("tcrcini", &self.tcrcini())
            .field("iolock", &self.iolock())
            .finish()
    }
}
impl W {
    ///Bit 0 - SPE
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W<'_, SPI2S_CR1rs> {
        SPE_W::new(self, 0)
    }
    ///Bit 8 - MASRX
    #[inline(always)]
    pub fn masrx(&mut self) -> MASRX_W<'_, SPI2S_CR1rs> {
        MASRX_W::new(self, 8)
    }
    ///Bit 9 - CSTART
    #[inline(always)]
    pub fn cstart(&mut self) -> CSTART_W<'_, SPI2S_CR1rs> {
        CSTART_W::new(self, 9)
    }
    ///Bit 10 - CSUSP
    #[inline(always)]
    pub fn csusp(&mut self) -> CSUSP_W<'_, SPI2S_CR1rs> {
        CSUSP_W::new(self, 10)
    }
    ///Bit 11 - HDDIR
    #[inline(always)]
    pub fn hddir(&mut self) -> HDDIR_W<'_, SPI2S_CR1rs> {
        HDDIR_W::new(self, 11)
    }
    ///Bit 12 - SSI
    #[inline(always)]
    pub fn ssi(&mut self) -> SSI_W<'_, SPI2S_CR1rs> {
        SSI_W::new(self, 12)
    }
    ///Bit 13 - CRC33_17
    #[inline(always)]
    pub fn crc33_17(&mut self) -> CRC33_17_W<'_, SPI2S_CR1rs> {
        CRC33_17_W::new(self, 13)
    }
    ///Bit 14 - RCRCINI
    #[inline(always)]
    pub fn rcrcini(&mut self) -> RCRCINI_W<'_, SPI2S_CR1rs> {
        RCRCINI_W::new(self, 14)
    }
    ///Bit 15 - TCRCINI
    #[inline(always)]
    pub fn tcrcini(&mut self) -> TCRCINI_W<'_, SPI2S_CR1rs> {
        TCRCINI_W::new(self, 15)
    }
    ///Bit 16 - IOLOCK
    #[inline(always)]
    pub fn iolock(&mut self) -> IOLOCK_W<'_, SPI2S_CR1rs> {
        IOLOCK_W::new(self, 16)
    }
}
/**SPI/I2S control register 1

You can [`read`](crate::Reg::read) this register and get [`spi2s_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2s_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:SPI2S_CR1)*/
pub struct SPI2S_CR1rs;
impl crate::RegisterSpec for SPI2S_CR1rs {
    type Ux = u32;
}
///`read()` method returns [`spi2s_cr1::R`](R) reader structure
impl crate::Readable for SPI2S_CR1rs {}
///`write(|w| ..)` method takes [`spi2s_cr1::W`](W) writer structure
impl crate::Writable for SPI2S_CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPI2S_CR1 to value 0
impl crate::Resettable for SPI2S_CR1rs {}
