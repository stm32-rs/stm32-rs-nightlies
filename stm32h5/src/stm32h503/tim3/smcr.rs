#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SMCRrs>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SMCRrs>;
#[doc = "Field `SMS_L` reader - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type SMS_L_R = crate::FieldReader;
#[doc = "Field `SMS_L` writer - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type SMS_L_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Field `OCCS` reader - OCREF clear selection This bit is used to select the OCREF clear source Note: If the OCREF clear selection feature is not supported, this bit is reserved and forced by hardware to ‘0’. ."]
pub type OCCS_R = crate::BitReader;
#[doc = "Field `OCCS` writer - OCREF clear selection This bit is used to select the OCREF clear source Note: If the OCREF clear selection feature is not supported, this bit is reserved and forced by hardware to ‘0’. ."]
pub type OCCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_L` reader - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type TS_L_R = crate::FieldReader;
#[doc = "Field `TS_L` writer - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type TS_L_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Master/Slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSM {
    #[doc = "0: No action"]
    NoSync = 0,
    #[doc = "1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    Sync = 1,
}
impl From<MSM> for bool {
    #[inline(always)]
    fn from(variant: MSM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSM` reader - Master/Slave mode"]
pub type MSM_R = crate::BitReader<MSM>;
impl MSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSM {
        match self.bits {
            false => MSM::NoSync,
            true => MSM::Sync,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_no_sync(&self) -> bool {
        *self == MSM::NoSync
    }
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == MSM::Sync
    }
}
#[doc = "Field `MSM` writer - Master/Slave mode"]
pub type MSM_W<'a, REG> = crate::BitWriter<'a, REG, MSM>;
impl<'a, REG> MSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_sync(self) -> &'a mut crate::W<REG> {
        self.variant(MSM::NoSync)
    }
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(MSM::Sync)
    }
}
#[doc = "External trigger filter This bit-field then defines the frequency used to sample tim_etrp signal and the length of the digital filter applied to tim_etrp. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETF {
    #[doc = "0: No filter, sampling is done at fDTS"]
    NoFilter = 0,
    #[doc = "1: fSAMPLING=fCK_INT, N=2"]
    FckIntN2 = 1,
    #[doc = "2: fSAMPLING=fCK_INT, N=4"]
    FckIntN4 = 2,
    #[doc = "3: fSAMPLING=fCK_INT, N=8"]
    FckIntN8 = 3,
    #[doc = "4: fSAMPLING=fDTS/2, N=6"]
    FdtsDiv2N6 = 4,
    #[doc = "5: fSAMPLING=fDTS/2, N=8"]
    FdtsDiv2N8 = 5,
    #[doc = "6: fSAMPLING=fDTS/4, N=6"]
    FdtsDiv4N6 = 6,
    #[doc = "7: fSAMPLING=fDTS/4, N=8"]
    FdtsDiv4N8 = 7,
    #[doc = "8: fSAMPLING=fDTS/8, N=6"]
    FdtsDiv8N6 = 8,
    #[doc = "9: fSAMPLING=fDTS/8, N=8"]
    FdtsDiv8N8 = 9,
    #[doc = "10: fSAMPLING=fDTS/16, N=5"]
    FdtsDiv16N5 = 10,
    #[doc = "11: fSAMPLING=fDTS/16, N=6"]
    FdtsDiv16N6 = 11,
    #[doc = "12: fSAMPLING=fDTS/16, N=8"]
    FdtsDiv16N8 = 12,
    #[doc = "13: fSAMPLING=fDTS/32, N=5"]
    FdtsDiv32N5 = 13,
    #[doc = "14: fSAMPLING=fDTS/32, N=6"]
    FdtsDiv32N6 = 14,
    #[doc = "15: fSAMPLING=fDTS/32, N=8"]
    FdtsDiv32N8 = 15,
}
impl From<ETF> for u8 {
    #[inline(always)]
    fn from(variant: ETF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETF {
    type Ux = u8;
}
#[doc = "Field `ETF` reader - External trigger filter This bit-field then defines the frequency used to sample tim_etrp signal and the length of the digital filter applied to tim_etrp. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type ETF_R = crate::FieldReader<ETF>;
impl ETF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETF {
        match self.bits {
            0 => ETF::NoFilter,
            1 => ETF::FckIntN2,
            2 => ETF::FckIntN4,
            3 => ETF::FckIntN8,
            4 => ETF::FdtsDiv2N6,
            5 => ETF::FdtsDiv2N8,
            6 => ETF::FdtsDiv4N6,
            7 => ETF::FdtsDiv4N8,
            8 => ETF::FdtsDiv8N6,
            9 => ETF::FdtsDiv8N8,
            10 => ETF::FdtsDiv16N5,
            11 => ETF::FdtsDiv16N6,
            12 => ETF::FdtsDiv16N8,
            13 => ETF::FdtsDiv32N5,
            14 => ETF::FdtsDiv32N6,
            15 => ETF::FdtsDiv32N8,
            _ => unreachable!(),
        }
    }
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == ETF::NoFilter
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == ETF::FckIntN2
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == ETF::FckIntN4
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == ETF::FckIntN8
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == ETF::FdtsDiv2N6
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == ETF::FdtsDiv2N8
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == ETF::FdtsDiv4N6
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == ETF::FdtsDiv4N8
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == ETF::FdtsDiv8N6
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == ETF::FdtsDiv8N8
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == ETF::FdtsDiv16N5
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == ETF::FdtsDiv16N6
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == ETF::FdtsDiv16N8
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == ETF::FdtsDiv32N5
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == ETF::FdtsDiv32N6
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == ETF::FdtsDiv32N8
    }
}
#[doc = "Field `ETF` writer - External trigger filter This bit-field then defines the frequency used to sample tim_etrp signal and the length of the digital filter applied to tim_etrp. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type ETF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, ETF>;
impl<'a, REG> ETF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::NoFilter)
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FckIntN2)
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FckIntN4)
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FckIntN8)
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv2N6)
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv2N8)
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv4N6)
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv4N8)
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv8N6)
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv8N8)
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv16N5)
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv16N6)
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv16N8)
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv32N5)
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv32N6)
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv32N8)
    }
}
#[doc = "External trigger prescaler External trigger signal tim_etrp frequency must be at most 1/4 of tim_ker_ck frequency. A prescaler can be enabled to reduce tim_etrp frequency. It is useful when inputting fast external clocks on tim_etr_in.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETPS {
    #[doc = "0: Prescaler OFF"]
    Div1 = 0,
    #[doc = "1: ETRP frequency divided by 2"]
    Div2 = 1,
    #[doc = "2: ETRP frequency divided by 4"]
    Div4 = 2,
    #[doc = "3: ETRP frequency divided by 8"]
    Div8 = 3,
}
impl From<ETPS> for u8 {
    #[inline(always)]
    fn from(variant: ETPS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETPS {
    type Ux = u8;
}
#[doc = "Field `ETPS` reader - External trigger prescaler External trigger signal tim_etrp frequency must be at most 1/4 of tim_ker_ck frequency. A prescaler can be enabled to reduce tim_etrp frequency. It is useful when inputting fast external clocks on tim_etr_in."]
pub type ETPS_R = crate::FieldReader<ETPS>;
impl ETPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETPS {
        match self.bits {
            0 => ETPS::Div1,
            1 => ETPS::Div2,
            2 => ETPS::Div4,
            3 => ETPS::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "Prescaler OFF"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == ETPS::Div1
    }
    #[doc = "ETRP frequency divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ETPS::Div2
    }
    #[doc = "ETRP frequency divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ETPS::Div4
    }
    #[doc = "ETRP frequency divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ETPS::Div8
    }
}
#[doc = "Field `ETPS` writer - External trigger prescaler External trigger signal tim_etrp frequency must be at most 1/4 of tim_ker_ck frequency. A prescaler can be enabled to reduce tim_etrp frequency. It is useful when inputting fast external clocks on tim_etr_in."]
pub type ETPS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ETPS>;
impl<'a, REG> ETPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler OFF"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS::Div1)
    }
    #[doc = "ETRP frequency divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS::Div2)
    }
    #[doc = "ETRP frequency divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS::Div4)
    }
    #[doc = "ETRP frequency divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS::Div8)
    }
}
#[doc = "External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with tim_trgi connected to tim_etrf (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, tim_trgi must not be connected to tim_etrf in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is tim_etrf.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECE {
    #[doc = "0: External clock mode 2 disabled"]
    Disabled = 0,
    #[doc = "1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    Enabled = 1,
}
impl From<ECE> for bool {
    #[inline(always)]
    fn from(variant: ECE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECE` reader - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with tim_trgi connected to tim_etrf (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, tim_trgi must not be connected to tim_etrf in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is tim_etrf."]
pub type ECE_R = crate::BitReader<ECE>;
impl ECE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECE {
        match self.bits {
            false => ECE::Disabled,
            true => ECE::Enabled,
        }
    }
    #[doc = "External clock mode 2 disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ECE::Disabled
    }
    #[doc = "External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ECE::Enabled
    }
}
#[doc = "Field `ECE` writer - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with tim_trgi connected to tim_etrf (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, tim_trgi must not be connected to tim_etrf in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is tim_etrf."]
pub type ECE_W<'a, REG> = crate::BitWriter<'a, REG, ECE>;
impl<'a, REG> ECE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External clock mode 2 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ECE::Disabled)
    }
    #[doc = "External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ECE::Enabled)
    }
}
#[doc = "External trigger polarity This bit selects whether tim_etr_in or tim_etr_in is used for trigger operations\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETP {
    #[doc = "0: ETR is noninverted, active at high level or rising edge"]
    NotInverted = 0,
    #[doc = "1: ETR is inverted, active at low level or falling edge"]
    Inverted = 1,
}
impl From<ETP> for bool {
    #[inline(always)]
    fn from(variant: ETP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETP` reader - External trigger polarity This bit selects whether tim_etr_in or tim_etr_in is used for trigger operations"]
pub type ETP_R = crate::BitReader<ETP>;
impl ETP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETP {
        match self.bits {
            false => ETP::NotInverted,
            true => ETP::Inverted,
        }
    }
    #[doc = "ETR is noninverted, active at high level or rising edge"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == ETP::NotInverted
    }
    #[doc = "ETR is inverted, active at low level or falling edge"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == ETP::Inverted
    }
}
#[doc = "Field `ETP` writer - External trigger polarity This bit selects whether tim_etr_in or tim_etr_in is used for trigger operations"]
pub type ETP_W<'a, REG> = crate::BitWriter<'a, REG, ETP>;
impl<'a, REG> ETP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ETR is noninverted, active at high level or rising edge"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(ETP::NotInverted)
    }
    #[doc = "ETR is inverted, active at low level or falling edge"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(ETP::Inverted)
    }
}
#[doc = "Field `SMS_H` reader - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type SMS_H_R = crate::BitReader;
#[doc = "Field `SMS_H` writer - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type SMS_H_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_H` reader - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type TS_H_R = crate::FieldReader;
#[doc = "Field `TS_H` writer - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type TS_H_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "SMS preload enable This bit selects whether the SMS\\[3:0\\]
bitfield is preloaded\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMSPE {
    #[doc = "0: SMSM\\[3:0\\]
is not preloaded"]
    NotPreloaded = 0,
    #[doc = "1: SMSM\\[3:0\\]
is preload is enabled"]
    PreloadEnabled = 1,
}
impl From<SMSPE> for bool {
    #[inline(always)]
    fn from(variant: SMSPE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMSPE` reader - SMS preload enable This bit selects whether the SMS\\[3:0\\]
bitfield is preloaded"]
pub type SMSPE_R = crate::BitReader<SMSPE>;
impl SMSPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMSPE {
        match self.bits {
            false => SMSPE::NotPreloaded,
            true => SMSPE::PreloadEnabled,
        }
    }
    #[doc = "SMSM\\[3:0\\]
is not preloaded"]
    #[inline(always)]
    pub fn is_not_preloaded(&self) -> bool {
        *self == SMSPE::NotPreloaded
    }
    #[doc = "SMSM\\[3:0\\]
is preload is enabled"]
    #[inline(always)]
    pub fn is_preload_enabled(&self) -> bool {
        *self == SMSPE::PreloadEnabled
    }
}
#[doc = "Field `SMSPE` writer - SMS preload enable This bit selects whether the SMS\\[3:0\\]
bitfield is preloaded"]
pub type SMSPE_W<'a, REG> = crate::BitWriter<'a, REG, SMSPE>;
impl<'a, REG> SMSPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMSM\\[3:0\\]
is not preloaded"]
    #[inline(always)]
    pub fn not_preloaded(self) -> &'a mut crate::W<REG> {
        self.variant(SMSPE::NotPreloaded)
    }
    #[doc = "SMSM\\[3:0\\]
is preload is enabled"]
    #[inline(always)]
    pub fn preload_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMSPE::PreloadEnabled)
    }
}
#[doc = "SMS preload source This bit selects whether the events that triggers the SMS\\[3:0\\]
bitfield transfer from preload to active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMSPS {
    #[doc = "0: SMSM\\[3:0\\]
is preloaded from Update event"]
    Update = 0,
    #[doc = "1: SMSM\\[3:0\\]
is preloaded from Index event"]
    Index = 1,
}
impl From<SMSPS> for bool {
    #[inline(always)]
    fn from(variant: SMSPS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMSPS` reader - SMS preload source This bit selects whether the events that triggers the SMS\\[3:0\\]
bitfield transfer from preload to active"]
pub type SMSPS_R = crate::BitReader<SMSPS>;
impl SMSPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMSPS {
        match self.bits {
            false => SMSPS::Update,
            true => SMSPS::Index,
        }
    }
    #[doc = "SMSM\\[3:0\\]
is preloaded from Update event"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == SMSPS::Update
    }
    #[doc = "SMSM\\[3:0\\]
is preloaded from Index event"]
    #[inline(always)]
    pub fn is_index(&self) -> bool {
        *self == SMSPS::Index
    }
}
#[doc = "Field `SMSPS` writer - SMS preload source This bit selects whether the events that triggers the SMS\\[3:0\\]
bitfield transfer from preload to active"]
pub type SMSPS_W<'a, REG> = crate::BitWriter<'a, REG, SMSPS>;
impl<'a, REG> SMSPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMSM\\[3:0\\]
is preloaded from Update event"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(SMSPS::Update)
    }
    #[doc = "SMSM\\[3:0\\]
