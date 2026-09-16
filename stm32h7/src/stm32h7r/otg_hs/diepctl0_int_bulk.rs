///Register `DIEPCTL0_INT_BULK` reader
pub type R = crate::R<DIEPCTL0_INT_BULKrs>;
///Register `DIEPCTL0_INT_BULK` writer
pub type W = crate::W<DIEPCTL0_INT_BULKrs>;
///Field `MPSIZ` reader - Maximum packet size The application must program this field with the maximum packet size for the current logical endpoint. This value is in bytes.
pub type MPSIZ_R = crate::FieldReader<u16>;
///Field `MPSIZ` writer - Maximum packet size The application must program this field with the maximum packet size for the current logical endpoint. This value is in bytes.
pub type MPSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `USBAEP` reader - USB active endpoint Indicates whether this endpoint is active in the current configuration and interface. The core clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving the SetConfiguration and SetInterface commands, the application must program endpoint registers accordingly and set this bit.
pub type USBAEP_R = crate::BitReader;
///Field `USBAEP` writer - USB active endpoint Indicates whether this endpoint is active in the current configuration and interface. The core clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving the SetConfiguration and SetInterface commands, the application must program endpoint registers accordingly and set this bit.
pub type USBAEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPID` reader - Endpoint data PID Applies to interrupt/bulk IN endpoints only. Contains the PID of the packet to be received or transmitted on this endpoint. The application must program the PID of the first packet to be received or transmitted on this endpoint, after the endpoint is activated. The application uses the SD0PID and SD1PID register fields to program either DATA0 or DATA1 PID.
pub type DPID_R = crate::BitReader;
///Field `NAKSTS` reader - NAK status It indicates the following: When either the application or the core sets this bit: For non-isochronous IN endpoints: The core stops transmitting any data on an IN endpoint, even if there are data available in the Tx FIFO. For isochronous IN endpoints: The core sends out a zero-length data packet, even if there are data available in the Tx FIFO. Irrespective of this bits setting, the core always responds to SETUP data packets with an ACK handshake.
pub type NAKSTS_R = crate::BitReader;
///Field `EPTYP` reader - Endpoint type This is the transfer type supported by this logical endpoint.
pub type EPTYP_R = crate::FieldReader;
///Field `EPTYP` writer - Endpoint type This is the transfer type supported by this logical endpoint.
pub type EPTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `STALL` reader - STALL handshake Applies to non-control, non-isochronous IN endpoints only (access type is rw). The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Only the application can clear this bit, never the core.
pub type STALL_R = crate::BitReader;
///Field `STALL` writer - STALL handshake Applies to non-control, non-isochronous IN endpoints only (access type is rw). The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Only the application can clear this bit, never the core.
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFNUM` reader - Tx FIFO number These bits specify the FIFO number associated with this endpoint. Each active IN endpoint must be programmed to a separate FIFO number. This field is valid only for IN endpoints.
pub type TXFNUM_R = crate::FieldReader;
///Field `TXFNUM` writer - Tx FIFO number These bits specify the FIFO number associated with this endpoint. Each active IN endpoint must be programmed to a separate FIFO number. This field is valid only for IN endpoints.
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CNAK` writer - Clear NAK A write to this bit clears the NAK bit for the endpoint.
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNAK` writer - Set NAK A write to this bit sets the NAK bit for the endpoint. Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit for OUT endpoints on a transfer completed interrupt, or after a SETUP is received on the endpoint.
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SD0PID` writer - Set DATA0 PID Applies to interrupt/bulk IN endpoints only. Writing to this field sets the endpoint data PID (DPID) field in this register to DATA0.
pub type SD0PID_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SD1PID` writer - Set DATA1 PID Writing to this field sets the endpoint data PID (DPID) field in this register to DATA1.
pub type SD1PID_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDIS` reader - Endpoint disable The application sets this bit to stop transmitting/receiving data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the endpoint disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the endpoint disabled interrupt. The application must set this bit only if endpoint enable is already set for this endpoint.
pub type EPDIS_R = crate::BitReader;
///Field `EPDIS` writer - Endpoint disable The application sets this bit to stop transmitting/receiving data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the endpoint disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the endpoint disabled interrupt. The application must set this bit only if endpoint enable is already set for this endpoint.
pub type EPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPENA` reader - Endpoint enable The application sets this bit to start transmitting data on an endpoint. The core clears this bit before setting any of the following interrupts on this endpoint: SETUP phase done Endpoint disabled Transfer completed
pub type EPENA_R = crate::BitReader;
///Field `EPENA` writer - Endpoint enable The application sets this bit to start transmitting data on an endpoint. The core clears this bit before setting any of the following interrupts on this endpoint: SETUP phase done Endpoint disabled Transfer completed
pub type EPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:10 - Maximum packet size The application must program this field with the maximum packet size for the current logical endpoint. This value is in bytes.
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 15 - USB active endpoint Indicates whether this endpoint is active in the current configuration and interface. The core clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving the SetConfiguration and SetInterface commands, the application must program endpoint registers accordingly and set this bit.
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Endpoint data PID Applies to interrupt/bulk IN endpoints only. Contains the PID of the packet to be received or transmitted on this endpoint. The application must program the PID of the first packet to be received or transmitted on this endpoint, after the endpoint is activated. The application uses the SD0PID and SD1PID register fields to program either DATA0 or DATA1 PID.
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - NAK status It indicates the following: When either the application or the core sets this bit: For non-isochronous IN endpoints: The core stops transmitting any data on an IN endpoint, even if there are data available in the Tx FIFO. For isochronous IN endpoints: The core sends out a zero-length data packet, even if there are data available in the Tx FIFO. Irrespective of this bits setting, the core always responds to SETUP data packets with an ACK handshake.
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - Endpoint type This is the transfer type supported by this logical endpoint.
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 21 - STALL handshake Applies to non-control, non-isochronous IN endpoints only (access type is rw). The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Only the application can clear this bit, never the core.
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:25 - Tx FIFO number These bits specify the FIFO number associated with this endpoint. Each active IN endpoint must be programmed to a separate FIFO number. This field is valid only for IN endpoints.
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    ///Bit 30 - Endpoint disable The application sets this bit to stop transmitting/receiving data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the endpoint disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the endpoint disabled interrupt. The application must set this bit only if endpoint enable is already set for this endpoint.
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Endpoint enable The application sets this bit to start transmitting data on an endpoint. The core clears this bit before setting any of the following interrupts on this endpoint: SETUP phase done Endpoint disabled Transfer completed
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPCTL0_INT_BULK")
            .field("mpsiz", &self.mpsiz())
            .field("usbaep", &self.usbaep())
            .field("dpid", &self.dpid())
            .field("naksts", &self.naksts())
            .field("eptyp", &self.eptyp())
            .field("stall", &self.stall())
            .field("txfnum", &self.txfnum())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Maximum packet size The application must program this field with the maximum packet size for the current logical endpoint. This value is in bytes.
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MPSIZ_W<'_, DIEPCTL0_INT_BULKrs> {
        MPSIZ_W::new(self, 0)
    }
    ///Bit 15 - USB active endpoint Indicates whether this endpoint is active in the current configuration and interface. The core clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving the SetConfiguration and SetInterface commands, the application must program endpoint registers accordingly and set this bit.
    #[inline(always)]
    pub fn usbaep(&mut self) -> USBAEP_W<'_, DIEPCTL0_INT_BULKrs> {
        USBAEP_W::new(self, 15)
    }
    ///Bits 18:19 - Endpoint type This is the transfer type supported by this logical endpoint.
    #[inline(always)]
    pub fn eptyp(&mut self) -> EPTYP_W<'_, DIEPCTL0_INT_BULKrs> {
        EPTYP_W::new(self, 18)
    }
    ///Bit 21 - STALL handshake Applies to non-control, non-isochronous IN endpoints only (access type is rw). The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Only the application can clear this bit, never the core.
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<'_, DIEPCTL0_INT_BULKrs> {
        STALL_W::new(self, 21)
    }
    ///Bits 22:25 - Tx FIFO number These bits specify the FIFO number associated with this endpoint. Each active IN endpoint must be programmed to a separate FIFO number. This field is valid only for IN endpoints.
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W<'_, DIEPCTL0_INT_BULKrs> {
        TXFNUM_W::new(self, 22)
    }
    ///Bit 26 - Clear NAK A write to this bit clears the NAK bit for the endpoint.
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W<'_, DIEPCTL0_INT_BULKrs> {
        CNAK_W::new(self, 26)
    }
    ///Bit 27 - Set NAK A write to this bit sets the NAK bit for the endpoint. Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit for OUT endpoints on a transfer completed interrupt, or after a SETUP is received on the endpoint.
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W<'_, DIEPCTL0_INT_BULKrs> {
        SNAK_W::new(self, 27)
    }
    ///Bit 28 - Set DATA0 PID Applies to interrupt/bulk IN endpoints only. Writing to this field sets the endpoint data PID (DPID) field in this register to DATA0.
    #[inline(always)]
    pub fn sd0pid(&mut self) -> SD0PID_W<'_, DIEPCTL0_INT_BULKrs> {
        SD0PID_W::new(self, 28)
    }
    ///Bit 29 - Set DATA1 PID Writing to this field sets the endpoint data PID (DPID) field in this register to DATA1.
    #[inline(always)]
    pub fn sd1pid(&mut self) -> SD1PID_W<'_, DIEPCTL0_INT_BULKrs> {
        SD1PID_W::new(self, 29)
    }
    ///Bit 30 - Endpoint disable The application sets this bit to stop transmitting/receiving data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the endpoint disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the endpoint disabled interrupt. The application must set this bit only if endpoint enable is already set for this endpoint.
    #[inline(always)]
    pub fn epdis(&mut self) -> EPDIS_W<'_, DIEPCTL0_INT_BULKrs> {
        EPDIS_W::new(self, 30)
    }
    ///Bit 31 - Endpoint enable The application sets this bit to start transmitting data on an endpoint. The core clears this bit before setting any of the following interrupts on this endpoint: SETUP phase done Endpoint disabled Transfer completed
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W<'_, DIEPCTL0_INT_BULKrs> {
        EPENA_W::new(self, 31)
    }
}
/**OTG device IN endpoint 0 control register

You can [`read`](crate::Reg::read) this register and get [`diepctl0_int_bulk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl0_int_bulk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#OTG_HS:DIEPCTL0_INT_BULK)*/
pub struct DIEPCTL0_INT_BULKrs;
impl crate::RegisterSpec for DIEPCTL0_INT_BULKrs {
    type Ux = u32;
}
///`read()` method returns [`diepctl0_int_bulk::R`](R) reader structure
impl crate::Readable for DIEPCTL0_INT_BULKrs {}
///`write(|w| ..)` method takes [`diepctl0_int_bulk::W`](W) writer structure
impl crate::Writable for DIEPCTL0_INT_BULKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPCTL0_INT_BULK to value 0
impl crate::Resettable for DIEPCTL0_INT_BULKrs {}
