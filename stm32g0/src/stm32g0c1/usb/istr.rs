///Register `ISTR` reader
pub type R = crate::R<ISTRrs>;
///Register `ISTR` writer
pub type W = crate::W<ISTRrs>;
///Field `IDN` reader - Device Endpoint / Host channel identification number These bits are written by the hardware according to the host channel or device endpoint number, which generated the interrupt request. If several endpoint/channel transactions are pending, the hardware writes the identification number related to the endpoint/channel having the highest priority defined in the following way: Two levels are defined, in order of priority: Isochronous and double-buffered bulk channels/endpoints are considered first and then the others are examined. If more than one endpoint/channel from the same set is requesting an interrupt, the IDN bits in USB_ISTR register are assigned according to the lowest requesting register, CHEP0R having the highest priority followed by CHEP1R and so on. The application software can assign a register to each endpoint/channel according to this priority scheme, so as to order the concurring endpoint/channel requests in a suitable way. These bits are read only.
pub type IDN_R = crate::FieldReader;
/**Direction of transaction This bit is written by the hardware according to the direction of the successful transaction, which generated the interrupt request. If DIR bit=0, VTTX bit is set in the USB_EPnR register related to the interrupting endpoint. The interrupting transaction is of IN type (data transmitted by the USB peripheral to the host PC). If DIR bit=1, VTRX bit or both VTTX/VTRX are set in the USB_EPnR register related to the interrupting endpoint. The interrupting transaction is of OUT type (data received by the USB peripheral from the host PC) or two pending transactions are waiting to be processed. This information can be used by the application software to access the USB_EPnR bits related to the triggering transaction since it represents the direction having the interrupt pending. This bit is read-only.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR {
    ///0: Data transmitted by the USB peripheral to the host PC
    To = 0,
    ///1: Data received by the USB peripheral from the host PC
    From = 1,
}
impl From<DIR> for bool {
    #[inline(always)]
    fn from(variant: DIR) -> Self {
        variant as u8 != 0
    }
}
///Field `DIR` reader - Direction of transaction This bit is written by the hardware according to the direction of the successful transaction, which generated the interrupt request. If DIR bit=0, VTTX bit is set in the USB_EPnR register related to the interrupting endpoint. The interrupting transaction is of IN type (data transmitted by the USB peripheral to the host PC). If DIR bit=1, VTRX bit or both VTTX/VTRX are set in the USB_EPnR register related to the interrupting endpoint. The interrupting transaction is of OUT type (data received by the USB peripheral from the host PC) or two pending transactions are waiting to be processed. This information can be used by the application software to access the USB_EPnR bits related to the triggering transaction since it represents the direction having the interrupt pending. This bit is read-only.
pub type DIR_R = crate::BitReader<DIR>;
impl DIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIR {
        match self.bits {
            false => DIR::To,
            true => DIR::From,
        }
    }
    ///Data transmitted by the USB peripheral to the host PC
    #[inline(always)]
    pub fn is_to(&self) -> bool {
        *self == DIR::To
    }
    ///Data received by the USB peripheral from the host PC
    #[inline(always)]
    pub fn is_from(&self) -> bool {
        *self == DIR::From
    }
}
/**LPM L1 state request This bit is set by the hardware when LPM command to enter the L1 state is successfully received and acknowledged. This bit is read/write but only '0 can be written and writing '1 has no effect.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1REQR {
    ///0: NotL1State
    NotL1state = 0,
    ///1: LPM command to enter the L1 state is successfully received and acknowledged
    L1state = 1,
}
impl From<L1REQR> for bool {
    #[inline(always)]
    fn from(variant: L1REQR) -> Self {
        variant as u8 != 0
    }
}
///Field `L1REQ` reader - LPM L1 state request This bit is set by the hardware when LPM command to enter the L1 state is successfully received and acknowledged. This bit is read/write but only '0 can be written and writing '1 has no effect.
pub type L1REQ_R = crate::BitReader<L1REQR>;
impl L1REQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1REQR {
        match self.bits {
            false => L1REQR::NotL1state,
            true => L1REQR::L1state,
        }
    }
    ///NotL1State
    #[inline(always)]
    pub fn is_not_l1state(&self) -> bool {
        *self == L1REQR::NotL1state
    }
    ///LPM command to enter the L1 state is successfully received and acknowledged
    #[inline(always)]
    pub fn is_l1state(&self) -> bool {
        *self == L1REQR::L1state
    }
}
/**LPM L1 state request This bit is set by the hardware when LPM command to enter the L1 state is successfully received and acknowledged. This bit is read/write but only '0 can be written and writing '1 has no effect.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1REQW {
    ///0: Clear flag
    Clear = 0,
}
impl From<L1REQW> for bool {
    #[inline(always)]
    fn from(variant: L1REQW) -> Self {
        variant as u8 != 0
    }
}
///Field `L1REQ` writer - LPM L1 state request This bit is set by the hardware when LPM command to enter the L1 state is successfully received and acknowledged. This bit is read/write but only '0 can be written and writing '1 has no effect.
pub type L1REQ_W<'a, REG> = crate::BitWriter0C<'a, REG, L1REQW>;
impl<'a, REG> L1REQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(L1REQW::Clear)
    }
}
/**Expected start of frame This bit is set by the hardware when an SOF packet is expected but not received. The host sends an SOF packet each 1ms, but if the device does not receive it properly, the Suspend Timer issues this interrupt. If three consecutive ESOF interrupts are generated (i.e. three SOF packets are lost) without any traffic occurring in between, a SUSP interrupt is generated. This bit is set even when the missing SOF packets occur while the Suspend Timer is not yet locked. This bit is read/write but only '0 can be written and writing '1 has no effect.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESOFR {
    ///0: NotExpectedStartOfFrame
    NotExpectedStartOfFrame = 0,
    ///1: An SOF packet is expected but not received
    ExpectedStartOfFrame = 1,
}
impl From<ESOFR> for bool {
    #[inline(always)]
    fn from(variant: ESOFR) -> Self {
        variant as u8 != 0
    }
}
///Field `ESOF` reader - Expected start of frame This bit is set by the hardware when an SOF packet is expected but not received. The host sends an SOF packet each 1ms, but if the device does not receive it properly, the Suspend Timer issues this interrupt. If three consecutive ESOF interrupts are generated (i.e. three SOF packets are lost) without any traffic occurring in between, a SUSP interrupt is generated. This bit is set even when the missing SOF packets occur while the Suspend Timer is not yet locked. This bit is read/write but only '0 can be written and writing '1 has no effect.
pub type ESOF_R = crate::BitReader<ESOFR>;
impl ESOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ESOFR {
        match self.bits {
            false => ESOFR::NotExpectedStartOfFrame,
            true => ESOFR::ExpectedStartOfFrame,
        }
    }
    ///NotExpectedStartOfFrame
    #[inline(always)]
    pub fn is_not_expected_start_of_frame(&self) -> bool {
        *self == ESOFR::NotExpectedStartOfFrame
    }
    ///An SOF packet is expected but not received
    #[inline(always)]
    pub fn is_expected_start_of_frame(&self) -> bool {
        *self == ESOFR::ExpectedStartOfFrame
    }
}
/**Expected start of frame This bit is set by the hardware when an SOF packet is expected but not received. The host sends an SOF packet each 1ms, but if the device does not receive it properly, the Suspend Timer issues this interrupt. If three consecutive ESOF interrupts are generated (i.e. three SOF packets are lost) without any traffic occurring in between, a SUSP interrupt is generated. This bit is set even when the missing SOF packets occur while the Suspend Timer is not yet locked. This bit is read/write but only '0 can be written and writing '1 has no effect.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESOFW {
    ///0: Clear flag
    Clear = 0,
}
impl From<ESOFW> for bool {
    #[inline(always)]
    fn from(variant: ESOFW) -> Self {
        variant as u8 != 0
    }
}
///Field `ESOF` writer - Expected start of frame This bit is set by the hardware when an SOF packet is expected but not received. The host sends an SOF packet each 1ms, but if the device does not receive it properly, the Suspend Timer issues this interrupt. If three consecutive ESOF interrupts are generated (i.e. three SOF packets are lost) without any traffic occurring in between, a SUSP interrupt is generated. This bit is set even when the missing SOF packets occur while the Suspend Timer is not yet locked. This bit is read/write but only '0 can be written and writing '1 has no effect.
pub type ESOF_W<'a, REG> = crate::BitWriter0C<'a, REG, ESOFW>;
impl<'a, REG> ESOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ESOFW::Clear)
    }
}
/**Start of frame This bit signals the beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus. The interrupt service routine may monitor the SOF events to have a 1ms synchronization event to the USB host and to safely read the USB_FNR register which is updated at the SOF packet reception (this could be useful for isochronous applications). This bit is read/write but only '0 can be written and writing '1 has no effect.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFR {
    ///0: NotStartOfFrame
    NotStartOfFrame = 0,
    ///1: Beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus
    StartOfFrame = 1,
}
impl From<SOFR> for bool {
    #[inline(always)]
    fn from(variant: SOFR) -> Self {
        variant as u8 != 0
    }
}
///Field `SOF` reader - Start of frame This bit signals the beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus. The interrupt service routine may monitor the SOF events to have a 1ms synchronization event to the USB host and to safely read the USB_FNR register which is updated at the SOF packet reception (this could be useful for isochronous applications). This bit is read/write but only '0 can be written and writing '1 has no effect.
pub type SOF_R = crate::BitReader<SOFR>;
impl SOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SOFR {
        match self.bits {
            false => SOFR::NotStartOfFrame,
            true => SOFR::StartOfFrame,
        }
    }
    ///NotStartOfFrame
    #[inline(always)]
    pub fn is_not_start_of_frame(&self) -> bool {
        *self == SOFR::NotStartOfFrame
    }
    ///Beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus
    #[inline(always)]
    pub fn is_start_of_frame(&self) -> bool {
        *self == SOFR::StartOfFrame
    }
}
/**Start of frame This bit signals the beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus. The interrupt service routine may monitor the SOF events to have a 1ms synchronization event to the USB host and to safely read the USB_FNR register which is updated at the SOF packet reception (this could be useful for isochronous applications). This bit is read/write but only '0 can be written and writing '1 has no effect.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFW {
    ///0: Clear flag
    Clear = 0,
}
impl From<SOFW> for bool {
    #[inline(always)]
    fn from(variant: SOFW) -> Self {
        variant as u8 != 0
    }
}
///Field `SOF` writer - Start of frame This bit signals the beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus. The interrupt service routine may monitor the SOF events to have a 1ms synchronization event to the USB host and to safely read the USB_FNR register which is updated at the SOF packet reception (this could be useful for isochronous applications). This bit is read/write but only '0 can be written and writing '1 has no effect.
pub type SOF_W<'a, REG> = crate::BitWriter0C<'a, REG, SOFW>;
impl<'a, REG> SOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SOFW::Clear)
    }
}
/**USB reset request Device mode This bit is set by hardware when an USB reset is released by the host and the bus returns to idle. USB reset state is internally detected after the sampling of 60 consecutive SE0 cycles. Host mode This bit is set by hardware when device connection or device disconnection is detected. Device connection is signaled after J state is sampled for 22cycles consecutively from unconnected state. Device disconnection is signaled after SE0 state is sampled for 22cycles consecutively from connected state.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_DCONR {
    ///0: NotReset
    NotReset = 0,
    ///1: Peripheral detects an active USB RESET signal at its inputs
    Reset = 1,
}
impl From<RST_DCONR> for bool {
    #[inline(always)]
    fn from(variant: RST_DCONR) -> Self {
        variant as u8 != 0
    }
}
///Field `RST_DCON` reader - USB reset request Device mode This bit is set by hardware when an USB reset is released by the host and the bus returns to idle. USB reset state is internally detected after the sampling of 60 consecutive SE0 cycles. Host mode This bit is set by hardware when device connection or device disconnection is detected. Device connection is signaled after J state is sampled for 22cycles consecutively from unconnected state. Device disconnection is signaled after SE0 state is sampled for 22cycles consecutively from connected state.
pub type RST_DCON_R = crate::BitReader<RST_DCONR>;
impl RST_DCON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RST_DCONR {
        match self.bits {
            false => RST_DCONR::NotReset,
            true => RST_DCONR::Reset,
        }
    }
    ///NotReset
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == RST_DCONR::NotReset
    }
    ///Peripheral detects an active USB RESET signal at its inputs
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RST_DCONR::Reset
    }
}
/**USB reset request Device mode This bit is set by hardware when an USB reset is released by the host and the bus returns to idle. USB reset state is internally detected after the sampling of 60 consecutive SE0 cycles. Host mode This bit is set by hardware when device connection or device disconnection is detected. Device connection is signaled after J state is sampled for 22cycles consecutively from unconnected state. Device disconnection is signaled after SE0 state is sampled for 22cycles consecutively from connected state.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_DCONW {
    ///0: Clear flag
    Clear = 0,
}
impl From<RST_DCONW> for bool {
    #[inline(always)]
    fn from(variant: RST_DCONW) -> Self {
        variant as u8 != 0
    }
}
///Field `RST_DCON` writer - USB reset request Device mode This bit is set by hardware when an USB reset is released by the host and the bus returns to idle. USB reset state is internally detected after the sampling of 60 consecutive SE0 cycles. Host mode This bit is set by hardware when device connection or device disconnection is detected. Device connection is signaled after J state is sampled for 22cycles consecutively from unconnected state. Device disconnection is signaled after SE0 state is sampled for 22cycles consecutively from connected state.
pub type RST_DCON_W<'a, REG> = crate::BitWriter0C<'a, REG, RST_DCONW>;
impl<'a, REG> RST_DCON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RST_DCONW::Clear)
    }
}
/**Suspend mode request This bit is set by the hardware when no traffic has been received for 3ms, indicating a suspend mode request from the USB bus. The suspend condition check is enabled immediately after any USB reset and it is disabled by the hardware when the suspend mode is active (SUSPEN=1) until the end of resume sequence. This bit is read/write but only '0 can be written and writing '1 has no effect.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPR {
    ///0: NotSuspend
    NotSuspend = 0,
    ///1: No traffic has been received for 3 ms, indicating a suspend mode request from the USB bus
    Suspend = 1,
}
impl From<SUSPR> for bool {
    #[inline(always)]
    fn from(variant: SUSPR) -> Self {
        variant as u8 != 0
    }
}
///Field `SUSP` reader - Suspend mode request This bit is set by the hardware when no traffic has been received for 3ms, indicating a suspend mode request from the USB bus. The suspend condition check is enabled immediately after any USB reset and it is disabled by the hardware when the suspend mode is active (SUSPEN=1) until the end of resume sequence. This bit is read/write but only '0 can be written and writing '1 has no effect.
pub type SUSP_R = crate::BitReader<SUSPR>;
impl SUSP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SUSPR {
        match self.bits {
            false => SUSPR::NotSuspend,
            true => SUSPR::Suspend,
        }
    }
    ///NotSuspend
    #[inline(always)]
    pub fn is_not_suspend(&self) -> bool {
        *self == SUSPR::NotSuspend
    }
    ///No traffic has been received for 3 ms, indicating a suspend mode request from the USB bus
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == SUSPR::Suspend
    }
}
/**Suspend mode request This bit is set by the hardware when no traffic has been received for 3ms, indicating a suspend mode request from the USB bus. The suspend condition check is enabled immediately after any USB reset and it is disabled by the hardware when the suspend mode is active (SUSPEN=1) until the end of resume sequence. This bit is read/write but only '0 can be written and writing '1 has no effect.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPW {
    ///0: Clear flag
    Clear = 0,
}
impl From<SUSPW> for bool {
    #[inline(always)]
    fn from(variant: SUSPW) -> Self {
        variant as u8 != 0
    }
}
///Field `SUSP` writer - Suspend mode request This bit is set by the hardware when no traffic has been received for 3ms, indicating a suspend mode request from the USB bus. The suspend condition check is enabled immediately after any USB reset and it is disabled by the hardware when the suspend mode is active (SUSPEN=1) until the end of resume sequence. This bit is read/write but only '0 can be written and writing '1 has no effect.
pub type SUSP_W<'a, REG> = crate::BitWriter0C<'a, REG, SUSPW>;
impl<'a, REG> SUSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPW::Clear)
    }
}
/**Wakeup This bit is set to 1 by the hardware when, during suspend mode, activity is detected that wakes up the USB peripheral. This event asynchronously clears the LP_MODE bit in the CTLR register and activates the USB_WAKEUP line, which can be used to notify the rest of the device (e.g. wakeup unit) about the start of the resume process. This bit is read/write but only '0 can be written and writing '1 has no effect.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPR {
    ///0: NotWakeup
    NotWakeup = 0,
    ///1: Activity is detected that wakes up the USB peripheral
    Wakeup = 1,
}
impl From<WKUPR> for bool {
    #[inline(always)]
    fn from(variant: WKUPR) -> Self {
        variant as u8 != 0
    }
}
///Field `WKUP` reader - Wakeup This bit is set to 1 by the hardware when, during suspend mode, activity is detected that wakes up the USB peripheral. This event asynchronously clears the LP_MODE bit in the CTLR register and activates the USB_WAKEUP line, which can be used to notify the rest of the device (e.g. wakeup unit) about the start of the resume process. This bit is read/write but only '0 can be written and writing '1 has no effect.
pub type WKUP_R = crate::BitReader<WKUPR>;
impl WKUP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WKUPR {
        match self.bits {
            false => WKUPR::NotWakeup,
            true => WKUPR::Wakeup,
        }
    }
    ///NotWakeup
    #[inline(always)]
    pub fn is_not_wakeup(&self) -> bool {
        *self == WKUPR::NotWakeup
    }
    ///Activity is detected that wakes up the USB peripheral
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WKUPR::Wakeup
    }
}
/**Wakeup This bit is set to 1 by the hardware when, during suspend mode, activity is detected that wakes up the USB peripheral. This event asynchronously clears the LP_MODE bit in the CTLR register and activates the USB_WAKEUP line, which can be used to notify the rest of the device (e.g. wakeup unit) about the start of the resume process. This bit is read/write but only '0 can be written and writing '1 has no effect.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPW {
    ///0: Clear flag
    Clear = 0,
}
impl From<WKUPW> for bool {
    #[inline(always)]
    fn from(variant: WKUPW) -> Self {
        variant as u8 != 0
    }
}
///Field `WKUP` writer - Wakeup This bit is set to 1 by the hardware when, during suspend mode, activity is detected that wakes up the USB peripheral. This event asynchronously clears the LP_MODE bit in the CTLR register and activates the USB_WAKEUP line, which can be used to notify the rest of the device (e.g. wakeup unit) about the start of the resume process. This bit is read/write but only '0 can be written and writing '1 has no effect.
pub type WKUP_W<'a, REG> = crate::BitWriter0C<'a, REG, WKUPW>;
impl<'a, REG> WKUP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPW::Clear)
    }
}
/**Error This flag is set whenever one of the errors listed below has occurred: NANS: No ANSwer. The timeout for a host response has expired. CRC: Cyclic Redundancy Check error. One of the received CRCs, either in the token or in the data, was wrong. BST: Bit Stuffing error. A bit stuffing error was detected anywhere in the PID, data, and/or CRC. FVIO: Framing format Violation. A non-standard frame was received (EOP not in the right place, wrong token sequence, etc.). The USB software can usually ignore errors, since the USB peripheral and the PC host manage retransmission in case of errors in a fully transparent way. This interrupt can be useful during the software development phase, or to monitor the quality of transmission over the USB bus, to flag possible problems to the user (e.g. loose connector, too noisy environment, broken conductor in the USB cable and so on). This bit is read/write but only '0 can be written and writing '1 has no effect.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRR {
    ///0: Errors are not occurred
    NotError = 0,
    ///1: One of No ANSwer, Cyclic Redundancy Check, Bit Stuffing or Framing format Violation error occurred
    Error = 1,
}
impl From<ERRR> for bool {
    #[inline(always)]
    fn from(variant: ERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `ERR` reader - Error This flag is set whenever one of the errors listed below has occurred: NANS: No ANSwer. The timeout for a host response has expired. CRC: Cyclic Redundancy Check error. One of the received CRCs, either in the token or in the data, was wrong. BST: Bit Stuffing error. A bit stuffing error was detected anywhere in the PID, data, and/or CRC. FVIO: Framing format Violation. A non-standard frame was received (EOP not in the right place, wrong token sequence, etc.). The USB software can usually ignore errors, since the USB peripheral and the PC host manage retransmission in case of errors in a fully transparent way. This interrupt can be useful during the software development phase, or to monitor the quality of transmission over the USB bus, to flag possible problems to the user (e.g. loose connector, too noisy environment, broken conductor in the USB cable and so on). This bit is read/write but only '0 can be written and writing '1 has no effect.
pub type ERR_R = crate::BitReader<ERRR>;
impl ERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ERRR {
        match self.bits {
            false => ERRR::NotError,
            true => ERRR::Error,
        }
    }
    ///Errors are not occurred
    #[inline(always)]
    pub fn is_not_error(&self) -> bool {
        *self == ERRR::NotError
    }
    ///One of No ANSwer, Cyclic Redundancy Check, Bit Stuffing or Framing format Violation error occurred
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ERRR::Error
    }
}
/**Error This flag is set whenever one of the errors listed below has occurred: NANS: No ANSwer. The timeout for a host response has expired. CRC: Cyclic Redundancy Check error. One of the received CRCs, either in the token or in the data, was wrong. BST: Bit Stuffing error. A bit stuffing error was detected anywhere in the PID, data, and/or CRC. FVIO: Framing format Violation. A non-standard frame was received (EOP not in the right place, wrong token sequence, etc.). The USB software can usually ignore errors, since the USB peripheral and the PC host manage retransmission in case of errors in a fully transparent way. This interrupt can be useful during the software development phase, or to monitor the quality of transmission over the USB bus, to flag possible problems to the user (e.g. loose connector, too noisy environment, broken conductor in the USB cable and so on). This bit is read/write but only '0 can be written and writing '1 has no effect.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRW {
    ///0: Clear flag
    Clear = 0,
}
impl From<ERRW> for bool {
    #[inline(always)]
    fn from(variant: ERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `ERR` writer - Error This flag is set whenever one of the errors listed below has occurred: NANS: No ANSwer. The timeout for a host response has expired. CRC: Cyclic Redundancy Check error. One of the received CRCs, either in the token or in the data, was wrong. BST: Bit Stuffing error. A bit stuffing error was detected anywhere in the PID, data, and/or CRC. FVIO: Framing format Violation. A non-standard frame was received (EOP not in the right place, wrong token sequence, etc.). The USB software can usually ignore errors, since the USB peripheral and the PC host manage retransmission in case of errors in a fully transparent way. This interrupt can be useful during the software development phase, or to monitor the quality of transmission over the USB bus, to flag possible problems to the user (e.g. loose connector, too noisy environment, broken conductor in the USB cable and so on). This bit is read/write but only '0 can be written and writing '1 has no effect.
pub type ERR_W<'a, REG> = crate::BitWriter0C<'a, REG, ERRW>;
impl<'a, REG> ERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ERRW::Clear)
    }
}
/**Packet memory area over / underrun This bit is set if the microcontroller has not been able to respond in time to an USB memory request. The USB peripheral handles this event in the following way: During reception an ACK handshake packet is not sent, during transmission a bit-stuff error is forced on the transmitted stream; in both cases the host will retry the transaction. The PMAOVR interrupt should never occur during normal operations. Since the failed transaction is retried by the host, the application software has the chance to speed-up device operations during this interrupt handling, to be ready for the next transaction retry; however this does not happen during Isochronous transfers (no isochronous transaction is anyway retried) leading to a loss of data in this case. This bit is read/write but only '0 can be written and writing '1 has no effect.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMAOVRR {
    ///0: Overrun is not occurred
    NotOverrun = 0,
    ///1: Microcontroller has not been able to respond in time to an USB memory request
    Overrun = 1,
}
impl From<PMAOVRR> for bool {
    #[inline(always)]
    fn from(variant: PMAOVRR) -> Self {
        variant as u8 != 0
    }
}
///Field `PMAOVR` reader - Packet memory area over / underrun This bit is set if the microcontroller has not been able to respond in time to an USB memory request. The USB peripheral handles this event in the following way: During reception an ACK handshake packet is not sent, during transmission a bit-stuff error is forced on the transmitted stream; in both cases the host will retry the transaction. The PMAOVR interrupt should never occur during normal operations. Since the failed transaction is retried by the host, the application software has the chance to speed-up device operations during this interrupt handling, to be ready for the next transaction retry; however this does not happen during Isochronous transfers (no isochronous transaction is anyway retried) leading to a loss of data in this case. This bit is read/write but only '0 can be written and writing '1 has no effect.
pub type PMAOVR_R = crate::BitReader<PMAOVRR>;
impl PMAOVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PMAOVRR {
        match self.bits {
            false => PMAOVRR::NotOverrun,
            true => PMAOVRR::Overrun,
        }
    }
    ///Overrun is not occurred
    #[inline(always)]
    pub fn is_not_overrun(&self) -> bool {
        *self == PMAOVRR::NotOverrun
    }
    ///Microcontroller has not been able to respond in time to an USB memory request
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == PMAOVRR::Overrun
    }
}
/**Packet memory area over / underrun This bit is set if the microcontroller has not been able to respond in time to an USB memory request. The USB peripheral handles this event in the following way: During reception an ACK handshake packet is not sent, during transmission a bit-stuff error is forced on the transmitted stream; in both cases the host will retry the transaction. The PMAOVR interrupt should never occur during normal operations. Since the failed transaction is retried by the host, the application software has the chance to speed-up device operations during this interrupt handling, to be ready for the next transaction retry; however this does not happen during Isochronous transfers (no isochronous transaction is anyway retried) leading to a loss of data in this case. This bit is read/write but only '0 can be written and writing '1 has no effect.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMAOVRW {
    ///0: Clear flag
    Clear = 0,
}
impl From<PMAOVRW> for bool {
    #[inline(always)]
    fn from(variant: PMAOVRW) -> Self {
        variant as u8 != 0
    }
}
///Field `PMAOVR` writer - Packet memory area over / underrun This bit is set if the microcontroller has not been able to respond in time to an USB memory request. The USB peripheral handles this event in the following way: During reception an ACK handshake packet is not sent, during transmission a bit-stuff error is forced on the transmitted stream; in both cases the host will retry the transaction. The PMAOVR interrupt should never occur during normal operations. Since the failed transaction is retried by the host, the application software has the chance to speed-up device operations during this interrupt handling, to be ready for the next transaction retry; however this does not happen during Isochronous transfers (no isochronous transaction is anyway retried) leading to a loss of data in this case. This bit is read/write but only '0 can be written and writing '1 has no effect.
pub type PMAOVR_W<'a, REG> = crate::BitWriter0C<'a, REG, PMAOVRW>;
impl<'a, REG> PMAOVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PMAOVRW::Clear)
    }
}
/**Correct transfer This bit is set by the hardware to indicate that an endpoint/channel has successfully completed a transaction; using DIR and EP_ID bits software can determine which endpoint/channel requested the interrupt. This bit is read-only.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTR {
    ///1: Endpoint has successfully completed a transaction
    Completed = 1,
}
impl From<CTR> for bool {
    #[inline(always)]
    fn from(variant: CTR) -> Self {
        variant as u8 != 0
    }
}
///Field `CTR` reader - Correct transfer This bit is set by the hardware to indicate that an endpoint/channel has successfully completed a transaction; using DIR and EP_ID bits software can determine which endpoint/channel requested the interrupt. This bit is read-only.
pub type CTR_R = crate::BitReader<CTR>;
impl CTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTR> {
        match self.bits {
            true => Some(CTR::Completed),
            _ => None,
        }
    }
    ///Endpoint has successfully completed a transaction
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == CTR::Completed
    }
}
/**512 byte threshold interrupt This bit is set to 1 by the hardware when 512 bytes have been transmitted or received during isochronous transfers. This bit is read/write but only 0 can be written and writing 1 has no effect. Note that no information is available to indicate the associated channel/endpoint, however in practice only one ISO endpoint/channel with such large packets can be supported, so that channel.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THR512R {
    ///0: 512 bytes threshold not reached
    NotReached = 0,
    ///1: 512 bytes have been transmitted or received during isochronous transfers
    Reached = 1,
}
impl From<THR512R> for bool {
    #[inline(always)]
    fn from(variant: THR512R) -> Self {
        variant as u8 != 0
    }
}
///Field `THR512` reader - 512 byte threshold interrupt This bit is set to 1 by the hardware when 512 bytes have been transmitted or received during isochronous transfers. This bit is read/write but only 0 can be written and writing 1 has no effect. Note that no information is available to indicate the associated channel/endpoint, however in practice only one ISO endpoint/channel with such large packets can be supported, so that channel.
pub type THR512_R = crate::BitReader<THR512R>;
impl THR512_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> THR512R {
        match self.bits {
            false => THR512R::NotReached,
            true => THR512R::Reached,
        }
    }
    ///512 bytes threshold not reached
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == THR512R::NotReached
    }
    ///512 bytes have been transmitted or received during isochronous transfers
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == THR512R::Reached
    }
}
/**512 byte threshold interrupt This bit is set to 1 by the hardware when 512 bytes have been transmitted or received during isochronous transfers. This bit is read/write but only 0 can be written and writing 1 has no effect. Note that no information is available to indicate the associated channel/endpoint, however in practice only one ISO endpoint/channel with such large packets can be supported, so that channel.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THR512W {
    ///0: Clear flag
    Clear = 0,
}
impl From<THR512W> for bool {
    #[inline(always)]
    fn from(variant: THR512W) -> Self {
        variant as u8 != 0
    }
}
///Field `THR512` writer - 512 byte threshold interrupt This bit is set to 1 by the hardware when 512 bytes have been transmitted or received during isochronous transfers. This bit is read/write but only 0 can be written and writing 1 has no effect. Note that no information is available to indicate the associated channel/endpoint, however in practice only one ISO endpoint/channel with such large packets can be supported, so that channel.
pub type THR512_W<'a, REG> = crate::BitWriter0C<'a, REG, THR512W>;
impl<'a, REG> THR512_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(THR512W::Clear)
    }
}
///Field `DCON_STAT` reader - Device connection status Host mode: This bit contains information about device connection status. It is set by hardware when a LS/FS device is attached to the host while it is reset when the device is disconnected.
pub type DCON_STAT_R = crate::BitReader;
///Field `LS_DCON` reader - Low Speed device connected Host mode: This bit is set by hardware when an LS device connection is detected. Device connection is signaled after LS J-state is sampled for 22 consecutive cycles of the USB clock (48 MHz) from the unconnected state.
pub type LS_DCON_R = crate::BitReader;
impl R {
    ///Bits 0:3 - Device Endpoint / Host channel identification number These bits are written by the hardware according to the host channel or device endpoint number, which generated the interrupt request. If several endpoint/channel transactions are pending, the hardware writes the identification number related to the endpoint/channel having the highest priority defined in the following way: Two levels are defined, in order of priority: Isochronous and double-buffered bulk channels/endpoints are considered first and then the others are examined. If more than one endpoint/channel from the same set is requesting an interrupt, the IDN bits in USB_ISTR register are assigned according to the lowest requesting register, CHEP0R having the highest priority followed by CHEP1R and so on. The application software can assign a register to each endpoint/channel according to this priority scheme, so as to order the concurring endpoint/channel requests in a suitable way. These bits are read only.
    #[inline(always)]
    pub fn idn(&self) -> IDN_R {
        IDN_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Direction of transaction This bit is written by the hardware according to the direction of the successful transaction, which generated the interrupt request. If DIR bit=0, VTTX bit is set in the USB_EPnR register related to the interrupting endpoint. The interrupting transaction is of IN type (data transmitted by the USB peripheral to the host PC). If DIR bit=1, VTRX bit or both VTTX/VTRX are set in the USB_EPnR register related to the interrupting endpoint. The interrupting transaction is of OUT type (data received by the USB peripheral from the host PC) or two pending transactions are waiting to be processed. This information can be used by the application software to access the USB_EPnR bits related to the triggering transaction since it represents the direction having the interrupt pending. This bit is read-only.
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - LPM L1 state request This bit is set by the hardware when LPM command to enter the L1 state is successfully received and acknowledged. This bit is read/write but only '0 can be written and writing '1 has no effect.
    #[inline(always)]
    pub fn l1req(&self) -> L1REQ_R {
        L1REQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Expected start of frame This bit is set by the hardware when an SOF packet is expected but not received. The host sends an SOF packet each 1ms, but if the device does not receive it properly, the Suspend Timer issues this interrupt. If three consecutive ESOF interrupts are generated (i.e. three SOF packets are lost) without any traffic occurring in between, a SUSP interrupt is generated. This bit is set even when the missing SOF packets occur while the Suspend Timer is not yet locked. This bit is read/write but only '0 can be written and writing '1 has no effect.
    #[inline(always)]
    pub fn esof(&self) -> ESOF_R {
        ESOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Start of frame This bit signals the beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus. The interrupt service routine may monitor the SOF events to have a 1ms synchronization event to the USB host and to safely read the USB_FNR register which is updated at the SOF packet reception (this could be useful for isochronous applications). This bit is read/write but only '0 can be written and writing '1 has no effect.
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - USB reset request Device mode This bit is set by hardware when an USB reset is released by the host and the bus returns to idle. USB reset state is internally detected after the sampling of 60 consecutive SE0 cycles. Host mode This bit is set by hardware when device connection or device disconnection is detected. Device connection is signaled after J state is sampled for 22cycles consecutively from unconnected state. Device disconnection is signaled after SE0 state is sampled for 22cycles consecutively from connected state.
    #[inline(always)]
    pub fn rst_dcon(&self) -> RST_DCON_R {
        RST_DCON_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Suspend mode request This bit is set by the hardware when no traffic has been received for 3ms, indicating a suspend mode request from the USB bus. The suspend condition check is enabled immediately after any USB reset and it is disabled by the hardware when the suspend mode is active (SUSPEN=1) until the end of resume sequence. This bit is read/write but only '0 can be written and writing '1 has no effect.
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Wakeup This bit is set to 1 by the hardware when, during suspend mode, activity is detected that wakes up the USB peripheral. This event asynchronously clears the LP_MODE bit in the CTLR register and activates the USB_WAKEUP line, which can be used to notify the rest of the device (e.g. wakeup unit) about the start of the resume process. This bit is read/write but only '0 can be written and writing '1 has no effect.
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Error This flag is set whenever one of the errors listed below has occurred: NANS: No ANSwer. The timeout for a host response has expired. CRC: Cyclic Redundancy Check error. One of the received CRCs, either in the token or in the data, was wrong. BST: Bit Stuffing error. A bit stuffing error was detected anywhere in the PID, data, and/or CRC. FVIO: Framing format Violation. A non-standard frame was received (EOP not in the right place, wrong token sequence, etc.). The USB software can usually ignore errors, since the USB peripheral and the PC host manage retransmission in case of errors in a fully transparent way. This interrupt can be useful during the software development phase, or to monitor the quality of transmission over the USB bus, to flag possible problems to the user (e.g. loose connector, too noisy environment, broken conductor in the USB cable and so on). This bit is read/write but only '0 can be written and writing '1 has no effect.
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Packet memory area over / underrun This bit is set if the microcontroller has not been able to respond in time to an USB memory request. The USB peripheral handles this event in the following way: During reception an ACK handshake packet is not sent, during transmission a bit-stuff error is forced on the transmitted stream; in both cases the host will retry the transaction. The PMAOVR interrupt should never occur during normal operations. Since the failed transaction is retried by the host, the application software has the chance to speed-up device operations during this interrupt handling, to be ready for the next transaction retry; however this does not happen during Isochronous transfers (no isochronous transaction is anyway retried) leading to a loss of data in this case. This bit is read/write but only '0 can be written and writing '1 has no effect.
    #[inline(always)]
    pub fn pmaovr(&self) -> PMAOVR_R {
        PMAOVR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Correct transfer This bit is set by the hardware to indicate that an endpoint/channel has successfully completed a transaction; using DIR and EP_ID bits software can determine which endpoint/channel requested the interrupt. This bit is read-only.
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - 512 byte threshold interrupt This bit is set to 1 by the hardware when 512 bytes have been transmitted or received during isochronous transfers. This bit is read/write but only 0 can be written and writing 1 has no effect. Note that no information is available to indicate the associated channel/endpoint, however in practice only one ISO endpoint/channel with such large packets can be supported, so that channel.
    #[inline(always)]
    pub fn thr512(&self) -> THR512_R {
        THR512_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 29 - Device connection status Host mode: This bit contains information about device connection status. It is set by hardware when a LS/FS device is attached to the host while it is reset when the device is disconnected.
    #[inline(always)]
    pub fn dcon_stat(&self) -> DCON_STAT_R {
        DCON_STAT_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Low Speed device connected Host mode: This bit is set by hardware when an LS device connection is detected. Device connection is signaled after LS J-state is sampled for 22 consecutive cycles of the USB clock (48 MHz) from the unconnected state.
    #[inline(always)]
    pub fn ls_dcon(&self) -> LS_DCON_R {
        LS_DCON_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISTR")
            .field("idn", &self.idn())
            .field("dir", &self.dir())
            .field("l1req", &self.l1req())
            .field("esof", &self.esof())
            .field("sof", &self.sof())
            .field("rst_dcon", &self.rst_dcon())
            .field("susp", &self.susp())
            .field("wkup", &self.wkup())
            .field("err", &self.err())
            .field("pmaovr", &self.pmaovr())
            .field("ctr", &self.ctr())
            .field("thr512", &self.thr512())
            .field("dcon_stat", &self.dcon_stat())
            .field("ls_dcon", &self.ls_dcon())
            .finish()
    }
}
impl W {
    ///Bit 7 - LPM L1 state request This bit is set by the hardware when LPM command to enter the L1 state is successfully received and acknowledged. This bit is read/write but only '0 can be written and writing '1 has no effect.
    #[inline(always)]
    pub fn l1req(&mut self) -> L1REQ_W<'_, ISTRrs> {
        L1REQ_W::new(self, 7)
    }
    ///Bit 8 - Expected start of frame This bit is set by the hardware when an SOF packet is expected but not received. The host sends an SOF packet each 1ms, but if the device does not receive it properly, the Suspend Timer issues this interrupt. If three consecutive ESOF interrupts are generated (i.e. three SOF packets are lost) without any traffic occurring in between, a SUSP interrupt is generated. This bit is set even when the missing SOF packets occur while the Suspend Timer is not yet locked. This bit is read/write but only '0 can be written and writing '1 has no effect.
    #[inline(always)]
    pub fn esof(&mut self) -> ESOF_W<'_, ISTRrs> {
        ESOF_W::new(self, 8)
    }
    ///Bit 9 - Start of frame This bit signals the beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus. The interrupt service routine may monitor the SOF events to have a 1ms synchronization event to the USB host and to safely read the USB_FNR register which is updated at the SOF packet reception (this could be useful for isochronous applications). This bit is read/write but only '0 can be written and writing '1 has no effect.
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<'_, ISTRrs> {
        SOF_W::new(self, 9)
    }
    ///Bit 10 - USB reset request Device mode This bit is set by hardware when an USB reset is released by the host and the bus returns to idle. USB reset state is internally detected after the sampling of 60 consecutive SE0 cycles. Host mode This bit is set by hardware when device connection or device disconnection is detected. Device connection is signaled after J state is sampled for 22cycles consecutively from unconnected state. Device disconnection is signaled after SE0 state is sampled for 22cycles consecutively from connected state.
    #[inline(always)]
    pub fn rst_dcon(&mut self) -> RST_DCON_W<'_, ISTRrs> {
        RST_DCON_W::new(self, 10)
    }
    ///Bit 11 - Suspend mode request This bit is set by the hardware when no traffic has been received for 3ms, indicating a suspend mode request from the USB bus. The suspend condition check is enabled immediately after any USB reset and it is disabled by the hardware when the suspend mode is active (SUSPEN=1) until the end of resume sequence. This bit is read/write but only '0 can be written and writing '1 has no effect.
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<'_, ISTRrs> {
        SUSP_W::new(self, 11)
    }
    ///Bit 12 - Wakeup This bit is set to 1 by the hardware when, during suspend mode, activity is detected that wakes up the USB peripheral. This event asynchronously clears the LP_MODE bit in the CTLR register and activates the USB_WAKEUP line, which can be used to notify the rest of the device (e.g. wakeup unit) about the start of the resume process. This bit is read/write but only '0 can be written and writing '1 has no effect.
    #[inline(always)]
    pub fn wkup(&mut self) -> WKUP_W<'_, ISTRrs> {
        WKUP_W::new(self, 12)
    }
    ///Bit 13 - Error This flag is set whenever one of the errors listed below has occurred: NANS: No ANSwer. The timeout for a host response has expired. CRC: Cyclic Redundancy Check error. One of the received CRCs, either in the token or in the data, was wrong. BST: Bit Stuffing error. A bit stuffing error was detected anywhere in the PID, data, and/or CRC. FVIO: Framing format Violation. A non-standard frame was received (EOP not in the right place, wrong token sequence, etc.). The USB software can usually ignore errors, since the USB peripheral and the PC host manage retransmission in case of errors in a fully transparent way. This interrupt can be useful during the software development phase, or to monitor the quality of transmission over the USB bus, to flag possible problems to the user (e.g. loose connector, too noisy environment, broken conductor in the USB cable and so on). This bit is read/write but only '0 can be written and writing '1 has no effect.
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W<'_, ISTRrs> {
        ERR_W::new(self, 13)
    }
    ///Bit 14 - Packet memory area over / underrun This bit is set if the microcontroller has not been able to respond in time to an USB memory request. The USB peripheral handles this event in the following way: During reception an ACK handshake packet is not sent, during transmission a bit-stuff error is forced on the transmitted stream; in both cases the host will retry the transaction. The PMAOVR interrupt should never occur during normal operations. Since the failed transaction is retried by the host, the application software has the chance to speed-up device operations during this interrupt handling, to be ready for the next transaction retry; however this does not happen during Isochronous transfers (no isochronous transaction is anyway retried) leading to a loss of data in this case. This bit is read/write but only '0 can be written and writing '1 has no effect.
    #[inline(always)]
    pub fn pmaovr(&mut self) -> PMAOVR_W<'_, ISTRrs> {
        PMAOVR_W::new(self, 14)
    }
    ///Bit 16 - 512 byte threshold interrupt This bit is set to 1 by the hardware when 512 bytes have been transmitted or received during isochronous transfers. This bit is read/write but only 0 can be written and writing 1 has no effect. Note that no information is available to indicate the associated channel/endpoint, however in practice only one ISO endpoint/channel with such large packets can be supported, so that channel.
    #[inline(always)]
    pub fn thr512(&mut self) -> THR512_W<'_, ISTRrs> {
        THR512_W::new(self, 16)
    }
}
/**USB interrupt status register

You can [`read`](crate::Reg::read) this register and get [`istr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`istr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#USB:ISTR)*/
pub struct ISTRrs;
impl crate::RegisterSpec for ISTRrs {
    type Ux = u32;
}
///`read()` method returns [`istr::R`](R) reader structure
impl crate::Readable for ISTRrs {}
///`write(|w| ..)` method takes [`istr::W`](W) writer structure
impl crate::Writable for ISTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_7f80;
}
///`reset()` method sets ISTR to value 0
impl crate::Resettable for ISTRrs {}