is preloaded from Index event"]
    #[inline(always)]
    pub fn index(self) -> &'a mut crate::W<REG> {
        self.variant(SMSPS::Index)
    }
}
impl R {
    #[doc = "Bits 0:2 - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms_l(&self) -> SMS_L_R {
        SMS_L_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - OCREF clear selection This bit is used to select the OCREF clear source Note: If the OCREF clear selection feature is not supported, this bit is reserved and forced by hardware to ‘0’. ."]
    #[inline(always)]
    pub fn occs(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts_l(&self) -> TS_L_R {
        TS_L_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - External trigger filter This bit-field then defines the frequency used to sample tim_etrp signal and the length of the digital filter applied to tim_etrp. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler External trigger signal tim_etrp frequency must be at most 1/4 of tim_ker_ck frequency. A prescaler can be enabled to reduce tim_etrp frequency. It is useful when inputting fast external clocks on tim_etr_in."]
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with tim_trgi connected to tim_etrf (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, tim_trgi must not be connected to tim_etrf in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is tim_etrf."]
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External trigger polarity This bit selects whether tim_etr_in or tim_etr_in is used for trigger operations"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms_h(&self) -> SMS_H_R {
        SMS_H_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts_h(&self) -> TS_H_R {
        TS_H_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - SMS preload enable This bit selects whether the SMS\\[3:0\\]
bitfield is preloaded"]
    #[inline(always)]
    pub fn smspe(&self) -> SMSPE_R {
        SMSPE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SMS preload source This bit selects whether the events that triggers the SMS\\[3:0\\]
bitfield transfer from preload to active"]
    #[inline(always)]
    pub fn smsps(&self) -> SMSPS_R {
        SMSPS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn sms_l(&mut self) -> SMS_L_W<SMCRrs> {
        SMS_L_W::new(self, 0)
    }
    #[doc = "Bit 3 - OCREF clear selection This bit is used to select the OCREF clear source Note: If the OCREF clear selection feature is not supported, this bit is reserved and forced by hardware to ‘0’. ."]
    #[inline(always)]
    #[must_use]
    pub fn occs(&mut self) -> OCCS_W<SMCRrs> {
        OCCS_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    #[must_use]
    pub fn ts_l(&mut self) -> TS_L_W<SMCRrs> {
        TS_L_W::new(self, 4)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<SMCRrs> {
        MSM_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - External trigger filter This bit-field then defines the frequency used to sample tim_etrp signal and the length of the digital filter applied to tim_etrp. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    #[must_use]
    pub fn etf(&mut self) -> ETF_W<SMCRrs> {
        ETF_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler External trigger signal tim_etrp frequency must be at most 1/4 of tim_ker_ck frequency. A prescaler can be enabled to reduce tim_etrp frequency. It is useful when inputting fast external clocks on tim_etr_in."]
    #[inline(always)]
    #[must_use]
    pub fn etps(&mut self) -> ETPS_W<SMCRrs> {
        ETPS_W::new(self, 12)
    }
    #[doc = "Bit 14 - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with tim_trgi connected to tim_etrf (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, tim_trgi must not be connected to tim_etrf in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is tim_etrf."]
    #[inline(always)]
    #[must_use]
    pub fn ece(&mut self) -> ECE_W<SMCRrs> {
        ECE_W::new(self, 14)
    }
    #[doc = "Bit 15 - External trigger polarity This bit selects whether tim_etr_in or tim_etr_in is used for trigger operations"]
    #[inline(always)]
    #[must_use]
    pub fn etp(&mut self) -> ETP_W<SMCRrs> {
        ETP_W::new(self, 15)
    }
    #[doc = "Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn sms_h(&mut self) -> SMS_H_W<SMCRrs> {
        SMS_H_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    #[must_use]
    pub fn ts_h(&mut self) -> TS_H_W<SMCRrs> {
        TS_H_W::new(self, 20)
    }
    #[doc = "Bit 24 - SMS preload enable This bit selects whether the SMS\\[3:0\\]
bitfield is preloaded"]
    #[inline(always)]
    #[must_use]
    pub fn smspe(&mut self) -> SMSPE_W<SMCRrs> {
        SMSPE_W::new(self, 24)
    }
    #[doc = "Bit 25 - SMS preload source This bit selects whether the events that triggers the SMS\\[3:0\\]
bitfield transfer from preload to active"]
    #[inline(always)]
    #[must_use]
    pub fn smsps(&mut self) -> SMSPS_W<SMCRrs> {
        SMSPS_W::new(self, 25)
    }
}
#[doc = "TIM3 slave mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMCRrs;
impl crate::RegisterSpec for SMCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcr::R`](R) reader structure"]
impl crate::Readable for SMCRrs {}
#[doc = "`write(|w| ..)` method takes [`smcr::W`](W) writer structure"]
impl crate::Writable for SMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMCR to value 0"]
impl crate::Resettable for SMCRrs {
    const RESET_VALUE: u32 = 0;
}
