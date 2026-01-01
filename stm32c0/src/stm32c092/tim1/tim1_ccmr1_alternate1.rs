///Register `TIM1_CCMR1_ALTERNATE1` reader
pub type R = crate::R<TIM1_CCMR1_ALTERNATE1rs>;
///Register `TIM1_CCMR1_ALTERNATE1` writer
pub type W = crate::W<TIM1_CCMR1_ALTERNATE1rs>;
/**Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC1S {
    ///0: CC1 channel is configured as output
    B0x0 = 0,
    ///1: CC1 channel is configured as input, IC1 is mapped on TI1
    B0x1 = 1,
    ///2: CC1 channel is configured as input, IC1 is mapped on TI2
    B0x2 = 2,
    ///3: CC1 channel is configured as input, IC1 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
    B0x3 = 3,
}
impl From<CC1S> for u8 {
    #[inline(always)]
    fn from(variant: CC1S) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC1S {
    type Ux = u8;
}
impl crate::IsEnum for CC1S {}
///Field `CC1S` reader - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
pub type CC1S_R = crate::FieldReader<CC1S>;
impl CC1S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1S {
        match self.bits {
            0 => CC1S::B0x0,
            1 => CC1S::B0x1,
            2 => CC1S::B0x2,
            3 => CC1S::B0x3,
            _ => unreachable!(),
        }
    }
    ///CC1 channel is configured as output
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1S::B0x0
    }
    ///CC1 channel is configured as input, IC1 is mapped on TI1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1S::B0x1
    }
    ///CC1 channel is configured as input, IC1 is mapped on TI2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == CC1S::B0x2
    }
    ///CC1 channel is configured as input, IC1 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == CC1S::B0x3
    }
}
///Field `CC1S` writer - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC1S, crate::Safe>;
impl<'a, REG> CC1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CC1 channel is configured as output
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S::B0x0)
    }
    ///CC1 channel is configured as input, IC1 is mapped on TI1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S::B0x1)
    }
    ///CC1 channel is configured as input, IC1 is mapped on TI2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S::B0x2)
    }
    ///CC1 channel is configured as input, IC1 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S::B0x3)
    }
}
/**Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC1FE {
    ///0: CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles.
    B0x0 = 0,
    ///1: An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode.
    B0x1 = 1,
}
impl From<OC1FE> for bool {
    #[inline(always)]
    fn from(variant: OC1FE) -> Self {
        variant as u8 != 0
    }
}
///Field `OC1FE` reader - Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.
pub type OC1FE_R = crate::BitReader<OC1FE>;
impl OC1FE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OC1FE {
        match self.bits {
            false => OC1FE::B0x0,
            true => OC1FE::B0x1,
        }
    }
    ///CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OC1FE::B0x0
    }
    ///An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OC1FE::B0x1
    }
}
///Field `OC1FE` writer - Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.
pub type OC1FE_W<'a, REG> = crate::BitWriter<'a, REG, OC1FE>;
impl<'a, REG> OC1FE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OC1FE::B0x0)
    }
    ///An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OC1FE::B0x1)
    }
}
/**Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S= 00 (the channel is configured in output).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC1PE {
    ///0: Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately.
    B0x0 = 0,
    ///1: Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event.
    B0x1 = 1,
}
impl From<OC1PE> for bool {
    #[inline(always)]
    fn from(variant: OC1PE) -> Self {
        variant as u8 != 0
    }
}
///Field `OC1PE` reader - Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S= 00 (the channel is configured in output).
pub type OC1PE_R = crate::BitReader<OC1PE>;
impl OC1PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OC1PE {
        match self.bits {
            false => OC1PE::B0x0,
            true => OC1PE::B0x1,
        }
    }
    ///Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OC1PE::B0x0
    }
    ///Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OC1PE::B0x1
    }
}
///Field `OC1PE` writer - Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S= 00 (the channel is configured in output).
pub type OC1PE_W<'a, REG> = crate::BitWriter<'a, REG, OC1PE>;
impl<'a, REG> OC1PE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OC1PE::B0x0)
    }
    ///Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OC1PE::B0x1)
    }
}
/**OC1M\[2:0\]: Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S= 00 (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from frozen mode to PWM mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\[3\] bit is not contiguous, located in bit 16.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OC1M {
    ///0: Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.This mode can be used when the timer serves as a software timebase. When the frozen mode is enabled during timer operation, the output keeps the state (active or inactive) it had before entering the frozen state.
    B0x0 = 0,
    ///1: Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1).
    B0x1 = 1,
    ///2: Set channel 1 to inactive level on match. OC1REF signal is forced low when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1).
    B0x2 = 2,
    ///3: Toggle - OC1REF toggles when TIMx_CNT=TIMx_CCR1.
    B0x3 = 3,
    ///4: Force inactive level - OC1REF is forced low.
    B0x4 = 4,
    ///5: Force active level - OC1REF is forced high.
    B0x5 = 5,
    ///6: PWM mode 1 - In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCR1 else inactive. In downcounting, channel 1 is inactive (OC1REF= 0 ) as long as TIMx_CNT>TIMx_CCR1 else active (OC1REF= 1 ).
    B0x6 = 6,
    ///7: PWM mode 2 - In upcounting, channel 1 is inactive as long as TIMx_CNT<TIMx_CCR1 else active. In downcounting, channel 1 is active as long as TIMx_CNT>TIMx_CCR1 else inactive.
    B0x7 = 7,
}
impl From<OC1M> for u8 {
    #[inline(always)]
    fn from(variant: OC1M) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OC1M {
    type Ux = u8;
}
impl crate::IsEnum for OC1M {}
///Field `OC1M` reader - OC1M\[2:0\]: Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S= 00 (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from frozen mode to PWM mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\[3\] bit is not contiguous, located in bit 16.
pub type OC1M_R = crate::FieldReader<OC1M>;
impl OC1M_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OC1M {
        match self.bits {
            0 => OC1M::B0x0,
            1 => OC1M::B0x1,
            2 => OC1M::B0x2,
            3 => OC1M::B0x3,
            4 => OC1M::B0x4,
            5 => OC1M::B0x5,
            6 => OC1M::B0x6,
            7 => OC1M::B0x7,
            _ => unreachable!(),
        }
    }
    ///Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.This mode can be used when the timer serves as a software timebase. When the frozen mode is enabled during timer operation, the output keeps the state (active or inactive) it had before entering the frozen state.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OC1M::B0x0
    }
    ///Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1).
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OC1M::B0x1
    }
    ///Set channel 1 to inactive level on match. OC1REF signal is forced low when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1).
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OC1M::B0x2
    }
    ///Toggle - OC1REF toggles when TIMx_CNT=TIMx_CCR1.
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OC1M::B0x3
    }
    ///Force inactive level - OC1REF is forced low.
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == OC1M::B0x4
    }
    ///Force active level - OC1REF is forced high.
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == OC1M::B0x5
    }
    ///PWM mode 1 - In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCR1 else inactive. In downcounting, channel 1 is inactive (OC1REF= 0 ) as long as TIMx_CNT>TIMx_CCR1 else active (OC1REF= 1 ).
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == OC1M::B0x6
    }
    ///PWM mode 2 - In upcounting, channel 1 is inactive as long as TIMx_CNT<TIMx_CCR1 else active. In downcounting, channel 1 is active as long as TIMx_CNT>TIMx_CCR1 else inactive.
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == OC1M::B0x7
    }
}
///Field `OC1M` writer - OC1M\[2:0\]: Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S= 00 (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from frozen mode to PWM mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\[3\] bit is not contiguous, located in bit 16.
pub type OC1M_W<'a, REG> = crate::FieldWriter<'a, REG, 3, OC1M, crate::Safe>;
impl<'a, REG> OC1M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.This mode can be used when the timer serves as a software timebase. When the frozen mode is enabled during timer operation, the output keeps the state (active or inactive) it had before entering the frozen state.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::B0x0)
    }
    ///Set channel 1 to active level on match. OC1REF signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::B0x1)
    }
    ///Set channel 1 to inactive level on match. OC1REF signal is forced low when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1).
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::B0x2)
    }
    ///Toggle - OC1REF toggles when TIMx_CNT=TIMx_CCR1.
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::B0x3)
    }
    ///Force inactive level - OC1REF is forced low.
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::B0x4)
    }
    ///Force active level - OC1REF is forced high.
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::B0x5)
    }
    ///PWM mode 1 - In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCR1 else inactive. In downcounting, channel 1 is inactive (OC1REF= 0 ) as long as TIMx_CNT>TIMx_CCR1 else active (OC1REF= 1 ).
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::B0x6)
    }
    ///PWM mode 2 - In upcounting, channel 1 is inactive as long as TIMx_CNT<TIMx_CCR1 else active. In downcounting, channel 1 is active as long as TIMx_CNT>TIMx_CCR1 else inactive.
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M::B0x7)
    }
}
/**Output Compare 1 clear enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC1CE {
    ///0: OC1Ref is not affected by the ocref_clr_int signal
    B0x0 = 0,
    ///1: OC1Ref is cleared as soon as a High level is detected on ocref_clr_int signal (OCREF_CLR input or ETRF input)
    B0x1 = 1,
}
impl From<OC1CE> for bool {
    #[inline(always)]
    fn from(variant: OC1CE) -> Self {
        variant as u8 != 0
    }
}
///Field `OC1CE` reader - Output Compare 1 clear enable
pub type OC1CE_R = crate::BitReader<OC1CE>;
impl OC1CE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OC1CE {
        match self.bits {
            false => OC1CE::B0x0,
            true => OC1CE::B0x1,
        }
    }
    ///OC1Ref is not affected by the ocref_clr_int signal
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OC1CE::B0x0
    }
    ///OC1Ref is cleared as soon as a High level is detected on ocref_clr_int signal (OCREF_CLR input or ETRF input)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OC1CE::B0x1
    }
}
///Field `OC1CE` writer - Output Compare 1 clear enable
pub type OC1CE_W<'a, REG> = crate::BitWriter<'a, REG, OC1CE>;
impl<'a, REG> OC1CE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OC1Ref is not affected by the ocref_clr_int signal
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OC1CE::B0x0)
    }
    ///OC1Ref is cleared as soon as a High level is detected on ocref_clr_int signal (OCREF_CLR input or ETRF input)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OC1CE::B0x1)
    }
}
/**Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC2S {
    ///0: CC2 channel is configured as output
    B0x0 = 0,
    ///1: CC2 channel is configured as input, IC2 is mapped on TI2
    B0x1 = 1,
    ///2: CC2 channel is configured as input, IC2 is mapped on TI1
    B0x2 = 2,
    ///3: CC2 channel is configured as input, IC2 is mapped on TRC. This mode is working only if an internal trigger input is selected through the TS bit (TIMx_SMCR register)
    B0x3 = 3,
}
impl From<CC2S> for u8 {
    #[inline(always)]
    fn from(variant: CC2S) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC2S {
    type Ux = u8;
}
impl crate::IsEnum for CC2S {}
///Field `CC2S` reader - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
pub type CC2S_R = crate::FieldReader<CC2S>;
impl CC2S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC2S {
        match self.bits {
            0 => CC2S::B0x0,
            1 => CC2S::B0x1,
            2 => CC2S::B0x2,
            3 => CC2S::B0x3,
            _ => unreachable!(),
        }
    }
    ///CC2 channel is configured as output
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC2S::B0x0
    }
    ///CC2 channel is configured as input, IC2 is mapped on TI2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC2S::B0x1
    }
    ///CC2 channel is configured as input, IC2 is mapped on TI1
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == CC2S::B0x2
    }
    ///CC2 channel is configured as input, IC2 is mapped on TRC. This mode is working only if an internal trigger input is selected through the TS bit (TIMx_SMCR register)
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == CC2S::B0x3
    }
}
///Field `CC2S` writer - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
pub type CC2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC2S, crate::Safe>;
impl<'a, REG> CC2S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CC2 channel is configured as output
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC2S::B0x0)
    }
    ///CC2 channel is configured as input, IC2 is mapped on TI2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC2S::B0x1)
    }
    ///CC2 channel is configured as input, IC2 is mapped on TI1
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CC2S::B0x2)
    }
    ///CC2 channel is configured as input, IC2 is mapped on TRC. This mode is working only if an internal trigger input is selected through the TS bit (TIMx_SMCR register)
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CC2S::B0x3)
    }
}
///Field `OC2FE` reader - Output Compare 2 fast enable Refer to OC1FE description.
pub type OC2FE_R = crate::BitReader;
///Field `OC2FE` writer - Output Compare 2 fast enable Refer to OC1FE description.
pub type OC2FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC2PE` reader - Output Compare 2 preload enable Refer to OC1PE description.
pub type OC2PE_R = crate::BitReader;
///Field `OC2PE` writer - Output Compare 2 preload enable Refer to OC1PE description.
pub type OC2PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC2M` reader - OC2M\[2:0\]: Output Compare 2 mode Refer to OC1M\[3:0\] description.
pub type OC2M_R = crate::FieldReader;
///Field `OC2M` writer - OC2M\[2:0\]: Output Compare 2 mode Refer to OC1M\[3:0\] description.
pub type OC2M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OC2CE` reader - Output Compare 2 clear enable Refer to OC1CE description.
pub type OC2CE_R = crate::BitReader;
///Field `OC2CE` writer - Output Compare 2 clear enable Refer to OC1CE description.
pub type OC2CE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC1M_1` reader - OC1M\[3\]
pub type OC1M_1_R = crate::BitReader;
///Field `OC1M_1` writer - OC1M\[3\]
pub type OC1M_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OC2M_1` reader - OC2M\[3\]
pub type OC2M_1_R = crate::BitReader;
///Field `OC2M_1` writer - OC2M\[3\]
pub type OC2M_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S= 00 (the channel is configured in output).
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - OC1M\[2:0\]: Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S= 00 (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from frozen mode to PWM mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\[3\] bit is not contiguous, located in bit 16.
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Output Compare 1 clear enable
    #[inline(always)]
    pub fn oc1ce(&self) -> OC1CE_R {
        OC1CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Output Compare 2 fast enable Refer to OC1FE description.
    #[inline(always)]
    pub fn oc2fe(&self) -> OC2FE_R {
        OC2FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output Compare 2 preload enable Refer to OC1PE description.
    #[inline(always)]
    pub fn oc2pe(&self) -> OC2PE_R {
        OC2PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - OC2M\[2:0\]: Output Compare 2 mode Refer to OC1M\[3:0\] description.
    #[inline(always)]
    pub fn oc2m(&self) -> OC2M_R {
        OC2M_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Output Compare 2 clear enable Refer to OC1CE description.
    #[inline(always)]
    pub fn oc2ce(&self) -> OC2CE_R {
        OC2CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - OC1M\[3\]
    #[inline(always)]
    pub fn oc1m_1(&self) -> OC1M_1_R {
        OC1M_1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - OC2M\[3\]
    #[inline(always)]
    pub fn oc2m_1(&self) -> OC2M_1_R {
        OC2M_1_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1_CCMR1_ALTERNATE1")
            .field("cc1s", &self.cc1s())
            .field("oc1fe", &self.oc1fe())
            .field("oc1pe", &self.oc1pe())
            .field("oc1m", &self.oc1m())
            .field("oc1ce", &self.oc1ce())
            .field("cc2s", &self.cc2s())
            .field("oc2fe", &self.oc2fe())
            .field("oc2pe", &self.oc2pe())
            .field("oc2m", &self.oc2m())
            .field("oc2ce", &self.oc2ce())
            .field("oc1m_1", &self.oc1m_1())
            .field("oc2m_1", &self.oc2m_1())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W<'_, TIM1_CCMR1_ALTERNATE1rs> {
        CC1S_W::new(self, 0)
    }
    ///Bit 2 - Output Compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.
    #[inline(always)]
    pub fn oc1fe(&mut self) -> OC1FE_W<'_, TIM1_CCMR1_ALTERNATE1rs> {
        OC1FE_W::new(self, 2)
    }
    ///Bit 3 - Output Compare 1 preload enable Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S= 00 (the channel is configured in output).
    #[inline(always)]
    pub fn oc1pe(&mut self) -> OC1PE_W<'_, TIM1_CCMR1_ALTERNATE1rs> {
        OC1PE_W::new(self, 3)
    }
    ///Bits 4:6 - OC1M\[2:0\]: Output Compare 1 mode These bits define the behavior of the output reference signal OC1REF from which OC1 and OC1N are derived. OC1REF is active high whereas OC1 and OC1N active level depends on CC1P and CC1NP bits. Note: These bits can not be modified as long as LOCK level 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S= 00 (the channel is configured in output). Note: In PWM mode, the OCREF level changes only when the result of the comparison changes or when the output compare mode switches from frozen mode to PWM mode. Note: On channels having a complementary output, this bit field is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the OC1M active bits take the new value from the preloaded bits only when a COM event is generated. Note: The OC1M\[3\] bit is not contiguous, located in bit 16.
    #[inline(always)]
    pub fn oc1m(&mut self) -> OC1M_W<'_, TIM1_CCMR1_ALTERNATE1rs> {
        OC1M_W::new(self, 4)
    }
    ///Bit 7 - Output Compare 1 clear enable
    #[inline(always)]
    pub fn oc1ce(&mut self) -> OC1CE_W<'_, TIM1_CCMR1_ALTERNATE1rs> {
        OC1CE_W::new(self, 7)
    }
    ///Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc2s(&mut self) -> CC2S_W<'_, TIM1_CCMR1_ALTERNATE1rs> {
        CC2S_W::new(self, 8)
    }
    ///Bit 10 - Output Compare 2 fast enable Refer to OC1FE description.
    #[inline(always)]
    pub fn oc2fe(&mut self) -> OC2FE_W<'_, TIM1_CCMR1_ALTERNATE1rs> {
        OC2FE_W::new(self, 10)
    }
    ///Bit 11 - Output Compare 2 preload enable Refer to OC1PE description.
    #[inline(always)]
    pub fn oc2pe(&mut self) -> OC2PE_W<'_, TIM1_CCMR1_ALTERNATE1rs> {
        OC2PE_W::new(self, 11)
    }
    ///Bits 12:14 - OC2M\[2:0\]: Output Compare 2 mode Refer to OC1M\[3:0\] description.
    #[inline(always)]
    pub fn oc2m(&mut self) -> OC2M_W<'_, TIM1_CCMR1_ALTERNATE1rs> {
        OC2M_W::new(self, 12)
    }
    ///Bit 15 - Output Compare 2 clear enable Refer to OC1CE description.
    #[inline(always)]
    pub fn oc2ce(&mut self) -> OC2CE_W<'_, TIM1_CCMR1_ALTERNATE1rs> {
        OC2CE_W::new(self, 15)
    }
    ///Bit 16 - OC1M\[3\]
    #[inline(always)]
    pub fn oc1m_1(&mut self) -> OC1M_1_W<'_, TIM1_CCMR1_ALTERNATE1rs> {
        OC1M_1_W::new(self, 16)
    }
    ///Bit 24 - OC2M\[3\]
    #[inline(always)]
    pub fn oc2m_1(&mut self) -> OC2M_1_W<'_, TIM1_CCMR1_ALTERNATE1rs> {
        OC2M_1_W::new(self, 24)
    }
}
/**TIM1 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim1_ccmr1_alternate1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccmr1_alternate1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CCMR1_ALTERNATE1)*/
pub struct TIM1_CCMR1_ALTERNATE1rs;
impl crate::RegisterSpec for TIM1_CCMR1_ALTERNATE1rs {
    type Ux = u32;
}
///`read()` method returns [`tim1_ccmr1_alternate1::R`](R) reader structure
impl crate::Readable for TIM1_CCMR1_ALTERNATE1rs {}
///`write(|w| ..)` method takes [`tim1_ccmr1_alternate1::W`](W) writer structure
impl crate::Writable for TIM1_CCMR1_ALTERNATE1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM1_CCMR1_ALTERNATE1 to value 0
impl crate::Resettable for TIM1_CCMR1_ALTERNATE1rs {}
