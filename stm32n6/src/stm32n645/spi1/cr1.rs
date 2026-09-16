///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `SPE` reader - serial peripheral enable
pub type SPE_R = crate::BitReader;
///Field `SPE` writer - serial peripheral enable
pub type SPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASRX` reader - master automatic suspension in Receive mode
pub type MASRX_R = crate::BitReader;
///Field `MASRX` writer - master automatic suspension in Receive mode
pub type MASRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSTART` reader - master transfer start
pub type CSTART_R = crate::BitReader;
///Field `CSTART` writer - master transfer start
pub type CSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSUSP` writer - master suspend request
pub type CSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HDDIR` reader - Rx/Tx direction at Half-duplex mode
pub type HDDIR_R = crate::BitReader;
///Field `HDDIR` writer - Rx/Tx direction at Half-duplex mode
pub type HDDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSI` reader - internal SS signal input level
pub type SSI_R = crate::BitReader;
///Field `SSI` writer - internal SS signal input level
pub type SSI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC33_17` reader - 32-bit CRC polynomial configuration
pub type CRC33_17_R = crate::BitReader;
///Field `CRC33_17` writer - 32-bit CRC polynomial configuration
pub type CRC33_17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCRCINI` reader - CRC calculation initialization pattern control for receiver
pub type RCRCINI_R = crate::BitReader;
///Field `RCRCINI` writer - CRC calculation initialization pattern control for receiver
pub type RCRCINI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCRCINI` reader - CRC calculation initialization pattern control for transmitter
pub type TCRCINI_R = crate::BitReader;
///Field `TCRCINI` writer - CRC calculation initialization pattern control for transmitter
pub type TCRCINI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IOLOCK` reader - locking the AF configuration of associated I/Os
pub type IOLOCK_R = crate::BitReader;
///Field `IOLOCK` writer - locking the AF configuration of associated I/Os
pub type IOLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - serial peripheral enable
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - master automatic suspension in Receive mode
    #[inline(always)]
    pub fn masrx(&self) -> MASRX_R {
        MASRX_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - master transfer start
    #[inline(always)]
    pub fn cstart(&self) -> CSTART_R {
        CSTART_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Rx/Tx direction at Half-duplex mode
    #[inline(always)]
    pub fn hddir(&self) -> HDDIR_R {
        HDDIR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - internal SS signal input level
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - 32-bit CRC polynomial configuration
    #[inline(always)]
    pub fn crc33_17(&self) -> CRC33_17_R {
        CRC33_17_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CRC calculation initialization pattern control for receiver
    #[inline(always)]
    pub fn rcrcini(&self) -> RCRCINI_R {
        RCRCINI_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CRC calculation initialization pattern control for transmitter
    #[inline(always)]
    pub fn tcrcini(&self) -> TCRCINI_R {
        TCRCINI_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - locking the AF configuration of associated I/Os
    #[inline(always)]
    pub fn iolock(&self) -> IOLOCK_R {
        IOLOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
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
    ///Bit 0 - serial peripheral enable
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W<'_, CR1rs> {
        SPE_W::new(self, 0)
    }
    ///Bit 8 - master automatic suspension in Receive mode
    #[inline(always)]
    pub fn masrx(&mut self) -> MASRX_W<'_, CR1rs> {
        MASRX_W::new(self, 8)
    }
    ///Bit 9 - master transfer start
    #[inline(always)]
    pub fn cstart(&mut self) -> CSTART_W<'_, CR1rs> {
        CSTART_W::new(self, 9)
    }
    ///Bit 10 - master suspend request
    #[inline(always)]
    pub fn csusp(&mut self) -> CSUSP_W<'_, CR1rs> {
        CSUSP_W::new(self, 10)
    }
    ///Bit 11 - Rx/Tx direction at Half-duplex mode
    #[inline(always)]
    pub fn hddir(&mut self) -> HDDIR_W<'_, CR1rs> {
        HDDIR_W::new(self, 11)
    }
    ///Bit 12 - internal SS signal input level
    #[inline(always)]
    pub fn ssi(&mut self) -> SSI_W<'_, CR1rs> {
        SSI_W::new(self, 12)
    }
    ///Bit 13 - 32-bit CRC polynomial configuration
    #[inline(always)]
    pub fn crc33_17(&mut self) -> CRC33_17_W<'_, CR1rs> {
        CRC33_17_W::new(self, 13)
    }
    ///Bit 14 - CRC calculation initialization pattern control for receiver
    #[inline(always)]
    pub fn rcrcini(&mut self) -> RCRCINI_W<'_, CR1rs> {
        RCRCINI_W::new(self, 14)
    }
    ///Bit 15 - CRC calculation initialization pattern control for transmitter
    #[inline(always)]
    pub fn tcrcini(&mut self) -> TCRCINI_W<'_, CR1rs> {
        TCRCINI_W::new(self, 15)
    }
    ///Bit 16 - locking the AF configuration of associated I/Os
    #[inline(always)]
    pub fn iolock(&mut self) -> IOLOCK_W<'_, CR1rs> {
        IOLOCK_W::new(self, 16)
    }
}
/**SPI/I2S control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#SPI1:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
