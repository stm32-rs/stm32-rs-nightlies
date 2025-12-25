///Register `CFG1` reader
pub type R = crate::R<CFG1rs>;
///Register `CFG1` writer
pub type W = crate::W<CFG1rs>;
///Field `DSIZE` reader - number of bits in at single SPI data frame
pub type DSIZE_R = crate::FieldReader;
///Field `DSIZE` writer - number of bits in at single SPI data frame
pub type DSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `FTHLV` reader - FIFO threshold level
pub type FTHLV_R = crate::FieldReader;
///Field `FTHLV` writer - FIFO threshold level
pub type FTHLV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `UDRCFG` reader - behavior of slave transmitter at underrun condition
pub type UDRCFG_R = crate::BitReader;
///Field `UDRCFG` writer - behavior of slave transmitter at underrun condition
pub type UDRCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXDMAEN` reader - Rx DMA stream enable
pub type RXDMAEN_R = crate::BitReader;
///Field `RXDMAEN` writer - Rx DMA stream enable
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXDMAEN` reader - Tx DMA stream enable
pub type TXDMAEN_R = crate::BitReader;
///Field `TXDMAEN` writer - Tx DMA stream enable
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCSIZE` reader - length of CRC frame to be transacted and compared
pub type CRCSIZE_R = crate::FieldReader;
///Field `CRCSIZE` writer - length of CRC frame to be transacted and compared
pub type CRCSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `CRCEN` reader - hardware CRC computation enable
pub type CRCEN_R = crate::BitReader;
///Field `CRCEN` writer - hardware CRC computation enable
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MBR` reader - master baud rate prescaler setting
pub type MBR_R = crate::FieldReader;
///Field `MBR` writer - master baud rate prescaler setting
pub type MBR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BPASS` reader - bypass of the prescaler at master baud rate clock generator
pub type BPASS_R = crate::BitReader;
///Field `BPASS` writer - bypass of the prescaler at master baud rate clock generator
pub type BPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - number of bits in at single SPI data frame
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:8 - FIFO threshold level
    #[inline(always)]
    pub fn fthlv(&self) -> FTHLV_R {
        FTHLV_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - behavior of slave transmitter at underrun condition
    #[inline(always)]
    pub fn udrcfg(&self) -> UDRCFG_R {
        UDRCFG_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - Rx DMA stream enable
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Tx DMA stream enable
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:20 - length of CRC frame to be transacted and compared
    #[inline(always)]
    pub fn crcsize(&self) -> CRCSIZE_R {
        CRCSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 22 - hardware CRC computation enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 28:30 - master baud rate prescaler setting
    #[inline(always)]
    pub fn mbr(&self) -> MBR_R {
        MBR_R::new(((self.bits >> 28) & 7) as u8)
    }
    ///Bit 31 - bypass of the prescaler at master baud rate clock generator
    #[inline(always)]
    pub fn bpass(&self) -> BPASS_R {
        BPASS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG1")
            .field("dsize", &self.dsize())
            .field("fthlv", &self.fthlv())
            .field("udrcfg", &self.udrcfg())
            .field("rxdmaen", &self.rxdmaen())
            .field("txdmaen", &self.txdmaen())
            .field("crcsize", &self.crcsize())
            .field("crcen", &self.crcen())
            .field("mbr", &self.mbr())
            .field("bpass", &self.bpass())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - number of bits in at single SPI data frame
    #[inline(always)]
    pub fn dsize(&mut self) -> DSIZE_W<'_, CFG1rs> {
        DSIZE_W::new(self, 0)
    }
    ///Bits 5:8 - FIFO threshold level
    #[inline(always)]
    pub fn fthlv(&mut self) -> FTHLV_W<'_, CFG1rs> {
        FTHLV_W::new(self, 5)
    }
    ///Bit 9 - behavior of slave transmitter at underrun condition
    #[inline(always)]
    pub fn udrcfg(&mut self) -> UDRCFG_W<'_, CFG1rs> {
        UDRCFG_W::new(self, 9)
    }
    ///Bit 14 - Rx DMA stream enable
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<'_, CFG1rs> {
        RXDMAEN_W::new(self, 14)
    }
    ///Bit 15 - Tx DMA stream enable
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<'_, CFG1rs> {
        TXDMAEN_W::new(self, 15)
    }
    ///Bits 16:20 - length of CRC frame to be transacted and compared
    #[inline(always)]
    pub fn crcsize(&mut self) -> CRCSIZE_W<'_, CFG1rs> {
        CRCSIZE_W::new(self, 16)
    }
    ///Bit 22 - hardware CRC computation enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, CFG1rs> {
        CRCEN_W::new(self, 22)
    }
    ///Bits 28:30 - master baud rate prescaler setting
    #[inline(always)]
    pub fn mbr(&mut self) -> MBR_W<'_, CFG1rs> {
        MBR_W::new(self, 28)
    }
    ///Bit 31 - bypass of the prescaler at master baud rate clock generator
    #[inline(always)]
    pub fn bpass(&mut self) -> BPASS_W<'_, CFG1rs> {
        BPASS_W::new(self, 31)
    }
}
/**SPI/I2S configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SPI1:CFG1)*/
pub struct CFG1rs;
impl crate::RegisterSpec for CFG1rs {
    type Ux = u32;
}
///`read()` method returns [`cfg1::R`](R) reader structure
impl crate::Readable for CFG1rs {}
///`write(|w| ..)` method takes [`cfg1::W`](W) writer structure
impl crate::Writable for CFG1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFG1 to value 0x0007_0007
impl crate::Resettable for CFG1rs {
    const RESET_VALUE: u32 = 0x0007_0007;
}
