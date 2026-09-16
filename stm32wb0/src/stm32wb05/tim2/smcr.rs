///Register `SMCR` reader
pub type R = crate::R<SMCRrs>;
///Register `SMCR` writer
pub type W = crate::W<SMCRrs>;
///Field `SMS` reader - SMS: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. 0000: Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock. 0001: Encoder mode 1 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level. 0010: Encoder mode 2 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level. 0011: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input. 0100: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers. 0101: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled. 0110: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled. 0111: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter. 1000: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter. Codes above 1000: Reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS='100'). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal.
pub type SMS_R = crate::FieldReader;
///Field `SMS` writer - SMS: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. 0000: Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock. 0001: Encoder mode 1 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level. 0010: Encoder mode 2 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level. 0011: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input. 0100: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers. 0101: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled. 0110: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled. 0111: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter. 1000: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter. Codes above 1000: Reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS='100'). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal.
pub type SMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OCCS` reader - OCCS: OCREF clear selection This bit is used to select the OCREF clear source. 0: OCREF_CLR_INT is connected to the OCREF_CLR input (stuck at 0 so no effect) 1: OCREF_CLR_INT is connected to ETRF
pub type OCCS_R = crate::BitReader;
///Field `OCCS` writer - OCCS: OCREF clear selection This bit is used to select the OCREF clear source. 0: OCREF_CLR_INT is connected to the OCREF_CLR input (stuck at 0 so no effect) 1: OCREF_CLR_INT is connected to ETRF
pub type OCCS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS` reader - TS\[2:0\]: Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. 101: Filtered Timer Input 1 (TI1FP1) 110: Filtered Timer Input 2 (TI2FP2) others: Reserved Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
pub type TS_R = crate::FieldReader;
///Field `TS` writer - TS\[2:0\]: Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. 101: Filtered Timer Input 1 (TI1FP1) 110: Filtered Timer Input 2 (TI2FP2) others: Reserved Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
pub type TS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**MSM: Master/Slave mode Not vailable in IUM. Not used in Blue51 as TRGO is not connected to any slave timer 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSM {
    ///0: No action
    NoSync = 0,
    ///1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.
    Sync = 1,
}
impl From<MSM> for bool {
    #[inline(always)]
    fn from(variant: MSM) -> Self {
        variant as u8 != 0
    }
}
///Field `MSM` reader - MSM: Master/Slave mode Not vailable in IUM. Not used in Blue51 as TRGO is not connected to any slave timer 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.
pub type MSM_R = crate::BitReader<MSM>;
impl MSM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSM {
        match self.bits {
            false => MSM::NoSync,
            true => MSM::Sync,
        }
    }
    ///No action
    #[inline(always)]
    pub fn is_no_sync(&self) -> bool {
        *self == MSM::NoSync
    }
    ///The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == MSM::Sync
    }
}
///Field `MSM` writer - MSM: Master/Slave mode Not vailable in IUM. Not used in Blue51 as TRGO is not connected to any slave timer 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.
pub type MSM_W<'a, REG> = crate::BitWriter<'a, REG, MSM>;
impl<'a, REG> MSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action
    #[inline(always)]
    pub fn no_sync(self) -> &'a mut crate::W<REG> {
        self.variant(MSM::NoSync)
    }
    ///The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(MSM::Sync)
    }
}
/**ETF\[3:0\]: External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N events are needed to validate a transition on the output: 0000: No filter, sampling is done at fDTS 0001: fSAMPLING=fCK_INT, N=2 0010: fSAMPLING=fCK_INT, N=4 0011: fSAMPLING=fCK_INT, N=8 0100: fSAMPLING=fDTS/2, N=6 0101: fSAMPLING=fDTS/2, N=8 0110: fSAMPLING=fDTS/4, N=6 0111: fSAMPLING=fDTS/4, N=8 1000: fSAMPLING=fDTS/8, N=6 1001: fSAMPLING=fDTS/8, N=8 1010: fSAMPLING=fDTS/16, N=5 1011: fSAMPLING=fDTS/16, N=6 1100: fSAMPLING=fDTS/16, N=8 1101: fSAMPLING=fDTS/32, N=5 1110: fSAMPLING=fDTS/32, N=6 1111: fSAMPLING=fDTS/32, N=8

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETF {
    ///0: No filter, sampling is done at fDTS
    NoFilter = 0,
    ///1: fSAMPLING=fCK_INT, N=2
    FckIntN2 = 1,
    ///2: fSAMPLING=fCK_INT, N=4
    FckIntN4 = 2,
    ///3: fSAMPLING=fCK_INT, N=8
    FckIntN8 = 3,
    ///4: fSAMPLING=fDTS/2, N=6
    FdtsDiv2N6 = 4,
    ///5: fSAMPLING=fDTS/2, N=8
    FdtsDiv2N8 = 5,
    ///6: fSAMPLING=fDTS/4, N=6
    FdtsDiv4N6 = 6,
    ///7: fSAMPLING=fDTS/4, N=8
    FdtsDiv4N8 = 7,
    ///8: fSAMPLING=fDTS/8, N=6
    FdtsDiv8N6 = 8,
    ///9: fSAMPLING=fDTS/8, N=8
    FdtsDiv8N8 = 9,
    ///10: fSAMPLING=fDTS/16, N=5
    FdtsDiv16N5 = 10,
    ///11: fSAMPLING=fDTS/16, N=6
    FdtsDiv16N6 = 11,
    ///12: fSAMPLING=fDTS/16, N=8
    FdtsDiv16N8 = 12,
    ///13: fSAMPLING=fDTS/32, N=5
    FdtsDiv32N5 = 13,
    ///14: fSAMPLING=fDTS/32, N=6
    FdtsDiv32N6 = 14,
    ///15: fSAMPLING=fDTS/32, N=8
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
impl crate::IsEnum for ETF {}
///Field `ETF` reader - ETF\[3:0\]: External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N events are needed to validate a transition on the output: 0000: No filter, sampling is done at fDTS 0001: fSAMPLING=fCK_INT, N=2 0010: fSAMPLING=fCK_INT, N=4 0011: fSAMPLING=fCK_INT, N=8 0100: fSAMPLING=fDTS/2, N=6 0101: fSAMPLING=fDTS/2, N=8 0110: fSAMPLING=fDTS/4, N=6 0111: fSAMPLING=fDTS/4, N=8 1000: fSAMPLING=fDTS/8, N=6 1001: fSAMPLING=fDTS/8, N=8 1010: fSAMPLING=fDTS/16, N=5 1011: fSAMPLING=fDTS/16, N=6 1100: fSAMPLING=fDTS/16, N=8 1101: fSAMPLING=fDTS/32, N=5 1110: fSAMPLING=fDTS/32, N=6 1111: fSAMPLING=fDTS/32, N=8
pub type ETF_R = crate::FieldReader<ETF>;
impl ETF_R {
    ///Get enumerated values variant
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
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == ETF::NoFilter
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == ETF::FckIntN2
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == ETF::FckIntN4
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == ETF::FckIntN8
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == ETF::FdtsDiv2N6
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == ETF::FdtsDiv2N8
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == ETF::FdtsDiv4N6
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == ETF::FdtsDiv4N8
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == ETF::FdtsDiv8N6
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == ETF::FdtsDiv8N8
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == ETF::FdtsDiv16N5
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == ETF::FdtsDiv16N6
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == ETF::FdtsDiv16N8
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == ETF::FdtsDiv32N5
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == ETF::FdtsDiv32N6
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == ETF::FdtsDiv32N8
    }
}
///Field `ETF` writer - ETF\[3:0\]: External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N events are needed to validate a transition on the output: 0000: No filter, sampling is done at fDTS 0001: fSAMPLING=fCK_INT, N=2 0010: fSAMPLING=fCK_INT, N=4 0011: fSAMPLING=fCK_INT, N=8 0100: fSAMPLING=fDTS/2, N=6 0101: fSAMPLING=fDTS/2, N=8 0110: fSAMPLING=fDTS/4, N=6 0111: fSAMPLING=fDTS/4, N=8 1000: fSAMPLING=fDTS/8, N=6 1001: fSAMPLING=fDTS/8, N=8 1010: fSAMPLING=fDTS/16, N=5 1011: fSAMPLING=fDTS/16, N=6 1100: fSAMPLING=fDTS/16, N=8 1101: fSAMPLING=fDTS/32, N=5 1110: fSAMPLING=fDTS/32, N=6 1111: fSAMPLING=fDTS/32, N=8
pub type ETF_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ETF, crate::Safe>;
impl<'a, REG> ETF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::NoFilter)
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FckIntN2)
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FckIntN4)
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FckIntN8)
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv2N6)
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv2N8)
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv4N6)
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv4N8)
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv8N6)
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv8N8)
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv16N5)
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv16N6)
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv16N8)
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv32N5)
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv32N6)
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::FdtsDiv32N8)
    }
}
/**ETPS\[1:0\]: External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of TIMxCLK frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks. 00: Prescaler OFF 01: ETRP frequency divided by 2 10: ETRP frequency divided by 4 11: ETRP frequency divided by 8

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETPS {
    ///0: Prescaler OFF
    Div1 = 0,
    ///1: ETRP frequency divided by 2
    Div2 = 1,
    ///2: ETRP frequency divided by 4
    Div4 = 2,
    ///3: ETRP frequency divided by 8
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
impl crate::IsEnum for ETPS {}
///Field `ETPS` reader - ETPS\[1:0\]: External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of TIMxCLK frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks. 00: Prescaler OFF 01: ETRP frequency divided by 2 10: ETRP frequency divided by 4 11: ETRP frequency divided by 8
pub type ETPS_R = crate::FieldReader<ETPS>;
impl ETPS_R {
    ///Get enumerated values variant
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
    ///Prescaler OFF
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == ETPS::Div1
    }
    ///ETRP frequency divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ETPS::Div2
    }
    ///ETRP frequency divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ETPS::Div4
    }
    ///ETRP frequency divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ETPS::Div8
    }
}
///Field `ETPS` writer - ETPS\[1:0\]: External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of TIMxCLK frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks. 00: Prescaler OFF 01: ETRP frequency divided by 2 10: ETRP frequency divided by 4 11: ETRP frequency divided by 8
pub type ETPS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ETPS, crate::Safe>;
impl<'a, REG> ETPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Prescaler OFF
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS::Div1)
    }
    ///ETRP frequency divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS::Div2)
    }
    ///ETRP frequency divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS::Div4)
    }
    ///ETRP frequency divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS::Div8)
    }
}
/**ECE: External clock enable This bit enables External clock mode 2. 0: External clock mode 2 disabled 1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal. Note: 1: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=111). Note: 2: It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 111). Note: 3: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECE {
    ///0: External clock mode 2 disabled
    Disabled = 0,
    ///1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal.
    Enabled = 1,
}
impl From<ECE> for bool {
    #[inline(always)]
    fn from(variant: ECE) -> Self {
        variant as u8 != 0
    }
}
///Field `ECE` reader - ECE: External clock enable This bit enables External clock mode 2. 0: External clock mode 2 disabled 1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal. Note: 1: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=111). Note: 2: It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 111). Note: 3: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF.
pub type ECE_R = crate::BitReader<ECE>;
impl ECE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECE {
        match self.bits {
            false => ECE::Disabled,
            true => ECE::Enabled,
        }
    }
    ///External clock mode 2 disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ECE::Disabled
    }
    ///External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ECE::Enabled
    }
}
///Field `ECE` writer - ECE: External clock enable This bit enables External clock mode 2. 0: External clock mode 2 disabled 1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal. Note: 1: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=111). Note: 2: It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 111). Note: 3: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF.
pub type ECE_W<'a, REG> = crate::BitWriter<'a, REG, ECE>;
impl<'a, REG> ECE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///External clock mode 2 disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ECE::Disabled)
    }
    ///External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ECE::Enabled)
    }
}
/**ETP: External trigger polarity This bit selects whether ETR or ETR is used for trigger operations 0: ETR is non-inverted, active at high level or rising edge. 1: ETR is inverted, active at low level or falling edge.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETP {
    ///0: ETR is noninverted, active at high level or rising edge
    NotInverted = 0,
    ///1: ETR is inverted, active at low level or falling edge
    Inverted = 1,
}
impl From<ETP> for bool {
    #[inline(always)]
    fn from(variant: ETP) -> Self {
        variant as u8 != 0
    }
}
///Field `ETP` reader - ETP: External trigger polarity This bit selects whether ETR or ETR is used for trigger operations 0: ETR is non-inverted, active at high level or rising edge. 1: ETR is inverted, active at low level or falling edge.
pub type ETP_R = crate::BitReader<ETP>;
impl ETP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ETP {
        match self.bits {
            false => ETP::NotInverted,
            true => ETP::Inverted,
        }
    }
    ///ETR is noninverted, active at high level or rising edge
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == ETP::NotInverted
    }
    ///ETR is inverted, active at low level or falling edge
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == ETP::Inverted
    }
}
///Field `ETP` writer - ETP: External trigger polarity This bit selects whether ETR or ETR is used for trigger operations 0: ETR is non-inverted, active at high level or rising edge. 1: ETR is inverted, active at low level or falling edge.
pub type ETP_W<'a, REG> = crate::BitWriter<'a, REG, ETP>;
impl<'a, REG> ETP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ETR is noninverted, active at high level or rising edge
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(ETP::NotInverted)
    }
    ///ETR is inverted, active at low level or falling edge
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(ETP::Inverted)
    }
}
///Field `SMS_3` reader - SMS\[3\]: Slave mode selection - bit 3 Refer to SMS description - bits2:0
pub type SMS_3_R = crate::BitReader;
///Field `SMS_3` writer - SMS\[3\]: Slave mode selection - bit 3 Refer to SMS description - bits2:0
pub type SMS_3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS2` reader - Extended trigger selection. Not used. Not available in IUM
pub type TS2_R = crate::FieldReader;
///Field `TS2` writer - Extended trigger selection. Not used. Not available in IUM
pub type TS2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - SMS: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. 0000: Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock. 0001: Encoder mode 1 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level. 0010: Encoder mode 2 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level. 0011: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input. 0100: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers. 0101: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled. 0110: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled. 0111: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter. 1000: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter. Codes above 1000: Reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS='100'). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal.
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - OCCS: OCREF clear selection This bit is used to select the OCREF clear source. 0: OCREF_CLR_INT is connected to the OCREF_CLR input (stuck at 0 so no effect) 1: OCREF_CLR_INT is connected to ETRF
    #[inline(always)]
    pub fn occs(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - TS\[2:0\]: Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. 101: Filtered Timer Input 1 (TI1FP1) 110: Filtered Timer Input 2 (TI2FP2) others: Reserved Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - MSM: Master/Slave mode Not vailable in IUM. Not used in Blue51 as TRGO is not connected to any slave timer 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - ETF\[3:0\]: External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N events are needed to validate a transition on the output: 0000: No filter, sampling is done at fDTS 0001: fSAMPLING=fCK_INT, N=2 0010: fSAMPLING=fCK_INT, N=4 0011: fSAMPLING=fCK_INT, N=8 0100: fSAMPLING=fDTS/2, N=6 0101: fSAMPLING=fDTS/2, N=8 0110: fSAMPLING=fDTS/4, N=6 0111: fSAMPLING=fDTS/4, N=8 1000: fSAMPLING=fDTS/8, N=6 1001: fSAMPLING=fDTS/8, N=8 1010: fSAMPLING=fDTS/16, N=5 1011: fSAMPLING=fDTS/16, N=6 1100: fSAMPLING=fDTS/16, N=8 1101: fSAMPLING=fDTS/32, N=5 1110: fSAMPLING=fDTS/32, N=6 1111: fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:13 - ETPS\[1:0\]: External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of TIMxCLK frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks. 00: Prescaler OFF 01: ETRP frequency divided by 2 10: ETRP frequency divided by 4 11: ETRP frequency divided by 8
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - ECE: External clock enable This bit enables External clock mode 2. 0: External clock mode 2 disabled 1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal. Note: 1: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=111). Note: 2: It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 111). Note: 3: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF.
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ETP: External trigger polarity This bit selects whether ETR or ETR is used for trigger operations 0: ETR is non-inverted, active at high level or rising edge. 1: ETR is inverted, active at low level or falling edge.
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SMS\[3\]: Slave mode selection - bit 3 Refer to SMS description - bits2:0
    #[inline(always)]
    pub fn sms_3(&self) -> SMS_3_R {
        SMS_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:21 - Extended trigger selection. Not used. Not available in IUM
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCR")
            .field("sms", &self.sms())
            .field("occs", &self.occs())
            .field("ts", &self.ts())
            .field("msm", &self.msm())
            .field("etf", &self.etf())
            .field("etps", &self.etps())
            .field("ece", &self.ece())
            .field("etp", &self.etp())
            .field("sms_3", &self.sms_3())
            .field("ts2", &self.ts2())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SMS: Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. 0000: Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock. 0001: Encoder mode 1 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level. 0010: Encoder mode 2 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level. 0011: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input. 0100: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers. 0101: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled. 0110: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled. 0111: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter. 1000: Combined reset + trigger mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter, generates an update of the registers and starts the counter. Codes above 1000: Reserved. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS='100'). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal.
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W<'_, SMCRrs> {
        SMS_W::new(self, 0)
    }
    ///Bit 3 - OCCS: OCREF clear selection This bit is used to select the OCREF clear source. 0: OCREF_CLR_INT is connected to the OCREF_CLR input (stuck at 0 so no effect) 1: OCREF_CLR_INT is connected to ETRF
    #[inline(always)]
    pub fn occs(&mut self) -> OCCS_W<'_, SMCRrs> {
        OCCS_W::new(self, 3)
    }
    ///Bits 4:6 - TS\[2:0\]: Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. 101: Filtered Timer Input 1 (TI1FP1) 110: Filtered Timer Input 2 (TI2FP2) others: Reserved Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W<'_, SMCRrs> {
        TS_W::new(self, 4)
    }
    ///Bit 7 - MSM: Master/Slave mode Not vailable in IUM. Not used in Blue51 as TRGO is not connected to any slave timer 0: No action 1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W<'_, SMCRrs> {
        MSM_W::new(self, 7)
    }
    ///Bits 8:11 - ETF\[3:0\]: External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N events are needed to validate a transition on the output: 0000: No filter, sampling is done at fDTS 0001: fSAMPLING=fCK_INT, N=2 0010: fSAMPLING=fCK_INT, N=4 0011: fSAMPLING=fCK_INT, N=8 0100: fSAMPLING=fDTS/2, N=6 0101: fSAMPLING=fDTS/2, N=8 0110: fSAMPLING=fDTS/4, N=6 0111: fSAMPLING=fDTS/4, N=8 1000: fSAMPLING=fDTS/8, N=6 1001: fSAMPLING=fDTS/8, N=8 1010: fSAMPLING=fDTS/16, N=5 1011: fSAMPLING=fDTS/16, N=6 1100: fSAMPLING=fDTS/16, N=8 1101: fSAMPLING=fDTS/32, N=5 1110: fSAMPLING=fDTS/32, N=6 1111: fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn etf(&mut self) -> ETF_W<'_, SMCRrs> {
        ETF_W::new(self, 8)
    }
    ///Bits 12:13 - ETPS\[1:0\]: External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of TIMxCLK frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks. 00: Prescaler OFF 01: ETRP frequency divided by 2 10: ETRP frequency divided by 4 11: ETRP frequency divided by 8
    #[inline(always)]
    pub fn etps(&mut self) -> ETPS_W<'_, SMCRrs> {
        ETPS_W::new(self, 12)
    }
    ///Bit 14 - ECE: External clock enable This bit enables External clock mode 2. 0: External clock mode 2 disabled 1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal. Note: 1: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=111). Note: 2: It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 111). Note: 3: If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF.
    #[inline(always)]
    pub fn ece(&mut self) -> ECE_W<'_, SMCRrs> {
        ECE_W::new(self, 14)
    }
    ///Bit 15 - ETP: External trigger polarity This bit selects whether ETR or ETR is used for trigger operations 0: ETR is non-inverted, active at high level or rising edge. 1: ETR is inverted, active at low level or falling edge.
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W<'_, SMCRrs> {
        ETP_W::new(self, 15)
    }
    ///Bit 16 - SMS\[3\]: Slave mode selection - bit 3 Refer to SMS description - bits2:0
    #[inline(always)]
    pub fn sms_3(&mut self) -> SMS_3_W<'_, SMCRrs> {
        SMS_3_W::new(self, 16)
    }
    ///Bits 20:21 - Extended trigger selection. Not used. Not available in IUM
    #[inline(always)]
    pub fn ts2(&mut self) -> TS2_W<'_, SMCRrs> {
        TS2_W::new(self, 20)
    }
}
/**SMCR register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#TIM2:SMCR)*/
pub struct SMCRrs;
impl crate::RegisterSpec for SMCRrs {
    type Ux = u32;
}
///`read()` method returns [`smcr::R`](R) reader structure
impl crate::Readable for SMCRrs {}
///`write(|w| ..)` method takes [`smcr::W`](W) writer structure
impl crate::Writable for SMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMCR to value 0
impl crate::Resettable for SMCRrs {}
