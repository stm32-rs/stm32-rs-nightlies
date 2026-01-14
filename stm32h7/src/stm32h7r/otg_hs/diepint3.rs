///Register `DIEPINT3` reader
pub type R = crate::R<DIEPINT3rs>;
///Register `DIEPINT3` writer
pub type W = crate::W<DIEPINT3rs>;
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
///Field `TOC` reader - Timeout condition Indicates that the core has detected a timeout condition on the USB for the last IN token on this endpoint.
pub type TOC_R = crate::BitReader;
///Field `TOC` writer - Timeout condition Indicates that the core has detected a timeout condition on the USB for the last IN token on this endpoint.
pub type TOC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITTXFE` reader - IN token received when Tx FIFO is empty Indicates that an IN token was received when the associated Tx FIFO (periodic/non-periodic) was empty. This interrupt is asserted on the endpoint for which the IN token was received.
pub type ITTXFE_R = crate::BitReader;
///Field `ITTXFE` writer - IN token received when Tx FIFO is empty Indicates that an IN token was received when the associated Tx FIFO (periodic/non-periodic) was empty. This interrupt is asserted on the endpoint for which the IN token was received.
pub type ITTXFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INEPNM` reader - IN token received with EP mismatch Indicates that the data in the top of the non-periodic TxFIFO belongs to an endpoint other than the one for which the IN token was received. This interrupt is asserted on the endpoint for which the IN token was received.
pub type INEPNM_R = crate::BitReader;
///Field `INEPNM` writer - IN token received with EP mismatch Indicates that the data in the top of the non-periodic TxFIFO belongs to an endpoint other than the one for which the IN token was received. This interrupt is asserted on the endpoint for which the IN token was received.
pub type INEPNM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INEPNE` reader - IN endpoint NAK effective This bit can be cleared when the application clears the IN endpoint NAK by writing to the CNAK bit in OTG_DIEPCTLx. This interrupt indicates that the core has sampled the NAK bit set (either by the application or by the core). The interrupt indicates that the IN endpoint NAK bit set by the application has taken effect in the core. This interrupt does not guarantee that a NAK handshake is sent on the USB. A STALL bit takes priority over a NAK bit.
pub type INEPNE_R = crate::BitReader;
///Field `INEPNE` writer - IN endpoint NAK effective This bit can be cleared when the application clears the IN endpoint NAK by writing to the CNAK bit in OTG_DIEPCTLx. This interrupt indicates that the core has sampled the NAK bit set (either by the application or by the core). The interrupt indicates that the IN endpoint NAK bit set by the application has taken effect in the core. This interrupt does not guarantee that a NAK handshake is sent on the USB. A STALL bit takes priority over a NAK bit.
pub type INEPNE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFE` reader - Transmit FIFO empty This interrupt is asserted when the Tx FIFO for this endpoint is either half or completely empty. The half or completely empty status is determined by the Tx FIFO Empty Level bit in the OTG_GAHBCFG register (TXFELVL bit in OTG_GAHBCFG).
pub type TXFE_R = crate::BitReader;
///Field `TXFIFOUDRN` reader - Transmit Fifo Underrun (TxfifoUndrn) The core generates this interrupt when it detects a transmit FIFO underrun condition for this endpoint. Dependency: This interrupt is valid only when Thresholding is enabled
pub type TXFIFOUDRN_R = crate::BitReader;
///Field `TXFIFOUDRN` writer - Transmit Fifo Underrun (TxfifoUndrn) The core generates this interrupt when it detects a transmit FIFO underrun condition for this endpoint. Dependency: This interrupt is valid only when Thresholding is enabled
pub type TXFIFOUDRN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKTDRPSTS` reader - Packet dropped status This bit indicates to the application that an ISOC OUT packet has been dropped. This bit does not have an associated mask bit and does not generate an interrupt.
pub type PKTDRPSTS_R = crate::BitReader;
///Field `PKTDRPSTS` writer - Packet dropped status This bit indicates to the application that an ISOC OUT packet has been dropped. This bit does not have an associated mask bit and does not generate an interrupt.
pub type PKTDRPSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NAK` reader - NAK input The core generates this interrupt when a NAK is transmitted or received by the device. In case of isochronous IN endpoints the interrupt gets generated when a zero length packet is transmitted due to unavailability of data in the Tx FIFO.
pub type NAK_R = crate::BitReader;
///Field `NAK` writer - NAK input The core generates this interrupt when a NAK is transmitted or received by the device. In case of isochronous IN endpoints the interrupt gets generated when a zero length packet is transmitted due to unavailability of data in the Tx FIFO.
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 3 - Timeout condition Indicates that the core has detected a timeout condition on the USB for the last IN token on this endpoint.
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IN token received when Tx FIFO is empty Indicates that an IN token was received when the associated Tx FIFO (periodic/non-periodic) was empty. This interrupt is asserted on the endpoint for which the IN token was received.
    #[inline(always)]
    pub fn ittxfe(&self) -> ITTXFE_R {
        ITTXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IN token received with EP mismatch Indicates that the data in the top of the non-periodic TxFIFO belongs to an endpoint other than the one for which the IN token was received. This interrupt is asserted on the endpoint for which the IN token was received.
    #[inline(always)]
    pub fn inepnm(&self) -> INEPNM_R {
        INEPNM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IN endpoint NAK effective This bit can be cleared when the application clears the IN endpoint NAK by writing to the CNAK bit in OTG_DIEPCTLx. This interrupt indicates that the core has sampled the NAK bit set (either by the application or by the core). The interrupt indicates that the IN endpoint NAK bit set by the application has taken effect in the core. This interrupt does not guarantee that a NAK handshake is sent on the USB. A STALL bit takes priority over a NAK bit.
    #[inline(always)]
    pub fn inepne(&self) -> INEPNE_R {
        INEPNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit FIFO empty This interrupt is asserted when the Tx FIFO for this endpoint is either half or completely empty. The half or completely empty status is determined by the Tx FIFO Empty Level bit in the OTG_GAHBCFG register (TXFELVL bit in OTG_GAHBCFG).
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Transmit Fifo Underrun (TxfifoUndrn) The core generates this interrupt when it detects a transmit FIFO underrun condition for this endpoint. Dependency: This interrupt is valid only when Thresholding is enabled
    #[inline(always)]
    pub fn txfifoudrn(&self) -> TXFIFOUDRN_R {
        TXFIFOUDRN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Packet dropped status This bit indicates to the application that an ISOC OUT packet has been dropped. This bit does not have an associated mask bit and does not generate an interrupt.
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - NAK input The core generates this interrupt when a NAK is transmitted or received by the device. In case of isochronous IN endpoints the interrupt gets generated when a zero length packet is transmitted due to unavailability of data in the Tx FIFO.
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPINT3")
            .field("xfrc", &self.xfrc())
            .field("epdisd", &self.epdisd())
            .field("ahberr", &self.ahberr())
            .field("toc", &self.toc())
            .field("ittxfe", &self.ittxfe())
            .field("inepnm", &self.inepnm())
            .field("inepne", &self.inepne())
            .field("txfe", &self.txfe())
            .field("txfifoudrn", &self.txfifoudrn())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("nak", &self.nak())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transfer completed interrupt This field indicates that the programmed transfer is complete on the AHB as well as on the USB, for this endpoint.
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<'_, DIEPINT3rs> {
        XFRC_W::new(self, 0)
    }
    ///Bit 1 - Endpoint disabled interrupt This bit indicates that the endpoint is disabled per the applications request.
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W<'_, DIEPINT3rs> {
        EPDISD_W::new(self, 1)
    }
    ///Bit 2 - AHB error This is generated only in internal DMA mode when there is an AHB error during an AHB read/write. The application can read the corresponding endpoint DMA address register to get the error address.
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W<'_, DIEPINT3rs> {
        AHBERR_W::new(self, 2)
    }
    ///Bit 3 - Timeout condition Indicates that the core has detected a timeout condition on the USB for the last IN token on this endpoint.
    #[inline(always)]
    pub fn toc(&mut self) -> TOC_W<'_, DIEPINT3rs> {
        TOC_W::new(self, 3)
    }
    ///Bit 4 - IN token received when Tx FIFO is empty Indicates that an IN token was received when the associated Tx FIFO (periodic/non-periodic) was empty. This interrupt is asserted on the endpoint for which the IN token was received.
    #[inline(always)]
    pub fn ittxfe(&mut self) -> ITTXFE_W<'_, DIEPINT3rs> {
        ITTXFE_W::new(self, 4)
    }
    ///Bit 5 - IN token received with EP mismatch Indicates that the data in the top of the non-periodic TxFIFO belongs to an endpoint other than the one for which the IN token was received. This interrupt is asserted on the endpoint for which the IN token was received.
    #[inline(always)]
    pub fn inepnm(&mut self) -> INEPNM_W<'_, DIEPINT3rs> {
        INEPNM_W::new(self, 5)
    }
    ///Bit 6 - IN endpoint NAK effective This bit can be cleared when the application clears the IN endpoint NAK by writing to the CNAK bit in OTG_DIEPCTLx. This interrupt indicates that the core has sampled the NAK bit set (either by the application or by the core). The interrupt indicates that the IN endpoint NAK bit set by the application has taken effect in the core. This interrupt does not guarantee that a NAK handshake is sent on the USB. A STALL bit takes priority over a NAK bit.
    #[inline(always)]
    pub fn inepne(&mut self) -> INEPNE_W<'_, DIEPINT3rs> {
        INEPNE_W::new(self, 6)
    }
    ///Bit 8 - Transmit Fifo Underrun (TxfifoUndrn) The core generates this interrupt when it detects a transmit FIFO underrun condition for this endpoint. Dependency: This interrupt is valid only when Thresholding is enabled
    #[inline(always)]
    pub fn txfifoudrn(&mut self) -> TXFIFOUDRN_W<'_, DIEPINT3rs> {
        TXFIFOUDRN_W::new(self, 8)
    }
    ///Bit 11 - Packet dropped status This bit indicates to the application that an ISOC OUT packet has been dropped. This bit does not have an associated mask bit and does not generate an interrupt.
    #[inline(always)]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W<'_, DIEPINT3rs> {
        PKTDRPSTS_W::new(self, 11)
    }
    ///Bit 13 - NAK input The core generates this interrupt when a NAK is transmitted or received by the device. In case of isochronous IN endpoints the interrupt gets generated when a zero length packet is transmitted due to unavailability of data in the Tx FIFO.
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W<'_, DIEPINT3rs> {
        NAK_W::new(self, 13)
    }
}
/**OTG device IN endpoint 3 interrupt register

You can [`read`](crate::Reg::read) this register and get [`diepint3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#OTG_HS:DIEPINT3)*/
pub struct DIEPINT3rs;
impl crate::RegisterSpec for DIEPINT3rs {
    type Ux = u32;
}
///`read()` method returns [`diepint3::R`](R) reader structure
impl crate::Readable for DIEPINT3rs {}
///`write(|w| ..)` method takes [`diepint3::W`](W) writer structure
impl crate::Writable for DIEPINT3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPINT3 to value 0x80
impl crate::Resettable for DIEPINT3rs {
    const RESET_VALUE: u32 = 0x80;
}
