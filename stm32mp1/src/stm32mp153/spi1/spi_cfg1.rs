///Register `SPI_CFG1` reader
pub type R = crate::R<SPI_CFG1rs>;
///Register `SPI_CFG1` writer
pub type W = crate::W<SPI_CFG1rs>;
///Field `DSIZE` reader - DSIZE
pub type DSIZE_R = crate::FieldReader;
///Field `DSIZE` writer - DSIZE
pub type DSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `FTHLV` reader - FTHLV
pub type FTHLV_R = crate::FieldReader;
///Field `FTHLV` writer - FTHLV
pub type FTHLV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `UDRCFG` reader - UDRCFG
pub type UDRCFG_R = crate::FieldReader;
///Field `UDRCFG` writer - UDRCFG
pub type UDRCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `UDRDET` reader - UDRDET
pub type UDRDET_R = crate::FieldReader;
///Field `UDRDET` writer - UDRDET
pub type UDRDET_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RXDMAEN` reader - RXDMAEN
pub type RXDMAEN_R = crate::BitReader;
///Field `RXDMAEN` writer - RXDMAEN
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXDMAEN` reader - TXDMAEN
pub type TXDMAEN_R = crate::BitReader;
///Field `TXDMAEN` writer - TXDMAEN
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCSIZE` reader - CRCSIZE
pub type CRCSIZE_R = crate::FieldReader;
///Field `CRCSIZE` writer - CRCSIZE
pub type CRCSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `CRCEN` reader - CRCEN
pub type CRCEN_R = crate::BitReader;
///Field `CRCEN` writer - CRCEN
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MBR` reader - MBR
pub type MBR_R = crate::FieldReader;
///Field `MBR` writer - MBR
pub type MBR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:4 - DSIZE
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:8 - FTHLV
    #[inline(always)]
    pub fn fthlv(&self) -> FTHLV_R {
        FTHLV_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bits 9:10 - UDRCFG
    #[inline(always)]
    pub fn udrcfg(&self) -> UDRCFG_R {
        UDRCFG_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 11:12 - UDRDET
    #[inline(always)]
    pub fn udrdet(&self) -> UDRDET_R {
        UDRDET_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 14 - RXDMAEN
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TXDMAEN
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:20 - CRCSIZE
    #[inline(always)]
    pub fn crcsize(&self) -> CRCSIZE_R {
        CRCSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 22 - CRCEN
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 28:30 - MBR
    #[inline(always)]
    pub fn mbr(&self) -> MBR_R {
        MBR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CFG1")
            .field("dsize", &self.dsize())
            .field("fthlv", &self.fthlv())
            .field("udrcfg", &self.udrcfg())
            .field("udrdet", &self.udrdet())
            .field("rxdmaen", &self.rxdmaen())
            .field("txdmaen", &self.txdmaen())
            .field("crcsize", &self.crcsize())
            .field("crcen", &self.crcen())
            .field("mbr", &self.mbr())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - DSIZE
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DSIZE_W<SPI_CFG1rs> {
        DSIZE_W::new(self, 0)
    }
    ///Bits 5:8 - FTHLV
    #[inline(always)]
    #[must_use]
    pub fn fthlv(&mut self) -> FTHLV_W<SPI_CFG1rs> {
        FTHLV_W::new(self, 5)
    }
    ///Bits 9:10 - UDRCFG
    #[inline(always)]
    #[must_use]
    pub fn udrcfg(&mut self) -> UDRCFG_W<SPI_CFG1rs> {
        UDRCFG_W::new(self, 9)
    }
    ///Bits 11:12 - UDRDET
    #[inline(always)]
    #[must_use]
    pub fn udrdet(&mut self) -> UDRDET_W<SPI_CFG1rs> {
        UDRDET_W::new(self, 11)
    }
    ///Bit 14 - RXDMAEN
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<SPI_CFG1rs> {
        RXDMAEN_W::new(self, 14)
    }
    ///Bit 15 - TXDMAEN
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<SPI_CFG1rs> {
        TXDMAEN_W::new(self, 15)
    }
    ///Bits 16:20 - CRCSIZE
    #[inline(always)]
    #[must_use]
    pub fn crcsize(&mut self) -> CRCSIZE_W<SPI_CFG1rs> {
        CRCSIZE_W::new(self, 16)
    }
    ///Bit 22 - CRCEN
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<SPI_CFG1rs> {
        CRCEN_W::new(self, 22)
    }
    ///Bits 28:30 - MBR
    #[inline(always)]
    #[must_use]
    pub fn mbr(&mut self) -> MBR_W<SPI_CFG1rs> {
        MBR_W::new(self, 28)
    }
}
/**Content of this register is write protected when SPI is enabled

You can [`read`](crate::Reg::read) this register and get [`spi_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:SPI_CFG1)*/
pub struct SPI_CFG1rs;
impl crate::RegisterSpec for SPI_CFG1rs {
    type Ux = u32;
}
///`read()` method returns [`spi_cfg1::R`](R) reader structure
impl crate::Readable for SPI_CFG1rs {}
///`write(|w| ..)` method takes [`spi_cfg1::W`](W) writer structure
impl crate::Writable for SPI_CFG1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SPI_CFG1 to value 0x0007_0007
impl crate::Resettable for SPI_CFG1rs {
    const RESET_VALUE: u32 = 0x0007_0007;
}
