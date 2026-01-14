///Register `CHEP%sR` reader
pub type R = crate::R<CHEPRrs>;
///Register `CHEP%sR` writer
pub type W = crate::W<CHEPRrs>;
///Field `EA` reader - endpoint/channel address Device mode Software must write in this field the 4-bit address used to identify the transactions directed to this endpoint. A value must be written before enabling the corresponding endpoint. Host mode Software must write in this field the 4-bit address used to identify the channel addressed by the host transaction.
pub type EA_R = crate::FieldReader;
///Field `EA` writer - endpoint/channel address Device mode Software must write in this field the 4-bit address used to identify the transactions directed to this endpoint. A value must be written before enabling the corresponding endpoint. Host mode Software must write in this field the 4-bit address used to identify the channel addressed by the host transaction.
pub type EA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**Status bits, for transmission transfers Device mode These bits contain the information about the endpoint status, listed in . These bits can be toggled by the software to initialize their value. When the application software writes '0, the value remains unchanged, while writing '1 makes the bit value toggle. Hardware sets the STTX bits to NAK, when a correct transfer has occurred (VTTX=1) corresponding to a IN or SETUP (control only) transaction addressed to this channel/endpoint. It then waits for the software to prepare the next set of data to be transmitted. Double-buffered bulk endpoints implement a special transaction flow control, which controls the status based on buffer availability condition (Refer to endpoints). If the endpoint is defined as Isochronous, its status can only be 'VALID' or 'DISABLED'. Therefore, the hardware cannot change the status of the channel/endpoint/channel after a successful transaction. If the software sets the STTX bits to 'STALL' or 'NAK' for an Isochronous channel/endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing '1. Host mode Same as STRX behaviour but for IN transactions (TBC)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATTXR {
    ///0: All transmission requests addressed to this endpoint/channel are ignored.
    Disabled = 0,
    ///1: Device mode: the endpoint is stalled and all transmission requests result in a STALL handshake. Host mode: this indicates that the device has STALLed the channel.
    Stall = 1,
    ///2: Device mode: the endpoint is NAKed and all transmission requests result in a NAK handshake. Host mode: this indicates that the device has NAKed the transmission request.
    Nak = 2,
    ///3: This endpoint/channel is enabled for transmission.
    Valid = 3,
}
impl From<STATTXR> for u8 {
    #[inline(always)]
    fn from(variant: STATTXR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STATTXR {
    type Ux = u8;
}
impl crate::IsEnum for STATTXR {}
///Field `STATTX` reader - Status bits, for transmission transfers Device mode These bits contain the information about the endpoint status, listed in . These bits can be toggled by the software to initialize their value. When the application software writes '0, the value remains unchanged, while writing '1 makes the bit value toggle. Hardware sets the STTX bits to NAK, when a correct transfer has occurred (VTTX=1) corresponding to a IN or SETUP (control only) transaction addressed to this channel/endpoint. It then waits for the software to prepare the next set of data to be transmitted. Double-buffered bulk endpoints implement a special transaction flow control, which controls the status based on buffer availability condition (Refer to endpoints). If the endpoint is defined as Isochronous, its status can only be 'VALID' or 'DISABLED'. Therefore, the hardware cannot change the status of the channel/endpoint/channel after a successful transaction. If the software sets the STTX bits to 'STALL' or 'NAK' for an Isochronous channel/endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing '1. Host mode Same as STRX behaviour but for IN transactions (TBC)
pub type STATTX_R = crate::FieldReader<STATTXR>;
impl STATTX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STATTXR {
        match self.bits {
            0 => STATTXR::Disabled,
            1 => STATTXR::Stall,
            2 => STATTXR::Nak,
            3 => STATTXR::Valid,
            _ => unreachable!(),
        }
    }
    ///All transmission requests addressed to this endpoint/channel are ignored.
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STATTXR::Disabled
    }
    ///Device mode: the endpoint is stalled and all transmission requests result in a STALL handshake. Host mode: this indicates that the device has STALLed the channel.
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == STATTXR::Stall
    }
    ///Device mode: the endpoint is NAKed and all transmission requests result in a NAK handshake. Host mode: this indicates that the device has NAKed the transmission request.
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == STATTXR::Nak
    }
    ///This endpoint/channel is enabled for transmission.
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == STATTXR::Valid
    }
}
/**Status bits, for transmission transfers Device mode These bits contain the information about the endpoint status, listed in . These bits can be toggled by the software to initialize their value. When the application software writes '0, the value remains unchanged, while writing '1 makes the bit value toggle. Hardware sets the STTX bits to NAK, when a correct transfer has occurred (VTTX=1) corresponding to a IN or SETUP (control only) transaction addressed to this channel/endpoint. It then waits for the software to prepare the next set of data to be transmitted. Double-buffered bulk endpoints implement a special transaction flow control, which controls the status based on buffer availability condition (Refer to endpoints). If the endpoint is defined as Isochronous, its status can only be 'VALID' or 'DISABLED'. Therefore, the hardware cannot change the status of the channel/endpoint/channel after a successful transaction. If the software sets the STTX bits to 'STALL' or 'NAK' for an Isochronous channel/endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing '1. Host mode Same as STRX behaviour but for IN transactions (TBC)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATTXW {
    ///0: Do not change bits
    Keep = 0,
}
impl From<STATTXW> for u8 {
    #[inline(always)]
    fn from(variant: STATTXW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STATTXW {
    type Ux = u8;
}
impl crate::IsEnum for STATTXW {}
///Field `STATTX` writer - Status bits, for transmission transfers Device mode These bits contain the information about the endpoint status, listed in . These bits can be toggled by the software to initialize their value. When the application software writes '0, the value remains unchanged, while writing '1 makes the bit value toggle. Hardware sets the STTX bits to NAK, when a correct transfer has occurred (VTTX=1) corresponding to a IN or SETUP (control only) transaction addressed to this channel/endpoint. It then waits for the software to prepare the next set of data to be transmitted. Double-buffered bulk endpoints implement a special transaction flow control, which controls the status based on buffer availability condition (Refer to endpoints). If the endpoint is defined as Isochronous, its status can only be 'VALID' or 'DISABLED'. Therefore, the hardware cannot change the status of the channel/endpoint/channel after a successful transaction. If the software sets the STTX bits to 'STALL' or 'NAK' for an Isochronous channel/endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing '1. Host mode Same as STRX behaviour but for IN transactions (TBC)
pub type STATTX_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STATTXW>;
impl<'a, REG> STATTX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Do not change bits
    #[inline(always)]
    pub fn keep(self) -> &'a mut crate::W<REG> {
        self.variant(STATTXW::Keep)
    }
}
/**Data Toggle, for transmission transfers If the endpoint/channel is non-isochronous, this bit contains the required value of the data toggle bit (0=DATA0, 1=DATA1) for the next data packet to be transmitted. Hardware toggles this bit when the ACK handshake is received from the USB host, following a data packet transmission. If the endpoint/channel is defined as a control one, hardware sets this bit to 1 at the reception of a SETUP PID addressed to this endpoint. If the endpoint/channel is using the double buffer feature, this bit is used to support packet buffer swapping too (Refer to ) If the endpoint/channel is Isochronous, this bit is used to support packet buffer swapping since no data toggling is used for this sort of endpoints and only DATA0 packet are transmitted (Refer to ). Hardware toggles this bit just after the end of data packet transmission, since no handshake is used for Isochronous transfers. This bit can also be toggled by the software to initialize its value (mandatory when the endpoint/channel is not a control one) or to force a specific data toggle/packet buffer usage. When the application software writes '0, the value of DTOGTX remains unchanged, while writing '1 makes the bit value toggle. This bit is read/write but it can only be toggled by writing 1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTOGTXW {
    ///1: Flip bit
    Toggle = 1,
}
impl From<DTOGTXW> for bool {
    #[inline(always)]
    fn from(variant: DTOGTXW) -> Self {
        variant as u8 != 0
    }
}
///Field `DTOGTX` writer - Data Toggle, for transmission transfers If the endpoint/channel is non-isochronous, this bit contains the required value of the data toggle bit (0=DATA0, 1=DATA1) for the next data packet to be transmitted. Hardware toggles this bit when the ACK handshake is received from the USB host, following a data packet transmission. If the endpoint/channel is defined as a control one, hardware sets this bit to 1 at the reception of a SETUP PID addressed to this endpoint. If the endpoint/channel is using the double buffer feature, this bit is used to support packet buffer swapping too (Refer to ) If the endpoint/channel is Isochronous, this bit is used to support packet buffer swapping since no data toggling is used for this sort of endpoints and only DATA0 packet are transmitted (Refer to ). Hardware toggles this bit just after the end of data packet transmission, since no handshake is used for Isochronous transfers. This bit can also be toggled by the software to initialize its value (mandatory when the endpoint/channel is not a control one) or to force a specific data toggle/packet buffer usage. When the application software writes '0, the value of DTOGTX remains unchanged, while writing '1 makes the bit value toggle. This bit is read/write but it can only be toggled by writing 1.
pub type DTOGTX_W<'a, REG> = crate::BitWriter1T<'a, REG, DTOGTXW>;
impl<'a, REG> DTOGTX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flip bit
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(DTOGTXW::Toggle)
    }
}
/**Valid USB transaction transmitted Device mode This bit is set by the hardware when an IN transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in the USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only '0 can be written. Host mode Same of VTRX behaviour but for USB OUT and SETUP transactions.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VTTXW {
    ///0: Clear flag
    Clear = 0,
}
impl From<VTTXW> for bool {
    #[inline(always)]
    fn from(variant: VTTXW) -> Self {
        variant as u8 != 0
    }
}
///Field `VTTX` reader - Valid USB transaction transmitted Device mode This bit is set by the hardware when an IN transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in the USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only '0 can be written. Host mode Same of VTRX behaviour but for USB OUT and SETUP transactions.
pub type VTTX_R = crate::BitReader<VTTXW>;
impl VTTX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<VTTXW> {
        match self.bits {
            false => Some(VTTXW::Clear),
            _ => None,
        }
    }
    ///Clear flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == VTTXW::Clear
    }
}
///Field `VTTX` writer - Valid USB transaction transmitted Device mode This bit is set by the hardware when an IN transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in the USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only '0 can be written. Host mode Same of VTRX behaviour but for USB OUT and SETUP transactions.
pub type VTTX_W<'a, REG> = crate::BitWriter0C<'a, REG, VTTXW>;
impl<'a, REG> VTTX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(VTTXW::Clear)
    }
}
///Field `EPKIND` reader - endpoint/channel kind The meaning of this bit depends on the endpoint/channel type configured by the EP_TYPE bits. summarizes the different meanings. DBL_BUF: This bit is set by the software to enable the double-buffering feature for this bulk endpoint. The usage of double-buffered bulk endpoints is explained in Double-buffered endpoints. STATUS_OUT: This bit is set by the software to indicate that a status out transaction is expected: in this case all OUT transactions containing more than zero data bytes are answered 'STALL' instead of 'ACK'. This bit may be used to improve the robustness of the application to protocol errors during control transfers and its usage is intended for control endpoints only. When STATUS_OUT is reset, OUT transactions can have any number of bytes, as required.
pub type EPKIND_R = crate::BitReader;
///Field `EPKIND` writer - endpoint/channel kind The meaning of this bit depends on the endpoint/channel type configured by the EP_TYPE bits. summarizes the different meanings. DBL_BUF: This bit is set by the software to enable the double-buffering feature for this bulk endpoint. The usage of double-buffered bulk endpoints is explained in Double-buffered endpoints. STATUS_OUT: This bit is set by the software to indicate that a status out transaction is expected: in this case all OUT transactions containing more than zero data bytes are answered 'STALL' instead of 'ACK'. This bit may be used to improve the robustness of the application to protocol errors during control transfers and its usage is intended for control endpoints only. When STATUS_OUT is reset, OUT transactions can have any number of bytes, as required.
pub type EPKIND_W<'a, REG> = crate::BitWriter<'a, REG>;
/**USB type of transaction These bits configure the behavior of this endpoint/channel as described in endpoint/channel type encoding on page2001. Channel0/Endpoint0 must always be a control endpoint/channel and each USB function must have at least one control endpoint/channel which has address 0, but there may be other control channels/endpoints if required. Only control channels/endpoints handle SETUP transactions, which are ignored by endpoints of other kinds. SETUP transactions cannot be answered with NAK or STALL. If a control endpoint/channel is defined as NAK, the USB peripheral will not answer, simulating a receive error, in the receive direction when a SETUP transaction is received. If the control endpoint/channel is defined as STALL in the receive direction, then the SETUP packet will be accepted anyway, transferring data and issuing the CTR interrupt. The reception of OUT transactions is handled in the normal way, even if the endpoint/channel is a control one. Bulk and interrupt endpoints have very similar behavior and they differ only in the special feature available using the EPKIND configuration bit. The usage of Isochronous channels/endpoints is explained in transfers

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UTYPE {
    ///0: Bulk endpoint
    Bulk = 0,
    ///1: Control endpoint
    Control = 1,
    ///2: Isochronous endpoint
    Iso = 2,
    ///3: Interrupt endpoint
    Interrupt = 3,
}
impl From<UTYPE> for u8 {
    #[inline(always)]
    fn from(variant: UTYPE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UTYPE {
    type Ux = u8;
}
impl crate::IsEnum for UTYPE {}
///Field `UTYPE` reader - USB type of transaction These bits configure the behavior of this endpoint/channel as described in endpoint/channel type encoding on page2001. Channel0/Endpoint0 must always be a control endpoint/channel and each USB function must have at least one control endpoint/channel which has address 0, but there may be other control channels/endpoints if required. Only control channels/endpoints handle SETUP transactions, which are ignored by endpoints of other kinds. SETUP transactions cannot be answered with NAK or STALL. If a control endpoint/channel is defined as NAK, the USB peripheral will not answer, simulating a receive error, in the receive direction when a SETUP transaction is received. If the control endpoint/channel is defined as STALL in the receive direction, then the SETUP packet will be accepted anyway, transferring data and issuing the CTR interrupt. The reception of OUT transactions is handled in the normal way, even if the endpoint/channel is a control one. Bulk and interrupt endpoints have very similar behavior and they differ only in the special feature available using the EPKIND configuration bit. The usage of Isochronous channels/endpoints is explained in transfers
pub type UTYPE_R = crate::FieldReader<UTYPE>;
impl UTYPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UTYPE {
        match self.bits {
            0 => UTYPE::Bulk,
            1 => UTYPE::Control,
            2 => UTYPE::Iso,
            3 => UTYPE::Interrupt,
            _ => unreachable!(),
        }
    }
    ///Bulk endpoint
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == UTYPE::Bulk
    }
    ///Control endpoint
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == UTYPE::Control
    }
    ///Isochronous endpoint
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == UTYPE::Iso
    }
    ///Interrupt endpoint
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == UTYPE::Interrupt
    }
}
///Field `UTYPE` writer - USB type of transaction These bits configure the behavior of this endpoint/channel as described in endpoint/channel type encoding on page2001. Channel0/Endpoint0 must always be a control endpoint/channel and each USB function must have at least one control endpoint/channel which has address 0, but there may be other control channels/endpoints if required. Only control channels/endpoints handle SETUP transactions, which are ignored by endpoints of other kinds. SETUP transactions cannot be answered with NAK or STALL. If a control endpoint/channel is defined as NAK, the USB peripheral will not answer, simulating a receive error, in the receive direction when a SETUP transaction is received. If the control endpoint/channel is defined as STALL in the receive direction, then the SETUP packet will be accepted anyway, transferring data and issuing the CTR interrupt. The reception of OUT transactions is handled in the normal way, even if the endpoint/channel is a control one. Bulk and interrupt endpoints have very similar behavior and they differ only in the special feature available using the EPKIND configuration bit. The usage of Isochronous channels/endpoints is explained in transfers
pub type UTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UTYPE, crate::Safe>;
impl<'a, REG> UTYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Bulk endpoint
    #[inline(always)]
    pub fn bulk(self) -> &'a mut crate::W<REG> {
        self.variant(UTYPE::Bulk)
    }
    ///Control endpoint
    #[inline(always)]
    pub fn control(self) -> &'a mut crate::W<REG> {
        self.variant(UTYPE::Control)
    }
    ///Isochronous endpoint
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(UTYPE::Iso)
    }
    ///Interrupt endpoint
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(UTYPE::Interrupt)
    }
}
///Field `SETUP` reader - Setup transaction completed Device mode This bit is read-only and it is set by the hardware when the last completed transaction is a SETUP. This bit changes its value only for control endpoints. It must be examined, in the case of a successful receive transaction (VTRX event), to determine the type of transaction occurred. To protect the interrupt service routine from the changes in SETUP bits due to next incoming tokens, this bit is kept frozen while VTRX bit is at 1; its state changes when VTRX is at 0. This bit is read-only. Host mode This bit is set by the software to send a SETUP transaction on a control endpoint. This bit changes its value only for control endpoints. It is cleared by hardware when the SETUP transaction is acknowledged and VTTX interrupt generated.
pub type SETUP_R = crate::BitReader;
/**Status bits, for reception transfers Device mode These bits contain information about the endpoint status, which are listed in Reception status encoding on page2000.These bits can be toggled by software to initialize their value. When the application software writes '0, the value remains unchanged, while writing '1 makes the bit value toggle. Hardware sets the STRX bits to NAK when a correct transfer has occurred (VTRX=1) corresponding to a OUT or SETUP (control only) transaction addressed to this endpoint, so the software has the time to elaborate the received data before it acknowledge a new transaction Double-buffered bulk endpoints implement a special transaction flow control, which control the status based upon buffer availability condition (Refer to endpoints). If the endpoint is defined as Isochronous, its status can be only 'VALID' or 'DISABLED', so that the hardware cannot change the status of the endpoint after a successful transaction. If the software sets the STRX bits to 'STALL' or 'NAK' for an Isochronous endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing '1. Host mode These bits are the host application controls to start, retry, or abort host transactions driven by the channel. These bits also contain information about the device answer to the last IN channel transaction and report the current status of the channel according to the following STRX table of states: - DISABLE DISABLE value is reported in case of ACK acknowledge is received on a single-buffer channel. When in DISABLE state the channel is unused or not active waiting for application to restart it by writing VALID. Application can reset a VALID channel to DISABLE to abort a transaction. In this case the transaction is immediately removed from the Host execution list. If the aborted transaction was already under execution it will be regularly terminated on the USB but the relative VTRX interrupt is not generated. - VALID An Host channel is actively trying to submit USB transaction to device only when in VALID state.VALID state can be set by software or automatically by hardware on a NAKED channel at the start of a new frame. When set to VALID, an host channel enters the host execution queue and waits permission from the Host Frame Schedure to submit its configured transaction. VALID value is also reported in case of ACK acknowledge is received on a double-buffered channel. In this case the channel remains active on the alternate buffer while application needs to read the current buffer and toggle DTOGTX. In case software is late in reading and the alternate buffer is not ready, the host channel is automatically suspended transparently to the application. The suspended double buffered channel will be re-activated as soon as delay is recovered and DTOGTX is toggled. - NAK NAK value is reported in case of NAK acknowledge received. When in NAK state the channel is suspended and does not try to transmit. NAK state is moved to VALID by hardware at the start of the next frame, or software can change it to immediately retry transmission by writing it to VALID, or can disable it and abort the transaction by writing DISABLE - STALL STALL value is reported in case of STALL acknowledge received. When in STALL state the channel behaves as disabled. Application should not retry transmission but reset the USB and re-enumerate.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATRXR {
    ///0: All reception requests addressed to this endpoint/channel are ignored.
    Disabled = 0,
    ///1: Device mode: the endpoint is stalled and all reception requests result in a STALL handshake. Host mode: this indicates that the device has STALLed the channel.
    Stall = 1,
    ///2: Device mode: the endpoint is NAKed and all reception requests result in a NAK handshake. Host mode: this indicates that the device has NAKed the reception request.
    Nak = 2,
    ///3: This endpoint/channel is enabled for reception.
    Valid = 3,
}
impl From<STATRXR> for u8 {
    #[inline(always)]
    fn from(variant: STATRXR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STATRXR {
    type Ux = u8;
}
impl crate::IsEnum for STATRXR {}
///Field `STATRX` reader - Status bits, for reception transfers Device mode These bits contain information about the endpoint status, which are listed in Reception status encoding on page2000.These bits can be toggled by software to initialize their value. When the application software writes '0, the value remains unchanged, while writing '1 makes the bit value toggle. Hardware sets the STRX bits to NAK when a correct transfer has occurred (VTRX=1) corresponding to a OUT or SETUP (control only) transaction addressed to this endpoint, so the software has the time to elaborate the received data before it acknowledge a new transaction Double-buffered bulk endpoints implement a special transaction flow control, which control the status based upon buffer availability condition (Refer to endpoints). If the endpoint is defined as Isochronous, its status can be only 'VALID' or 'DISABLED', so that the hardware cannot change the status of the endpoint after a successful transaction. If the software sets the STRX bits to 'STALL' or 'NAK' for an Isochronous endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing '1. Host mode These bits are the host application controls to start, retry, or abort host transactions driven by the channel. These bits also contain information about the device answer to the last IN channel transaction and report the current status of the channel according to the following STRX table of states: - DISABLE DISABLE value is reported in case of ACK acknowledge is received on a single-buffer channel. When in DISABLE state the channel is unused or not active waiting for application to restart it by writing VALID. Application can reset a VALID channel to DISABLE to abort a transaction. In this case the transaction is immediately removed from the Host execution list. If the aborted transaction was already under execution it will be regularly terminated on the USB but the relative VTRX interrupt is not generated. - VALID An Host channel is actively trying to submit USB transaction to device only when in VALID state.VALID state can be set by software or automatically by hardware on a NAKED channel at the start of a new frame. When set to VALID, an host channel enters the host execution queue and waits permission from the Host Frame Schedure to submit its configured transaction. VALID value is also reported in case of ACK acknowledge is received on a double-buffered channel. In this case the channel remains active on the alternate buffer while application needs to read the current buffer and toggle DTOGTX. In case software is late in reading and the alternate buffer is not ready, the host channel is automatically suspended transparently to the application. The suspended double buffered channel will be re-activated as soon as delay is recovered and DTOGTX is toggled. - NAK NAK value is reported in case of NAK acknowledge received. When in NAK state the channel is suspended and does not try to transmit. NAK state is moved to VALID by hardware at the start of the next frame, or software can change it to immediately retry transmission by writing it to VALID, or can disable it and abort the transaction by writing DISABLE - STALL STALL value is reported in case of STALL acknowledge received. When in STALL state the channel behaves as disabled. Application should not retry transmission but reset the USB and re-enumerate.
pub type STATRX_R = crate::FieldReader<STATRXR>;
impl STATRX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STATRXR {
        match self.bits {
            0 => STATRXR::Disabled,
            1 => STATRXR::Stall,
            2 => STATRXR::Nak,
            3 => STATRXR::Valid,
            _ => unreachable!(),
        }
    }
    ///All reception requests addressed to this endpoint/channel are ignored.
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STATRXR::Disabled
    }
    ///Device mode: the endpoint is stalled and all reception requests result in a STALL handshake. Host mode: this indicates that the device has STALLed the channel.
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == STATRXR::Stall
    }
    ///Device mode: the endpoint is NAKed and all reception requests result in a NAK handshake. Host mode: this indicates that the device has NAKed the reception request.
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == STATRXR::Nak
    }
    ///This endpoint/channel is enabled for reception.
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == STATRXR::Valid
    }
}
/**Status bits, for reception transfers Device mode These bits contain information about the endpoint status, which are listed in Reception status encoding on page2000.These bits can be toggled by software to initialize their value. When the application software writes '0, the value remains unchanged, while writing '1 makes the bit value toggle. Hardware sets the STRX bits to NAK when a correct transfer has occurred (VTRX=1) corresponding to a OUT or SETUP (control only) transaction addressed to this endpoint, so the software has the time to elaborate the received data before it acknowledge a new transaction Double-buffered bulk endpoints implement a special transaction flow control, which control the status based upon buffer availability condition (Refer to endpoints). If the endpoint is defined as Isochronous, its status can be only 'VALID' or 'DISABLED', so that the hardware cannot change the status of the endpoint after a successful transaction. If the software sets the STRX bits to 'STALL' or 'NAK' for an Isochronous endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing '1. Host mode These bits are the host application controls to start, retry, or abort host transactions driven by the channel. These bits also contain information about the device answer to the last IN channel transaction and report the current status of the channel according to the following STRX table of states: - DISABLE DISABLE value is reported in case of ACK acknowledge is received on a single-buffer channel. When in DISABLE state the channel is unused or not active waiting for application to restart it by writing VALID. Application can reset a VALID channel to DISABLE to abort a transaction. In this case the transaction is immediately removed from the Host execution list. If the aborted transaction was already under execution it will be regularly terminated on the USB but the relative VTRX interrupt is not generated. - VALID An Host channel is actively trying to submit USB transaction to device only when in VALID state.VALID state can be set by software or automatically by hardware on a NAKED channel at the start of a new frame. When set to VALID, an host channel enters the host execution queue and waits permission from the Host Frame Schedure to submit its configured transaction. VALID value is also reported in case of ACK acknowledge is received on a double-buffered channel. In this case the channel remains active on the alternate buffer while application needs to read the current buffer and toggle DTOGTX. In case software is late in reading and the alternate buffer is not ready, the host channel is automatically suspended transparently to the application. The suspended double buffered channel will be re-activated as soon as delay is recovered and DTOGTX is toggled. - NAK NAK value is reported in case of NAK acknowledge received. When in NAK state the channel is suspended and does not try to transmit. NAK state is moved to VALID by hardware at the start of the next frame, or software can change it to immediately retry transmission by writing it to VALID, or can disable it and abort the transaction by writing DISABLE - STALL STALL value is reported in case of STALL acknowledge received. When in STALL state the channel behaves as disabled. Application should not retry transmission but reset the USB and re-enumerate.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATRXW {
    ///0: Do not change bits
    Keep = 0,
}
impl From<STATRXW> for u8 {
    #[inline(always)]
    fn from(variant: STATRXW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STATRXW {
    type Ux = u8;
}
impl crate::IsEnum for STATRXW {}
///Field `STATRX` writer - Status bits, for reception transfers Device mode These bits contain information about the endpoint status, which are listed in Reception status encoding on page2000.These bits can be toggled by software to initialize their value. When the application software writes '0, the value remains unchanged, while writing '1 makes the bit value toggle. Hardware sets the STRX bits to NAK when a correct transfer has occurred (VTRX=1) corresponding to a OUT or SETUP (control only) transaction addressed to this endpoint, so the software has the time to elaborate the received data before it acknowledge a new transaction Double-buffered bulk endpoints implement a special transaction flow control, which control the status based upon buffer availability condition (Refer to endpoints). If the endpoint is defined as Isochronous, its status can be only 'VALID' or 'DISABLED', so that the hardware cannot change the status of the endpoint after a successful transaction. If the software sets the STRX bits to 'STALL' or 'NAK' for an Isochronous endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing '1. Host mode These bits are the host application controls to start, retry, or abort host transactions driven by the channel. These bits also contain information about the device answer to the last IN channel transaction and report the current status of the channel according to the following STRX table of states: - DISABLE DISABLE value is reported in case of ACK acknowledge is received on a single-buffer channel. When in DISABLE state the channel is unused or not active waiting for application to restart it by writing VALID. Application can reset a VALID channel to DISABLE to abort a transaction. In this case the transaction is immediately removed from the Host execution list. If the aborted transaction was already under execution it will be regularly terminated on the USB but the relative VTRX interrupt is not generated. - VALID An Host channel is actively trying to submit USB transaction to device only when in VALID state.VALID state can be set by software or automatically by hardware on a NAKED channel at the start of a new frame. When set to VALID, an host channel enters the host execution queue and waits permission from the Host Frame Schedure to submit its configured transaction. VALID value is also reported in case of ACK acknowledge is received on a double-buffered channel. In this case the channel remains active on the alternate buffer while application needs to read the current buffer and toggle DTOGTX. In case software is late in reading and the alternate buffer is not ready, the host channel is automatically suspended transparently to the application. The suspended double buffered channel will be re-activated as soon as delay is recovered and DTOGTX is toggled. - NAK NAK value is reported in case of NAK acknowledge received. When in NAK state the channel is suspended and does not try to transmit. NAK state is moved to VALID by hardware at the start of the next frame, or software can change it to immediately retry transmission by writing it to VALID, or can disable it and abort the transaction by writing DISABLE - STALL STALL value is reported in case of STALL acknowledge received. When in STALL state the channel behaves as disabled. Application should not retry transmission but reset the USB and re-enumerate.
pub type STATRX_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STATRXW>;
impl<'a, REG> STATRX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Do not change bits
    #[inline(always)]
    pub fn keep(self) -> &'a mut crate::W<REG> {
        self.variant(STATRXW::Keep)
    }
}
/**Data Toggle, for reception transfers If the endpoint/channel is not Isochronous, this bit contains the expected value of the data toggle bit (0=DATA0, 1=DATA1) for the next data packet to be received. Hardware toggles this bit, when the ACK handshake is sent following a data packet reception having a matching data PID value; if the endpoint is defined as a control one, hardware clears this bit at the reception of a SETUP PID received from host (in device) or acknowledged by device (in host). If the endpoint/channel is using the double-buffering feature this bit is used to support packet buffer swapping too (Refer to ). If the endpoint/channel is Isochronous, this bit is used only to support packet buffer swapping for data transmission since no data toggling is used for this kind of channels/endpoints and only DATA0 packet are transmitted (Refer to Isochronous transfers). Hardware toggles this bit just after the end of data packet reception, since no handshake is used for isochronous transfers. This bit can also be toggled by the software to initialize its value (mandatory when the endpoint is not a control one) or to force specific data toggle/packet buffer usage. When the application software writes '0, the value of DTOGRX remains unchanged, while writing '1 makes the bit value toggle. This bit is read/write but it can be only toggled by writing 1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTOGRXW {
    ///1: Flip bit
    Toggle = 1,
}
impl From<DTOGRXW> for bool {
    #[inline(always)]
    fn from(variant: DTOGRXW) -> Self {
        variant as u8 != 0
    }
}
///Field `DTOGRX` writer - Data Toggle, for reception transfers If the endpoint/channel is not Isochronous, this bit contains the expected value of the data toggle bit (0=DATA0, 1=DATA1) for the next data packet to be received. Hardware toggles this bit, when the ACK handshake is sent following a data packet reception having a matching data PID value; if the endpoint is defined as a control one, hardware clears this bit at the reception of a SETUP PID received from host (in device) or acknowledged by device (in host). If the endpoint/channel is using the double-buffering feature this bit is used to support packet buffer swapping too (Refer to ). If the endpoint/channel is Isochronous, this bit is used only to support packet buffer swapping for data transmission since no data toggling is used for this kind of channels/endpoints and only DATA0 packet are transmitted (Refer to Isochronous transfers). Hardware toggles this bit just after the end of data packet reception, since no handshake is used for isochronous transfers. This bit can also be toggled by the software to initialize its value (mandatory when the endpoint is not a control one) or to force specific data toggle/packet buffer usage. When the application software writes '0, the value of DTOGRX remains unchanged, while writing '1 makes the bit value toggle. This bit is read/write but it can be only toggled by writing 1.
pub type DTOGRX_W<'a, REG> = crate::BitWriter1T<'a, REG, DTOGRXW>;
impl<'a, REG> DTOGRX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flip bit
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(DTOGRXW::Toggle)
    }
}
/**USB valid transaction received Device mode This bit is set by the hardware when an OUT/SETUP transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. The type of occurred transaction, OUT or SETUP, can be determined from the SETUP bit described below. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only '0 can be written, writing 1 has no effect. Host mode This bit is set by the hardware when an IN transaction is successfully completed on this channel. The software can only clear this bit. If the VTRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated. - A transaction ended with a NAK sets this bit and NAK answer is reported to application reading the NAK state from the STRX field of this register. One naked transaction keeps pending and is automatically retried by the Host at the next frame, or the Host can immediately retry by resetting STRX state to VALID. - A transaction ended by STALL handshake sets this bit and the STALL answer is reported to application reading the STALL state from the STRX field of this register. Host application should consequently disable the channel and re-enumerate. - A transaction ended with ACK handshake sets this bit If double buffering is disabled, ACK answer is reported by application reading the DISABLE state from the STRX field of this register. Host application should read received data from USBRAM and re-arm the channel by writing VALID to the STRX field of this register. If double buffering is enabled, ACK answer is reported by application reading VALID state from the STRX field of this register. Host application should read received data from USBRAM and toggle the DTOGTX bit of this register. This bit is read/write but only '0 can be written, writing 1 has no effect.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VTRXW {
    ///0: Clear flag
    Clear = 0,
}
impl From<VTRXW> for bool {
    #[inline(always)]
    fn from(variant: VTRXW) -> Self {
        variant as u8 != 0
    }
}
///Field `VTRX` reader - USB valid transaction received Device mode This bit is set by the hardware when an OUT/SETUP transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. The type of occurred transaction, OUT or SETUP, can be determined from the SETUP bit described below. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only '0 can be written, writing 1 has no effect. Host mode This bit is set by the hardware when an IN transaction is successfully completed on this channel. The software can only clear this bit. If the VTRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated. - A transaction ended with a NAK sets this bit and NAK answer is reported to application reading the NAK state from the STRX field of this register. One naked transaction keeps pending and is automatically retried by the Host at the next frame, or the Host can immediately retry by resetting STRX state to VALID. - A transaction ended by STALL handshake sets this bit and the STALL answer is reported to application reading the STALL state from the STRX field of this register. Host application should consequently disable the channel and re-enumerate. - A transaction ended with ACK handshake sets this bit If double buffering is disabled, ACK answer is reported by application reading the DISABLE state from the STRX field of this register. Host application should read received data from USBRAM and re-arm the channel by writing VALID to the STRX field of this register. If double buffering is enabled, ACK answer is reported by application reading VALID state from the STRX field of this register. Host application should read received data from USBRAM and toggle the DTOGTX bit of this register. This bit is read/write but only '0 can be written, writing 1 has no effect.
pub type VTRX_R = crate::BitReader<VTRXW>;
impl VTRX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<VTRXW> {
        match self.bits {
            false => Some(VTRXW::Clear),
            _ => None,
        }
    }
    ///Clear flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == VTRXW::Clear
    }
}
///Field `VTRX` writer - USB valid transaction received Device mode This bit is set by the hardware when an OUT/SETUP transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. The type of occurred transaction, OUT or SETUP, can be determined from the SETUP bit described below. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only '0 can be written, writing 1 has no effect. Host mode This bit is set by the hardware when an IN transaction is successfully completed on this channel. The software can only clear this bit. If the VTRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated. - A transaction ended with a NAK sets this bit and NAK answer is reported to application reading the NAK state from the STRX field of this register. One naked transaction keeps pending and is automatically retried by the Host at the next frame, or the Host can immediately retry by resetting STRX state to VALID. - A transaction ended by STALL handshake sets this bit and the STALL answer is reported to application reading the STALL state from the STRX field of this register. Host application should consequently disable the channel and re-enumerate. - A transaction ended with ACK handshake sets this bit If double buffering is disabled, ACK answer is reported by application reading the DISABLE state from the STRX field of this register. Host application should read received data from USBRAM and re-arm the channel by writing VALID to the STRX field of this register. If double buffering is enabled, ACK answer is reported by application reading VALID state from the STRX field of this register. Host application should read received data from USBRAM and toggle the DTOGTX bit of this register. This bit is read/write but only '0 can be written, writing 1 has no effect.
pub type VTRX_W<'a, REG> = crate::BitWriter0C<'a, REG, VTRXW>;
impl<'a, REG> VTRX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(VTRXW::Clear)
    }
}
///Field `DEVADDR` reader - Host mode Device address assigned to the endpoint during the enumeration process.
pub type DEVADDR_R = crate::FieldReader;
///Field `DEVADDR` writer - Host mode Device address assigned to the endpoint during the enumeration process.
pub type DEVADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**Host mode This bit is set by the hardware when a device responds with a NAK. Software can be use this bit to monitoring the number of NAKs received from a device.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NAKW {
    ///0: Clear flag
    Clear = 0,
}
impl From<NAKW> for bool {
    #[inline(always)]
    fn from(variant: NAKW) -> Self {
        variant as u8 != 0
    }
}
///Field `NAK` reader - Host mode This bit is set by the hardware when a device responds with a NAK. Software can be use this bit to monitoring the number of NAKs received from a device.
pub type NAK_R = crate::BitReader<NAKW>;
impl NAK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<NAKW> {
        match self.bits {
            false => Some(NAKW::Clear),
            _ => None,
        }
    }
    ///Clear flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == NAKW::Clear
    }
}
///Field `NAK` writer - Host mode This bit is set by the hardware when a device responds with a NAK. Software can be use this bit to monitoring the number of NAKs received from a device.
pub type NAK_W<'a, REG> = crate::BitWriter0C<'a, REG, NAKW>;
impl<'a, REG> NAK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(NAKW::Clear)
    }
}
///Field `LS_EP` reader - Low speed endpoint Host with HUB only Host mode This bit is set by the software to send an LS transaction to the corresponding endpoint.
pub type LS_EP_R = crate::BitReader;
///Field `LS_EP` writer - Low speed endpoint Host with HUB only Host mode This bit is set by the software to send an LS transaction to the corresponding endpoint.
pub type LS_EP_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Transmit error Host mode This bit is set by the hardware when an error (e.g. no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an OUT or SETUP transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_TXW {
    ///0: Clear flag
    Clear = 0,
}
impl From<ERR_TXW> for bool {
    #[inline(always)]
    fn from(variant: ERR_TXW) -> Self {
        variant as u8 != 0
    }
}
///Field `ERR_TX` reader - Transmit error Host mode This bit is set by the hardware when an error (e.g. no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an OUT or SETUP transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated.
pub type ERR_TX_R = crate::BitReader<ERR_TXW>;
impl ERR_TX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ERR_TXW> {
        match self.bits {
            false => Some(ERR_TXW::Clear),
            _ => None,
        }
    }
    ///Clear flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ERR_TXW::Clear
    }
}
///Field `ERR_TX` writer - Transmit error Host mode This bit is set by the hardware when an error (e.g. no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an OUT or SETUP transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated.
pub type ERR_TX_W<'a, REG> = crate::BitWriter0C<'a, REG, ERR_TXW>;
impl<'a, REG> ERR_TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_TXW::Clear)
    }
}
/**Receive error Host mode This bit is set by the hardware when an error (e.g. no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an IN transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_RXW {
    ///0: Clear flag
    Clear = 0,
}
impl From<ERR_RXW> for bool {
    #[inline(always)]
    fn from(variant: ERR_RXW) -> Self {
        variant as u8 != 0
    }
}
///Field `ERR_RX` reader - Receive error Host mode This bit is set by the hardware when an error (e.g. no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an IN transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated.
pub type ERR_RX_R = crate::BitReader<ERR_RXW>;
impl ERR_RX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ERR_RXW> {
        match self.bits {
            false => Some(ERR_RXW::Clear),
            _ => None,
        }
    }
    ///Clear flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ERR_RXW::Clear
    }
}
///Field `ERR_RX` writer - Receive error Host mode This bit is set by the hardware when an error (e.g. no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an IN transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated.
pub type ERR_RX_W<'a, REG> = crate::BitWriter0C<'a, REG, ERR_RXW>;
impl<'a, REG> ERR_RX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_RXW::Clear)
    }
}
impl R {
    ///Bits 0:3 - endpoint/channel address Device mode Software must write in this field the 4-bit address used to identify the transactions directed to this endpoint. A value must be written before enabling the corresponding endpoint. Host mode Software must write in this field the 4-bit address used to identify the channel addressed by the host transaction.
    #[inline(always)]
    pub fn ea(&self) -> EA_R {
        EA_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - Status bits, for transmission transfers Device mode These bits contain the information about the endpoint status, listed in . These bits can be toggled by the software to initialize their value. When the application software writes '0, the value remains unchanged, while writing '1 makes the bit value toggle. Hardware sets the STTX bits to NAK, when a correct transfer has occurred (VTTX=1) corresponding to a IN or SETUP (control only) transaction addressed to this channel/endpoint. It then waits for the software to prepare the next set of data to be transmitted. Double-buffered bulk endpoints implement a special transaction flow control, which controls the status based on buffer availability condition (Refer to endpoints). If the endpoint is defined as Isochronous, its status can only be 'VALID' or 'DISABLED'. Therefore, the hardware cannot change the status of the channel/endpoint/channel after a successful transaction. If the software sets the STTX bits to 'STALL' or 'NAK' for an Isochronous channel/endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing '1. Host mode Same as STRX behaviour but for IN transactions (TBC)
    #[inline(always)]
    pub fn stattx(&self) -> STATTX_R {
        STATTX_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - Valid USB transaction transmitted Device mode This bit is set by the hardware when an IN transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in the USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only '0 can be written. Host mode Same of VTRX behaviour but for USB OUT and SETUP transactions.
    #[inline(always)]
    pub fn vttx(&self) -> VTTX_R {
        VTTX_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - endpoint/channel kind The meaning of this bit depends on the endpoint/channel type configured by the EP_TYPE bits. summarizes the different meanings. DBL_BUF: This bit is set by the software to enable the double-buffering feature for this bulk endpoint. The usage of double-buffered bulk endpoints is explained in Double-buffered endpoints. STATUS_OUT: This bit is set by the software to indicate that a status out transaction is expected: in this case all OUT transactions containing more than zero data bytes are answered 'STALL' instead of 'ACK'. This bit may be used to improve the robustness of the application to protocol errors during control transfers and its usage is intended for control endpoints only. When STATUS_OUT is reset, OUT transactions can have any number of bytes, as required.
    #[inline(always)]
    pub fn epkind(&self) -> EPKIND_R {
        EPKIND_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - USB type of transaction These bits configure the behavior of this endpoint/channel as described in endpoint/channel type encoding on page2001. Channel0/Endpoint0 must always be a control endpoint/channel and each USB function must have at least one control endpoint/channel which has address 0, but there may be other control channels/endpoints if required. Only control channels/endpoints handle SETUP transactions, which are ignored by endpoints of other kinds. SETUP transactions cannot be answered with NAK or STALL. If a control endpoint/channel is defined as NAK, the USB peripheral will not answer, simulating a receive error, in the receive direction when a SETUP transaction is received. If the control endpoint/channel is defined as STALL in the receive direction, then the SETUP packet will be accepted anyway, transferring data and issuing the CTR interrupt. The reception of OUT transactions is handled in the normal way, even if the endpoint/channel is a control one. Bulk and interrupt endpoints have very similar behavior and they differ only in the special feature available using the EPKIND configuration bit. The usage of Isochronous channels/endpoints is explained in transfers
    #[inline(always)]
    pub fn utype(&self) -> UTYPE_R {
        UTYPE_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 11 - Setup transaction completed Device mode This bit is read-only and it is set by the hardware when the last completed transaction is a SETUP. This bit changes its value only for control endpoints. It must be examined, in the case of a successful receive transaction (VTRX event), to determine the type of transaction occurred. To protect the interrupt service routine from the changes in SETUP bits due to next incoming tokens, this bit is kept frozen while VTRX bit is at 1; its state changes when VTRX is at 0. This bit is read-only. Host mode This bit is set by the software to send a SETUP transaction on a control endpoint. This bit changes its value only for control endpoints. It is cleared by hardware when the SETUP transaction is acknowledged and VTTX interrupt generated.
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Status bits, for reception transfers Device mode These bits contain information about the endpoint status, which are listed in Reception status encoding on page2000.These bits can be toggled by software to initialize their value. When the application software writes '0, the value remains unchanged, while writing '1 makes the bit value toggle. Hardware sets the STRX bits to NAK when a correct transfer has occurred (VTRX=1) corresponding to a OUT or SETUP (control only) transaction addressed to this endpoint, so the software has the time to elaborate the received data before it acknowledge a new transaction Double-buffered bulk endpoints implement a special transaction flow control, which control the status based upon buffer availability condition (Refer to endpoints). If the endpoint is defined as Isochronous, its status can be only 'VALID' or 'DISABLED', so that the hardware cannot change the status of the endpoint after a successful transaction. If the software sets the STRX bits to 'STALL' or 'NAK' for an Isochronous endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing '1. Host mode These bits are the host application controls to start, retry, or abort host transactions driven by the channel. These bits also contain information about the device answer to the last IN channel transaction and report the current status of the channel according to the following STRX table of states: - DISABLE DISABLE value is reported in case of ACK acknowledge is received on a single-buffer channel. When in DISABLE state the channel is unused or not active waiting for application to restart it by writing VALID. Application can reset a VALID channel to DISABLE to abort a transaction. In this case the transaction is immediately removed from the Host execution list. If the aborted transaction was already under execution it will be regularly terminated on the USB but the relative VTRX interrupt is not generated. - VALID An Host channel is actively trying to submit USB transaction to device only when in VALID state.VALID state can be set by software or automatically by hardware on a NAKED channel at the start of a new frame. When set to VALID, an host channel enters the host execution queue and waits permission from the Host Frame Schedure to submit its configured transaction. VALID value is also reported in case of ACK acknowledge is received on a double-buffered channel. In this case the channel remains active on the alternate buffer while application needs to read the current buffer and toggle DTOGTX. In case software is late in reading and the alternate buffer is not ready, the host channel is automatically suspended transparently to the application. The suspended double buffered channel will be re-activated as soon as delay is recovered and DTOGTX is toggled. - NAK NAK value is reported in case of NAK acknowledge received. When in NAK state the channel is suspended and does not try to transmit. NAK state is moved to VALID by hardware at the start of the next frame, or software can change it to immediately retry transmission by writing it to VALID, or can disable it and abort the transaction by writing DISABLE - STALL STALL value is reported in case of STALL acknowledge received. When in STALL state the channel behaves as disabled. Application should not retry transmission but reset the USB and re-enumerate.
    #[inline(always)]
    pub fn statrx(&self) -> STATRX_R {
        STATRX_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 15 - USB valid transaction received Device mode This bit is set by the hardware when an OUT/SETUP transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. The type of occurred transaction, OUT or SETUP, can be determined from the SETUP bit described below. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only '0 can be written, writing 1 has no effect. Host mode This bit is set by the hardware when an IN transaction is successfully completed on this channel. The software can only clear this bit. If the VTRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated. - A transaction ended with a NAK sets this bit and NAK answer is reported to application reading the NAK state from the STRX field of this register. One naked transaction keeps pending and is automatically retried by the Host at the next frame, or the Host can immediately retry by resetting STRX state to VALID. - A transaction ended by STALL handshake sets this bit and the STALL answer is reported to application reading the STALL state from the STRX field of this register. Host application should consequently disable the channel and re-enumerate. - A transaction ended with ACK handshake sets this bit If double buffering is disabled, ACK answer is reported by application reading the DISABLE state from the STRX field of this register. Host application should read received data from USBRAM and re-arm the channel by writing VALID to the STRX field of this register. If double buffering is enabled, ACK answer is reported by application reading VALID state from the STRX field of this register. Host application should read received data from USBRAM and toggle the DTOGTX bit of this register. This bit is read/write but only '0 can be written, writing 1 has no effect.
    #[inline(always)]
    pub fn vtrx(&self) -> VTRX_R {
        VTRX_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:22 - Host mode Device address assigned to the endpoint during the enumeration process.
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 23 - Host mode This bit is set by the hardware when a device responds with a NAK. Software can be use this bit to monitoring the number of NAKs received from a device.
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Low speed endpoint Host with HUB only Host mode This bit is set by the software to send an LS transaction to the corresponding endpoint.
    #[inline(always)]
    pub fn ls_ep(&self) -> LS_EP_R {
        LS_EP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Transmit error Host mode This bit is set by the hardware when an error (e.g. no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an OUT or SETUP transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated.
    #[inline(always)]
    pub fn err_tx(&self) -> ERR_TX_R {
        ERR_TX_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Receive error Host mode This bit is set by the hardware when an error (e.g. no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an IN transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated.
    #[inline(always)]
    pub fn err_rx(&self) -> ERR_RX_R {
        ERR_RX_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHEPR")
            .field("ea", &self.ea())
            .field("stattx", &self.stattx())
            .field("vttx", &self.vttx())
            .field("epkind", &self.epkind())
            .field("utype", &self.utype())
            .field("setup", &self.setup())
            .field("statrx", &self.statrx())
            .field("vtrx", &self.vtrx())
            .field("devaddr", &self.devaddr())
            .field("nak", &self.nak())
            .field("ls_ep", &self.ls_ep())
            .field("err_tx", &self.err_tx())
            .field("err_rx", &self.err_rx())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - endpoint/channel address Device mode Software must write in this field the 4-bit address used to identify the transactions directed to this endpoint. A value must be written before enabling the corresponding endpoint. Host mode Software must write in this field the 4-bit address used to identify the channel addressed by the host transaction.
    #[inline(always)]
    pub fn ea(&mut self) -> EA_W<'_, CHEPRrs> {
        EA_W::new(self, 0)
    }
    ///Bits 4:5 - Status bits, for transmission transfers Device mode These bits contain the information about the endpoint status, listed in . These bits can be toggled by the software to initialize their value. When the application software writes '0, the value remains unchanged, while writing '1 makes the bit value toggle. Hardware sets the STTX bits to NAK, when a correct transfer has occurred (VTTX=1) corresponding to a IN or SETUP (control only) transaction addressed to this channel/endpoint. It then waits for the software to prepare the next set of data to be transmitted. Double-buffered bulk endpoints implement a special transaction flow control, which controls the status based on buffer availability condition (Refer to endpoints). If the endpoint is defined as Isochronous, its status can only be 'VALID' or 'DISABLED'. Therefore, the hardware cannot change the status of the channel/endpoint/channel after a successful transaction. If the software sets the STTX bits to 'STALL' or 'NAK' for an Isochronous channel/endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing '1. Host mode Same as STRX behaviour but for IN transactions (TBC)
    #[inline(always)]
    pub fn stattx(&mut self) -> STATTX_W<'_, CHEPRrs> {
        STATTX_W::new(self, 4)
    }
    ///Bit 6 - Data Toggle, for transmission transfers If the endpoint/channel is non-isochronous, this bit contains the required value of the data toggle bit (0=DATA0, 1=DATA1) for the next data packet to be transmitted. Hardware toggles this bit when the ACK handshake is received from the USB host, following a data packet transmission. If the endpoint/channel is defined as a control one, hardware sets this bit to 1 at the reception of a SETUP PID addressed to this endpoint. If the endpoint/channel is using the double buffer feature, this bit is used to support packet buffer swapping too (Refer to ) If the endpoint/channel is Isochronous, this bit is used to support packet buffer swapping since no data toggling is used for this sort of endpoints and only DATA0 packet are transmitted (Refer to ). Hardware toggles this bit just after the end of data packet transmission, since no handshake is used for Isochronous transfers. This bit can also be toggled by the software to initialize its value (mandatory when the endpoint/channel is not a control one) or to force a specific data toggle/packet buffer usage. When the application software writes '0, the value of DTOGTX remains unchanged, while writing '1 makes the bit value toggle. This bit is read/write but it can only be toggled by writing 1.
    #[inline(always)]
    pub fn dtogtx(&mut self) -> DTOGTX_W<'_, CHEPRrs> {
        DTOGTX_W::new(self, 6)
    }
    ///Bit 7 - Valid USB transaction transmitted Device mode This bit is set by the hardware when an IN transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in the USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only '0 can be written. Host mode Same of VTRX behaviour but for USB OUT and SETUP transactions.
    #[inline(always)]
    pub fn vttx(&mut self) -> VTTX_W<'_, CHEPRrs> {
        VTTX_W::new(self, 7)
    }
    ///Bit 8 - endpoint/channel kind The meaning of this bit depends on the endpoint/channel type configured by the EP_TYPE bits. summarizes the different meanings. DBL_BUF: This bit is set by the software to enable the double-buffering feature for this bulk endpoint. The usage of double-buffered bulk endpoints is explained in Double-buffered endpoints. STATUS_OUT: This bit is set by the software to indicate that a status out transaction is expected: in this case all OUT transactions containing more than zero data bytes are answered 'STALL' instead of 'ACK'. This bit may be used to improve the robustness of the application to protocol errors during control transfers and its usage is intended for control endpoints only. When STATUS_OUT is reset, OUT transactions can have any number of bytes, as required.
    #[inline(always)]
    pub fn epkind(&mut self) -> EPKIND_W<'_, CHEPRrs> {
        EPKIND_W::new(self, 8)
    }
    ///Bits 9:10 - USB type of transaction These bits configure the behavior of this endpoint/channel as described in endpoint/channel type encoding on page2001. Channel0/Endpoint0 must always be a control endpoint/channel and each USB function must have at least one control endpoint/channel which has address 0, but there may be other control channels/endpoints if required. Only control channels/endpoints handle SETUP transactions, which are ignored by endpoints of other kinds. SETUP transactions cannot be answered with NAK or STALL. If a control endpoint/channel is defined as NAK, the USB peripheral will not answer, simulating a receive error, in the receive direction when a SETUP transaction is received. If the control endpoint/channel is defined as STALL in the receive direction, then the SETUP packet will be accepted anyway, transferring data and issuing the CTR interrupt. The reception of OUT transactions is handled in the normal way, even if the endpoint/channel is a control one. Bulk and interrupt endpoints have very similar behavior and they differ only in the special feature available using the EPKIND configuration bit. The usage of Isochronous channels/endpoints is explained in transfers
    #[inline(always)]
    pub fn utype(&mut self) -> UTYPE_W<'_, CHEPRrs> {
        UTYPE_W::new(self, 9)
    }
    ///Bits 12:13 - Status bits, for reception transfers Device mode These bits contain information about the endpoint status, which are listed in Reception status encoding on page2000.These bits can be toggled by software to initialize their value. When the application software writes '0, the value remains unchanged, while writing '1 makes the bit value toggle. Hardware sets the STRX bits to NAK when a correct transfer has occurred (VTRX=1) corresponding to a OUT or SETUP (control only) transaction addressed to this endpoint, so the software has the time to elaborate the received data before it acknowledge a new transaction Double-buffered bulk endpoints implement a special transaction flow control, which control the status based upon buffer availability condition (Refer to endpoints). If the endpoint is defined as Isochronous, its status can be only 'VALID' or 'DISABLED', so that the hardware cannot change the status of the endpoint after a successful transaction. If the software sets the STRX bits to 'STALL' or 'NAK' for an Isochronous endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing '1. Host mode These bits are the host application controls to start, retry, or abort host transactions driven by the channel. These bits also contain information about the device answer to the last IN channel transaction and report the current status of the channel according to the following STRX table of states: - DISABLE DISABLE value is reported in case of ACK acknowledge is received on a single-buffer channel. When in DISABLE state the channel is unused or not active waiting for application to restart it by writing VALID. Application can reset a VALID channel to DISABLE to abort a transaction. In this case the transaction is immediately removed from the Host execution list. If the aborted transaction was already under execution it will be regularly terminated on the USB but the relative VTRX interrupt is not generated. - VALID An Host channel is actively trying to submit USB transaction to device only when in VALID state.VALID state can be set by software or automatically by hardware on a NAKED channel at the start of a new frame. When set to VALID, an host channel enters the host execution queue and waits permission from the Host Frame Schedure to submit its configured transaction. VALID value is also reported in case of ACK acknowledge is received on a double-buffered channel. In this case the channel remains active on the alternate buffer while application needs to read the current buffer and toggle DTOGTX. In case software is late in reading and the alternate buffer is not ready, the host channel is automatically suspended transparently to the application. The suspended double buffered channel will be re-activated as soon as delay is recovered and DTOGTX is toggled. - NAK NAK value is reported in case of NAK acknowledge received. When in NAK state the channel is suspended and does not try to transmit. NAK state is moved to VALID by hardware at the start of the next frame, or software can change it to immediately retry transmission by writing it to VALID, or can disable it and abort the transaction by writing DISABLE - STALL STALL value is reported in case of STALL acknowledge received. When in STALL state the channel behaves as disabled. Application should not retry transmission but reset the USB and re-enumerate.
    #[inline(always)]
    pub fn statrx(&mut self) -> STATRX_W<'_, CHEPRrs> {
        STATRX_W::new(self, 12)
    }
    ///Bit 14 - Data Toggle, for reception transfers If the endpoint/channel is not Isochronous, this bit contains the expected value of the data toggle bit (0=DATA0, 1=DATA1) for the next data packet to be received. Hardware toggles this bit, when the ACK handshake is sent following a data packet reception having a matching data PID value; if the endpoint is defined as a control one, hardware clears this bit at the reception of a SETUP PID received from host (in device) or acknowledged by device (in host). If the endpoint/channel is using the double-buffering feature this bit is used to support packet buffer swapping too (Refer to ). If the endpoint/channel is Isochronous, this bit is used only to support packet buffer swapping for data transmission since no data toggling is used for this kind of channels/endpoints and only DATA0 packet are transmitted (Refer to Isochronous transfers). Hardware toggles this bit just after the end of data packet reception, since no handshake is used for isochronous transfers. This bit can also be toggled by the software to initialize its value (mandatory when the endpoint is not a control one) or to force specific data toggle/packet buffer usage. When the application software writes '0, the value of DTOGRX remains unchanged, while writing '1 makes the bit value toggle. This bit is read/write but it can be only toggled by writing 1.
    #[inline(always)]
    pub fn dtogrx(&mut self) -> DTOGRX_W<'_, CHEPRrs> {
        DTOGRX_W::new(self, 14)
    }
    ///Bit 15 - USB valid transaction received Device mode This bit is set by the hardware when an OUT/SETUP transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. The type of occurred transaction, OUT or SETUP, can be determined from the SETUP bit described below. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only '0 can be written, writing 1 has no effect. Host mode This bit is set by the hardware when an IN transaction is successfully completed on this channel. The software can only clear this bit. If the VTRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated. - A transaction ended with a NAK sets this bit and NAK answer is reported to application reading the NAK state from the STRX field of this register. One naked transaction keeps pending and is automatically retried by the Host at the next frame, or the Host can immediately retry by resetting STRX state to VALID. - A transaction ended by STALL handshake sets this bit and the STALL answer is reported to application reading the STALL state from the STRX field of this register. Host application should consequently disable the channel and re-enumerate. - A transaction ended with ACK handshake sets this bit If double buffering is disabled, ACK answer is reported by application reading the DISABLE state from the STRX field of this register. Host application should read received data from USBRAM and re-arm the channel by writing VALID to the STRX field of this register. If double buffering is enabled, ACK answer is reported by application reading VALID state from the STRX field of this register. Host application should read received data from USBRAM and toggle the DTOGTX bit of this register. This bit is read/write but only '0 can be written, writing 1 has no effect.
    #[inline(always)]
    pub fn vtrx(&mut self) -> VTRX_W<'_, CHEPRrs> {
        VTRX_W::new(self, 15)
    }
    ///Bits 16:22 - Host mode Device address assigned to the endpoint during the enumeration process.
    #[inline(always)]
    pub fn devaddr(&mut self) -> DEVADDR_W<'_, CHEPRrs> {
        DEVADDR_W::new(self, 16)
    }
    ///Bit 23 - Host mode This bit is set by the hardware when a device responds with a NAK. Software can be use this bit to monitoring the number of NAKs received from a device.
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W<'_, CHEPRrs> {
        NAK_W::new(self, 23)
    }
    ///Bit 24 - Low speed endpoint Host with HUB only Host mode This bit is set by the software to send an LS transaction to the corresponding endpoint.
    #[inline(always)]
    pub fn ls_ep(&mut self) -> LS_EP_W<'_, CHEPRrs> {
        LS_EP_W::new(self, 24)
    }
    ///Bit 25 - Transmit error Host mode This bit is set by the hardware when an error (e.g. no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an OUT or SETUP transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated.
    #[inline(always)]
    pub fn err_tx(&mut self) -> ERR_TX_W<'_, CHEPRrs> {
        ERR_TX_W::new(self, 25)
    }
    ///Bit 26 - Receive error Host mode This bit is set by the hardware when an error (e.g. no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an IN transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated.
    #[inline(always)]
    pub fn err_rx(&mut self) -> ERR_RX_W<'_, CHEPRrs> {
        ERR_RX_W::new(self, 26)
    }
}
/**USB endpoint/channel %s register

You can [`read`](crate::Reg::read) this register and get [`chepr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chepr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#USB:CHEP[0]R)*/
pub struct CHEPRrs;
impl crate::RegisterSpec for CHEPRrs {
    type Ux = u32;
}
///`read()` method returns [`chepr::R`](R) reader structure
impl crate::Readable for CHEPRrs {}
///`write(|w| ..)` method takes [`chepr::W`](W) writer structure
impl crate::Writable for CHEPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0680_8080;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x7070;
}
///`reset()` method sets CHEP%sR to value 0
impl crate::Resettable for CHEPRrs {}
