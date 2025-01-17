///Register `CCMR1_Input` reader
pub type R = crate::R<CCMR1_INPUTrs>;
///Register `CCMR1_Input` writer
pub type W = crate::W<CCMR1_INPUTrs>;
/**Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC1S {
    ///1: CC1 channel is configured as input, IC1 is mapped on TI1
    Ti1 = 1,
    ///2: CC1 channel is configured as input, IC1 is mapped on TI2
    Ti2 = 2,
    ///3: CC1 channel is configured as input, IC1 is mapped on TRC
    Trc = 3,
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
    pub const fn variant(&self) -> Option<CC1S> {
        match self.bits {
            1 => Some(CC1S::Ti1),
            2 => Some(CC1S::Ti2),
            3 => Some(CC1S::Trc),
            _ => None,
        }
    }
    ///CC1 channel is configured as input, IC1 is mapped on TI1
    #[inline(always)]
    pub fn is_ti1(&self) -> bool {
        *self == CC1S::Ti1
    }
    ///CC1 channel is configured as input, IC1 is mapped on TI2
    #[inline(always)]
    pub fn is_ti2(&self) -> bool {
        *self == CC1S::Ti2
    }
    ///CC1 channel is configured as input, IC1 is mapped on TRC
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        *self == CC1S::Trc
    }
}
///Field `CC1S` writer - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC1S>;
impl<'a, REG> CC1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CC1 channel is configured as input, IC1 is mapped on TI1
    #[inline(always)]
    pub fn ti1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S::Ti1)
    }
    ///CC1 channel is configured as input, IC1 is mapped on TI2
    #[inline(always)]
    pub fn ti2(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S::Ti2)
    }
    ///CC1 channel is configured as input, IC1 is mapped on TRC
    #[inline(always)]
    pub fn trc(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S::Trc)
    }
}
///Field `IC1PSC` reader - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E=0 (TIMx_CCER register).
pub type IC1PSC_R = crate::FieldReader;
///Field `IC1PSC` writer - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E=0 (TIMx_CCER register).
pub type IC1PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IC1F {
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
///Field `IC1F` reader - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
pub type IC1F_R = crate::FieldReader<IC1F>;
impl IC1F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IC1F {
        match self.bits {
            0 => IC1F::NoFilter,
            1 => IC1F::FckIntN2,
            2 => IC1F::FckIntN4,
            3 => IC1F::FckIntN8,
            4 => IC1F::FdtsDiv2N6,
            5 => IC1F::FdtsDiv2N8,
            6 => IC1F::FdtsDiv4N6,
            7 => IC1F::FdtsDiv4N8,
            8 => IC1F::FdtsDiv8N6,
            9 => IC1F::FdtsDiv8N8,
            10 => IC1F::FdtsDiv16N5,
            11 => IC1F::FdtsDiv16N6,
            12 => IC1F::FdtsDiv16N8,
            13 => IC1F::FdtsDiv32N5,
            14 => IC1F::FdtsDiv32N6,
            15 => IC1F::FdtsDiv32N8,
            _ => unreachable!(),
        }
    }
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == IC1F::NoFilter
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == IC1F::FckIntN2
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == IC1F::FckIntN4
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == IC1F::FckIntN8
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == IC1F::FdtsDiv2N6
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == IC1F::FdtsDiv2N8
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == IC1F::FdtsDiv4N6
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == IC1F::FdtsDiv4N8
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == IC1F::FdtsDiv8N6
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == IC1F::FdtsDiv8N8
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == IC1F::FdtsDiv16N5
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == IC1F::FdtsDiv16N6
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == IC1F::FdtsDiv16N8
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == IC1F::FdtsDiv32N5
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == IC1F::FdtsDiv32N6
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == IC1F::FdtsDiv32N8
    }
}
///Field `IC1F` writer - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
pub type IC1F_W<'a, REG> = crate::FieldWriter<'a, REG, 4, IC1F, crate::Safe>;
impl<'a, REG> IC1F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::NoFilter)
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FckIntN2)
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FckIntN4)
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FckIntN8)
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv2N6)
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv2N8)
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv4N6)
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv4N8)
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv8N6)
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv8N8)
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv16N5)
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv16N6)
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv16N8)
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv32N5)
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv32N6)
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv32N8)
    }
}
/**Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC2S {
    ///1: CC2 channel is configured as input, IC2 is mapped on TI2
    Ti2 = 1,
    ///2: CC2 channel is configured as input, IC2 is mapped on TI1
    Ti1 = 2,
    ///3: CC2 channel is configured as input, IC2 is mapped on TRC
    Trc = 3,
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
///Field `CC2S` reader - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
pub type CC2S_R = crate::FieldReader<CC2S>;
impl CC2S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CC2S> {
        match self.bits {
            1 => Some(CC2S::Ti2),
            2 => Some(CC2S::Ti1),
            3 => Some(CC2S::Trc),
            _ => None,
        }
    }
    ///CC2 channel is configured as input, IC2 is mapped on TI2
    #[inline(always)]
    pub fn is_ti2(&self) -> bool {
        *self == CC2S::Ti2
    }
    ///CC2 channel is configured as input, IC2 is mapped on TI1
    #[inline(always)]
    pub fn is_ti1(&self) -> bool {
        *self == CC2S::Ti1
    }
    ///CC2 channel is configured as input, IC2 is mapped on TRC
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        *self == CC2S::Trc
    }
}
///Field `CC2S` writer - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
pub type CC2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC2S>;
impl<'a, REG> CC2S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CC2 channel is configured as input, IC2 is mapped on TI2
    #[inline(always)]
    pub fn ti2(self) -> &'a mut crate::W<REG> {
        self.variant(CC2S::Ti2)
    }
    ///CC2 channel is configured as input, IC2 is mapped on TI1
    #[inline(always)]
    pub fn ti1(self) -> &'a mut crate::W<REG> {
        self.variant(CC2S::Ti1)
    }
    ///CC2 channel is configured as input, IC2 is mapped on TRC
    #[inline(always)]
    pub fn trc(self) -> &'a mut crate::W<REG> {
        self.variant(CC2S::Trc)
    }
}
///Field `IC2PSC` reader - Input capture 2 prescaler
pub type IC2PSC_R = crate::FieldReader;
///Field `IC2PSC` writer - Input capture 2 prescaler
pub type IC2PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC2F` reader - Input capture 2 filter
pub type IC2F_R = crate::FieldReader;
///Field `IC2F` writer - Input capture 2 filter
pub type IC2F_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    ///Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E=0 (TIMx_CCER register).
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Input capture 2 prescaler
    #[inline(always)]
    pub fn ic2psc(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:15 - Input capture 2 filter
    #[inline(always)]
    pub fn ic2f(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR1_Input")
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
    ///Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W<CCMR1_INPUTrs> {
        CC1S_W::new(self, 0)
    }
    ///Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E=0 (TIMx_CCER register).
    #[inline(always)]
    pub fn ic1psc(&mut self) -> IC1PSC_W<CCMR1_INPUTrs> {
        IC1PSC_W::new(self, 2)
    }
    ///Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
    #[inline(always)]
    pub fn ic1f(&mut self) -> IC1F_W<CCMR1_INPUTrs> {
        IC1F_W::new(self, 4)
    }
    ///Bits 8:9 - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc2s(&mut self) -> CC2S_W<CCMR1_INPUTrs> {
        CC2S_W::new(self, 8)
    }
    ///Bits 10:11 - Input capture 2 prescaler
    #[inline(always)]
    pub fn ic2psc(&mut self) -> IC2PSC_W<CCMR1_INPUTrs> {
        IC2PSC_W::new(self, 10)
    }
    ///Bits 12:15 - Input capture 2 filter
    #[inline(always)]
    pub fn ic2f(&mut self) -> IC2F_W<CCMR1_INPUTrs> {
        IC2F_W::new(self, 12)
    }
}
/**TIM4 capture/compare mode register 1 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`ccmr1_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#TIM4:CCMR1_Input)*/
pub struct CCMR1_INPUTrs;
impl crate::RegisterSpec for CCMR1_INPUTrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr1_input::R`](R) reader structure
impl crate::Readable for CCMR1_INPUTrs {}
///`write(|w| ..)` method takes [`ccmr1_input::W`](W) writer structure
impl crate::Writable for CCMR1_INPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCMR1_Input to value 0
impl crate::Resettable for CCMR1_INPUTrs {
    const RESET_VALUE: u32 = 0;
}
