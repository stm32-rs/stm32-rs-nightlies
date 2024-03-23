#[doc = "Register `SPI_CR1` reader"]
pub type R = crate::R<SPI_CR1rs>;
#[doc = "Register `SPI_CR1` writer"]
pub type W = crate::W<SPI_CR1rs>;
#[doc = "Field `SPE` reader - serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, part of SPI_AUTOCR register and IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active."]
pub type SPE_R = crate::BitReader;
#[doc = "Field `SPE` writer - serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, part of SPI_AUTOCR register and IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active."]
pub type SPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASRX` reader - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension."]
pub type MASRX_R = crate::BitReader;
#[doc = "Field `MASRX` writer - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension."]
pub type MASRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTART` reader - master transfer start This bit can be set by software if SPI is enabled only to start an SPI communication. it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO."]
pub type CSTART_R = crate::BitReader;
#[doc = "Field `CSTART` writer - master transfer start This bit can be set by software if SPI is enabled only to start an SPI communication. it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO."]
pub type CSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSUSP` writer - master SUSPend request This bit reads as zero. In Master mode, when this bit is set by software, the CSTART bit is reset at the end of the current frame and SPI communication is suspended. The user has to check SUSP flag to check end of the frame transaction. The Master mode communication must be suspended (using this bit or keeping TXDR empty) before disabling the SPI or going to Low-power mode. After software suspension, SUSP flag has to be cleared and SPI disabled and re-enabled before the next transaction starts."]
pub type CSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDDIR` reader - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration."]
pub type HDDIR_R = crate::BitReader;
#[doc = "Field `HDDIR` writer - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration."]
pub type HDDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSI` reader - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored."]
pub type SSI_R = crate::BitReader;
#[doc = "Field `SSI` writer - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored."]
pub type SSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC33_17` reader - 32-bit CRC polynomial configuration"]
pub type CRC33_17_R = crate::BitReader;
#[doc = "Field `CRC33_17` writer - 32-bit CRC polynomial configuration"]
pub type CRC33_17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCRCINI` reader - CRC calculation initialization pattern control for receiver"]
pub type RCRCINI_R = crate::BitReader;
#[doc = "Field `RCRCINI` writer - CRC calculation initialization pattern control for receiver"]
pub type RCRCINI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCRCINI` reader - CRC calculation initialization pattern control for transmitter"]
pub type TCRCINI_R = crate::BitReader;
#[doc = "Field `TCRCINI` writer - CRC calculation initialization pattern control for transmitter"]
pub type TCRCINI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOLOCK` reader - locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set."]
pub type IOLOCK_R = crate::BitReader;
#[doc = "Field `IOLOCK` writer - locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set."]
pub type IOLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, part of SPI_AUTOCR register and IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active."]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension."]
    #[inline(always)]
    pub fn masrx(&self) -> MASRX_R {
        MASRX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - master transfer start This bit can be set by software if SPI is enabled only to start an SPI communication. it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO."]
    #[inline(always)]
    pub fn cstart(&self) -> CSTART_R {
        CSTART_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration."]
    #[inline(always)]
    pub fn hddir(&self) -> HDDIR_R {
        HDDIR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored."]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 32-bit CRC polynomial configuration"]
    #[inline(always)]
    pub fn crc33_17(&self) -> CRC33_17_R {
        CRC33_17_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CRC calculation initialization pattern control for receiver"]
    #[inline(always)]
    pub fn rcrcini(&self) -> RCRCINI_R {
        RCRCINI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CRC calculation initialization pattern control for transmitter"]
    #[inline(always)]
    pub fn tcrcini(&self) -> TCRCINI_R {
        TCRCINI_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set."]
    #[inline(always)]
    pub fn iolock(&self) -> IOLOCK_R {
        IOLOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - serial peripheral enable This bit is set by and cleared by software. When SPE=1, SPI data transfer is enabled, SPI_CFG1 and SPI_CFG2 configuration registers, CRCPOLY, UDRDR, part of SPI_AUTOCR register and IOLOCK bit in the SPI_CR1 register are write protected. They can be changed only when SPE=0. When SPE=0 any SPI operation is stopped and disabled, all the pending requests of the events with enabled interrupt are blocked except the MODF interrupt request (but their pending still propagates the request of the spi_plck clock), the SS output is deactivated at master, the RDY signal keeps not ready status at slave, the internal state machine is reseted, all the FIFOs content is flushed, CRC calculation initialized, receive data register is read zero. SPE is cleared and cannot be set when MODF error flag is active."]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SPE_W<SPI_CR1rs> {
        SPE_W::new(self, 0)
    }
    #[doc = "Bit 8 - master automatic suspension in Receive mode This bit is set and cleared by software to control continuous SPI transfer in master receiver mode and automatic management in order to avoid overrun condition. When SPI communication is suspended by hardware automatically, it could happen that few bits of next frame are already clocked out due to internal synchronization delay. This is why, the automatic suspension is not quite reliable when size of data drops below 8 bits. In this case, a safe suspension can be achieved by combination with delay inserted between data frames applied when MIDI parameter keeps a non zero value; sum of data size and the interleaved SPI cycles should always produce interval at length of 8 SPI clock periods at minimum. After software clearing of the SUSP bit, the communication resumes and continues by subsequent bits transaction without any next constraint. Prior the SUSP bit is cleared, the user must release the RxFIFO space as much as possible by reading out all the data packets available at RxFIFO based on the RXP flag indication to prevent any subsequent suspension."]
    #[inline(always)]
    #[must_use]
    pub fn masrx(&mut self) -> MASRX_W<SPI_CR1rs> {
        MASRX_W::new(self, 8)
    }
    #[doc = "Bit 9 - master transfer start This bit can be set by software if SPI is enabled only to start an SPI communication. it is cleared by hardware when end of transfer (EOT) flag is set or when a transaction suspend request is accepted. In SPI mode, the bit is taken into account at master mode only. If transmission is enabled, communication starts or continues only if any data is available in the transmission FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn cstart(&mut self) -> CSTART_W<SPI_CR1rs> {
        CSTART_W::new(self, 9)
    }
    #[doc = "Bit 10 - master SUSPend request This bit reads as zero. In Master mode, when this bit is set by software, the CSTART bit is reset at the end of the current frame and SPI communication is suspended. The user has to check SUSP flag to check end of the frame transaction. The Master mode communication must be suspended (using this bit or keeping TXDR empty) before disabling the SPI or going to Low-power mode. After software suspension, SUSP flag has to be cleared and SPI disabled and re-enabled before the next transaction starts."]
    #[inline(always)]
    #[must_use]
    pub fn csusp(&mut self) -> CSUSP_W<SPI_CR1rs> {
        CSUSP_W::new(self, 10)
    }
    #[doc = "Bit 11 - Rx/Tx direction at Half-duplex mode In Half-Duplex configuration the HDDIR bit establishes the Rx/Tx direction of the data transfer. This bit is ignored in Full-Duplex or any Simplex configuration."]
    #[inline(always)]
    #[must_use]
    pub fn hddir(&mut self) -> HDDIR_W<SPI_CR1rs> {
        HDDIR_W::new(self, 11)
    }
    #[doc = "Bit 12 - internal SS signal input level This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the peripheral SS input internally and the I/O value of the SS pin is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn ssi(&mut self) -> SSI_W<SPI_CR1rs> {
        SSI_W::new(self, 12)
    }
    #[doc = "Bit 13 - 32-bit CRC polynomial configuration"]
    #[inline(always)]
    #[must_use]
    pub fn crc33_17(&mut self) -> CRC33_17_W<SPI_CR1rs> {
        CRC33_17_W::new(self, 13)
    }
    #[doc = "Bit 14 - CRC calculation initialization pattern control for receiver"]
    #[inline(always)]
    #[must_use]
    pub fn rcrcini(&mut self) -> RCRCINI_W<SPI_CR1rs> {
        RCRCINI_W::new(self, 14)
    }
    #[doc = "Bit 15 - CRC calculation initialization pattern control for transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn tcrcini(&mut self) -> TCRCINI_W<SPI_CR1rs> {
        TCRCINI_W::new(self, 15)
    }
    #[doc = "Bit 16 - locking the AF configuration of associated IOs This bit is set by software and cleared by hardware whenever the SPE bit is changed from 1 to 0. When this bit is set, SPI_CFG2 register content cannot be modified. This bit can be set when SPI is disabled only else it is write protected. It is cleared and cannot be set when MODF bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn iolock(&mut self) -> IOLOCK_W<SPI_CR1rs> {
        IOLOCK_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_CR1rs;
impl crate::RegisterSpec for SPI_CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_cr1::R`](R) reader structure"]
impl crate::Readable for SPI_CR1rs {}
#[doc = "`write(|w| ..)` method takes [`spi_cr1::W`](W) writer structure"]
impl crate::Writable for SPI_CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_CR1 to value 0"]
impl crate::Resettable for SPI_CR1rs {
    const RESET_VALUE: u32 = 0;
}
