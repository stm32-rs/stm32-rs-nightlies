///Register `TIM15_SMCR` reader
pub type R = crate::R<TIM15_SMCRrs>;
///Register `TIM15_SMCR` writer
pub type W = crate::W<TIM15_SMCRrs>;
/**SMS\[2:0\]: Slave mode selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMS {
    ///0: Slave mode disabled - if CEN = 1' then the prescaler is clocked directly by the internal clock.
    B0x0 = 0,
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
    pub const fn variant(&self) -> Option<SMS> {
        match self.bits {
            0 => Some(SMS::B0x0),
            4 => Some(SMS::B0x4),
            5 => Some(SMS::B0x5),
            6 => Some(SMS::B0x6),
            7 => Some(SMS::B0x7),
            _ => None,
        }
    }
    ///Slave mode disabled - if CEN = 1' then the prescaler is clocked directly by the internal clock.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMS::B0x0
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
pub type SMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMS>;
impl<'a, REG> SMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Slave mode disabled - if CEN = 1' then the prescaler is clocked directly by the internal clock.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMS::B0x0)
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
    pub const fn variant(&self) -> Option<TS> {
        match self.bits {
            0 => Some(TS::B0x0),
            1 => Some(TS::B0x1),
            2 => Some(TS::B0x2),
            3 => Some(TS::B0x3),
            4 => Some(TS::B0x4),
            5 => Some(TS::B0x5),
            6 => Some(TS::B0x6),
            _ => None,
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
}
///Field `TS` writer - TS\[2:0\]: Trigger selection
pub type TS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TS>;
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
}
/**Master/slave mode

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
///Field `MSM` reader - Master/slave mode
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
///Field `MSM` writer - Master/slave mode
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
    ///Bits 4:6 - TS\[2:0\]: Trigger selection
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Master/slave mode
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
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
        f.debug_struct("TIM15_SMCR")
            .field("sms", &self.sms())
            .field("ts", &self.ts())
            .field("msm", &self.msm())
            .field("sms_1", &self.sms_1())
            .field("ts_1", &self.ts_1())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SMS\[2:0\]: Slave mode selection
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W<'_, TIM15_SMCRrs> {
        SMS_W::new(self, 0)
    }
    ///Bits 4:6 - TS\[2:0\]: Trigger selection
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W<'_, TIM15_SMCRrs> {
        TS_W::new(self, 4)
    }
    ///Bit 7 - Master/slave mode
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W<'_, TIM15_SMCRrs> {
        MSM_W::new(self, 7)
    }
    ///Bit 16 - SMS\[3\]
    #[inline(always)]
    pub fn sms_1(&mut self) -> SMS_1_W<'_, TIM15_SMCRrs> {
        SMS_1_W::new(self, 16)
    }
    ///Bits 20:21 - TS\[4:3\]
    #[inline(always)]
    pub fn ts_1(&mut self) -> TS_1_W<'_, TIM15_SMCRrs> {
        TS_1_W::new(self, 20)
    }
}
/**TIM15 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`tim15_smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_SMCR)*/
pub struct TIM15_SMCRrs;
impl crate::RegisterSpec for TIM15_SMCRrs {
    type Ux = u32;
}
///`read()` method returns [`tim15_smcr::R`](R) reader structure
impl crate::Readable for TIM15_SMCRrs {}
///`write(|w| ..)` method takes [`tim15_smcr::W`](W) writer structure
impl crate::Writable for TIM15_SMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM15_SMCR to value 0
impl crate::Resettable for TIM15_SMCRrs {}
