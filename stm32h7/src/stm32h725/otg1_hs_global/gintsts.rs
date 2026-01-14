///Register `GINTSTS` reader
pub type R = crate::R<GINTSTSrs>;
///Register `GINTSTS` writer
pub type W = crate::W<GINTSTSrs>;
///Field `CMOD` reader - Current mode of operation Indicates the current mode. Note: Accessible in both host and device modes.
pub type CMOD_R = crate::BitReader;
///Field `MMIS` reader - Mode mismatch interrupt The core sets this bit when the application is trying to access: A host mode register, when the core is operating in device mode A device mode register, when the core is operating in host mode The register access is completed on the AHB with an OKAY response, but is ignored by the core internally and does not affect the operation of the core. Note: Accessible in both host and device modes.
pub type MMIS_R = crate::BitReader;
///Field `MMIS` writer - Mode mismatch interrupt The core sets this bit when the application is trying to access: A host mode register, when the core is operating in device mode A device mode register, when the core is operating in host mode The register access is completed on the AHB with an OKAY response, but is ignored by the core internally and does not affect the operation of the core. Note: Accessible in both host and device modes.
pub type MMIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGINT` reader - OTG interrupt The core sets this bit to indicate an OTG protocol event. The application must read the OTG interrupt status (OTG_GOTGINT) register to determine the exact event that caused this interrupt. The application must clear the appropriate status bit in the OTG_GOTGINT register to clear this bit. Note: Accessible in both host and device modes.
pub type OTGINT_R = crate::BitReader;
///Field `SOF` reader - Start of frame In host mode, the core sets this bit to indicate that an SOF (FS), or Keep-Alive (LS) is transmitted on the USB. The application must write a 1 to this bit to clear the interrupt. In device mode, in the core sets this bit to indicate that an SOF token has been received on the USB. The application can read the OTG_DSTS register to get the current frame number. This interrupt is seen only when the core is operating in FS. Note: This register may return '1' if read immediately after power on reset. If the register bit reads '1' immediately after power on reset it does not indicate that an SOF has been sent (in case of host mode) or SOF has been received (in case of device mode). The read value of this interrupt is valid only after a valid connection between host and device is established. If the bit is set after power on reset the application can clear the bit. Note: Accessible in both host and device modes.
pub type SOF_R = crate::BitReader;
///Field `SOF` writer - Start of frame In host mode, the core sets this bit to indicate that an SOF (FS), or Keep-Alive (LS) is transmitted on the USB. The application must write a 1 to this bit to clear the interrupt. In device mode, in the core sets this bit to indicate that an SOF token has been received on the USB. The application can read the OTG_DSTS register to get the current frame number. This interrupt is seen only when the core is operating in FS. Note: This register may return '1' if read immediately after power on reset. If the register bit reads '1' immediately after power on reset it does not indicate that an SOF has been sent (in case of host mode) or SOF has been received (in case of device mode). The read value of this interrupt is valid only after a valid connection between host and device is established. If the bit is set after power on reset the application can clear the bit. Note: Accessible in both host and device modes.
pub type SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFLVL` reader - Rx FIFO non-empty Indicates that there is at least one packet pending to be read from the Rx FIFO. Note: Accessible in both host and device modes.
pub type RXFLVL_R = crate::BitReader;
///Field `NPTXFE` reader - Non-periodic Tx FIFO empty This interrupt is asserted when the non-periodic Tx FIFO is either half or completely empty, and there is space for at least one entry to be written to the non-periodic transmit request queue. The half or completely empty status is determined by the non-periodic Tx FIFO empty level bit in the OTG_GAHBCFG register (TXFELVL bit in OTG_GAHBCFG). Note: Accessible in host mode only.
pub type NPTXFE_R = crate::BitReader;
///Field `GINAKEFF` reader - Global IN non-periodic NAK effective Indicates that the Set global non-periodic IN NAK bit in the OTG_DCTL register (SGINAK bit in OTG_DCTL), set by the application, has taken effect in the core. That is, the core has sampled the Global IN NAK bit set by the application. This bit can be cleared by clearing the Clear global non-periodic IN NAK bit in the OTG_DCTL register (CGINAK bit in OTG_DCTL). This interrupt does not necessarily mean that a NAK handshake is sent out on the USB. The STALL bit takes precedence over the NAK bit. Note: Only accessible in device mode.
pub type GINAKEFF_R = crate::BitReader;
///Field `GONAKEFF` reader - Global OUT NAK effective Indicates that the Set global OUT NAK bit in the OTG_DCTL register (SGONAK bit in OTG_DCTL), set by the application, has taken effect in the core. This bit can be cleared by writing the Clear global OUT NAK bit in the OTG_DCTL register (CGONAK bit in OTG_DCTL). Note: Only accessible in device mode.
pub type GONAKEFF_R = crate::BitReader;
///Field `ESUSP` reader - Early suspend The core sets this bit to indicate that an Idle state has been detected on the USB for 3 ms. Note: Only accessible in device mode.
pub type ESUSP_R = crate::BitReader;
///Field `ESUSP` writer - Early suspend The core sets this bit to indicate that an Idle state has been detected on the USB for 3 ms. Note: Only accessible in device mode.
pub type ESUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBSUSP` reader - USB suspend The core sets this bit to indicate that a suspend was detected on the USB. The core enters the suspended state when there is no activity on the data lines for an extended period of time. Note: Only accessible in device mode.
pub type USBSUSP_R = crate::BitReader;
///Field `USBSUSP` writer - USB suspend The core sets this bit to indicate that a suspend was detected on the USB. The core enters the suspended state when there is no activity on the data lines for an extended period of time. Note: Only accessible in device mode.
pub type USBSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBRST` reader - USB reset The core sets this bit to indicate that a reset is detected on the USB. Note: Only accessible in device mode.
pub type USBRST_R = crate::BitReader;
///Field `USBRST` writer - USB reset The core sets this bit to indicate that a reset is detected on the USB. Note: Only accessible in device mode.
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENUMDNE` reader - Enumeration done The core sets this bit to indicate that speed enumeration is complete. The application must read the OTG_DSTS register to obtain the enumerated speed. Note: Only accessible in device mode.
pub type ENUMDNE_R = crate::BitReader;
///Field `ENUMDNE` writer - Enumeration done The core sets this bit to indicate that speed enumeration is complete. The application must read the OTG_DSTS register to obtain the enumerated speed. Note: Only accessible in device mode.
pub type ENUMDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ISOODRP` reader - Isochronous OUT packet dropped interrupt The core sets this bit when it fails to write an isochronous OUT packet into the Rx FIFO because the Rx FIFO does not have enough space to accommodate a maximum size packet for the isochronous OUT endpoint. Note: Only accessible in device mode.
pub type ISOODRP_R = crate::BitReader;
///Field `ISOODRP` writer - Isochronous OUT packet dropped interrupt The core sets this bit when it fails to write an isochronous OUT packet into the Rx FIFO because the Rx FIFO does not have enough space to accommodate a maximum size packet for the isochronous OUT endpoint. Note: Only accessible in device mode.
pub type ISOODRP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOPF` reader - End of periodic frame interrupt Indicates that the period specified in the periodic frame interval field of the OTG_DCFG register (PFIVL bit in OTG_DCFG) has been reached in the current frame. Note: Only accessible in device mode.
pub type EOPF_R = crate::BitReader;
///Field `EOPF` writer - End of periodic frame interrupt Indicates that the period specified in the periodic frame interval field of the OTG_DCFG register (PFIVL bit in OTG_DCFG) has been reached in the current frame. Note: Only accessible in device mode.
pub type EOPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IEPINT` reader - IN endpoint interrupt The core sets this bit to indicate that an interrupt is pending on one of the IN endpoints of the core (in device mode). The application must read the OTG_DAINT register to determine the exact number of the IN endpoint on which the interrupt occurred, and then read the corresponding OTG_DIEPINTx register to determine the exact cause of the interrupt. The application must clear the appropriate status bit in the corresponding OTG_DIEPINTx register to clear this bit. Note: Only accessible in device mode.
pub type IEPINT_R = crate::BitReader;
///Field `OEPINT` reader - OUT endpoint interrupt The core sets this bit to indicate that an interrupt is pending on one of the OUT endpoints of the core (in device mode). The application must read the OTG_DAINT register to determine the exact number of the OUT endpoint on which the interrupt occurred, and then read the corresponding OTG_DOEPINTx register to determine the exact cause of the interrupt. The application must clear the appropriate status bit in the corresponding OTG_DOEPINTx register to clear this bit. Note: Only accessible in device mode.
pub type OEPINT_R = crate::BitReader;
///Field `IISOIXFR` reader - Incomplete isochronous IN transfer The core sets this interrupt to indicate that there is at least one isochronous IN endpoint on which the transfer is not completed in the current frame. This interrupt is asserted along with the End of periodic frame interrupt (EOPF) bit in this register. Note: Only accessible in device mode.
pub type IISOIXFR_R = crate::BitReader;
///Field `IISOIXFR` writer - Incomplete isochronous IN transfer The core sets this interrupt to indicate that there is at least one isochronous IN endpoint on which the transfer is not completed in the current frame. This interrupt is asserted along with the End of periodic frame interrupt (EOPF) bit in this register. Note: Only accessible in device mode.
pub type IISOIXFR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IPXFR_INCOMPISOOUT` reader -
pub type IPXFR_INCOMPISOOUT_R = crate::BitReader;
///Field `IPXFR_INCOMPISOOUT` writer -
pub type IPXFR_INCOMPISOOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATAFSUSP` reader - Data fetch suspended This interrupt is valid only in DMA mode. This interrupt indicates that the core has stopped fetching data for IN endpoints due to the unavailability of TxFIFO space or request queue space. This interrupt is used by the application for an endpoint mismatch algorithm. For example, after detecting an endpoint mismatch, the application: Sets a global nonperiodic IN NAK handshake Disables IN endpoints Flushes the FIFO Determines the token sequence from the IN token sequence learning queue Re-enables the endpoints Clears the global nonperiodic IN NAK handshake If the global nonperiodic IN NAK is cleared, the core has not yet fetched data for the IN endpoint, and the IN token is received: the core generates an IN token received when FIFO empty interrupt. The OTG then sends a NAK response to the host. To avoid this scenario, the application can check the FetSusp interrupt in OTG_GINTSTS, which ensures that the FIFO is full before clearing a global NAK handshake. Alternatively, the application can mask the IN token received when FIFO empty interrupt when clearing a global IN NAK handshake.
pub type DATAFSUSP_R = crate::BitReader;
///Field `DATAFSUSP` writer - Data fetch suspended This interrupt is valid only in DMA mode. This interrupt indicates that the core has stopped fetching data for IN endpoints due to the unavailability of TxFIFO space or request queue space. This interrupt is used by the application for an endpoint mismatch algorithm. For example, after detecting an endpoint mismatch, the application: Sets a global nonperiodic IN NAK handshake Disables IN endpoints Flushes the FIFO Determines the token sequence from the IN token sequence learning queue Re-enables the endpoints Clears the global nonperiodic IN NAK handshake If the global nonperiodic IN NAK is cleared, the core has not yet fetched data for the IN endpoint, and the IN token is received: the core generates an IN token received when FIFO empty interrupt. The OTG then sends a NAK response to the host. To avoid this scenario, the application can check the FetSusp interrupt in OTG_GINTSTS, which ensures that the FIFO is full before clearing a global NAK handshake. Alternatively, the application can mask the IN token received when FIFO empty interrupt when clearing a global IN NAK handshake.
pub type DATAFSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTDET` reader - Reset detected interrupt In device mode, this interrupt is asserted when a reset is detected on the USB in partial power-down mode when the device is in suspend. Note: Only accessible in device mode.
pub type RSTDET_R = crate::BitReader;
///Field `RSTDET` writer - Reset detected interrupt In device mode, this interrupt is asserted when a reset is detected on the USB in partial power-down mode when the device is in suspend. Note: Only accessible in device mode.
pub type RSTDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPRTINT` reader - Host port interrupt The core sets this bit to indicate a change in port status of one of the OTG_HS controller ports in host mode. The application must read the OTG_HPRT register to determine the exact event that caused this interrupt. The application must clear the appropriate status bit in the OTG_HPRT register to clear this bit. Note: Only accessible in host mode.
pub type HPRTINT_R = crate::BitReader;
///Field `HCINT` reader - Host channels interrupt The core sets this bit to indicate that an interrupt is pending on one of the channels of the core (in host mode). The application must read the OTG_HAINT register to determine the exact number of the channel on which the interrupt occurred, and then read the corresponding OTG_HCINTx register to determine the exact cause of the interrupt. The application must clear the appropriate status bit in the OTG_HCINTx register to clear this bit. Note: Only accessible in host mode.
pub type HCINT_R = crate::BitReader;
///Field `PTXFE` reader - Periodic Tx FIFO empty Asserted when the periodic transmit FIFO is either half or completely empty and there is space for at least one entry to be written in the periodic request queue. The half or completely empty status is determined by the periodic Tx FIFO empty level bit in the OTG_GAHBCFG register (PTXFELVL bit in OTG_GAHBCFG). Note: Only accessible in host mode.
pub type PTXFE_R = crate::BitReader;
///Field `LPMINT` reader - LPM interrupt In device mode, this interrupt is asserted when the device receives an LPM transaction and responds with a non-ERRORed response. In host mode, this interrupt is asserted when the device responds to an LPM transaction with a non-ERRORed response or when the host core has completed LPM transactions for the programmed number of times (RETRYCNT bit in OTG_GLPMCFG). This field is valid only if the LPMEN bit in OTG_GLPMCFG is set to 1.
pub type LPMINT_R = crate::BitReader;
///Field `LPMINT` writer - LPM interrupt In device mode, this interrupt is asserted when the device receives an LPM transaction and responds with a non-ERRORed response. In host mode, this interrupt is asserted when the device responds to an LPM transaction with a non-ERRORed response or when the host core has completed LPM transactions for the programmed number of times (RETRYCNT bit in OTG_GLPMCFG). This field is valid only if the LPMEN bit in OTG_GLPMCFG is set to 1.
pub type LPMINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIDSCHG` reader - Connector ID status change The core sets this bit when there is a change in connector ID status. Note: Accessible in both device and host modes.
pub type CIDSCHG_R = crate::BitReader;
///Field `CIDSCHG` writer - Connector ID status change The core sets this bit when there is a change in connector ID status. Note: Accessible in both device and host modes.
pub type CIDSCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISCINT` reader - Disconnect detected interrupt Asserted when a device disconnect is detected. Note: Only accessible in host mode.
pub type DISCINT_R = crate::BitReader;
///Field `DISCINT` writer - Disconnect detected interrupt Asserted when a device disconnect is detected. Note: Only accessible in host mode.
pub type DISCINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRQINT` reader - Session request/new session detected interrupt In host mode, this interrupt is asserted when a session request is detected from the device. In device mode, this interrupt is asserted when VBUS is in the valid range for a B-peripheral device. Accessible in both device and host modes.
pub type SRQINT_R = crate::BitReader;
///Field `SRQINT` writer - Session request/new session detected interrupt In host mode, this interrupt is asserted when a session request is detected from the device. In device mode, this interrupt is asserted when VBUS is in the valid range for a B-peripheral device. Accessible in both device and host modes.
pub type SRQINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPINT` reader - Resume/remote wakeup detected interrupt Wakeup interrupt during suspend(L2) or LPM(L1) state. During suspend(L2): In device mode, this interrupt is asserted when a resume is detected on the USB. In host mode, this interrupt is asserted when a remote wakeup is detected on the USB. During LPM(L1): This interrupt is asserted for either host initiated resume or device initiated remote wakeup on USB. Note: Accessible in both device and host modes.
pub type WKUPINT_R = crate::BitReader;
///Field `WKUPINT` writer - Resume/remote wakeup detected interrupt Wakeup interrupt during suspend(L2) or LPM(L1) state. During suspend(L2): In device mode, this interrupt is asserted when a resume is detected on the USB. In host mode, this interrupt is asserted when a remote wakeup is detected on the USB. During LPM(L1): This interrupt is asserted for either host initiated resume or device initiated remote wakeup on USB. Note: Accessible in both device and host modes.
pub type WKUPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Current mode of operation Indicates the current mode. Note: Accessible in both host and device modes.
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Mode mismatch interrupt The core sets this bit when the application is trying to access: A host mode register, when the core is operating in device mode A device mode register, when the core is operating in host mode The register access is completed on the AHB with an OKAY response, but is ignored by the core internally and does not affect the operation of the core. Note: Accessible in both host and device modes.
    #[inline(always)]
    pub fn mmis(&self) -> MMIS_R {
        MMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - OTG interrupt The core sets this bit to indicate an OTG protocol event. The application must read the OTG interrupt status (OTG_GOTGINT) register to determine the exact event that caused this interrupt. The application must clear the appropriate status bit in the OTG_GOTGINT register to clear this bit. Note: Accessible in both host and device modes.
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Start of frame In host mode, the core sets this bit to indicate that an SOF (FS), or Keep-Alive (LS) is transmitted on the USB. The application must write a 1 to this bit to clear the interrupt. In device mode, in the core sets this bit to indicate that an SOF token has been received on the USB. The application can read the OTG_DSTS register to get the current frame number. This interrupt is seen only when the core is operating in FS. Note: This register may return '1' if read immediately after power on reset. If the register bit reads '1' immediately after power on reset it does not indicate that an SOF has been sent (in case of host mode) or SOF has been received (in case of device mode). The read value of this interrupt is valid only after a valid connection between host and device is established. If the bit is set after power on reset the application can clear the bit. Note: Accessible in both host and device modes.
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rx FIFO non-empty Indicates that there is at least one packet pending to be read from the Rx FIFO. Note: Accessible in both host and device modes.
    #[inline(always)]
    pub fn rxflvl(&self) -> RXFLVL_R {
        RXFLVL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Non-periodic Tx FIFO empty This interrupt is asserted when the non-periodic Tx FIFO is either half or completely empty, and there is space for at least one entry to be written to the non-periodic transmit request queue. The half or completely empty status is determined by the non-periodic Tx FIFO empty level bit in the OTG_GAHBCFG register (TXFELVL bit in OTG_GAHBCFG). Note: Accessible in host mode only.
    #[inline(always)]
    pub fn nptxfe(&self) -> NPTXFE_R {
        NPTXFE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Global IN non-periodic NAK effective Indicates that the Set global non-periodic IN NAK bit in the OTG_DCTL register (SGINAK bit in OTG_DCTL), set by the application, has taken effect in the core. That is, the core has sampled the Global IN NAK bit set by the application. This bit can be cleared by clearing the Clear global non-periodic IN NAK bit in the OTG_DCTL register (CGINAK bit in OTG_DCTL). This interrupt does not necessarily mean that a NAK handshake is sent out on the USB. The STALL bit takes precedence over the NAK bit. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn ginakeff(&self) -> GINAKEFF_R {
        GINAKEFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Global OUT NAK effective Indicates that the Set global OUT NAK bit in the OTG_DCTL register (SGONAK bit in OTG_DCTL), set by the application, has taken effect in the core. This bit can be cleared by writing the Clear global OUT NAK bit in the OTG_DCTL register (CGONAK bit in OTG_DCTL). Note: Only accessible in device mode.
    #[inline(always)]
    pub fn gonakeff(&self) -> GONAKEFF_R {
        GONAKEFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 10 - Early suspend The core sets this bit to indicate that an Idle state has been detected on the USB for 3 ms. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn esusp(&self) -> ESUSP_R {
        ESUSP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - USB suspend The core sets this bit to indicate that a suspend was detected on the USB. The core enters the suspended state when there is no activity on the data lines for an extended period of time. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - USB reset The core sets this bit to indicate that a reset is detected on the USB. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Enumeration done The core sets this bit to indicate that speed enumeration is complete. The application must read the OTG_DSTS register to obtain the enumerated speed. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn enumdne(&self) -> ENUMDNE_R {
        ENUMDNE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Isochronous OUT packet dropped interrupt The core sets this bit when it fails to write an isochronous OUT packet into the Rx FIFO because the Rx FIFO does not have enough space to accommodate a maximum size packet for the isochronous OUT endpoint. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn isoodrp(&self) -> ISOODRP_R {
        ISOODRP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - End of periodic frame interrupt Indicates that the period specified in the periodic frame interval field of the OTG_DCFG register (PFIVL bit in OTG_DCFG) has been reached in the current frame. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 18 - IN endpoint interrupt The core sets this bit to indicate that an interrupt is pending on one of the IN endpoints of the core (in device mode). The application must read the OTG_DAINT register to determine the exact number of the IN endpoint on which the interrupt occurred, and then read the corresponding OTG_DIEPINTx register to determine the exact cause of the interrupt. The application must clear the appropriate status bit in the corresponding OTG_DIEPINTx register to clear this bit. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - OUT endpoint interrupt The core sets this bit to indicate that an interrupt is pending on one of the OUT endpoints of the core (in device mode). The application must read the OTG_DAINT register to determine the exact number of the OUT endpoint on which the interrupt occurred, and then read the corresponding OTG_DOEPINTx register to determine the exact cause of the interrupt. The application must clear the appropriate status bit in the corresponding OTG_DOEPINTx register to clear this bit. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Incomplete isochronous IN transfer The core sets this interrupt to indicate that there is at least one isochronous IN endpoint on which the transfer is not completed in the current frame. This interrupt is asserted along with the End of periodic frame interrupt (EOPF) bit in this register. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn iisoixfr(&self) -> IISOIXFR_R {
        IISOIXFR_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21
    #[inline(always)]
    pub fn ipxfr_incompisoout(&self) -> IPXFR_INCOMPISOOUT_R {
        IPXFR_INCOMPISOOUT_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Data fetch suspended This interrupt is valid only in DMA mode. This interrupt indicates that the core has stopped fetching data for IN endpoints due to the unavailability of TxFIFO space or request queue space. This interrupt is used by the application for an endpoint mismatch algorithm. For example, after detecting an endpoint mismatch, the application: Sets a global nonperiodic IN NAK handshake Disables IN endpoints Flushes the FIFO Determines the token sequence from the IN token sequence learning queue Re-enables the endpoints Clears the global nonperiodic IN NAK handshake If the global nonperiodic IN NAK is cleared, the core has not yet fetched data for the IN endpoint, and the IN token is received: the core generates an IN token received when FIFO empty interrupt. The OTG then sends a NAK response to the host. To avoid this scenario, the application can check the FetSusp interrupt in OTG_GINTSTS, which ensures that the FIFO is full before clearing a global NAK handshake. Alternatively, the application can mask the IN token received when FIFO empty interrupt when clearing a global IN NAK handshake.
    #[inline(always)]
    pub fn datafsusp(&self) -> DATAFSUSP_R {
        DATAFSUSP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Reset detected interrupt In device mode, this interrupt is asserted when a reset is detected on the USB in partial power-down mode when the device is in suspend. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn rstdet(&self) -> RSTDET_R {
        RSTDET_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Host port interrupt The core sets this bit to indicate a change in port status of one of the OTG_HS controller ports in host mode. The application must read the OTG_HPRT register to determine the exact event that caused this interrupt. The application must clear the appropriate status bit in the OTG_HPRT register to clear this bit. Note: Only accessible in host mode.
    #[inline(always)]
    pub fn hprtint(&self) -> HPRTINT_R {
        HPRTINT_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Host channels interrupt The core sets this bit to indicate that an interrupt is pending on one of the channels of the core (in host mode). The application must read the OTG_HAINT register to determine the exact number of the channel on which the interrupt occurred, and then read the corresponding OTG_HCINTx register to determine the exact cause of the interrupt. The application must clear the appropriate status bit in the OTG_HCINTx register to clear this bit. Note: Only accessible in host mode.
    #[inline(always)]
    pub fn hcint(&self) -> HCINT_R {
        HCINT_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Periodic Tx FIFO empty Asserted when the periodic transmit FIFO is either half or completely empty and there is space for at least one entry to be written in the periodic request queue. The half or completely empty status is determined by the periodic Tx FIFO empty level bit in the OTG_GAHBCFG register (PTXFELVL bit in OTG_GAHBCFG). Note: Only accessible in host mode.
    #[inline(always)]
    pub fn ptxfe(&self) -> PTXFE_R {
        PTXFE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - LPM interrupt In device mode, this interrupt is asserted when the device receives an LPM transaction and responds with a non-ERRORed response. In host mode, this interrupt is asserted when the device responds to an LPM transaction with a non-ERRORed response or when the host core has completed LPM transactions for the programmed number of times (RETRYCNT bit in OTG_GLPMCFG). This field is valid only if the LPMEN bit in OTG_GLPMCFG is set to 1.
    #[inline(always)]
    pub fn lpmint(&self) -> LPMINT_R {
        LPMINT_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Connector ID status change The core sets this bit when there is a change in connector ID status. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn cidschg(&self) -> CIDSCHG_R {
        CIDSCHG_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Disconnect detected interrupt Asserted when a device disconnect is detected. Note: Only accessible in host mode.
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Session request/new session detected interrupt In host mode, this interrupt is asserted when a session request is detected from the device. In device mode, this interrupt is asserted when VBUS is in the valid range for a B-peripheral device. Accessible in both device and host modes.
    #[inline(always)]
    pub fn srqint(&self) -> SRQINT_R {
        SRQINT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Resume/remote wakeup detected interrupt Wakeup interrupt during suspend(L2) or LPM(L1) state. During suspend(L2): In device mode, this interrupt is asserted when a resume is detected on the USB. In host mode, this interrupt is asserted when a remote wakeup is detected on the USB. During LPM(L1): This interrupt is asserted for either host initiated resume or device initiated remote wakeup on USB. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn wkupint(&self) -> WKUPINT_R {
        WKUPINT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GINTSTS")
            .field("cmod", &self.cmod())
            .field("mmis", &self.mmis())
            .field("otgint", &self.otgint())
            .field("sof", &self.sof())
            .field("rxflvl", &self.rxflvl())
            .field("nptxfe", &self.nptxfe())
            .field("ginakeff", &self.ginakeff())
            .field("gonakeff", &self.gonakeff())
            .field("esusp", &self.esusp())
            .field("usbsusp", &self.usbsusp())
            .field("usbrst", &self.usbrst())
            .field("enumdne", &self.enumdne())
            .field("isoodrp", &self.isoodrp())
            .field("eopf", &self.eopf())
            .field("iepint", &self.iepint())
            .field("oepint", &self.oepint())
            .field("iisoixfr", &self.iisoixfr())
            .field("ipxfr_incompisoout", &self.ipxfr_incompisoout())
            .field("datafsusp", &self.datafsusp())
            .field("rstdet", &self.rstdet())
            .field("hprtint", &self.hprtint())
            .field("hcint", &self.hcint())
            .field("ptxfe", &self.ptxfe())
            .field("lpmint", &self.lpmint())
            .field("cidschg", &self.cidschg())
            .field("discint", &self.discint())
            .field("srqint", &self.srqint())
            .field("wkupint", &self.wkupint())
            .finish()
    }
}
impl W {
    ///Bit 1 - Mode mismatch interrupt The core sets this bit when the application is trying to access: A host mode register, when the core is operating in device mode A device mode register, when the core is operating in host mode The register access is completed on the AHB with an OKAY response, but is ignored by the core internally and does not affect the operation of the core. Note: Accessible in both host and device modes.
    #[inline(always)]
    pub fn mmis(&mut self) -> MMIS_W<'_, GINTSTSrs> {
        MMIS_W::new(self, 1)
    }
    ///Bit 3 - Start of frame In host mode, the core sets this bit to indicate that an SOF (FS), or Keep-Alive (LS) is transmitted on the USB. The application must write a 1 to this bit to clear the interrupt. In device mode, in the core sets this bit to indicate that an SOF token has been received on the USB. The application can read the OTG_DSTS register to get the current frame number. This interrupt is seen only when the core is operating in FS. Note: This register may return '1' if read immediately after power on reset. If the register bit reads '1' immediately after power on reset it does not indicate that an SOF has been sent (in case of host mode) or SOF has been received (in case of device mode). The read value of this interrupt is valid only after a valid connection between host and device is established. If the bit is set after power on reset the application can clear the bit. Note: Accessible in both host and device modes.
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<'_, GINTSTSrs> {
        SOF_W::new(self, 3)
    }
    ///Bit 10 - Early suspend The core sets this bit to indicate that an Idle state has been detected on the USB for 3 ms. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn esusp(&mut self) -> ESUSP_W<'_, GINTSTSrs> {
        ESUSP_W::new(self, 10)
    }
    ///Bit 11 - USB suspend The core sets this bit to indicate that a suspend was detected on the USB. The core enters the suspended state when there is no activity on the data lines for an extended period of time. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn usbsusp(&mut self) -> USBSUSP_W<'_, GINTSTSrs> {
        USBSUSP_W::new(self, 11)
    }
    ///Bit 12 - USB reset The core sets this bit to indicate that a reset is detected on the USB. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<'_, GINTSTSrs> {
        USBRST_W::new(self, 12)
    }
    ///Bit 13 - Enumeration done The core sets this bit to indicate that speed enumeration is complete. The application must read the OTG_DSTS register to obtain the enumerated speed. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn enumdne(&mut self) -> ENUMDNE_W<'_, GINTSTSrs> {
        ENUMDNE_W::new(self, 13)
    }
    ///Bit 14 - Isochronous OUT packet dropped interrupt The core sets this bit when it fails to write an isochronous OUT packet into the Rx FIFO because the Rx FIFO does not have enough space to accommodate a maximum size packet for the isochronous OUT endpoint. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn isoodrp(&mut self) -> ISOODRP_W<'_, GINTSTSrs> {
        ISOODRP_W::new(self, 14)
    }
    ///Bit 15 - End of periodic frame interrupt Indicates that the period specified in the periodic frame interval field of the OTG_DCFG register (PFIVL bit in OTG_DCFG) has been reached in the current frame. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn eopf(&mut self) -> EOPF_W<'_, GINTSTSrs> {
        EOPF_W::new(self, 15)
    }
    ///Bit 20 - Incomplete isochronous IN transfer The core sets this interrupt to indicate that there is at least one isochronous IN endpoint on which the transfer is not completed in the current frame. This interrupt is asserted along with the End of periodic frame interrupt (EOPF) bit in this register. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn iisoixfr(&mut self) -> IISOIXFR_W<'_, GINTSTSrs> {
        IISOIXFR_W::new(self, 20)
    }
    ///Bit 21
    #[inline(always)]
    pub fn ipxfr_incompisoout(&mut self) -> IPXFR_INCOMPISOOUT_W<'_, GINTSTSrs> {
        IPXFR_INCOMPISOOUT_W::new(self, 21)
    }
    ///Bit 22 - Data fetch suspended This interrupt is valid only in DMA mode. This interrupt indicates that the core has stopped fetching data for IN endpoints due to the unavailability of TxFIFO space or request queue space. This interrupt is used by the application for an endpoint mismatch algorithm. For example, after detecting an endpoint mismatch, the application: Sets a global nonperiodic IN NAK handshake Disables IN endpoints Flushes the FIFO Determines the token sequence from the IN token sequence learning queue Re-enables the endpoints Clears the global nonperiodic IN NAK handshake If the global nonperiodic IN NAK is cleared, the core has not yet fetched data for the IN endpoint, and the IN token is received: the core generates an IN token received when FIFO empty interrupt. The OTG then sends a NAK response to the host. To avoid this scenario, the application can check the FetSusp interrupt in OTG_GINTSTS, which ensures that the FIFO is full before clearing a global NAK handshake. Alternatively, the application can mask the IN token received when FIFO empty interrupt when clearing a global IN NAK handshake.
    #[inline(always)]
    pub fn datafsusp(&mut self) -> DATAFSUSP_W<'_, GINTSTSrs> {
        DATAFSUSP_W::new(self, 22)
    }
    ///Bit 23 - Reset detected interrupt In device mode, this interrupt is asserted when a reset is detected on the USB in partial power-down mode when the device is in suspend. Note: Only accessible in device mode.
    #[inline(always)]
    pub fn rstdet(&mut self) -> RSTDET_W<'_, GINTSTSrs> {
        RSTDET_W::new(self, 23)
    }
    ///Bit 27 - LPM interrupt In device mode, this interrupt is asserted when the device receives an LPM transaction and responds with a non-ERRORed response. In host mode, this interrupt is asserted when the device responds to an LPM transaction with a non-ERRORed response or when the host core has completed LPM transactions for the programmed number of times (RETRYCNT bit in OTG_GLPMCFG). This field is valid only if the LPMEN bit in OTG_GLPMCFG is set to 1.
    #[inline(always)]
    pub fn lpmint(&mut self) -> LPMINT_W<'_, GINTSTSrs> {
        LPMINT_W::new(self, 27)
    }
    ///Bit 28 - Connector ID status change The core sets this bit when there is a change in connector ID status. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn cidschg(&mut self) -> CIDSCHG_W<'_, GINTSTSrs> {
        CIDSCHG_W::new(self, 28)
    }
    ///Bit 29 - Disconnect detected interrupt Asserted when a device disconnect is detected. Note: Only accessible in host mode.
    #[inline(always)]
    pub fn discint(&mut self) -> DISCINT_W<'_, GINTSTSrs> {
        DISCINT_W::new(self, 29)
    }
    ///Bit 30 - Session request/new session detected interrupt In host mode, this interrupt is asserted when a session request is detected from the device. In device mode, this interrupt is asserted when VBUS is in the valid range for a B-peripheral device. Accessible in both device and host modes.
    #[inline(always)]
    pub fn srqint(&mut self) -> SRQINT_W<'_, GINTSTSrs> {
        SRQINT_W::new(self, 30)
    }
    ///Bit 31 - Resume/remote wakeup detected interrupt Wakeup interrupt during suspend(L2) or LPM(L1) state. During suspend(L2): In device mode, this interrupt is asserted when a resume is detected on the USB. In host mode, this interrupt is asserted when a remote wakeup is detected on the USB. During LPM(L1): This interrupt is asserted for either host initiated resume or device initiated remote wakeup on USB. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn wkupint(&mut self) -> WKUPINT_W<'_, GINTSTSrs> {
        WKUPINT_W::new(self, 31)
    }
}
/**OTG_HS core interrupt register

You can [`read`](crate::Reg::read) this register and get [`gintsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#OTG1_HS_GLOBAL:GINTSTS)*/
pub struct GINTSTSrs;
impl crate::RegisterSpec for GINTSTSrs {
    type Ux = u32;
}
///`read()` method returns [`gintsts::R`](R) reader structure
impl crate::Readable for GINTSTSrs {}
///`write(|w| ..)` method takes [`gintsts::W`](W) writer structure
impl crate::Writable for GINTSTSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GINTSTS to value 0x0400_0020
impl crate::Resettable for GINTSTSrs {
    const RESET_VALUE: u32 = 0x0400_0020;
}
