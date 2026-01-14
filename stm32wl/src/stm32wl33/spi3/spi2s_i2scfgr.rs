///Register `SPI2S_I2SCFGR` reader
pub type R = crate::R<SPI2S_I2SCFGRrs>;
///Register `SPI2S_I2SCFGR` writer
pub type W = crate::W<SPI2S_I2SCFGRrs>;
///Field `CHLEN` reader - Channel length (number of bits per audio channel) - 0: 16-bit wide - 1: 32-bit wide The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in.
pub type CHLEN_R = crate::BitReader;
///Field `CHLEN` writer - Channel length (number of bits per audio channel) - 0: 16-bit wide - 1: 32-bit wide The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in.
pub type CHLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATLEN` reader - Data length to be transferred - 00: 16-bit data length - 01: 24-bit data length - 10: 32-bit data length - 11: Not allowed
pub type DATLEN_R = crate::FieldReader;
///Field `DATLEN` writer - Data length to be transferred - 00: 16-bit data length - 01: 24-bit data length - 10: 32-bit data length - 11: Not allowed
pub type DATLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CKPOL` reader - Steady state clock polarity - 0: I2S clock steady state is low level - 1: I2S clock steady state is high level
pub type CKPOL_R = crate::BitReader;
///Field `CKPOL` writer - Steady state clock polarity - 0: I2S clock steady state is low level - 1: I2S clock steady state is high level
pub type CKPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2SSTD` reader - I2S standard selection - 00: I2S Philips standard. - 01: MSB justified standard (left justified) - 10: LSB justified standard (right justified) - 11: PCM standard
pub type I2SSTD_R = crate::FieldReader;
///Field `I2SSTD` writer - I2S standard selection - 00: I2S Philips standard. - 01: MSB justified standard (left justified) - 10: LSB justified standard (right justified) - 11: PCM standard
pub type I2SSTD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PCMSYNC` reader - PCM frame synchronization - 0: Short frame synchronization - 1: Long frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode.
pub type PCMSYNC_R = crate::BitReader;
///Field `PCMSYNC` writer - PCM frame synchronization - 0: Short frame synchronization - 1: Long frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode.
pub type PCMSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2SCFG` reader - I2S configuration mode - 00: Slave - transmit - 01: Slave - receive - 10: Master - transmit - 11: Master - receive
pub type I2SCFG_R = crate::FieldReader;
///Field `I2SCFG` writer - I2S configuration mode - 00: Slave - transmit - 01: Slave - receive - 10: Master - transmit - 11: Master - receive
pub type I2SCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2SE` reader - I2S enable - 0: I2S peripheral is disabled - 1: I2S peripheral is enabled Note: This bit is not used in SPI mode.
pub type I2SE_R = crate::BitReader;
///Field `I2SE` writer - I2S enable - 0: I2S peripheral is disabled - 1: I2S peripheral is enabled Note: This bit is not used in SPI mode.
pub type I2SE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2SMOD` reader - I2S mode selection - 0: SPI mode is selected - 1: I2S mode is selected Note: This bit should be configured when the SPI is disabled.
pub type I2SMOD_R = crate::BitReader;
///Field `I2SMOD` writer - I2S mode selection - 0: SPI mode is selected - 1: I2S mode is selected Note: This bit should be configured when the SPI is disabled.
pub type I2SMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASTREN` reader - Asynchronous start enable. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards.
pub type ASTREN_R = crate::BitReader;
///Field `ASTREN` writer - Asynchronous start enable. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards.
pub type ASTREN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Channel length (number of bits per audio channel) - 0: 16-bit wide - 1: 32-bit wide The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in.
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Data length to be transferred - 00: 16-bit data length - 01: 24-bit data length - 10: 32-bit data length - 11: Not allowed
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - Steady state clock polarity - 0: I2S clock steady state is low level - 1: I2S clock steady state is high level
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - I2S standard selection - 00: I2S Philips standard. - 01: MSB justified standard (left justified) - 10: LSB justified standard (right justified) - 11: PCM standard
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - PCM frame synchronization - 0: Short frame synchronization - 1: Long frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode.
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - I2S configuration mode - 00: Slave - transmit - 01: Slave - receive - 10: Master - transmit - 11: Master - receive
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - I2S enable - 0: I2S peripheral is disabled - 1: I2S peripheral is enabled Note: This bit is not used in SPI mode.
    #[inline(always)]
    pub fn i2se(&self) -> I2SE_R {
        I2SE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - I2S mode selection - 0: SPI mode is selected - 1: I2S mode is selected Note: This bit should be configured when the SPI is disabled.
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Asynchronous start enable. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards.
    #[inline(always)]
    pub fn astren(&self) -> ASTREN_R {
        ASTREN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI2S_I2SCFGR")
            .field("chlen", &self.chlen())
            .field("datlen", &self.datlen())
            .field("ckpol", &self.ckpol())
            .field("i2sstd", &self.i2sstd())
            .field("pcmsync", &self.pcmsync())
            .field("i2scfg", &self.i2scfg())
            .field("i2se", &self.i2se())
            .field("i2smod", &self.i2smod())
            .field("astren", &self.astren())
            .finish()
    }
}
impl W {
    ///Bit 0 - Channel length (number of bits per audio channel) - 0: 16-bit wide - 1: 32-bit wide The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in.
    #[inline(always)]
    pub fn chlen(&mut self) -> CHLEN_W<'_, SPI2S_I2SCFGRrs> {
        CHLEN_W::new(self, 0)
    }
    ///Bits 1:2 - Data length to be transferred - 00: 16-bit data length - 01: 24-bit data length - 10: 32-bit data length - 11: Not allowed
    #[inline(always)]
    pub fn datlen(&mut self) -> DATLEN_W<'_, SPI2S_I2SCFGRrs> {
        DATLEN_W::new(self, 1)
    }
    ///Bit 3 - Steady state clock polarity - 0: I2S clock steady state is low level - 1: I2S clock steady state is high level
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W<'_, SPI2S_I2SCFGRrs> {
        CKPOL_W::new(self, 3)
    }
    ///Bits 4:5 - I2S standard selection - 00: I2S Philips standard. - 01: MSB justified standard (left justified) - 10: LSB justified standard (right justified) - 11: PCM standard
    #[inline(always)]
    pub fn i2sstd(&mut self) -> I2SSTD_W<'_, SPI2S_I2SCFGRrs> {
        I2SSTD_W::new(self, 4)
    }
    ///Bit 7 - PCM frame synchronization - 0: Short frame synchronization - 1: Long frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode.
    #[inline(always)]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<'_, SPI2S_I2SCFGRrs> {
        PCMSYNC_W::new(self, 7)
    }
    ///Bits 8:9 - I2S configuration mode - 00: Slave - transmit - 01: Slave - receive - 10: Master - transmit - 11: Master - receive
    #[inline(always)]
    pub fn i2scfg(&mut self) -> I2SCFG_W<'_, SPI2S_I2SCFGRrs> {
        I2SCFG_W::new(self, 8)
    }
    ///Bit 10 - I2S enable - 0: I2S peripheral is disabled - 1: I2S peripheral is enabled Note: This bit is not used in SPI mode.
    #[inline(always)]
    pub fn i2se(&mut self) -> I2SE_W<'_, SPI2S_I2SCFGRrs> {
        I2SE_W::new(self, 10)
    }
    ///Bit 11 - I2S mode selection - 0: SPI mode is selected - 1: I2S mode is selected Note: This bit should be configured when the SPI is disabled.
    #[inline(always)]
    pub fn i2smod(&mut self) -> I2SMOD_W<'_, SPI2S_I2SCFGRrs> {
        I2SMOD_W::new(self, 11)
    }
    ///Bit 12 - Asynchronous start enable. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards.
    #[inline(always)]
    pub fn astren(&mut self) -> ASTREN_W<'_, SPI2S_I2SCFGRrs> {
        ASTREN_W::new(self, 12)
    }
}
/**SPI2S_I2SCFGR register

You can [`read`](crate::Reg::read) this register and get [`spi2s_i2scfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2s_i2scfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI3:SPI2S_I2SCFGR)*/
pub struct SPI2S_I2SCFGRrs;
impl crate::RegisterSpec for SPI2S_I2SCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`spi2s_i2scfgr::R`](R) reader structure
impl crate::Readable for SPI2S_I2SCFGRrs {}
///`write(|w| ..)` method takes [`spi2s_i2scfgr::W`](W) writer structure
impl crate::Writable for SPI2S_I2SCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPI2S_I2SCFGR to value 0
impl crate::Resettable for SPI2S_I2SCFGRrs {}
