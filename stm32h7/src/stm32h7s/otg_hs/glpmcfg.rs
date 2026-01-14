///Register `GLPMCFG` reader
pub type R = crate::R<GLPMCFGrs>;
///Register `GLPMCFG` writer
pub type W = crate::W<GLPMCFGrs>;
///Field `LPMEN` reader - LPM support enable The application uses this bit to control the OTG_HS core LPM capabilities. If the core operates as a non-LPM-capable host, it cannot request the connected device or hub to activate LPM mode. If the core operates as a non-LPM-capable device, it cannot respond to any LPM transactions.
pub type LPMEN_R = crate::BitReader;
///Field `LPMEN` writer - LPM support enable The application uses this bit to control the OTG_HS core LPM capabilities. If the core operates as a non-LPM-capable host, it cannot request the connected device or hub to activate LPM mode. If the core operates as a non-LPM-capable device, it cannot respond to any LPM transactions.
pub type LPMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPMACK` reader - LPM token acknowledge enable Handshake response to LPM token preprogrammed by device application software. Even though ACK is preprogrammed, the core device responds with ACK only on successful LPM transaction. The LPM transaction is successful if: No PID/CRC5 errors in either EXT token or LPM token (else ERROR) Valid bLinkState = 0001B (L1) received in LPM transaction (else STALL) No data pending in transmit queue (else NYET). The preprogrammed software bit is over-ridden for response to LPM token when: The received bLinkState is not L1 (STALL response), or An error is detected in either of the LPM token packets because of corruption (ERROR response). Note: Accessible only in device mode.
pub type LPMACK_R = crate::BitReader;
///Field `LPMACK` writer - LPM token acknowledge enable Handshake response to LPM token preprogrammed by device application software. Even though ACK is preprogrammed, the core device responds with ACK only on successful LPM transaction. The LPM transaction is successful if: No PID/CRC5 errors in either EXT token or LPM token (else ERROR) Valid bLinkState = 0001B (L1) received in LPM transaction (else STALL) No data pending in transmit queue (else NYET). The preprogrammed software bit is over-ridden for response to LPM token when: The received bLinkState is not L1 (STALL response), or An error is detected in either of the LPM token packets because of corruption (ERROR response). Note: Accessible only in device mode.
pub type LPMACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BESL` reader - Best effort service latency Host mode
pub type BESL_R = crate::FieldReader;
///Field `BESL` writer - Best effort service latency Host mode
pub type BESL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `REMWAKE` reader - bRemoteWake value Host mode: The value of remote wake up to be sent in the wIndex field of LPM transaction. Device mode (read-only): This field is updated with the received LPM token bRemoteWake bmAttribute when an ACK, NYET, or STALL response is sent to an LPM transaction.
pub type REMWAKE_R = crate::BitReader;
///Field `REMWAKE` writer - bRemoteWake value Host mode: The value of remote wake up to be sent in the wIndex field of LPM transaction. Device mode (read-only): This field is updated with the received LPM token bRemoteWake bmAttribute when an ACK, NYET, or STALL response is sent to an LPM transaction.
pub type REMWAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1SSEN` reader - L1 Shallow Sleep enable Enables suspending the PHY in L1 Sleep mode. For maximum power saving during L1 Sleep mode, this bit should be set to '1' by application SW in all the cases.
pub type L1SSEN_R = crate::BitReader;
///Field `L1SSEN` writer - L1 Shallow Sleep enable Enables suspending the PHY in L1 Sleep mode. For maximum power saving during L1 Sleep mode, this bit should be set to '1' by application SW in all the cases.
pub type L1SSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BESLTHRS` reader - BESL threshold
pub type BESLTHRS_R = crate::FieldReader;
///Field `BESLTHRS` writer - BESL threshold
pub type BESLTHRS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `L1DSEN` reader - L1 deep sleep enable Enables suspending the PHY in L1 Sleep mode. For maximum power saving during L1 Sleep mode, this bit should be set to '1' by application SW in all the cases.
pub type L1DSEN_R = crate::BitReader;
///Field `L1DSEN` writer - L1 deep sleep enable Enables suspending the PHY in L1 Sleep mode. For maximum power saving during L1 Sleep mode, this bit should be set to '1' by application SW in all the cases.
pub type L1DSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPMRSP` reader - LPM response Device mode: The response of the core to LPM transaction received is reflected in these two bits. Host mode: Handshake response received from local device for LPM transaction
pub type LPMRSP_R = crate::FieldReader;
///Field `SLPSTS` reader - Port sleep status Device mode: This bit is set as long as a Sleep condition is present on the USB bus. The core enters the Sleep state when an ACK response is sent to an LPM transaction and the T<sub>L1TokenRetry</sub> timer has expired. To stop the PHY clock, the application must set the STPPCLK bit in OTG_PCGCCTL, which asserts the PHY suspend input signal. The application must rely on SLPSTS and not ACK in LPMRSP to confirm transition into sleep. The core comes out of sleep: When there is any activity on the USB linestate When the application writes to the RWUSIG bit in OTG_DCTL or when the application resets or soft-disconnects the device. Host mode: The host transitions to Sleep (L1) state as a side-effect of a successful LPM transaction by the core to the local port with ACK response from the device. The read value of this bit reflects the current Sleep status of the port. The core clears this bit after: The core detects a remote L1 wakeup signal, The application sets the PRST bit or the PRES bit in the OTG_HPRT register, or The application sets the L1Resume/ remote wakeup detected interrupt bit or disconnect detected interrupt bit in the core interrupt register (WKUPINT or DISCINT bit in OTG_GINTSTS, respectively).
pub type SLPSTS_R = crate::BitReader;
///Field `L1RSMOK` reader - Sleep state resume OK
pub type L1RSMOK_R = crate::BitReader;
///Field `LPMCHIDX` reader - LPM Channel Index The channel number on which the LPM transaction has to be applied while sending an LPM transaction to the local device. Based on the LPM channel index, the core automatically inserts the device address and endpoint number programmed in the corresponding channel into the LPM transaction. Note: Accessible only in host mode.
pub type LPMCHIDX_R = crate::FieldReader;
///Field `LPMCHIDX` writer - LPM Channel Index The channel number on which the LPM transaction has to be applied while sending an LPM transaction to the local device. Based on the LPM channel index, the core automatically inserts the device address and endpoint number programmed in the corresponding channel into the LPM transaction. Note: Accessible only in host mode.
pub type LPMCHIDX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LPMRCNT` reader - LPM retry count When the device gives an ERROR response, this is the number of additional LPM retries that the host performs until a valid device response (STALL, NYET, or ACK) is received. Note: Accessible only in host mode.
pub type LPMRCNT_R = crate::FieldReader;
///Field `LPMRCNT` writer - LPM retry count When the device gives an ERROR response, this is the number of additional LPM retries that the host performs until a valid device response (STALL, NYET, or ACK) is received. Note: Accessible only in host mode.
pub type LPMRCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SNDLPM` reader - Send LPM transaction When the application software sets this bit, an LPM transaction containing two tokens, EXT and LPM is sent. The hardware clears this bit once a valid response (STALL, NYET, or ACK) is received from the device or the core has finished transmitting the programmed number of LPM retries. Note: This bit must be set only when the host is connected to a local port. Note: Accessible only in host mode.
pub type SNDLPM_R = crate::BitReader;
///Field `SNDLPM` writer - Send LPM transaction When the application software sets this bit, an LPM transaction containing two tokens, EXT and LPM is sent. The hardware clears this bit once a valid response (STALL, NYET, or ACK) is received from the device or the core has finished transmitting the programmed number of LPM retries. Note: This bit must be set only when the host is connected to a local port. Note: Accessible only in host mode.
pub type SNDLPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPMRCNTSTS` reader - LPM retry count status Number of LPM host retries still remaining to be transmitted for the current LPM sequence. Note: Accessible only in host mode.
pub type LPMRCNTSTS_R = crate::FieldReader;
///Field `ENBESL` reader - Enable best effort service latency This bit enables the BESL feature as defined in the LPM errata: USB 2.0 Link Power Management Addendum Engineering Change Notice to the USB 2.0 specification, July 16, 2007 Errata for USB 2.0 ECN: Link Power Management (LPM) - 7/2007 Note: Only the updated behavior (described in LPM Errata) is considered in this document and so the ENBESL bit should be set to '1' by application SW.
pub type ENBESL_R = crate::BitReader;
///Field `ENBESL` writer - Enable best effort service latency This bit enables the BESL feature as defined in the LPM errata: USB 2.0 Link Power Management Addendum Engineering Change Notice to the USB 2.0 specification, July 16, 2007 Errata for USB 2.0 ECN: Link Power Management (LPM) - 7/2007 Note: Only the updated behavior (described in LPM Errata) is considered in this document and so the ENBESL bit should be set to '1' by application SW.
pub type ENBESL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LPM support enable The application uses this bit to control the OTG_HS core LPM capabilities. If the core operates as a non-LPM-capable host, it cannot request the connected device or hub to activate LPM mode. If the core operates as a non-LPM-capable device, it cannot respond to any LPM transactions.
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPM token acknowledge enable Handshake response to LPM token preprogrammed by device application software. Even though ACK is preprogrammed, the core device responds with ACK only on successful LPM transaction. The LPM transaction is successful if: No PID/CRC5 errors in either EXT token or LPM token (else ERROR) Valid bLinkState = 0001B (L1) received in LPM transaction (else STALL) No data pending in transmit queue (else NYET). The preprogrammed software bit is over-ridden for response to LPM token when: The received bLinkState is not L1 (STALL response), or An error is detected in either of the LPM token packets because of corruption (ERROR response). Note: Accessible only in device mode.
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5 - Best effort service latency Host mode
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bit 6 - bRemoteWake value Host mode: The value of remote wake up to be sent in the wIndex field of LPM transaction. Device mode (read-only): This field is updated with the received LPM token bRemoteWake bmAttribute when an ACK, NYET, or STALL response is sent to an LPM transaction.
    #[inline(always)]
    pub fn remwake(&self) -> REMWAKE_R {
        REMWAKE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - L1 Shallow Sleep enable Enables suspending the PHY in L1 Sleep mode. For maximum power saving during L1 Sleep mode, this bit should be set to '1' by application SW in all the cases.
    #[inline(always)]
    pub fn l1ssen(&self) -> L1SSEN_R {
        L1SSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - BESL threshold
    #[inline(always)]
    pub fn beslthrs(&self) -> BESLTHRS_R {
        BESLTHRS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - L1 deep sleep enable Enables suspending the PHY in L1 Sleep mode. For maximum power saving during L1 Sleep mode, this bit should be set to '1' by application SW in all the cases.
    #[inline(always)]
    pub fn l1dsen(&self) -> L1DSEN_R {
        L1DSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - LPM response Device mode: The response of the core to LPM transaction received is reflected in these two bits. Host mode: Handshake response received from local device for LPM transaction
    #[inline(always)]
    pub fn lpmrsp(&self) -> LPMRSP_R {
        LPMRSP_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - Port sleep status Device mode: This bit is set as long as a Sleep condition is present on the USB bus. The core enters the Sleep state when an ACK response is sent to an LPM transaction and the T<sub>L1TokenRetry</sub> timer has expired. To stop the PHY clock, the application must set the STPPCLK bit in OTG_PCGCCTL, which asserts the PHY suspend input signal. The application must rely on SLPSTS and not ACK in LPMRSP to confirm transition into sleep. The core comes out of sleep: When there is any activity on the USB linestate When the application writes to the RWUSIG bit in OTG_DCTL or when the application resets or soft-disconnects the device. Host mode: The host transitions to Sleep (L1) state as a side-effect of a successful LPM transaction by the core to the local port with ACK response from the device. The read value of this bit reflects the current Sleep status of the port. The core clears this bit after: The core detects a remote L1 wakeup signal, The application sets the PRST bit or the PRES bit in the OTG_HPRT register, or The application sets the L1Resume/ remote wakeup detected interrupt bit or disconnect detected interrupt bit in the core interrupt register (WKUPINT or DISCINT bit in OTG_GINTSTS, respectively).
    #[inline(always)]
    pub fn slpsts(&self) -> SLPSTS_R {
        SLPSTS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Sleep state resume OK
    #[inline(always)]
    pub fn l1rsmok(&self) -> L1RSMOK_R {
        L1RSMOK_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:20 - LPM Channel Index The channel number on which the LPM transaction has to be applied while sending an LPM transaction to the local device. Based on the LPM channel index, the core automatically inserts the device address and endpoint number programmed in the corresponding channel into the LPM transaction. Note: Accessible only in host mode.
    #[inline(always)]
    pub fn lpmchidx(&self) -> LPMCHIDX_R {
        LPMCHIDX_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    ///Bits 21:23 - LPM retry count When the device gives an ERROR response, this is the number of additional LPM retries that the host performs until a valid device response (STALL, NYET, or ACK) is received. Note: Accessible only in host mode.
    #[inline(always)]
    pub fn lpmrcnt(&self) -> LPMRCNT_R {
        LPMRCNT_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bit 24 - Send LPM transaction When the application software sets this bit, an LPM transaction containing two tokens, EXT and LPM is sent. The hardware clears this bit once a valid response (STALL, NYET, or ACK) is received from the device or the core has finished transmitting the programmed number of LPM retries. Note: This bit must be set only when the host is connected to a local port. Note: Accessible only in host mode.
    #[inline(always)]
    pub fn sndlpm(&self) -> SNDLPM_R {
        SNDLPM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27 - LPM retry count status Number of LPM host retries still remaining to be transmitted for the current LPM sequence. Note: Accessible only in host mode.
    #[inline(always)]
    pub fn lpmrcntsts(&self) -> LPMRCNTSTS_R {
        LPMRCNTSTS_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - Enable best effort service latency This bit enables the BESL feature as defined in the LPM errata: USB 2.0 Link Power Management Addendum Engineering Change Notice to the USB 2.0 specification, July 16, 2007 Errata for USB 2.0 ECN: Link Power Management (LPM) - 7/2007 Note: Only the updated behavior (described in LPM Errata) is considered in this document and so the ENBESL bit should be set to '1' by application SW.
    #[inline(always)]
    pub fn enbesl(&self) -> ENBESL_R {
        ENBESL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GLPMCFG")
            .field("lpmen", &self.lpmen())
            .field("lpmack", &self.lpmack())
            .field("besl", &self.besl())
            .field("remwake", &self.remwake())
            .field("l1ssen", &self.l1ssen())
            .field("beslthrs", &self.beslthrs())
            .field("l1dsen", &self.l1dsen())
            .field("lpmrsp", &self.lpmrsp())
            .field("slpsts", &self.slpsts())
            .field("l1rsmok", &self.l1rsmok())
            .field("lpmchidx", &self.lpmchidx())
            .field("lpmrcnt", &self.lpmrcnt())
            .field("sndlpm", &self.sndlpm())
            .field("lpmrcntsts", &self.lpmrcntsts())
            .field("enbesl", &self.enbesl())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPM support enable The application uses this bit to control the OTG_HS core LPM capabilities. If the core operates as a non-LPM-capable host, it cannot request the connected device or hub to activate LPM mode. If the core operates as a non-LPM-capable device, it cannot respond to any LPM transactions.
    #[inline(always)]
    pub fn lpmen(&mut self) -> LPMEN_W<'_, GLPMCFGrs> {
        LPMEN_W::new(self, 0)
    }
    ///Bit 1 - LPM token acknowledge enable Handshake response to LPM token preprogrammed by device application software. Even though ACK is preprogrammed, the core device responds with ACK only on successful LPM transaction. The LPM transaction is successful if: No PID/CRC5 errors in either EXT token or LPM token (else ERROR) Valid bLinkState = 0001B (L1) received in LPM transaction (else STALL) No data pending in transmit queue (else NYET). The preprogrammed software bit is over-ridden for response to LPM token when: The received bLinkState is not L1 (STALL response), or An error is detected in either of the LPM token packets because of corruption (ERROR response). Note: Accessible only in device mode.
    #[inline(always)]
    pub fn lpmack(&mut self) -> LPMACK_W<'_, GLPMCFGrs> {
        LPMACK_W::new(self, 1)
    }
    ///Bits 2:5 - Best effort service latency Host mode
    #[inline(always)]
    pub fn besl(&mut self) -> BESL_W<'_, GLPMCFGrs> {
        BESL_W::new(self, 2)
    }
    ///Bit 6 - bRemoteWake value Host mode: The value of remote wake up to be sent in the wIndex field of LPM transaction. Device mode (read-only): This field is updated with the received LPM token bRemoteWake bmAttribute when an ACK, NYET, or STALL response is sent to an LPM transaction.
    #[inline(always)]
    pub fn remwake(&mut self) -> REMWAKE_W<'_, GLPMCFGrs> {
        REMWAKE_W::new(self, 6)
    }
    ///Bit 7 - L1 Shallow Sleep enable Enables suspending the PHY in L1 Sleep mode. For maximum power saving during L1 Sleep mode, this bit should be set to '1' by application SW in all the cases.
    #[inline(always)]
    pub fn l1ssen(&mut self) -> L1SSEN_W<'_, GLPMCFGrs> {
        L1SSEN_W::new(self, 7)
    }
    ///Bits 8:11 - BESL threshold
    #[inline(always)]
    pub fn beslthrs(&mut self) -> BESLTHRS_W<'_, GLPMCFGrs> {
        BESLTHRS_W::new(self, 8)
    }
    ///Bit 12 - L1 deep sleep enable Enables suspending the PHY in L1 Sleep mode. For maximum power saving during L1 Sleep mode, this bit should be set to '1' by application SW in all the cases.
    #[inline(always)]
    pub fn l1dsen(&mut self) -> L1DSEN_W<'_, GLPMCFGrs> {
        L1DSEN_W::new(self, 12)
    }
    ///Bits 17:20 - LPM Channel Index The channel number on which the LPM transaction has to be applied while sending an LPM transaction to the local device. Based on the LPM channel index, the core automatically inserts the device address and endpoint number programmed in the corresponding channel into the LPM transaction. Note: Accessible only in host mode.
    #[inline(always)]
    pub fn lpmchidx(&mut self) -> LPMCHIDX_W<'_, GLPMCFGrs> {
        LPMCHIDX_W::new(self, 17)
    }
    ///Bits 21:23 - LPM retry count When the device gives an ERROR response, this is the number of additional LPM retries that the host performs until a valid device response (STALL, NYET, or ACK) is received. Note: Accessible only in host mode.
    #[inline(always)]
    pub fn lpmrcnt(&mut self) -> LPMRCNT_W<'_, GLPMCFGrs> {
        LPMRCNT_W::new(self, 21)
    }
    ///Bit 24 - Send LPM transaction When the application software sets this bit, an LPM transaction containing two tokens, EXT and LPM is sent. The hardware clears this bit once a valid response (STALL, NYET, or ACK) is received from the device or the core has finished transmitting the programmed number of LPM retries. Note: This bit must be set only when the host is connected to a local port. Note: Accessible only in host mode.
    #[inline(always)]
    pub fn sndlpm(&mut self) -> SNDLPM_W<'_, GLPMCFGrs> {
        SNDLPM_W::new(self, 24)
    }
    ///Bit 28 - Enable best effort service latency This bit enables the BESL feature as defined in the LPM errata: USB 2.0 Link Power Management Addendum Engineering Change Notice to the USB 2.0 specification, July 16, 2007 Errata for USB 2.0 ECN: Link Power Management (LPM) - 7/2007 Note: Only the updated behavior (described in LPM Errata) is considered in this document and so the ENBESL bit should be set to '1' by application SW.
    #[inline(always)]
    pub fn enbesl(&mut self) -> ENBESL_W<'_, GLPMCFGrs> {
        ENBESL_W::new(self, 28)
    }
}
/**OTG core LPM configuration register

You can [`read`](crate::Reg::read) this register and get [`glpmcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`glpmcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#OTG_HS:GLPMCFG)*/
pub struct GLPMCFGrs;
impl crate::RegisterSpec for GLPMCFGrs {
    type Ux = u32;
}
///`read()` method returns [`glpmcfg::R`](R) reader structure
impl crate::Readable for GLPMCFGrs {}
///`write(|w| ..)` method takes [`glpmcfg::W`](W) writer structure
impl crate::Writable for GLPMCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GLPMCFG to value 0
impl crate::Resettable for GLPMCFGrs {}
