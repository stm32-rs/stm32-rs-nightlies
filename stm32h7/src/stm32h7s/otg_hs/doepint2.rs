///Register `DOEPINT2` reader
pub type R = crate::R<DOEPINT2rs>;
///Register `DOEPINT2` writer
pub type W = crate::W<DOEPINT2rs>;
///Field `XFRC` reader - Transfer completed interrupt This field indicates that the programmed transfer is complete on the AHB as well as on the USB, for this endpoint.
pub type XFRC_R = crate::BitReader;
///Field `XFRC` writer - Transfer completed interrupt This field indicates that the programmed transfer is complete on the AHB as well as on the USB, for this endpoint.
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDISD` reader - Endpoint disabled interrupt This bit indicates that the endpoint is disabled per the applications request.
pub type EPDISD_R = crate::BitReader;
///Field `EPDISD` writer - Endpoint disabled interrupt This bit indicates that the endpoint is disabled per the applications request.
pub type EPDISD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBERR` reader - AHB error This is generated only in internal DMA mode when there is an AHB error during an AHB read/write. The application can read the corresponding endpoint DMA address register to get the error address.
pub type AHBERR_R = crate::BitReader;
///Field `AHBERR` writer - AHB error This is generated only in internal DMA mode when there is an AHB error during an AHB read/write. The application can read the corresponding endpoint DMA address register to get the error address.
pub type AHBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STUP` reader - SETUP phase done Applies to control OUT endpoint only.Indicates that the SETUP phase for the control endpoint is complete and no more back-to-back SETUP packets were received for the current control transfer. On this interrupt, the application can decode the received SETUP data packet.
pub type STUP_R = crate::BitReader;
///Field `STUP` writer - SETUP phase done Applies to control OUT endpoint only.Indicates that the SETUP phase for the control endpoint is complete and no more back-to-back SETUP packets were received for the current control transfer. On this interrupt, the application can decode the received SETUP data packet.
pub type STUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTEPDIS` reader - OUT token received when endpoint disabled Applies only to control OUT endpoints. Indicates that an OUT token was received when the endpoint was not yet enabled. This interrupt is asserted on the endpoint for which the OUT token was received.
pub type OTEPDIS_R = crate::BitReader;
///Field `OTEPDIS` writer - OUT token received when endpoint disabled Applies only to control OUT endpoints. Indicates that an OUT token was received when the endpoint was not yet enabled. This interrupt is asserted on the endpoint for which the OUT token was received.
pub type OTEPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STSPHSRX` reader - Status phase received for control write This interrupt is valid only for control OUT endpoints. This interrupt is generated only after OTG_HS has transferred all the data that the host has sent during the data phase of a control write transfer, to the system memory buffer. The interrupt indicates to the application that the host has switched from data phase to the status phase of a control write transfer. The application can use this interrupt to ACK or STALL the status phase, after it has decoded the data phase.
pub type STSPHSRX_R = crate::BitReader;
///Field `STSPHSRX` writer - Status phase received for control write This interrupt is valid only for control OUT endpoints. This interrupt is generated only after OTG_HS has transferred all the data that the host has sent during the data phase of a control write transfer, to the system memory buffer. The interrupt indicates to the application that the host has switched from data phase to the status phase of a control write transfer. The application can use this interrupt to ACK or STALL the status phase, after it has decoded the data phase.
pub type STSPHSRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B2BSTUP` reader - Back-to-back SETUP packets received Applies to control OUT endpoint only. This bit indicates that the core has received more than three back-to-back SETUP packets for this particular endpoint.
pub type B2BSTUP_R = crate::BitReader;
///Field `B2BSTUP` writer - Back-to-back SETUP packets received Applies to control OUT endpoint only. This bit indicates that the core has received more than three back-to-back SETUP packets for this particular endpoint.
pub type B2BSTUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTPKTERR` reader - OUT packet error This interrupt is asserted when the core detects an overflow or a CRC error for an OUT packet. This interrupt is valid only when thresholding is enabled.
pub type OUTPKTERR_R = crate::BitReader;
///Field `OUTPKTERR` writer - OUT packet error This interrupt is asserted when the core detects an overflow or a CRC error for an OUT packet. This interrupt is valid only when thresholding is enabled.
pub type OUTPKTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BERR` reader - Babble error interrupt The core generates this interrupt when babble is received for the endpoint.
pub type BERR_R = crate::BitReader;
///Field `BERR` writer - Babble error interrupt The core generates this interrupt when babble is received for the endpoint.
pub type BERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NAK` reader - NAK input The core generates this interrupt when a NAK is transmitted or received by the device. In case of isochronous IN endpoints the interrupt gets generated when a zero length packet is transmitted due to unavailability of data in the Tx FIFO.
pub type NAK_R = crate::BitReader;
///Field `NAK` writer - NAK input The core generates this interrupt when a NAK is transmitted or received by the device. In case of isochronous IN endpoints the interrupt gets generated when a zero length packet is transmitted due to unavailability of data in the Tx FIFO.
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NYET` reader - NYET interrupt This interrupt is generated when a NYET response is transmitted for a non isochronous OUT endpoint.
pub type NYET_R = crate::BitReader;
///Field `NYET` writer - NYET interrupt This interrupt is generated when a NYET response is transmitted for a non isochronous OUT endpoint.
pub type NYET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STPKTRX` reader - Setup packet received Applicable for control OUT endpoints in only in the Buffer DMA Mode. Set by the OTG_HS, this bit indicates that this buffer holds 8 bytes of setup data. There is only one setup packet per buffer. On receiving a setup packet, the OTG_HS closes the buffer and disables the corresponding endpoint after SETUP_COMPLETE status is seen in the Rx FIFO. OTG_HS puts a SETUP_COMPLETE status into the Rx FIFO when it sees the first IN or OUT token after the SETUP packet for that particular endpoint. The application must then re-enable the endpoint to receive any OUT data for the control transfer and reprogram the buffer start address. Because of the above behavior, OTG_HS can receive any number of back to back setup packets and one buffer for every setup packet is used.
pub type STPKTRX_R = crate::BitReader;
///Field `STPKTRX` writer - Setup packet received Applicable for control OUT endpoints in only in the Buffer DMA Mode. Set by the OTG_HS, this bit indicates that this buffer holds 8 bytes of setup data. There is only one setup packet per buffer. On receiving a setup packet, the OTG_HS closes the buffer and disables the corresponding endpoint after SETUP_COMPLETE status is seen in the Rx FIFO. OTG_HS puts a SETUP_COMPLETE status into the Rx FIFO when it sees the first IN or OUT token after the SETUP packet for that particular endpoint. The application must then re-enable the endpoint to receive any OUT data for the control transfer and reprogram the buffer start address. Because of the above behavior, OTG_HS can receive any number of back to back setup packets and one buffer for every setup packet is used.
pub type STPKTRX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Transfer completed interrupt This field indicates that the programmed transfer is complete on the AHB as well as on the USB, for this endpoint.
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Endpoint disabled interrupt This bit indicates that the endpoint is disabled per the applications request.
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AHB error This is generated only in internal DMA mode when there is an AHB error during an AHB read/write. The application can read the corresponding endpoint DMA address register to get the error address.
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SETUP phase done Applies to control OUT endpoint only.Indicates that the SETUP phase for the control endpoint is complete and no more back-to-back SETUP packets were received for the current control transfer. On this interrupt, the application can decode the received SETUP data packet.
    #[inline(always)]
    pub fn stup(&self) -> STUP_R {
        STUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OUT token received when endpoint disabled Applies only to control OUT endpoints. Indicates that an OUT token was received when the endpoint was not yet enabled. This interrupt is asserted on the endpoint for which the OUT token was received.
    #[inline(always)]
    pub fn otepdis(&self) -> OTEPDIS_R {
        OTEPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Status phase received for control write This interrupt is valid only for control OUT endpoints. This interrupt is generated only after OTG_HS has transferred all the data that the host has sent during the data phase of a control write transfer, to the system memory buffer. The interrupt indicates to the application that the host has switched from data phase to the status phase of a control write transfer. The application can use this interrupt to ACK or STALL the status phase, after it has decoded the data phase.
    #[inline(always)]
    pub fn stsphsrx(&self) -> STSPHSRX_R {
        STSPHSRX_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Back-to-back SETUP packets received Applies to control OUT endpoint only. This bit indicates that the core has received more than three back-to-back SETUP packets for this particular endpoint.
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - OUT packet error This interrupt is asserted when the core detects an overflow or a CRC error for an OUT packet. This interrupt is valid only when thresholding is enabled.
    #[inline(always)]
    pub fn outpkterr(&self) -> OUTPKTERR_R {
        OUTPKTERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Babble error interrupt The core generates this interrupt when babble is received for the endpoint.
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - NAK input The core generates this interrupt when a NAK is transmitted or received by the device. In case of isochronous IN endpoints the interrupt gets generated when a zero length packet is transmitted due to unavailability of data in the Tx FIFO.
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - NYET interrupt This interrupt is generated when a NYET response is transmitted for a non isochronous OUT endpoint.
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Setup packet received Applicable for control OUT endpoints in only in the Buffer DMA Mode. Set by the OTG_HS, this bit indicates that this buffer holds 8 bytes of setup data. There is only one setup packet per buffer. On receiving a setup packet, the OTG_HS closes the buffer and disables the corresponding endpoint after SETUP_COMPLETE status is seen in the Rx FIFO. OTG_HS puts a SETUP_COMPLETE status into the Rx FIFO when it sees the first IN or OUT token after the SETUP packet for that particular endpoint. The application must then re-enable the endpoint to receive any OUT data for the control transfer and reprogram the buffer start address. Because of the above behavior, OTG_HS can receive any number of back to back setup packets and one buffer for every setup packet is used.
    #[inline(always)]
    pub fn stpktrx(&self) -> STPKTRX_R {
        STPKTRX_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPINT2")
            .field("xfrc", &self.xfrc())
            .field("epdisd", &self.epdisd())
            .field("ahberr", &self.ahberr())
            .field("stup", &self.stup())
            .field("otepdis", &self.otepdis())
            .field("stsphsrx", &self.stsphsrx())
            .field("b2bstup", &self.b2bstup())
            .field("outpkterr", &self.outpkterr())
            .field("berr", &self.berr())
            .field("nak", &self.nak())
            .field("nyet", &self.nyet())
            .field("stpktrx", &self.stpktrx())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transfer completed interrupt This field indicates that the programmed transfer is complete on the AHB as well as on the USB, for this endpoint.
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<'_, DOEPINT2rs> {
        XFRC_W::new(self, 0)
    }
    ///Bit 1 - Endpoint disabled interrupt This bit indicates that the endpoint is disabled per the applications request.
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W<'_, DOEPINT2rs> {
        EPDISD_W::new(self, 1)
    }
    ///Bit 2 - AHB error This is generated only in internal DMA mode when there is an AHB error during an AHB read/write. The application can read the corresponding endpoint DMA address register to get the error address.
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W<'_, DOEPINT2rs> {
        AHBERR_W::new(self, 2)
    }
    ///Bit 3 - SETUP phase done Applies to control OUT endpoint only.Indicates that the SETUP phase for the control endpoint is complete and no more back-to-back SETUP packets were received for the current control transfer. On this interrupt, the application can decode the received SETUP data packet.
    #[inline(always)]
    pub fn stup(&mut self) -> STUP_W<'_, DOEPINT2rs> {
        STUP_W::new(self, 3)
    }
    ///Bit 4 - OUT token received when endpoint disabled Applies only to control OUT endpoints. Indicates that an OUT token was received when the endpoint was not yet enabled. This interrupt is asserted on the endpoint for which the OUT token was received.
    #[inline(always)]
    pub fn otepdis(&mut self) -> OTEPDIS_W<'_, DOEPINT2rs> {
        OTEPDIS_W::new(self, 4)
    }
    ///Bit 5 - Status phase received for control write This interrupt is valid only for control OUT endpoints. This interrupt is generated only after OTG_HS has transferred all the data that the host has sent during the data phase of a control write transfer, to the system memory buffer. The interrupt indicates to the application that the host has switched from data phase to the status phase of a control write transfer. The application can use this interrupt to ACK or STALL the status phase, after it has decoded the data phase.
    #[inline(always)]
    pub fn stsphsrx(&mut self) -> STSPHSRX_W<'_, DOEPINT2rs> {
        STSPHSRX_W::new(self, 5)
    }
    ///Bit 6 - Back-to-back SETUP packets received Applies to control OUT endpoint only. This bit indicates that the core has received more than three back-to-back SETUP packets for this particular endpoint.
    #[inline(always)]
    pub fn b2bstup(&mut self) -> B2BSTUP_W<'_, DOEPINT2rs> {
        B2BSTUP_W::new(self, 6)
    }
    ///Bit 8 - OUT packet error This interrupt is asserted when the core detects an overflow or a CRC error for an OUT packet. This interrupt is valid only when thresholding is enabled.
    #[inline(always)]
    pub fn outpkterr(&mut self) -> OUTPKTERR_W<'_, DOEPINT2rs> {
        OUTPKTERR_W::new(self, 8)
    }
    ///Bit 12 - Babble error interrupt The core generates this interrupt when babble is received for the endpoint.
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W<'_, DOEPINT2rs> {
        BERR_W::new(self, 12)
    }
    ///Bit 13 - NAK input The core generates this interrupt when a NAK is transmitted or received by the device. In case of isochronous IN endpoints the interrupt gets generated when a zero length packet is transmitted due to unavailability of data in the Tx FIFO.
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W<'_, DOEPINT2rs> {
        NAK_W::new(self, 13)
    }
    ///Bit 14 - NYET interrupt This interrupt is generated when a NYET response is transmitted for a non isochronous OUT endpoint.
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W<'_, DOEPINT2rs> {
        NYET_W::new(self, 14)
    }
    ///Bit 15 - Setup packet received Applicable for control OUT endpoints in only in the Buffer DMA Mode. Set by the OTG_HS, this bit indicates that this buffer holds 8 bytes of setup data. There is only one setup packet per buffer. On receiving a setup packet, the OTG_HS closes the buffer and disables the corresponding endpoint after SETUP_COMPLETE status is seen in the Rx FIFO. OTG_HS puts a SETUP_COMPLETE status into the Rx FIFO when it sees the first IN or OUT token after the SETUP packet for that particular endpoint. The application must then re-enable the endpoint to receive any OUT data for the control transfer and reprogram the buffer start address. Because of the above behavior, OTG_HS can receive any number of back to back setup packets and one buffer for every setup packet is used.
    #[inline(always)]
    pub fn stpktrx(&mut self) -> STPKTRX_W<'_, DOEPINT2rs> {
        STPKTRX_W::new(self, 15)
    }
}
/**OTG device OUT endpoint 2 interrupt register

You can [`read`](crate::Reg::read) this register and get [`doepint2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#OTG_HS:DOEPINT2)*/
pub struct DOEPINT2rs;
impl crate::RegisterSpec for DOEPINT2rs {
    type Ux = u32;
}
///`read()` method returns [`doepint2::R`](R) reader structure
impl crate::Readable for DOEPINT2rs {}
///`write(|w| ..)` method takes [`doepint2::W`](W) writer structure
impl crate::Writable for DOEPINT2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOEPINT2 to value 0x80
impl crate::Resettable for DOEPINT2rs {
    const RESET_VALUE: u32 = 0x80;
}
