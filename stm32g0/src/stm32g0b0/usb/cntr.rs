///Register `CNTR` reader
pub type R = crate::R<CNTRrs>;
///Register `CNTR` writer
pub type W = crate::W<CNTRrs>;
/**USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBRST {
    ///0: No effect
    NoEffect = 0,
    ///1: USB core is under reset / USB reset driven
    Reset = 1,
}
impl From<USBRST> for bool {
    #[inline(always)]
    fn from(variant: USBRST) -> Self {
        variant as u8 != 0
    }
}
///Field `USBRST` reader - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software.
pub type USBRST_R = crate::BitReader<USBRST>;
impl USBRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USBRST {
        match self.bits {
            false => USBRST::NoEffect,
            true => USBRST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == USBRST::NoEffect
    }
    ///USB core is under reset / USB reset driven
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == USBRST::Reset
    }
}
///Field `USBRST` writer - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software.
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG, USBRST>;
impl<'a, REG> USBRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(USBRST::NoEffect)
    }
    ///USB core is under reset / USB reset driven
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(USBRST::Reset)
    }
}
///Field `PDWN` reader - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used.
pub type PDWN_R = crate::BitReader;
///Field `PDWN` writer - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used.
pub type PDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUSPRDY` reader - Suspend state effective This bit is set by hardware as soon as the suspend state entered through the SUSPEN control gets internally effective. In this state USB activity is suspended, USB clock is gated, transceiver is set in low power mode by disabling the differential receiver. Only asynchronous wakeup logic and single ended receiver is kept alive to detect remote wakeup or resume events. Software must poll this bit to confirm it to be set before any STOP mode entry. This bit is cleared by hardware simultaneously to the WAKEUP flag being set.
pub type SUSPRDY_R = crate::BitReader;
/**Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY=1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPEN {
    ///0: No effect
    NoEffect = 0,
    ///1: Enter L1/L2 suspend
    Suspend = 1,
}
impl From<SUSPEN> for bool {
    #[inline(always)]
    fn from(variant: SUSPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SUSPEN` reader - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY=1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set.
pub type SUSPEN_R = crate::BitReader<SUSPEN>;
impl SUSPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SUSPEN {
        match self.bits {
            false => SUSPEN::NoEffect,
            true => SUSPEN::Suspend,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SUSPEN::NoEffect
    }
    ///Enter L1/L2 suspend
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == SUSPEN::Suspend
    }
}
///Field `SUSPEN` writer - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY=1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set.
pub type SUSPEN_W<'a, REG> = crate::BitWriter<'a, REG, SUSPEN>;
impl<'a, REG> SUSPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPEN::NoEffect)
    }
    ///Enter L1/L2 suspend
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPEN::Suspend)
    }
}
///Field `L2RESUME` reader - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1ms and no more than 15ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt.
pub type L2RESUME_R = crate::BitReader;
///Field `L2RESUME` writer - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1ms and no more than 15ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt.
pub type L2RESUME_W<'a, REG> = crate::BitWriter<'a, REG>;
/**L1 remote wakeup / resume driver Device mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1RESUME {
    ///0: No effect
    NoEffect = 0,
    ///1: Send 50us remote-wakeup signaling to host / Send L1 resume signaling to device
    WakeupResume = 1,
}
impl From<L1RESUME> for bool {
    #[inline(always)]
    fn from(variant: L1RESUME) -> Self {
        variant as u8 != 0
    }
}
///Field `L1RESUME` reader - L1 remote wakeup / resume driver Device mode
pub type L1RESUME_R = crate::BitReader<L1RESUME>;
impl L1RESUME_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1RESUME {
        match self.bits {
            false => L1RESUME::NoEffect,
            true => L1RESUME::WakeupResume,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == L1RESUME::NoEffect
    }
    ///Send 50us remote-wakeup signaling to host / Send L1 resume signaling to device
    #[inline(always)]
    pub fn is_wakeup_resume(&self) -> bool {
        *self == L1RESUME::WakeupResume
    }
}
///Field `L1RESUME` writer - L1 remote wakeup / resume driver Device mode
pub type L1RESUME_W<'a, REG> = crate::BitWriter<'a, REG, L1RESUME>;
impl<'a, REG> L1RESUME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(L1RESUME::NoEffect)
    }
    ///Send 50us remote-wakeup signaling to host / Send L1 resume signaling to device
    #[inline(always)]
    pub fn wakeup_resume(self) -> &'a mut crate::W<REG> {
        self.variant(L1RESUME::WakeupResume)
    }
}
///Field `L1REQM` reader - LPM L1 state request interrupt mask
pub type L1REQM_R = crate::BitReader;
///Field `L1REQM` writer - LPM L1 state request interrupt mask
pub type L1REQM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ESOFM` reader - Expected start of frame interrupt mask
pub type ESOFM_R = crate::BitReader;
///Field `ESOFM` writer - Expected start of frame interrupt mask
pub type ESOFM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOFM` reader - Start of frame interrupt mask
pub type SOFM_R = crate::BitReader;
///Field `SOFM` writer - Start of frame interrupt mask
pub type SOFM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RST_DCONM` reader - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask
pub type RST_DCONM_R = crate::BitReader;
///Field `RST_DCONM` writer - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask
pub type RST_DCONM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUSPM` reader - Suspend mode interrupt mask
pub type SUSPM_R = crate::BitReader;
///Field `SUSPM` writer - Suspend mode interrupt mask
pub type SUSPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPM` reader - Wakeup interrupt mask
pub type WKUPM_R = crate::BitReader;
///Field `WKUPM` writer - Wakeup interrupt mask
pub type WKUPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRM` reader - Error interrupt mask
pub type ERRM_R = crate::BitReader;
///Field `ERRM` writer - Error interrupt mask
pub type ERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PMAOVRM` reader - Packet memory area over / underrun interrupt mask
pub type PMAOVRM_R = crate::BitReader;
///Field `PMAOVRM` writer - Packet memory area over / underrun interrupt mask
pub type PMAOVRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTRM` reader - Correct transfer interrupt mask
pub type CTRM_R = crate::BitReader;
///Field `CTRM` writer - Correct transfer interrupt mask
pub type CTRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THR512M` reader - 512 byte threshold interrupt mask
pub type THR512M_R = crate::BitReader;
///Field `THR512M` writer - 512 byte threshold interrupt mask
pub type THR512M_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HOST` reader - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit.
pub type HOST_R = crate::BitReader;
///Field `HOST` writer - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit.
pub type HOST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software.
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used.
    #[inline(always)]
    pub fn pdwn(&self) -> PDWN_R {
        PDWN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Suspend state effective This bit is set by hardware as soon as the suspend state entered through the SUSPEN control gets internally effective. In this state USB activity is suspended, USB clock is gated, transceiver is set in low power mode by disabling the differential receiver. Only asynchronous wakeup logic and single ended receiver is kept alive to detect remote wakeup or resume events. Software must poll this bit to confirm it to be set before any STOP mode entry. This bit is cleared by hardware simultaneously to the WAKEUP flag being set.
    #[inline(always)]
    pub fn susprdy(&self) -> SUSPRDY_R {
        SUSPRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY=1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set.
    #[inline(always)]
    pub fn suspen(&self) -> SUSPEN_R {
        SUSPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1ms and no more than 15ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt.
    #[inline(always)]
    pub fn l2resume(&self) -> L2RESUME_R {
        L2RESUME_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - L1 remote wakeup / resume driver Device mode
    #[inline(always)]
    pub fn l1resume(&self) -> L1RESUME_R {
        L1RESUME_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - LPM L1 state request interrupt mask
    #[inline(always)]
    pub fn l1reqm(&self) -> L1REQM_R {
        L1REQM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Expected start of frame interrupt mask
    #[inline(always)]
    pub fn esofm(&self) -> ESOFM_R {
        ESOFM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Start of frame interrupt mask
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask
    #[inline(always)]
    pub fn rst_dconm(&self) -> RST_DCONM_R {
        RST_DCONM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Suspend mode interrupt mask
    #[inline(always)]
    pub fn suspm(&self) -> SUSPM_R {
        SUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Wakeup interrupt mask
    #[inline(always)]
    pub fn wkupm(&self) -> WKUPM_R {
        WKUPM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Error interrupt mask
    #[inline(always)]
    pub fn errm(&self) -> ERRM_R {
        ERRM_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Packet memory area over / underrun interrupt mask
    #[inline(always)]
    pub fn pmaovrm(&self) -> PMAOVRM_R {
        PMAOVRM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Correct transfer interrupt mask
    #[inline(always)]
    pub fn ctrm(&self) -> CTRM_R {
        CTRM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - 512 byte threshold interrupt mask
    #[inline(always)]
    pub fn thr512m(&self) -> THR512M_R {
        THR512M_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 31 - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit.
    #[inline(always)]
    pub fn host(&self) -> HOST_R {
        HOST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTR")
            .field("usbrst", &self.usbrst())
            .field("pdwn", &self.pdwn())
            .field("susprdy", &self.susprdy())
            .field("suspen", &self.suspen())
            .field("l2resume", &self.l2resume())
            .field("l1resume", &self.l1resume())
            .field("l1reqm", &self.l1reqm())
            .field("esofm", &self.esofm())
            .field("sofm", &self.sofm())
            .field("rst_dconm", &self.rst_dconm())
            .field("suspm", &self.suspm())
            .field("wkupm", &self.wkupm())
            .field("errm", &self.errm())
            .field("pmaovrm", &self.pmaovrm())
            .field("ctrm", &self.ctrm())
            .field("thr512m", &self.thr512m())
            .field("host", &self.host())
            .finish()
    }
}
impl W {
    ///Bit 0 - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software.
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<CNTRrs> {
        USBRST_W::new(self, 0)
    }
    ///Bit 1 - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used.
    #[inline(always)]
    pub fn pdwn(&mut self) -> PDWN_W<CNTRrs> {
        PDWN_W::new(self, 1)
    }
    ///Bit 3 - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY=1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set.
    #[inline(always)]
    pub fn suspen(&mut self) -> SUSPEN_W<CNTRrs> {
        SUSPEN_W::new(self, 3)
    }
    ///Bit 4 - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1ms and no more than 15ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt.
    #[inline(always)]
    pub fn l2resume(&mut self) -> L2RESUME_W<CNTRrs> {
        L2RESUME_W::new(self, 4)
    }
    ///Bit 5 - L1 remote wakeup / resume driver Device mode
    #[inline(always)]
    pub fn l1resume(&mut self) -> L1RESUME_W<CNTRrs> {
        L1RESUME_W::new(self, 5)
    }
    ///Bit 7 - LPM L1 state request interrupt mask
    #[inline(always)]
    pub fn l1reqm(&mut self) -> L1REQM_W<CNTRrs> {
        L1REQM_W::new(self, 7)
    }
    ///Bit 8 - Expected start of frame interrupt mask
    #[inline(always)]
    pub fn esofm(&mut self) -> ESOFM_W<CNTRrs> {
        ESOFM_W::new(self, 8)
    }
    ///Bit 9 - Start of frame interrupt mask
    #[inline(always)]
    pub fn sofm(&mut self) -> SOFM_W<CNTRrs> {
        SOFM_W::new(self, 9)
    }
    ///Bit 10 - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask
    #[inline(always)]
    pub fn rst_dconm(&mut self) -> RST_DCONM_W<CNTRrs> {
        RST_DCONM_W::new(self, 10)
    }
    ///Bit 11 - Suspend mode interrupt mask
    #[inline(always)]
    pub fn suspm(&mut self) -> SUSPM_W<CNTRrs> {
        SUSPM_W::new(self, 11)
    }
    ///Bit 12 - Wakeup interrupt mask
    #[inline(always)]
    pub fn wkupm(&mut self) -> WKUPM_W<CNTRrs> {
        WKUPM_W::new(self, 12)
    }
    ///Bit 13 - Error interrupt mask
    #[inline(always)]
    pub fn errm(&mut self) -> ERRM_W<CNTRrs> {
        ERRM_W::new(self, 13)
    }
    ///Bit 14 - Packet memory area over / underrun interrupt mask
    #[inline(always)]
    pub fn pmaovrm(&mut self) -> PMAOVRM_W<CNTRrs> {
        PMAOVRM_W::new(self, 14)
    }
    ///Bit 15 - Correct transfer interrupt mask
    #[inline(always)]
    pub fn ctrm(&mut self) -> CTRM_W<CNTRrs> {
        CTRM_W::new(self, 15)
    }
    ///Bit 16 - 512 byte threshold interrupt mask
    #[inline(always)]
    pub fn thr512m(&mut self) -> THR512M_W<CNTRrs> {
        THR512M_W::new(self, 16)
    }
    ///Bit 31 - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit.
    #[inline(always)]
    pub fn host(&mut self) -> HOST_W<CNTRrs> {
        HOST_W::new(self, 31)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`cntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#USB:CNTR)*/
pub struct CNTRrs;
impl crate::RegisterSpec for CNTRrs {
    type Ux = u32;
}
///`read()` method returns [`cntr::R`](R) reader structure
impl crate::Readable for CNTRrs {}
///`write(|w| ..)` method takes [`cntr::W`](W) writer structure
impl crate::Writable for CNTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CNTR to value 0x03
impl crate::Resettable for CNTRrs {
    const RESET_VALUE: u32 = 0x03;
}
