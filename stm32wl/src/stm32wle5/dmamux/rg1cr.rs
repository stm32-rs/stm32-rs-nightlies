#[doc = "Register `RG1CR` reader"]
pub struct R(crate::R<RG1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RG1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RG1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RG1CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RG1CR` writer"]
pub struct W(crate::W<RG1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RG1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RG1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RG1CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GNBREQ` reader - Number of DMA requests to be generated (minus 1)"]
pub struct GNBREQ_R(crate::FieldReader<u8, u8>);
impl GNBREQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        GNBREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GNBREQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GNBREQ` writer - Number of DMA requests to be generated (minus 1)"]
pub struct GNBREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> GNBREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "DMA request generator trigger polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPOL_A {
    #[doc = "0: No event, i.e. no detection nor generation"]
    NOEDGE = 0,
    #[doc = "1: Rising edge"]
    RISINGEDGE = 1,
    #[doc = "2: Falling edge"]
    FALLINGEDGE = 2,
    #[doc = "3: Rising and falling edges"]
    BOTHEDGES = 3,
}
impl From<GPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: GPOL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPOL` reader - DMA request generator trigger polarity"]
pub struct GPOL_R(crate::FieldReader<u8, GPOL_A>);
impl GPOL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPOL_A {
        match self.bits {
            0 => GPOL_A::NOEDGE,
            1 => GPOL_A::RISINGEDGE,
            2 => GPOL_A::FALLINGEDGE,
            3 => GPOL_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOEDGE`"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        **self == GPOL_A::NOEDGE
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == GPOL_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == GPOL_A::FALLINGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        **self == GPOL_A::BOTHEDGES
    }
}
impl core::ops::Deref for GPOL_R {
    type Target = crate::FieldReader<u8, GPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPOL` writer - DMA request generator trigger polarity"]
pub struct GPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPOL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No event, i.e. no detection nor generation"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(GPOL_A::NOEDGE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(GPOL_A::RISINGEDGE)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(GPOL_A::FALLINGEDGE)
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(GPOL_A::BOTHEDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "DMA request generator channel x enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GE_A {
    #[doc = "0: DMA request generation disabled"]
    DISABLED = 0,
    #[doc = "1: DMA request enabled"]
    ENABLED = 1,
}
impl From<GE_A> for bool {
    #[inline(always)]
    fn from(variant: GE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GE` reader - DMA request generator channel x enable"]
pub struct GE_R(crate::FieldReader<bool, GE_A>);
impl GE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GE_A {
        match self.bits {
            false => GE_A::DISABLED,
            true => GE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == GE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == GE_A::ENABLED
    }
}
impl core::ops::Deref for GE_R {
    type Target = crate::FieldReader<bool, GE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GE` writer - DMA request generator channel x enable"]
pub struct GE_W<'a> {
    w: &'a mut W,
}
impl<'a> GE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA request generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GE_A::DISABLED)
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GE_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Trigger overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIE_A {
    #[doc = "0: Trigger overrun interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Trigger overrun interrupt enabled"]
    ENABLED = 1,
}
impl From<OIE_A> for bool {
    #[inline(always)]
    fn from(variant: OIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIE` reader - Trigger overrun interrupt enable"]
pub struct OIE_R(crate::FieldReader<bool, OIE_A>);
impl OIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OIE_A {
        match self.bits {
            false => OIE_A::DISABLED,
            true => OIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OIE_A::ENABLED
    }
}
impl core::ops::Deref for OIE_R {
    type Target = crate::FieldReader<bool, OIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIE` writer - Trigger overrun interrupt enable"]
pub struct OIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger overrun interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OIE_A::DISABLED)
    }
    #[doc = "Trigger overrun interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OIE_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Signal identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIG_ID_A {
    #[doc = "0: Signal `EXTIx` selected as synchronization input"]
    EXTI0 = 0,
    #[doc = "1: Signal `EXTIx` selected as synchronization input"]
    EXTI1 = 1,
    #[doc = "2: Signal `EXTIx` selected as synchronization input"]
    EXTI2 = 2,
    #[doc = "3: Signal `EXTIx` selected as synchronization input"]
    EXTI3 = 3,
    #[doc = "4: Signal `EXTIx` selected as synchronization input"]
    EXTI4 = 4,
    #[doc = "5: Signal `EXTIx` selected as synchronization input"]
    EXTI5 = 5,
    #[doc = "6: Signal `EXTIx` selected as synchronization input"]
    EXTI6 = 6,
    #[doc = "7: Signal `EXTIx` selected as synchronization input"]
    EXTI7 = 7,
    #[doc = "8: Signal `EXTIx` selected as synchronization input"]
    EXTI8 = 8,
    #[doc = "9: Signal `EXTIx` selected as synchronization input"]
    EXTI9 = 9,
    #[doc = "10: Signal `EXTIx` selected as synchronization input"]
    EXTI10 = 10,
    #[doc = "11: Signal `EXTIx` selected as synchronization input"]
    EXTI11 = 11,
    #[doc = "12: Signal `EXTIx` selected as synchronization input"]
    EXTI12 = 12,
    #[doc = "13: Signal `EXTIx` selected as synchronization input"]
    EXTI13 = 13,
    #[doc = "14: Signal `EXTIx` selected as synchronization input"]
    EXTI14 = 14,
    #[doc = "15: Signal `EXTIx` selected as synchronization input"]
    EXTI15 = 15,
    #[doc = "16: Signal `dmamux1_evt0` selected as synchronization input"]
    DMAMUX1_EVT0 = 16,
    #[doc = "17: Signal `dmamux1_evt1` selected as synchronization input"]
    DMAMUX1_EVT1 = 17,
    #[doc = "18: Signal `lptim1_out` selected as synchronization input"]
    LPTIM1_OUT = 18,
    #[doc = "19: Signal `lptim2_out` selected as synchronization input"]
    LPTIM2_OUT = 19,
    #[doc = "20: Signal `lptim3_out` selected as synchronization input"]
    LPTIM3_OUT = 20,
}
impl From<SIG_ID_A> for u8 {
    #[inline(always)]
    fn from(variant: SIG_ID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SIG_ID` reader - Signal identification"]
pub struct SIG_ID_R(crate::FieldReader<u8, SIG_ID_A>);
impl SIG_ID_R {
    pub(crate) fn new(bits: u8) -> Self {
        SIG_ID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIG_ID_A> {
        match self.bits {
            0 => Some(SIG_ID_A::EXTI0),
            1 => Some(SIG_ID_A::EXTI1),
            2 => Some(SIG_ID_A::EXTI2),
            3 => Some(SIG_ID_A::EXTI3),
            4 => Some(SIG_ID_A::EXTI4),
            5 => Some(SIG_ID_A::EXTI5),
            6 => Some(SIG_ID_A::EXTI6),
            7 => Some(SIG_ID_A::EXTI7),
            8 => Some(SIG_ID_A::EXTI8),
            9 => Some(SIG_ID_A::EXTI9),
            10 => Some(SIG_ID_A::EXTI10),
            11 => Some(SIG_ID_A::EXTI11),
            12 => Some(SIG_ID_A::EXTI12),
            13 => Some(SIG_ID_A::EXTI13),
            14 => Some(SIG_ID_A::EXTI14),
            15 => Some(SIG_ID_A::EXTI15),
            16 => Some(SIG_ID_A::DMAMUX1_EVT0),
            17 => Some(SIG_ID_A::DMAMUX1_EVT1),
            18 => Some(SIG_ID_A::LPTIM1_OUT),
            19 => Some(SIG_ID_A::LPTIM2_OUT),
            20 => Some(SIG_ID_A::LPTIM3_OUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EXTI0`"]
    #[inline(always)]
    pub fn is_exti0(&self) -> bool {
        **self == SIG_ID_A::EXTI0
    }
    #[doc = "Checks if the value of the field is `EXTI1`"]
    #[inline(always)]
    pub fn is_exti1(&self) -> bool {
        **self == SIG_ID_A::EXTI1
    }
    #[doc = "Checks if the value of the field is `EXTI2`"]
    #[inline(always)]
    pub fn is_exti2(&self) -> bool {
        **self == SIG_ID_A::EXTI2
    }
    #[doc = "Checks if the value of the field is `EXTI3`"]
    #[inline(always)]
    pub fn is_exti3(&self) -> bool {
        **self == SIG_ID_A::EXTI3
    }
    #[doc = "Checks if the value of the field is `EXTI4`"]
    #[inline(always)]
    pub fn is_exti4(&self) -> bool {
        **self == SIG_ID_A::EXTI4
    }
    #[doc = "Checks if the value of the field is `EXTI5`"]
    #[inline(always)]
    pub fn is_exti5(&self) -> bool {
        **self == SIG_ID_A::EXTI5
    }
    #[doc = "Checks if the value of the field is `EXTI6`"]
    #[inline(always)]
    pub fn is_exti6(&self) -> bool {
        **self == SIG_ID_A::EXTI6
    }
    #[doc = "Checks if the value of the field is `EXTI7`"]
    #[inline(always)]
    pub fn is_exti7(&self) -> bool {
        **self == SIG_ID_A::EXTI7
    }
    #[doc = "Checks if the value of the field is `EXTI8`"]
    #[inline(always)]
    pub fn is_exti8(&self) -> bool {
        **self == SIG_ID_A::EXTI8
    }
    #[doc = "Checks if the value of the field is `EXTI9`"]
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        **self == SIG_ID_A::EXTI9
    }
    #[doc = "Checks if the value of the field is `EXTI10`"]
    #[inline(always)]
    pub fn is_exti10(&self) -> bool {
        **self == SIG_ID_A::EXTI10
    }
    #[doc = "Checks if the value of the field is `EXTI11`"]
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        **self == SIG_ID_A::EXTI11
    }
    #[doc = "Checks if the value of the field is `EXTI12`"]
    #[inline(always)]
    pub fn is_exti12(&self) -> bool {
        **self == SIG_ID_A::EXTI12
    }
    #[doc = "Checks if the value of the field is `EXTI13`"]
    #[inline(always)]
    pub fn is_exti13(&self) -> bool {
        **self == SIG_ID_A::EXTI13
    }
    #[doc = "Checks if the value of the field is `EXTI14`"]
    #[inline(always)]
    pub fn is_exti14(&self) -> bool {
        **self == SIG_ID_A::EXTI14
    }
    #[doc = "Checks if the value of the field is `EXTI15`"]
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        **self == SIG_ID_A::EXTI15
    }
    #[doc = "Checks if the value of the field is `DMAMUX1_EVT0`"]
    #[inline(always)]
    pub fn is_dmamux1_evt0(&self) -> bool {
        **self == SIG_ID_A::DMAMUX1_EVT0
    }
    #[doc = "Checks if the value of the field is `DMAMUX1_EVT1`"]
    #[inline(always)]
    pub fn is_dmamux1_evt1(&self) -> bool {
        **self == SIG_ID_A::DMAMUX1_EVT1
    }
    #[doc = "Checks if the value of the field is `LPTIM1_OUT`"]
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        **self == SIG_ID_A::LPTIM1_OUT
    }
    #[doc = "Checks if the value of the field is `LPTIM2_OUT`"]
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        **self == SIG_ID_A::LPTIM2_OUT
    }
    #[doc = "Checks if the value of the field is `LPTIM3_OUT`"]
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        **self == SIG_ID_A::LPTIM3_OUT
    }
}
impl core::ops::Deref for SIG_ID_R {
    type Target = crate::FieldReader<u8, SIG_ID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIG_ID` writer - Signal identification"]
pub struct SIG_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_ID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIG_ID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti0(self) -> &'a mut W {
        self.variant(SIG_ID_A::EXTI0)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti1(self) -> &'a mut W {
        self.variant(SIG_ID_A::EXTI1)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti2(self) -> &'a mut W {
        self.variant(SIG_ID_A::EXTI2)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti3(self) -> &'a mut W {
        self.variant(SIG_ID_A::EXTI3)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti4(self) -> &'a mut W {
        self.variant(SIG_ID_A::EXTI4)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti5(self) -> &'a mut W {
        self.variant(SIG_ID_A::EXTI5)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti6(self) -> &'a mut W {
        self.variant(SIG_ID_A::EXTI6)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti7(self) -> &'a mut W {
        self.variant(SIG_ID_A::EXTI7)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti8(self) -> &'a mut W {
        self.variant(SIG_ID_A::EXTI8)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti9(self) -> &'a mut W {
        self.variant(SIG_ID_A::EXTI9)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti10(self) -> &'a mut W {
        self.variant(SIG_ID_A::EXTI10)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti11(self) -> &'a mut W {
        self.variant(SIG_ID_A::EXTI11)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti12(self) -> &'a mut W {
        self.variant(SIG_ID_A::EXTI12)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti13(self) -> &'a mut W {
        self.variant(SIG_ID_A::EXTI13)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti14(self) -> &'a mut W {
        self.variant(SIG_ID_A::EXTI14)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti15(self) -> &'a mut W {
        self.variant(SIG_ID_A::EXTI15)
    }
    #[doc = "Signal `dmamux1_evt0` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux1_evt0(self) -> &'a mut W {
        self.variant(SIG_ID_A::DMAMUX1_EVT0)
    }
    #[doc = "Signal `dmamux1_evt1` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux1_evt1(self) -> &'a mut W {
        self.variant(SIG_ID_A::DMAMUX1_EVT1)
    }
    #[doc = "Signal `lptim1_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::LPTIM1_OUT)
    }
    #[doc = "Signal `lptim2_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::LPTIM2_OUT)
    }
    #[doc = "Signal `lptim3_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::LPTIM3_OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 19:23 - Number of DMA requests to be generated (minus 1)"]
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity"]
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - DMA request generator channel x enable"]
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - Signal identification"]
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 19:23 - Number of DMA requests to be generated (minus 1)"]
    #[inline(always)]
    pub fn gnbreq(&mut self) -> GNBREQ_W {
        GNBREQ_W { w: self }
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity"]
    #[inline(always)]
    pub fn gpol(&mut self) -> GPOL_W {
        GPOL_W { w: self }
    }
    #[doc = "Bit 16 - DMA request generator channel x enable"]
    #[inline(always)]
    pub fn ge(&mut self) -> GE_W {
        GE_W { w: self }
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn oie(&mut self) -> OIE_W {
        OIE_W { w: self }
    }
    #[doc = "Bits 0:4 - Signal identification"]
    #[inline(always)]
    pub fn sig_id(&mut self) -> SIG_ID_W {
        SIG_ID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "request generator channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rg1cr](index.html) module"]
pub struct RG1CR_SPEC;
impl crate::RegisterSpec for RG1CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rg1cr::R](R) reader structure"]
impl crate::Readable for RG1CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rg1cr::W](W) writer structure"]
impl crate::Writable for RG1CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RG1CR to value 0"]
impl crate::Resettable for RG1CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
