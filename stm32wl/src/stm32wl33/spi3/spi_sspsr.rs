///Register `SPI_SSPSR` reader
pub type R = crate::R<SPI_SSPSRrs>;
///Register `SPI_SSPSR` writer
pub type W = crate::W<SPI_SSPSRrs>;
///Field `RXNE` reader - Receive buffer not empty - 0: Rx buffer empty - 1: Rx buffer not empty
pub type RXNE_R = crate::BitReader;
///Field `TXE` reader - Transmit buffer empty - 0: No more empty space in Tx buffer. (software shall not write data to the Tx buffer). - 1: At least one empty space in Tx buffer. (software may write data to the Tx buffer).
pub type TXE_R = crate::BitReader;
///Field `CHSIDE` reader - Channel side - 0: Channel Left has to be transmitted or has been received - 1: Channel Right has to be transmitted or has been received
pub type CHSIDE_R = crate::BitReader;
///Field `UDR` reader - Underrun flag - 0: No underrun occurred - 1: Underrun occurred
pub type UDR_R = crate::BitReader;
///Field `CRCERR` reader - CRC error flag - 0: CRC value received matches the SPIx_RXCRCR value - 1: CRC value received does not match the SPIx_RXCRCR value This flag is set by hardware and cleared by software writing 0.
pub type CRCERR_R = crate::BitReader;
///Field `CRCERR` writer - CRC error flag - 0: CRC value received matches the SPIx_RXCRCR value - 1: CRC value received does not match the SPIx_RXCRCR value This flag is set by hardware and cleared by software writing 0.
pub type CRCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODF` reader - Mode fault - 0: No mode fault occurred - 1: Mode fault occurred
pub type MODF_R = crate::BitReader;
///Field `OVR` reader - Overrun flag - 0: No overrun occurred - 1: Overrun occurred
pub type OVR_R = crate::BitReader;
///Field `BSY` reader - Busy flag - 0: SPI (or I2S) not busy - 1: SPI (or I2S) is busy in communication or Tx buffer is not empty This flag is set and cleared by hardware.
pub type BSY_R = crate::BitReader;
///Field `FRE` reader - Frame format error This flag is used for SPI in TI slave mode and I2S slave mode. Refer to Section 18.5.10: SPI error flags and Section 18.7.6: I2S error flags. This flag is set by hardware and reset when SPIx_SR is read by software. - 0: No frame format error - 1: A frame format error occurred
pub type FRE_R = crate::BitReader;
///Field `FRLVL` reader - FIFO reception level These bits are set and cleared by hardware. - 00: FIFO empty - 01: 1/4 FIFO - 10: 1/2 FIFO - 11: FIFO full
pub type FRLVL_R = crate::FieldReader;
///Field `FTLVL` reader - FIFO Transmission Level These bits are set and cleared by hardware. - 00: FIFO empty - 01: 1/4 FIFO - 10: 1/2 FIFO - 11: FIFO full (considered as FULL when the FIFO threshold is greater than 1/2)
pub type FTLVL_R = crate::FieldReader;
impl R {
    ///Bit 0 - Receive buffer not empty - 0: Rx buffer empty - 1: Rx buffer not empty
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit buffer empty - 0: No more empty space in Tx buffer. (software shall not write data to the Tx buffer). - 1: At least one empty space in Tx buffer. (software may write data to the Tx buffer).
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel side - 0: Channel Left has to be transmitted or has been received - 1: Channel Right has to be transmitted or has been received
    #[inline(always)]
    pub fn chside(&self) -> CHSIDE_R {
        CHSIDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Underrun flag - 0: No underrun occurred - 1: Underrun occurred
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CRC error flag - 0: CRC value received matches the SPIx_RXCRCR value - 1: CRC value received does not match the SPIx_RXCRCR value This flag is set by hardware and cleared by software writing 0.
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Mode fault - 0: No mode fault occurred - 1: Mode fault occurred
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Overrun flag - 0: No overrun occurred - 1: Overrun occurred
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Busy flag - 0: SPI (or I2S) not busy - 1: SPI (or I2S) is busy in communication or Tx buffer is not empty This flag is set and cleared by hardware.
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Frame format error This flag is used for SPI in TI slave mode and I2S slave mode. Refer to Section 18.5.10: SPI error flags and Section 18.7.6: I2S error flags. This flag is set by hardware and reset when SPIx_SR is read by software. - 0: No frame format error - 1: A frame format error occurred
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - FIFO reception level These bits are set and cleared by hardware. - 00: FIFO empty - 01: 1/4 FIFO - 10: 1/2 FIFO - 11: FIFO full
    #[inline(always)]
    pub fn frlvl(&self) -> FRLVL_R {
        FRLVL_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 11:12 - FIFO Transmission Level These bits are set and cleared by hardware. - 00: FIFO empty - 01: 1/4 FIFO - 10: 1/2 FIFO - 11: FIFO full (considered as FULL when the FIFO threshold is greater than 1/2)
    #[inline(always)]
    pub fn ftlvl(&self) -> FTLVL_R {
        FTLVL_R::new(((self.bits >> 11) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SSPSR")
            .field("rxne", &self.rxne())
            .field("txe", &self.txe())
            .field("chside", &self.chside())
            .field("udr", &self.udr())
            .field("crcerr", &self.crcerr())
            .field("modf", &self.modf())
            .field("ovr", &self.ovr())
            .field("bsy", &self.bsy())
            .field("fre", &self.fre())
            .field("frlvl", &self.frlvl())
            .field("ftlvl", &self.ftlvl())
            .finish()
    }
}
impl W {
    ///Bit 4 - CRC error flag - 0: CRC value received matches the SPIx_RXCRCR value - 1: CRC value received does not match the SPIx_RXCRCR value This flag is set by hardware and cleared by software writing 0.
    #[inline(always)]
    pub fn crcerr(&mut self) -> CRCERR_W<'_, SPI_SSPSRrs> {
        CRCERR_W::new(self, 4)
    }
}
/**SPI_SSPSR register

You can [`read`](crate::Reg::read) this register and get [`spi_sspsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sspsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI3:SPI_SSPSR)*/
pub struct SPI_SSPSRrs;
impl crate::RegisterSpec for SPI_SSPSRrs {
    type Ux = u32;
}
///`read()` method returns [`spi_sspsr::R`](R) reader structure
impl crate::Readable for SPI_SSPSRrs {}
///`write(|w| ..)` method takes [`spi_sspsr::W`](W) writer structure
impl crate::Writable for SPI_SSPSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPI_SSPSR to value 0x02
impl crate::Resettable for SPI_SSPSRrs {
    const RESET_VALUE: u32 = 0x02;
}
