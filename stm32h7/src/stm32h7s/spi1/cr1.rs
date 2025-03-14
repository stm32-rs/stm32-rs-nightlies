///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `SPE` reader - serial peripheral enable This bit is set by and cleared by software. When SPE = 1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE = 0. When SPE = 0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active.
pub type SPE_R = crate::BitReader;
///Field `SPE` writer - serial peripheral enable This bit is set by and cleared by software. When SPE = 1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE = 0. When SPE = 0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active.
pub type SPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASRX` reader - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it may happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension.
pub type MASRX_R = crate::BitReader;
///Field `MASRX` writer - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it may happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension.
pub type MASRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSTART` reader - master transfer start This bit can be set by software if SPI is enabled only to start an SPI or I2S/PCM communication. In SPI mode, it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In I2S/PCM mode, it is also cleared by hardware as described in the Section 80.9.8: Stop sequence. In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO.
pub type CSTART_R = crate::BitReader;
///Field `CSTART` writer - master transfer start This bit can be set by software if SPI is enabled only to start an SPI or I2S/PCM communication. In SPI mode, it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In I2S/PCM mode, it is also cleared by hardware as described in the Section 80.9.8: Stop sequence. In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO.
pub type CSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSUSP` writer - master suspend request This bit reads as zero. In Master mode, when this bit is set by software, the CSTART bit is reset at the end of the current frame and communication is suspended. The user has to check SUSP flag to check end of the frame transaction. The Master mode communication must be suspended (using this bit or keeping TXDR empty) before going to Low-power mode. Can be used in SPI or I2S mode. After software suspension, SUSP flag must be cleared and SPI disabled and re-enabled before the next transaction starts.
pub type CSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HDDIR` reader - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration.
pub type HDDIR_R = crate::BitReader;
///Field `HDDIR` writer - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration.
pub type HDDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSI` reader - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored.
pub type SSI_R = crate::BitReader;
///Field `SSI` writer - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored.
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
///Field `IOLOCK` reader - locking the AF configuration of associated I/Os This bit can be changed by software only when SPI is disabled (SPE = 0). It is cleared by hardware if a MODF event occurs When this bit is set, SPI_CFG2 register content cannot be modified. This bit is write-protected when SPI is enabled (SPE = 1).
pub type IOLOCK_R = crate::BitReader;
///Field `IOLOCK` writer - locking the AF configuration of associated I/Os This bit can be changed by software only when SPI is disabled (SPE = 0). It is cleared by hardware if a MODF event occurs When this bit is set, SPI_CFG2 register content cannot be modified. This bit is write-protected when SPI is enabled (SPE = 1).
pub type IOLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - serial peripheral enable This bit is set by and cleared by software. When SPE = 1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE = 0. When SPE = 0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active.
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it may happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension.
    #[inline(always)]
    pub fn masrx(&self) -> MASRX_R {
        MASRX_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - master transfer start This bit can be set by software if SPI is enabled only to start an SPI or I2S/PCM communication. In SPI mode, it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In I2S/PCM mode, it is also cleared by hardware as described in the Section 80.9.8: Stop sequence. In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO.
    #[inline(always)]
    pub fn cstart(&self) -> CSTART_R {
        CSTART_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration.
    #[inline(always)]
    pub fn hddir(&self) -> HDDIR_R {
        HDDIR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored.
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
    ///Bit 16 - locking the AF configuration of associated I/Os This bit can be changed by software only when SPI is disabled (SPE = 0). It is cleared by hardware if a MODF event occurs When this bit is set, SPI_CFG2 register content cannot be modified. This bit is write-protected when SPI is enabled (SPE = 1).
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
    ///Bit 0 - serial peripheral enable This bit is set by and cleared by software. When SPE = 1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE = 0. When SPE = 0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active.
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W<CR1rs> {
        SPE_W::new(self, 0)
    }
    ///Bit 8 - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it may happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension.
    #[inline(always)]
    pub fn masrx(&mut self) -> MASRX_W<CR1rs> {
        MASRX_W::new(self, 8)
    }
    ///Bit 9 - master transfer start This bit can be set by software if SPI is enabled only to start an SPI or I2S/PCM communication. In SPI mode, it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In I2S/PCM mode, it is also cleared by hardware as described in the Section 80.9.8: Stop sequence. In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO.
    #[inline(always)]
    pub fn cstart(&mut self) -> CSTART_W<CR1rs> {
        CSTART_W::new(self, 9)
    }
    ///Bit 10 - master suspend request This bit reads as zero. In Master mode, when this bit is set by software, the CSTART bit is reset at the end of the current frame and communication is suspended. The user has to check SUSP flag to check end of the frame transaction. The Master mode communication must be suspended (using this bit or keeping TXDR empty) before going to Low-power mode. Can be used in SPI or I2S mode. After software suspension, SUSP flag must be cleared and SPI disabled and re-enabled before the next transaction starts.
    #[inline(always)]
    pub fn csusp(&mut self) -> CSUSP_W<CR1rs> {
        CSUSP_W::new(self, 10)
    }
    ///Bit 11 - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration.
    #[inline(always)]
    pub fn hddir(&mut self) -> HDDIR_W<CR1rs> {
        HDDIR_W::new(self, 11)
    }
    ///Bit 12 - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored.
    #[inline(always)]
    pub fn ssi(&mut self) -> SSI_W<CR1rs> {
        SSI_W::new(self, 12)
    }
    ///Bit 13 - 32-bit CRC polynomial configuration
    #[inline(always)]
    pub fn crc33_17(&mut self) -> CRC33_17_W<CR1rs> {
        CRC33_17_W::new(self, 13)
    }
    ///Bit 14 - CRC calculation initialization pattern control for receiver
    #[inline(always)]
    pub fn rcrcini(&mut self) -> RCRCINI_W<CR1rs> {
        RCRCINI_W::new(self, 14)
    }
    ///Bit 15 - CRC calculation initialization pattern control for transmitter
    #[inline(always)]
    pub fn tcrcini(&mut self) -> TCRCINI_W<CR1rs> {
        TCRCINI_W::new(self, 15)
    }
    ///Bit 16 - locking the AF configuration of associated I/Os This bit can be changed by software only when SPI is disabled (SPE = 0). It is cleared by hardware if a MODF event occurs When this bit is set, SPI_CFG2 register content cannot be modified. This bit is write-protected when SPI is enabled (SPE = 1).
    #[inline(always)]
    pub fn iolock(&mut self) -> IOLOCK_W<CR1rs> {
        IOLOCK_W::new(self, 16)
    }
}
/**SPI/I2S control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SPI1:CR1)*/
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
