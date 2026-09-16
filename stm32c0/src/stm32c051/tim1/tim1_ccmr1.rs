///Register `TIM1_CCMR1` reader
pub type R = crate::R<TIM1_CCMR1rs>;
///Register `TIM1_CCMR1` writer
pub type W = crate::W<TIM1_CCMR1rs>;
/**Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).

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
///Field `CC1S` reader - Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
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
///Field `CC1S` writer - Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
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
/**Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E= 0 (TIMx_CCER register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IC1PSC {
    ///0: no prescaler, capture is done each time an edge is detected on the capture input
    B0x0 = 0,
    ///1: capture is done once every 2 events
    B0x1 = 1,
    ///2: capture is done once every 4 events
    B0x2 = 2,
    ///3: capture is done once every 8 events
    B0x3 = 3,
}
impl From<IC1PSC> for u8 {
    #[inline(always)]
    fn from(variant: IC1PSC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IC1PSC {
    type Ux = u8;
}
impl crate::IsEnum for IC1PSC {}
///Field `IC1PSC` reader - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E= 0 (TIMx_CCER register).
pub type IC1PSC_R = crate::FieldReader<IC1PSC>;
impl IC1PSC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IC1PSC {
        match self.bits {
            0 => IC1PSC::B0x0,
            1 => IC1PSC::B0x1,
            2 => IC1PSC::B0x2,
            3 => IC1PSC::B0x3,
            _ => unreachable!(),
        }
    }
    ///no prescaler, capture is done each time an edge is detected on the capture input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IC1PSC::B0x0
    }
    ///capture is done once every 2 events
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IC1PSC::B0x1
    }
    ///capture is done once every 4 events
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == IC1PSC::B0x2
    }
    ///capture is done once every 8 events
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == IC1PSC::B0x3
    }
}
///Field `IC1PSC` writer - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E= 0 (TIMx_CCER register).
pub type IC1PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IC1PSC, crate::Safe>;
impl<'a, REG> IC1PSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///no prescaler, capture is done each time an edge is detected on the capture input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IC1PSC::B0x0)
    }
    ///capture is done once every 2 events
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IC1PSC::B0x1)
    }
    ///capture is done once every 4 events
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(IC1PSC::B0x2)
    }
    ///capture is done once every 8 events
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(IC1PSC::B0x3)
    }
}
/**Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IC1F {
    ///0: No filter, sampling is done at f<sub>DTS</sub>
    B0x0 = 0,
    ///1: f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=2
    B0x1 = 1,
    ///2: f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=4
    B0x2 = 2,
    ///3: f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=8
    B0x3 = 3,
    ///4: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/2, N=6
    B0x4 = 4,
    ///5: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/2, N=8
    B0x5 = 5,
    ///6: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/4, N=6
    B0x6 = 6,
    ///7: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/4, N=8
    B0x7 = 7,
    ///8: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/8, N=6
    B0x8 = 8,
    ///9: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/8, N=8
    B0x9 = 9,
    ///10: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=5
    B0xA = 10,
    ///11: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=6
    B0xB = 11,
    ///12: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=8
    B0xC = 12,
    ///13: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=5
    B0xD = 13,
    ///14: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=6
    B0xE = 14,
    ///15: f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=8
    B0xF = 15,
}
impl From<IC1F> for u8 {
    #[inline(always)]
    fn from(variant: IC1F) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IC1F {
    type Ux = u8;
}
impl crate::IsEnum for IC1F {}
///Field `IC1F` reader - Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
pub type IC1F_R = crate::FieldReader<IC1F>;
impl IC1F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IC1F {
        match self.bits {
            0 => IC1F::B0x0,
            1 => IC1F::B0x1,
            2 => IC1F::B0x2,
            3 => IC1F::B0x3,
            4 => IC1F::B0x4,
            5 => IC1F::B0x5,
            6 => IC1F::B0x6,
            7 => IC1F::B0x7,
            8 => IC1F::B0x8,
            9 => IC1F::B0x9,
            10 => IC1F::B0xA,
            11 => IC1F::B0xB,
            12 => IC1F::B0xC,
            13 => IC1F::B0xD,
            14 => IC1F::B0xE,
            15 => IC1F::B0xF,
            _ => unreachable!(),
        }
    }
    ///No filter, sampling is done at f<sub>DTS</sub>
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IC1F::B0x0
    }
    ///f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IC1F::B0x1
    }
    ///f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=4
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == IC1F::B0x2
    }
    ///f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=8
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == IC1F::B0x3
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/2, N=6
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == IC1F::B0x4
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/2, N=8
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == IC1F::B0x5
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/4, N=6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == IC1F::B0x6
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/4, N=8
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == IC1F::B0x7
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/8, N=6
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == IC1F::B0x8
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/8, N=8
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == IC1F::B0x9
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=5
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == IC1F::B0xA
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=6
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == IC1F::B0xB
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=8
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == IC1F::B0xC
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=5
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == IC1F::B0xD
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=6
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == IC1F::B0xE
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=8
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == IC1F::B0xF
    }
}
///Field `IC1F` writer - Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
pub type IC1F_W<'a, REG> = crate::FieldWriter<'a, REG, 4, IC1F, crate::Safe>;
impl<'a, REG> IC1F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No filter, sampling is done at f<sub>DTS</sub>
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::B0x0)
    }
    ///f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::B0x1)
    }
    ///f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::B0x2)
    }
    ///f<sub>SAMPLING</sub>=f<sub>CK_INT</sub>, N=8
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::B0x3)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/2, N=6
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::B0x4)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/2, N=8
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::B0x5)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/4, N=6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::B0x6)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/4, N=8
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::B0x7)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/8, N=6
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::B0x8)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/8, N=8
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::B0x9)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=5
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::B0xA)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=6
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::B0xB)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/16, N=8
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::B0xC)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=5
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::B0xD)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=6
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::B0xE)
    }
    ///f<sub>SAMPLING</sub>=f<sub>DTS</sub>/32, N=8
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::B0xF)
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
    ///3: CC2 channel is configured as input, IC2 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
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
    ///CC2 channel is configured as input, IC2 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
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
    ///CC2 channel is configured as input, IC2 is mapped on TRC. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CC2S::B0x3)
    }
}
///Field `IC2PSC` reader - Input capture 2 prescaler Refer to IC1PSC\[1:0\] description.
pub type IC2PSC_R = crate::FieldReader;
///Field `IC2PSC` writer - Input capture 2 prescaler Refer to IC1PSC\[1:0\] description.
pub type IC2PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC2F` reader - Input capture 2 filter Refer to IC1F\[3:0\] description.
pub type IC2F_R = crate::FieldReader;
///Field `IC2F` writer - Input capture 2 filter Refer to IC1F\[3:0\] description.
pub type IC2F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:1 - Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E= 0 (TIMx_CCER register).
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Input capture 2 prescaler Refer to IC1PSC\[1:0\] description.
    #[inline(always)]
    pub fn ic2psc(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:15 - Input capture 2 filter Refer to IC1F\[3:0\] description.
    #[inline(always)]
    pub fn ic2f(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1_CCMR1")
            .field("cc1s", &self.cc1s())
            .field("ic1psc", &self.ic1psc())
            .field("ic1f", &self.ic1f())
            .field("cc2s", &self.cc2s())
            .field("ic2psc", &self.ic2psc())
            .field("ic2f", &self.ic2f())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W<'_, TIM1_CCMR1rs> {
        CC1S_W::new(self, 0)
    }
    ///Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E= 0 (TIMx_CCER register).
    #[inline(always)]
    pub fn ic1psc(&mut self) -> IC1PSC_W<'_, TIM1_CCMR1rs> {
        IC1PSC_W::new(self, 2)
    }
    ///Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
    #[inline(always)]
    pub fn ic1f(&mut self) -> IC1F_W<'_, TIM1_CCMR1rs> {
        IC1F_W::new(self, 4)
    }
    ///Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc2s(&mut self) -> CC2S_W<'_, TIM1_CCMR1rs> {
        CC2S_W::new(self, 8)
    }
    ///Bits 10:11 - Input capture 2 prescaler Refer to IC1PSC\[1:0\] description.
    #[inline(always)]
    pub fn ic2psc(&mut self) -> IC2PSC_W<'_, TIM1_CCMR1rs> {
        IC2PSC_W::new(self, 10)
    }
    ///Bits 12:15 - Input capture 2 filter Refer to IC1F\[3:0\] description.
    #[inline(always)]
    pub fn ic2f(&mut self) -> IC2F_W<'_, TIM1_CCMR1rs> {
        IC2F_W::new(self, 12)
    }
}
/**TIM1 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`tim1_ccmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#TIM1:TIM1_CCMR1)*/
pub struct TIM1_CCMR1rs;
impl crate::RegisterSpec for TIM1_CCMR1rs {
    type Ux = u32;
}
///`read()` method returns [`tim1_ccmr1::R`](R) reader structure
impl crate::Readable for TIM1_CCMR1rs {}
///`write(|w| ..)` method takes [`tim1_ccmr1::W`](W) writer structure
impl crate::Writable for TIM1_CCMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM1_CCMR1 to value 0
impl crate::Resettable for TIM1_CCMR1rs {}
