///Register `TIM2_SMCR` reader
pub type R = crate::R<TIM2_SMCRrs>;
///Register `TIM2_SMCR` writer
pub type W = crate::W<TIM2_SMCRrs>;
/**SMS\[2:0\]: Slave mode selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMS {
    ///0: Slave mode disabled - if CEN = 1 then the prescaler is clocked directly by the internal clock.
    B0x0 = 0,
    ///1: Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level.
    B0x1 = 1,
    ///2: Encoder mode 2 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level.
    B0x2 = 2,
    ///3: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input.
    B0x3 = 3,
    ///4: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers.
    B0x4 = 4,
    ///5: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high.
    B0x5 = 5,
    ///6: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset).
    B0x6 = 6,
    ///7: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter.
    B0x7 = 7,
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
///Field `SMS` reader - SMS\[2:0\]: Slave mode selection
pub type SMS_R = crate::FieldReader<SMS>;
impl SMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMS {
        match self.bits {
            0 => SMS::B0x0,
            1 => SMS::B0x1,
            2 => SMS::B0x2,
            3 => SMS::B0x3,
            4 => SMS::B0x4,
            5 => SMS::B0x5,
            6 => SMS::B0x6,
            7 => SMS::B0x7,
            _ => unreachable!(),
        }
    }
    ///Slave mode disabled - if CEN = 1 then the prescaler is clocked directly by the internal clock.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMS::B0x0
    }
    ///Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMS::B0x1
    }
    ///Encoder mode 2 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level.
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SMS::B0x2
    }
    ///Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input.
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SMS::B0x3
    }
    ///Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers.
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == SMS::B0x4
    }
    ///Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high.
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == SMS::B0x5
    }
    ///Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset).
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == SMS::B0x6
    }
    ///External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter.
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == SMS::B0x7
    }
}
///Field `SMS` writer - SMS\[2:0\]: Slave mode selection
pub type SMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMS, crate::Safe>;
impl<'a, REG> SMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Slave mode disabled - if CEN = 1 then the prescaler is clocked directly by the internal clock.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMS::B0x0)
    }
    ///Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMS::B0x1)
    }
    ///Encoder mode 2 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level.
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMS::B0x2)
    }
    ///Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input.
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMS::B0x3)
    }
    ///Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers.
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMS::B0x4)
    }
    ///Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high.
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMS::B0x5)
    }
    ///Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset).
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMS::B0x6)
    }
    ///External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter.
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMS::B0x7)
    }
}
/**OCREF clear selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCCS {
    ///0: OCREF_CLR_INT is unconnected.
    B0x0 = 0,
    ///1: OCREF_CLR_INT is connected to ETRF
    B0x1 = 1,
}
impl From<OCCS> for bool {
    #[inline(always)]
    fn from(variant: OCCS) -> Self {
        variant as u8 != 0
    }
}
///Field `OCCS` reader - OCREF clear selection
pub type OCCS_R = crate::BitReader<OCCS>;
impl OCCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OCCS {
        match self.bits {
            false => OCCS::B0x0,
            true => OCCS::B0x1,
        }
    }
    ///OCREF_CLR_INT is unconnected.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OCCS::B0x0
    }
    ///OCREF_CLR_INT is connected to ETRF
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OCCS::B0x1
    }
}
///Field `OCCS` writer - OCREF clear selection
pub type OCCS_W<'a, REG> = crate::BitWriter<'a, REG, OCCS>;
impl<'a, REG> OCCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OCREF_CLR_INT is unconnected.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OCCS::B0x0)
    }
    ///OCREF_CLR_INT is connected to ETRF
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OCCS::B0x1)
    }
}
/**TS\[2:0\]: Trigger selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TS {
    ///0: Internal Trigger 0 (ITR0)
    B0x0 = 0,
    ///1: Internal Trigger 1 (ITR1)
    B0x1 = 1,
    ///2: Internal Trigger 2 (ITR2)
    B0x2 = 2,
    ///3: Internal Trigger 3 (ITR3)
    B0x3 = 3,
    ///4: TI1 Edge Detector (TI1F_ED)
    B0x4 = 4,
    ///5: Filtered Timer Input 1 (TI1FP1)
    B0x5 = 5,
    ///6: Filtered Timer Input 2 (TI2FP2)
    B0x6 = 6,
    ///7: External Trigger input (ETRF)
    B0x7 = 7,
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
///Field `TS` reader - TS\[2:0\]: Trigger selection
pub type TS_R = crate::FieldReader<TS>;
impl TS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TS {
        match self.bits {
            0 => TS::B0x0,
            1 => TS::B0x1,
            2 => TS::B0x2,
            3 => TS::B0x3,
            4 => TS::B0x4,
            5 => TS::B0x5,
            6 => TS::B0x6,
            7 => TS::B0x7,
            _ => unreachable!(),
        }
    }
    ///Internal Trigger 0 (ITR0)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TS::B0x0
    }
    ///Internal Trigger 1 (ITR1)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TS::B0x1
    }
    ///Internal Trigger 2 (ITR2)
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TS::B0x2
    }
    ///Internal Trigger 3 (ITR3)
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == TS::B0x3
    }
    ///TI1 Edge Detector (TI1F_ED)
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == TS::B0x4
    }
    ///Filtered Timer Input 1 (TI1FP1)
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == TS::B0x5
    }
    ///Filtered Timer Input 2 (TI2FP2)
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == TS::B0x6
    }
    ///External Trigger input (ETRF)
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == TS::B0x7
    }
}
///Field `TS` writer - TS\[2:0\]: Trigger selection
pub type TS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TS, crate::Safe>;
impl<'a, REG> TS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Internal Trigger 0 (ITR0)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TS::B0x0)
    }
    ///Internal Trigger 1 (ITR1)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TS::B0x1)
    }
    ///Internal Trigger 2 (ITR2)
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TS::B0x2)
    }
    ///Internal Trigger 3 (ITR3)
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TS::B0x3)
    }
    ///TI1 Edge Detector (TI1F_ED)
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(TS::B0x4)
    }
    ///Filtered Timer Input 1 (TI1FP1)
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(TS::B0x5)
    }
    ///Filtered Timer Input 2 (TI2FP2)
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(TS::B0x6)
    }
    ///External Trigger input (ETRF)
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(TS::B0x7)
    }
}
/**Master/Slave mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSM {
    ///0: No action
    B0x0 = 0,
    ///1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO).
    B0x1 = 1,
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
            false => MSM::B0x0,
            true => MSM::B0x1,
        }
    }
    ///No action
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MSM::B0x0
    }
    ///The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO).
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MSM::B0x1
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
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSM::B0x0)
    }
    ///The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSM::B0x1)
    }
}
/**External trigger filter

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETF {
    ///0: No filter, sampling is done at fless thansub>DTSless than/sub>
    B0x0 = 0,
    ///1: fless thansub>SAMPLINGless than/sub>=fless thansub>CK_INTless than/sub>, N=2
    B0x1 = 1,
    ///2: fless thansub>SAMPLINGless than/sub>=fless thansub>CK_INTless than/sub>, N=4
    B0x2 = 2,
    ///3: fless thansub>SAMPLINGless than/sub>=fless thansub>CK_INTless than/sub>, N=8
    B0x3 = 3,
    ///4: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/2, N=6
    B0x4 = 4,
    ///5: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/2, N=8
    B0x5 = 5,
    ///6: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/4, N=6
    B0x6 = 6,
    ///7: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/4, N=8
    B0x7 = 7,
    ///8: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/8, N=6
    B0x8 = 8,
    ///9: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/8, N=8
    B0x9 = 9,
    ///10: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/16, N=5
    B0xA = 10,
    ///11: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/16, N=6
    B0xB = 11,
    ///12: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/16, N=8
    B0xC = 12,
    ///13: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/32, N=5
    B0xD = 13,
    ///14: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/32, N=6
    B0xE = 14,
    ///15: fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/32, N=8
    B0xF = 15,
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
            0 => ETF::B0x0,
            1 => ETF::B0x1,
            2 => ETF::B0x2,
            3 => ETF::B0x3,
            4 => ETF::B0x4,
            5 => ETF::B0x5,
            6 => ETF::B0x6,
            7 => ETF::B0x7,
            8 => ETF::B0x8,
            9 => ETF::B0x9,
            10 => ETF::B0xA,
            11 => ETF::B0xB,
            12 => ETF::B0xC,
            13 => ETF::B0xD,
            14 => ETF::B0xE,
            15 => ETF::B0xF,
            _ => unreachable!(),
        }
    }
    ///No filter, sampling is done at fless thansub>DTSless than/sub>
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETF::B0x0
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>CK_INTless than/sub>, N=2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETF::B0x1
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>CK_INTless than/sub>, N=4
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ETF::B0x2
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>CK_INTless than/sub>, N=8
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ETF::B0x3
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/2, N=6
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == ETF::B0x4
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/2, N=8
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == ETF::B0x5
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/4, N=6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == ETF::B0x6
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/4, N=8
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == ETF::B0x7
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/8, N=6
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == ETF::B0x8
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/8, N=8
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == ETF::B0x9
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/16, N=5
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == ETF::B0xA
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/16, N=6
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == ETF::B0xB
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/16, N=8
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == ETF::B0xC
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/32, N=5
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == ETF::B0xD
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/32, N=6
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == ETF::B0xE
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/32, N=8
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == ETF::B0xF
    }
}
///Field `ETF` writer - External trigger filter
pub type ETF_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ETF, crate::Safe>;
impl<'a, REG> ETF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No filter, sampling is done at fless thansub>DTSless than/sub>
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::B0x0)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>CK_INTless than/sub>, N=2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::B0x1)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>CK_INTless than/sub>, N=4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::B0x2)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>CK_INTless than/sub>, N=8
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::B0x3)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/2, N=6
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::B0x4)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/2, N=8
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::B0x5)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/4, N=6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::B0x6)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/4, N=8
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::B0x7)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/8, N=6
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::B0x8)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/8, N=8
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::B0x9)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/16, N=5
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::B0xA)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/16, N=6
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::B0xB)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/16, N=8
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::B0xC)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/32, N=5
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::B0xD)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/32, N=6
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::B0xE)
    }
    ///fless thansub>SAMPLINGless than/sub>=fless thansub>DTSless than/sub>/32, N=8
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(ETF::B0xF)
    }
}
/**External trigger prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETPS {
    ///0: Prescaler OFF
    B0x0 = 0,
    ///1: ETRP frequency divided by 2
    B0x1 = 1,
    ///2: ETRP frequency divided by 4
    B0x2 = 2,
    ///3: ETRP frequency divided by 8
    B0x3 = 3,
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
            0 => ETPS::B0x0,
            1 => ETPS::B0x1,
            2 => ETPS::B0x2,
            3 => ETPS::B0x3,
            _ => unreachable!(),
        }
    }
    ///Prescaler OFF
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETPS::B0x0
    }
    ///ETRP frequency divided by 2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETPS::B0x1
    }
    ///ETRP frequency divided by 4
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ETPS::B0x2
    }
    ///ETRP frequency divided by 8
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ETPS::B0x3
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
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS::B0x0)
    }
    ///ETRP frequency divided by 2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS::B0x1)
    }
    ///ETRP frequency divided by 4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS::B0x2)
    }
    ///ETRP frequency divided by 8
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS::B0x3)
    }
}
/**External clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECE {
    ///0: External clock mode 2 disabled
    B0x0 = 0,
    ///1: External clock mode 2 enabled.
    B0x1 = 1,
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
            false => ECE::B0x0,
            true => ECE::B0x1,
        }
    }
    ///External clock mode 2 disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ECE::B0x0
    }
    ///External clock mode 2 enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ECE::B0x1
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
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ECE::B0x0)
    }
    ///External clock mode 2 enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ECE::B0x1)
    }
}
/**External trigger polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETP {
    ///0: ETR is non-inverted, active at high level or rising edge
    B0x0 = 0,
    ///1: ETR is inverted, active at low level or falling edge
    B0x1 = 1,
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
            false => ETP::B0x0,
            true => ETP::B0x1,
        }
    }
    ///ETR is non-inverted, active at high level or rising edge
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETP::B0x0
    }
    ///ETR is inverted, active at low level or falling edge
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETP::B0x1
    }
}
///Field `ETP` writer - External trigger polarity
pub type ETP_W<'a, REG> = crate::BitWriter<'a, REG, ETP>;
impl<'a, REG> ETP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ETR is non-inverted, active at high level or rising edge
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ETP::B0x0)
    }
    ///ETR is inverted, active at low level or falling edge
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ETP::B0x1)
    }
}
///Field `SMS_1` reader - SMS\[3\]
pub type SMS_1_R = crate::BitReader;
///Field `SMS_1` writer - SMS\[3\]
pub type SMS_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS_1` reader - TS\[4:3\]
pub type TS_1_R = crate::FieldReader;
///Field `TS_1` writer - TS\[4:3\]
pub type TS_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - SMS\[2:0\]: Slave mode selection
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - OCREF clear selection
    #[inline(always)]
    pub fn occs(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - TS\[2:0\]: Trigger selection
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
    ///Bit 16 - SMS\[3\]
    #[inline(always)]
    pub fn sms_1(&self) -> SMS_1_R {
        SMS_1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:21 - TS\[4:3\]
    #[inline(always)]
    pub fn ts_1(&self) -> TS_1_R {
        TS_1_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM2_SMCR")
            .field("sms", &self.sms())
            .field("occs", &self.occs())
            .field("ts", &self.ts())
            .field("msm", &self.msm())
            .field("etf", &self.etf())
            .field("etps", &self.etps())
            .field("ece", &self.ece())
            .field("etp", &self.etp())
            .field("sms_1", &self.sms_1())
            .field("ts_1", &self.ts_1())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SMS\[2:0\]: Slave mode selection
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W<'_, TIM2_SMCRrs> {
        SMS_W::new(self, 0)
    }
    ///Bit 3 - OCREF clear selection
    #[inline(always)]
    pub fn occs(&mut self) -> OCCS_W<'_, TIM2_SMCRrs> {
        OCCS_W::new(self, 3)
    }
    ///Bits 4:6 - TS\[2:0\]: Trigger selection
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W<'_, TIM2_SMCRrs> {
        TS_W::new(self, 4)
    }
    ///Bit 7 - Master/Slave mode
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W<'_, TIM2_SMCRrs> {
        MSM_W::new(self, 7)
    }
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    pub fn etf(&mut self) -> ETF_W<'_, TIM2_SMCRrs> {
        ETF_W::new(self, 8)
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    pub fn etps(&mut self) -> ETPS_W<'_, TIM2_SMCRrs> {
        ETPS_W::new(self, 12)
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    pub fn ece(&mut self) -> ECE_W<'_, TIM2_SMCRrs> {
        ECE_W::new(self, 14)
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W<'_, TIM2_SMCRrs> {
        ETP_W::new(self, 15)
    }
    ///Bit 16 - SMS\[3\]
    #[inline(always)]
    pub fn sms_1(&mut self) -> SMS_1_W<'_, TIM2_SMCRrs> {
        SMS_1_W::new(self, 16)
    }
    ///Bits 20:21 - TS\[4:3\]
    #[inline(always)]
    pub fn ts_1(&mut self) -> TS_1_W<'_, TIM2_SMCRrs> {
        TS_1_W::new(self, 20)
    }
}
/**TIM2 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`tim2_smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM2:TIM2_SMCR)*/
pub struct TIM2_SMCRrs;
impl crate::RegisterSpec for TIM2_SMCRrs {
    type Ux = u32;
}
///`read()` method returns [`tim2_smcr::R`](R) reader structure
impl crate::Readable for TIM2_SMCRrs {}
///`write(|w| ..)` method takes [`tim2_smcr::W`](W) writer structure
impl crate::Writable for TIM2_SMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM2_SMCR to value 0
impl crate::Resettable for TIM2_SMCRrs {}
