///Register `DOEPCTL5_INT_BULK` reader
pub type R = crate::R<DOEPCTL5_INT_BULKrs>;
///Register `DOEPCTL5_INT_BULK` writer
pub type W = crate::W<DOEPCTL5_INT_BULKrs>;
///Field `MPSIZ` reader - Maximum packet size The application must program this field with the maximum packet size for the current logical endpoint. This value is in bytes.
pub type MPSIZ_R = crate::FieldReader<u16>;
///Field `MPSIZ` writer - Maximum packet size The application must program this field with the maximum packet size for the current logical endpoint. This value is in bytes.
pub type MPSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `USBAEP` reader - USB active endpoint Indicates whether this endpoint is active in the current configuration and interface. The core clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving the SetConfiguration and SetInterface commands, the application must program endpoint registers accordingly and set this bit.
pub type USBAEP_R = crate::BitReader;
///Field `USBAEP` writer - USB active endpoint Indicates whether this endpoint is active in the current configuration and interface. The core clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving the SetConfiguration and SetInterface commands, the application must program endpoint registers accordingly and set this bit.
pub type USBAEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPID` reader - Endpoint data PID Applies to interrupt/bulk OUT endpoints only. Contains the PID of the packet to be received or transmitted on this endpoint. The application must program the PID of the first packet to be received or transmitted on this endpoint, after the endpoint is activated. The application uses the SD0PID and SD1PID register fields to program either DATA0 or DATA1 PID.
pub type DPID_R = crate::BitReader;
///Field `NAKSTS` reader - NAK status Indicates the following: When either the application or the core sets this bit: The core stops receiving any data on an OUT endpoint, even if there is space in the Rx FIFO to accommodate the incoming packet. Irrespective of this bits setting, the core always responds to SETUP data packets with an ACK handshake.
pub type NAKSTS_R = crate::BitReader;
///Field `EPTYP` reader - Endpoint type This is the transfer type supported by this logical endpoint.
pub type EPTYP_R = crate::FieldReader;
///Field `EPTYP` writer - Endpoint type This is the transfer type supported by this logical endpoint.
pub type EPTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SNPM` reader - Snoop mode This bit configures the endpoint to Snoop mode. In Snoop mode, the core does not check the correctness of OUT packets before transferring them to application memory.
pub type SNPM_R = crate::BitReader;
///Field `SNPM` writer - Snoop mode This bit configures the endpoint to Snoop mode. In Snoop mode, the core does not check the correctness of OUT packets before transferring them to application memory.
pub type SNPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STALL` reader - STALL handshake Applies to non-control, non-isochronous OUT endpoints only (access type is rw). The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Only the application can clear this bit, never the core. Applies to control endpoints only (access type is rs). The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Irrespective of this bits setting, the core always responds to SETUP data packets with an ACK handshake.
pub type STALL_R = crate::BitReader;
///Field `STALL` writer - STALL handshake Applies to non-control, non-isochronous OUT endpoints only (access type is rw). The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Only the application can clear this bit, never the core. Applies to control endpoints only (access type is rs). The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Irrespective of this bits setting, the core always responds to SETUP data packets with an ACK handshake.
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNAK` writer - Clear NAK A write to this bit clears the NAK bit for the endpoint.
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNAK` writer - Set NAK A write to this bit sets the NAK bit for the endpoint. Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit for OUT endpoints on a transfer completed interrupt, or after a SETUP is received on the endpoint.
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SD0PID` writer - Set DATA0 PID Applies to interrupt/bulk OUT endpoints only. Writing to this field sets the endpoint data PID (DPID) field in this register to DATA0.
pub type SD0PID_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SD1PID` writer - Set DATA1 PID Writing to this field sets the endpoint data PID (DPID) field in this register to DATA1.
pub type SD1PID_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDIS` reader - Endpoint disable The application sets this bit to stop transmitting/receiving data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the endpoint disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the endpoint disabled interrupt. The application must set this bit only if endpoint enable is already set for this endpoint.
pub type EPDIS_R = crate::BitReader;
///Field `EPDIS` writer - Endpoint disable The application sets this bit to stop transmitting/receiving data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the endpoint disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the endpoint disabled interrupt. The application must set this bit only if endpoint enable is already set for this endpoint.
pub type EPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPENA` reader - Endpoint enable Applies to IN and OUT endpoints. The application sets this bit to start transmitting data on an endpoint. The core clears this bit before setting any of the following interrupts on this endpoint: SETUP phase done Endpoint disabled Transfer completed
pub type EPENA_R = crate::BitReader;
///Field `EPENA` writer - Endpoint enable Applies to IN and OUT endpoints. The application sets this bit to start transmitting data on an endpoint. The core clears this bit before setting any of the following interrupts on this endpoint: SETUP phase done Endpoint disabled Transfer completed
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
    ///Bit 16 - Endpoint data PID Applies to interrupt/bulk OUT endpoints only. Contains the PID of the packet to be received or transmitted on this endpoint. The application must program the PID of the first packet to be received or transmitted on this endpoint, after the endpoint is activated. The application uses the SD0PID and SD1PID register fields to program either DATA0 or DATA1 PID.
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - NAK status Indicates the following: When either the application or the core sets this bit: The core stops receiving any data on an OUT endpoint, even if there is space in the Rx FIFO to accommodate the incoming packet. Irrespective of this bits setting, the core always responds to SETUP data packets with an ACK handshake.
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - Endpoint type This is the transfer type supported by this logical endpoint.
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 20 - Snoop mode This bit configures the endpoint to Snoop mode. In Snoop mode, the core does not check the correctness of OUT packets before transferring them to application memory.
    #[inline(always)]
    pub fn snpm(&self) -> SNPM_R {
        SNPM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - STALL handshake Applies to non-control, non-isochronous OUT endpoints only (access type is rw). The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Only the application can clear this bit, never the core. Applies to control endpoints only (access type is rs). The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Irrespective of this bits setting, the core always responds to SETUP data packets with an ACK handshake.
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 30 - Endpoint disable The application sets this bit to stop transmitting/receiving data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the endpoint disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the endpoint disabled interrupt. The application must set this bit only if endpoint enable is already set for this endpoint.
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Endpoint enable Applies to IN and OUT endpoints. The application sets this bit to start transmitting data on an endpoint. The core clears this bit before setting any of the following interrupts on this endpoint: SETUP phase done Endpoint disabled Transfer completed
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPCTL5_INT_BULK")
            .field("mpsiz", &self.mpsiz())
            .field("usbaep", &self.usbaep())
            .field("dpid", &self.dpid())
            .field("naksts", &self.naksts())
            .field("eptyp", &self.eptyp())
            .field("snpm", &self.snpm())
            .field("stall", &self.stall())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Maximum packet size The application must program this field with the maximum packet size for the current logical endpoint. This value is in bytes.
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MPSIZ_W<DOEPCTL5_INT_BULKrs> {
        MPSIZ_W::new(self, 0)
    }
    ///Bit 15 - USB active endpoint Indicates whether this endpoint is active in the current configuration and interface. The core clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving the SetConfiguration and SetInterface commands, the application must program endpoint registers accordingly and set this bit.
    #[inline(always)]
    pub fn usbaep(&mut self) -> USBAEP_W<DOEPCTL5_INT_BULKrs> {
        USBAEP_W::new(self, 15)
    }
    ///Bits 18:19 - Endpoint type This is the transfer type supported by this logical endpoint.
    #[inline(always)]
    pub fn eptyp(&mut self) -> EPTYP_W<DOEPCTL5_INT_BULKrs> {
        EPTYP_W::new(self, 18)
    }
    ///Bit 20 - Snoop mode This bit configures the endpoint to Snoop mode. In Snoop mode, the core does not check the correctness of OUT packets before transferring them to application memory.
    #[inline(always)]
    pub fn snpm(&mut self) -> SNPM_W<DOEPCTL5_INT_BULKrs> {
        SNPM_W::new(self, 20)
    }
    ///Bit 21 - STALL handshake Applies to non-control, non-isochronous OUT endpoints only (access type is rw). The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Only the application can clear this bit, never the core. Applies to control endpoints only (access type is rs). The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit, Global IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Irrespective of this bits setting, the core always responds to SETUP data packets with an ACK handshake.
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<DOEPCTL5_INT_BULKrs> {
        STALL_W::new(self, 21)
    }
    ///Bit 26 - Clear NAK A write to this bit clears the NAK bit for the endpoint.
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W<DOEPCTL5_INT_BULKrs> {
        CNAK_W::new(self, 26)
    }
    ///Bit 27 - Set NAK A write to this bit sets the NAK bit for the endpoint. Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit for OUT endpoints on a transfer completed interrupt, or after a SETUP is received on the endpoint.
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W<DOEPCTL5_INT_BULKrs> {
        SNAK_W::new(self, 27)
    }
    ///Bit 28 - Set DATA0 PID Applies to interrupt/bulk OUT endpoints only. Writing to this field sets the endpoint data PID (DPID) field in this register to DATA0.
    #[inline(always)]
    pub fn sd0pid(&mut self) -> SD0PID_W<DOEPCTL5_INT_BULKrs> {
        SD0PID_W::new(self, 28)
    }
    ///Bit 29 - Set DATA1 PID Writing to this field sets the endpoint data PID (DPID) field in this register to DATA1.
    #[inline(always)]
    pub fn sd1pid(&mut self) -> SD1PID_W<DOEPCTL5_INT_BULKrs> {
        SD1PID_W::new(self, 29)
    }
    ///Bit 30 - Endpoint disable The application sets this bit to stop transmitting/receiving data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the endpoint disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the endpoint disabled interrupt. The application must set this bit only if endpoint enable is already set for this endpoint.
    #[inline(always)]
    pub fn epdis(&mut self) -> EPDIS_W<DOEPCTL5_INT_BULKrs> {
        EPDIS_W::new(self, 30)
    }
    ///Bit 31 - Endpoint enable Applies to IN and OUT endpoints. The application sets this bit to start transmitting data on an endpoint. The core clears this bit before setting any of the following interrupts on this endpoint: SETUP phase done Endpoint disabled Transfer completed
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W<DOEPCTL5_INT_BULKrs> {
        EPENA_W::new(self, 31)
    }
}
/**OTG device OUT endpoint 5 control register

You can [`read`](crate::Reg::read) this register and get [`doepctl5_int_bulk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl5_int_bulk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#OTG_HS:DOEPCTL5_INT_BULK)*/
pub struct DOEPCTL5_INT_BULKrs;
impl crate::RegisterSpec for DOEPCTL5_INT_BULKrs {
    type Ux = u32;
}
///`read()` method returns [`doepctl5_int_bulk::R`](R) reader structure
impl crate::Readable for DOEPCTL5_INT_BULKrs {}
///`write(|w| ..)` method takes [`doepctl5_int_bulk::W`](W) writer structure
impl crate::Writable for DOEPCTL5_INT_BULKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOEPCTL5_INT_BULK to value 0
impl crate::Resettable for DOEPCTL5_INT_BULKrs {}
