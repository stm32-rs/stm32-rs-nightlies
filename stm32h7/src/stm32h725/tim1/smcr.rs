///Register `SMCR` reader
pub type R = crate::R<SMCRrs>;
///Register `SMCR` writer
pub type W = crate::W<SMCRrs>;
/**Slave mode selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMS {
    ///0: Slave mode disabled - if CEN = ‘1 then the prescaler is clocked directly by the internal clock.
    Disabled = 0,
    ///1: Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level.
    EncoderMode1 = 1,
    ///2: Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level.
    EncoderMode2 = 2,
    ///3: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input.
    EncoderMode3 = 3,
    ///4: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers.
    ResetMode = 4,
    ///5: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled.
    GatedMode = 5,
    ///6: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled.
    TriggerMode = 6,
    ///7: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter.
    ExtClockMode = 7,
}
impl From<SMS> for u8 {
    #[inline(always)]
    fn from(variant: SMS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMS {
    type Ux = u8;
}
impl crate::IsEnum for SMS {}
///Field `SMS` reader - Slave mode selection
pub type SMS_R = crate::FieldReader<SMS>;
impl SMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMS {
        match self.bits {
            0 => SMS::Disabled,
            1 => SMS::EncoderMode1,
            2 => SMS::EncoderMode2,
            3 => SMS::EncoderMode3,
            4 => SMS::ResetMode,
            5 => SMS::GatedMode,
            6 => SMS::TriggerMode,
            7 => SMS::ExtClockMode,
            _ => unreachable!(),
        }
    }
    ///Slave mode disabled - if CEN = ‘1 then the prescaler is clocked directly by the internal clock.
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMS::Disabled
    }
    ///Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level.
    #[inline(always)]
    pub fn is_encoder_mode_1(&self) -> bool {
        *self == SMS::EncoderMode1
    }
    ///Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level.
    #[inline(always)]
    pub fn is_encoder_mode_2(&self) -> bool {
        *self == SMS::EncoderMode2
    }
    ///Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input.
    #[inline(always)]
    pub fn is_encoder_mode_3(&self) -> bool {
        *self == SMS::EncoderMode3
    }
    ///Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers.
    #[inline(always)]
    pub fn is_reset_mode(&self) -> bool {
        *self == SMS::ResetMode
    }
    ///Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled.
    #[inline(always)]
    pub fn is_gated_mode(&self) -> bool {
        *self == SMS::GatedMode
    }
    ///Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled.
    #[inline(always)]
    pub fn is_trigger_mode(&self) -> bool {
        *self == SMS::TriggerMode
    }
    ///External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter.
    #[inline(always)]
    pub fn is_ext_clock_mode(&self) -> bool {
        *self == SMS::ExtClockMode
    }
}
///Field `SMS` writer - Slave mode selection
pub type SMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMS, crate::Safe>;
impl<'a, REG> SMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Slave mode disabled - if CEN = ‘1 then the prescaler is clocked directly by the internal clock.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMS::Disabled)
    }
    ///Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level.
    #[inline(always)]
    pub fn encoder_mode_1(self) -> &'a mut crate::W<REG> {
        self.variant(SMS::EncoderMode1)
    }
    ///Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level.
    #[inline(always)]
    pub fn encoder_mode_2(self) -> &'a mut crate::W<REG> {
        self.variant(SMS::EncoderMode2)
    }
    ///Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input.
    #[inline(always)]
    pub fn encoder_mode_3(self) -> &'a mut crate::W<REG> {
        self.variant(SMS::EncoderMode3)
    }
    ///Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers.
    #[inline(always)]
    pub fn reset_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SMS::ResetMode)
    }
    ///Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled.
    #[inline(always)]
    pub fn gated_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SMS::GatedMode)
    }
    ///Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled.
    #[inline(always)]
    pub fn trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SMS::TriggerMode)
    }
    ///External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter.
    #[inline(always)]
    pub fn ext_clock_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SMS::ExtClockMode)
    }
}
/**Trigger selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TS {
    ///0: Internal Trigger 0 (ITR0)
    Itr0 = 0,
    ///1: Internal Trigger 1 (ITR1)
    Itr1 = 1,
    ///2: Internal Trigger 2 (ITR2)
    Itr2 = 2,
    ///4: TI1 Edge Detector (TI1F_ED)
    Ti1fEd = 4,
    ///5: Filtered Timer Input 1 (TI1FP1)
    Ti1fp1 = 5,
    ///6: Filtered Timer Input 2 (TI2FP2)
    Ti2fp2 = 6,
    ///7: External Trigger input (ETRF)
    Etrf = 7,
}
impl From<TS> for u8 {
    #[inline(always)]
    fn from(variant: TS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TS {
    type Ux = u8;
}
impl crate::IsEnum for TS {}
///Field `TS` reader - Trigger selection
pub type TS_R = crate::FieldReader<TS>;
impl TS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TS> {
        match self.bits {
            0 => Some(TS::Itr0),
            1 => Some(TS::Itr1),
            2 => Some(TS::Itr2),
            4 => Some(TS::Ti1fEd),
            5 => Some(TS::Ti1fp1),
            6 => Some(TS::Ti2fp2),
            7 => Some(TS::Etrf),
            _ => None,
        }
    }
    ///Internal Trigger 0 (ITR0)
    #[inline(always)]
    pub fn is_itr0(&self) -> bool {
        *self == TS::Itr0
    }
    ///Internal Trigger 1 (ITR1)
    #[inline(always)]
    pub fn is_itr1(&self) -> bool {
        *self == TS::Itr1
    }
    ///Internal Trigger 2 (ITR2)
    #[inline(always)]
    pub fn is_itr2(&self) -> bool {
        *self == TS::Itr2
    }
    ///TI1 Edge Detector (TI1F_ED)
    #[inline(always)]
    pub fn is_ti1f_ed(&self) -> bool {
        *self == TS::Ti1fEd
    }
    ///Filtered Timer Input 1 (TI1FP1)
    #[inline(always)]
    pub fn is_ti1fp1(&self) -> bool {
        *self == TS::Ti1fp1
    }
    ///Filtered Timer Input 2 (TI2FP2)
    #[inline(always)]
    pub fn is_ti2fp2(&self) -> bool {
        *self == TS::Ti2fp2
    }
    ///External Trigger input (ETRF)
    #[inline(always)]
    pub fn is_etrf(&self) -> bool {
        *self == TS::Etrf
    }
}
///Field `TS` writer - Trigger selection
pub type TS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TS>;
impl<'a, REG> TS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Internal Trigger 0 (ITR0)
    #[inline(always)]
    pub fn itr0(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Itr0)
    }
    ///Internal Trigger 1 (ITR1)
    #[inline(always)]
    pub fn itr1(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Itr1)
    }
    ///Internal Trigger 2 (ITR2)
    #[inline(always)]
    pub fn itr2(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Itr2)
    }
    ///TI1 Edge Detector (TI1F_ED)
    #[inline(always)]
    pub fn ti1f_ed(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Ti1fEd)
    }
    ///Filtered Timer Input 1 (TI1FP1)
    #[inline(always)]
    pub fn ti1fp1(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Ti1fp1)
    }
    ///Filtered Timer Input 2 (TI2FP2)
    #[inline(always)]
    pub fn ti2fp2(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Ti2fp2)
    }
    ///External Trigger input (ETRF)
    #[inline(always)]
    pub fn etrf(self) -> &'a mut crate::W<REG> {
        self.variant(TS::Etrf)
    }
}
/**Master/Slave mode

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
///Field `MSM` reader - Master/Slave mode
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
///Field `MSM` writer - Master/Slave mode
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
/**External trigger filter

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
///Field `ETF` reader - External trigger filter
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
///Field `ETF` writer - External trigger filter
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
/**External trigger prescaler

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
///Field `ETPS` reader - External trigger prescaler
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
///Field `ETPS` writer - External trigger prescaler
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
/**External clock enable

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
///Field `ECE` reader - External clock enable
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
///Field `ECE` writer - External clock enable
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
/**External trigger polarity

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
///Field `ETP` reader - External trigger polarity
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
///Field `ETP` writer - External trigger polarity
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
///Field `SMS_3` reader - Slave mode selection - bit 3
pub type SMS_3_R = crate::BitReader;
///Field `SMS_3` writer - Slave mode selection - bit 3
pub type SMS_3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS2` reader - Trigger selection - bit 4:3
pub type TS2_R = crate::FieldReader;
///Field `TS2` writer - Trigger selection - bit 4:3
pub type TS2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Trigger selection
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Master/Slave mode
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Slave mode selection - bit 3
    #[inline(always)]
    pub fn sms_3(&self) -> SMS_3_R {
        SMS_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:21 - Trigger selection - bit 4:3
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCR")
            .field("sms", &self.sms())
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
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W<SMCRrs> {
        SMS_W::new(self, 0)
    }
    ///Bits 4:6 - Trigger selection
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W<SMCRrs> {
        TS_W::new(self, 4)
    }
    ///Bit 7 - Master/Slave mode
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W<SMCRrs> {
        MSM_W::new(self, 7)
    }
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    pub fn etf(&mut self) -> ETF_W<SMCRrs> {
        ETF_W::new(self, 8)
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    pub fn etps(&mut self) -> ETPS_W<SMCRrs> {
        ETPS_W::new(self, 12)
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    pub fn ece(&mut self) -> ECE_W<SMCRrs> {
        ECE_W::new(self, 14)
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W<SMCRrs> {
        ETP_W::new(self, 15)
    }
    ///Bit 16 - Slave mode selection - bit 3
    #[inline(always)]
    pub fn sms_3(&mut self) -> SMS_3_W<SMCRrs> {
        SMS_3_W::new(self, 16)
    }
    ///Bits 20:21 - Trigger selection - bit 4:3
    #[inline(always)]
    pub fn ts2(&mut self) -> TS2_W<SMCRrs> {
        TS2_W::new(self, 20)
    }
}
/**slave mode control register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#TIM1:SMCR)*/
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
