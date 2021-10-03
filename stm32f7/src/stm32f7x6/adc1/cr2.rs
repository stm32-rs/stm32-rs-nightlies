#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Start conversion of regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWSTART_A {
    #[doc = "1: Starts conversion of regular channels"]
    START = 1,
}
impl From<SWSTART_A> for bool {
    #[inline(always)]
    fn from(variant: SWSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWSTART` reader - Start conversion of regular channels"]
pub struct SWSTART_R(crate::FieldReader<bool, SWSTART_A>);
impl SWSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWSTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWSTART_A> {
        match self.bits {
            true => Some(SWSTART_A::START),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == SWSTART_A::START
    }
}
impl core::ops::Deref for SWSTART_R {
    type Target = crate::FieldReader<bool, SWSTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWSTART` writer - Start conversion of regular channels"]
pub struct SWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SWSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWSTART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Starts conversion of regular channels"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SWSTART_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "External trigger enable for regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTEN_A {
    #[doc = "0: Trigger detection disabled"]
    DISABLED = 0,
    #[doc = "1: Trigger detection on the rising edge"]
    RISINGEDGE = 1,
    #[doc = "2: Trigger detection on the falling edge"]
    FALLINGEDGE = 2,
    #[doc = "3: Trigger detection on both the rising and falling edges"]
    BOTHEDGES = 3,
}
impl From<EXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTEN` reader - External trigger enable for regular channels"]
pub struct EXTEN_R(crate::FieldReader<u8, EXTEN_A>);
impl EXTEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTEN_A {
        match self.bits {
            0 => EXTEN_A::DISABLED,
            1 => EXTEN_A::RISINGEDGE,
            2 => EXTEN_A::FALLINGEDGE,
            3 => EXTEN_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EXTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == EXTEN_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == EXTEN_A::FALLINGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        **self == EXTEN_A::BOTHEDGES
    }
}
impl core::ops::Deref for EXTEN_R {
    type Target = crate::FieldReader<u8, EXTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTEN` writer - External trigger enable for regular channels"]
pub struct EXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Trigger detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTEN_A::DISABLED)
    }
    #[doc = "Trigger detection on the rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::RISINGEDGE)
    }
    #[doc = "Trigger detection on the falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::FALLINGEDGE)
    }
    #[doc = "Trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(EXTEN_A::BOTHEDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "External event select for regular group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTSEL_A {
    #[doc = "0: Timer 1 CC1 event"]
    TIM1CC1 = 0,
    #[doc = "1: Timer 1 CC2 event"]
    TIM1CC2 = 1,
    #[doc = "2: Timer 1 CC3 event"]
    TIM1CC3 = 2,
    #[doc = "3: Timer 2 CC2 event"]
    TIM2CC2 = 3,
    #[doc = "4: Timer 2 CC3 event"]
    TIM2CC3 = 4,
    #[doc = "5: Timer 2 CC4 event"]
    TIM2CC4 = 5,
    #[doc = "6: Timer 2 TRGO event"]
    TIM2TRGO = 6,
}
impl From<EXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTSEL` reader - External event select for regular group"]
pub struct EXTSEL_R(crate::FieldReader<u8, EXTSEL_A>);
impl EXTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTSEL_A> {
        match self.bits {
            0 => Some(EXTSEL_A::TIM1CC1),
            1 => Some(EXTSEL_A::TIM1CC2),
            2 => Some(EXTSEL_A::TIM1CC3),
            3 => Some(EXTSEL_A::TIM2CC2),
            4 => Some(EXTSEL_A::TIM2CC3),
            5 => Some(EXTSEL_A::TIM2CC4),
            6 => Some(EXTSEL_A::TIM2TRGO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIM1CC1`"]
    #[inline(always)]
    pub fn is_tim1cc1(&self) -> bool {
        **self == EXTSEL_A::TIM1CC1
    }
    #[doc = "Checks if the value of the field is `TIM1CC2`"]
    #[inline(always)]
    pub fn is_tim1cc2(&self) -> bool {
        **self == EXTSEL_A::TIM1CC2
    }
    #[doc = "Checks if the value of the field is `TIM1CC3`"]
    #[inline(always)]
    pub fn is_tim1cc3(&self) -> bool {
        **self == EXTSEL_A::TIM1CC3
    }
    #[doc = "Checks if the value of the field is `TIM2CC2`"]
    #[inline(always)]
    pub fn is_tim2cc2(&self) -> bool {
        **self == EXTSEL_A::TIM2CC2
    }
    #[doc = "Checks if the value of the field is `TIM2CC3`"]
    #[inline(always)]
    pub fn is_tim2cc3(&self) -> bool {
        **self == EXTSEL_A::TIM2CC3
    }
    #[doc = "Checks if the value of the field is `TIM2CC4`"]
    #[inline(always)]
    pub fn is_tim2cc4(&self) -> bool {
        **self == EXTSEL_A::TIM2CC4
    }
    #[doc = "Checks if the value of the field is `TIM2TRGO`"]
    #[inline(always)]
    pub fn is_tim2trgo(&self) -> bool {
        **self == EXTSEL_A::TIM2TRGO
    }
}
impl core::ops::Deref for EXTSEL_R {
    type Target = crate::FieldReader<u8, EXTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSEL` writer - External event select for regular group"]
pub struct EXTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timer 1 CC1 event"]
    #[inline(always)]
    pub fn tim1cc1(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1CC1)
    }
    #[doc = "Timer 1 CC2 event"]
    #[inline(always)]
    pub fn tim1cc2(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1CC2)
    }
    #[doc = "Timer 1 CC3 event"]
    #[inline(always)]
    pub fn tim1cc3(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1CC3)
    }
    #[doc = "Timer 2 CC2 event"]
    #[inline(always)]
    pub fn tim2cc2(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2CC2)
    }
    #[doc = "Timer 2 CC3 event"]
    #[inline(always)]
    pub fn tim2cc3(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2CC3)
    }
    #[doc = "Timer 2 CC4 event"]
    #[inline(always)]
    pub fn tim2cc4(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2CC4)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn tim2trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2TRGO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Start conversion of injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSWSTART_A {
    #[doc = "1: Starts conversion of injected channels"]
    START = 1,
}
impl From<JSWSTART_A> for bool {
    #[inline(always)]
    fn from(variant: JSWSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSWSTART` reader - Start conversion of injected channels"]
pub struct JSWSTART_R(crate::FieldReader<bool, JSWSTART_A>);
impl JSWSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        JSWSTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<JSWSTART_A> {
        match self.bits {
            true => Some(JSWSTART_A::START),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == JSWSTART_A::START
    }
}
impl core::ops::Deref for JSWSTART_R {
    type Target = crate::FieldReader<bool, JSWSTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JSWSTART` writer - Start conversion of injected channels"]
pub struct JSWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> JSWSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JSWSTART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Starts conversion of injected channels"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(JSWSTART_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "External trigger enable for injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum JEXTEN_A {
    #[doc = "0: Trigger detection disabled"]
    DISABLED = 0,
    #[doc = "1: Trigger detection on the rising edge"]
    RISINGEDGE = 1,
    #[doc = "2: Trigger detection on the falling edge"]
    FALLINGEDGE = 2,
    #[doc = "3: Trigger detection on both the rising and falling edges"]
    BOTHEDGES = 3,
}
impl From<JEXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: JEXTEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `JEXTEN` reader - External trigger enable for injected channels"]
pub struct JEXTEN_R(crate::FieldReader<u8, JEXTEN_A>);
impl JEXTEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        JEXTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEXTEN_A {
        match self.bits {
            0 => JEXTEN_A::DISABLED,
            1 => JEXTEN_A::RISINGEDGE,
            2 => JEXTEN_A::FALLINGEDGE,
            3 => JEXTEN_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == JEXTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == JEXTEN_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == JEXTEN_A::FALLINGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        **self == JEXTEN_A::BOTHEDGES
    }
}
impl core::ops::Deref for JEXTEN_R {
    type Target = crate::FieldReader<u8, JEXTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEXTEN` writer - External trigger enable for injected channels"]
pub struct JEXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JEXTEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Trigger detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JEXTEN_A::DISABLED)
    }
    #[doc = "Trigger detection on the rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(JEXTEN_A::RISINGEDGE)
    }
    #[doc = "Trigger detection on the falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(JEXTEN_A::FALLINGEDGE)
    }
    #[doc = "Trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(JEXTEN_A::BOTHEDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "External event select for injected group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum JEXTSEL_A {
    #[doc = "0: Timer 1 TRGO event"]
    TIM1TRGO = 0,
    #[doc = "1: Timer 1 CC4 event"]
    TIM1CC4 = 1,
    #[doc = "2: Timer 2 TRGO event"]
    TIM2TRGO = 2,
    #[doc = "3: Timer 2 CC1 event"]
    TIM2CC1 = 3,
    #[doc = "4: Timer 3 CC4 event"]
    TIM3CC4 = 4,
    #[doc = "5: Timer 4 TRGO event"]
    TIM4TRGO = 5,
    #[doc = "7: Timer 8 CC4 event"]
    TIM8CC4 = 7,
    #[doc = "8: Timer 1 TRGO(2) event"]
    TIM1TRGO2 = 8,
    #[doc = "9: Timer 8 TRGO event"]
    TIM8TRGO = 9,
    #[doc = "10: Timer 8 TRGO(2) event"]
    TIM8TRGO2 = 10,
    #[doc = "11: Timer 3 CC3 event"]
    TIM3CC3 = 11,
    #[doc = "12: Timer 5 TRGO event"]
    TIM5TRGO = 12,
    #[doc = "13: Timer 3 CC1 event"]
    TIM3CC1 = 13,
    #[doc = "14: Timer 6 TRGO event"]
    TIM6TRGO = 14,
}
impl From<JEXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: JEXTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `JEXTSEL` reader - External event select for injected group"]
pub struct JEXTSEL_R(crate::FieldReader<u8, JEXTSEL_A>);
impl JEXTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        JEXTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<JEXTSEL_A> {
        match self.bits {
            0 => Some(JEXTSEL_A::TIM1TRGO),
            1 => Some(JEXTSEL_A::TIM1CC4),
            2 => Some(JEXTSEL_A::TIM2TRGO),
            3 => Some(JEXTSEL_A::TIM2CC1),
            4 => Some(JEXTSEL_A::TIM3CC4),
            5 => Some(JEXTSEL_A::TIM4TRGO),
            7 => Some(JEXTSEL_A::TIM8CC4),
            8 => Some(JEXTSEL_A::TIM1TRGO2),
            9 => Some(JEXTSEL_A::TIM8TRGO),
            10 => Some(JEXTSEL_A::TIM8TRGO2),
            11 => Some(JEXTSEL_A::TIM3CC3),
            12 => Some(JEXTSEL_A::TIM5TRGO),
            13 => Some(JEXTSEL_A::TIM3CC1),
            14 => Some(JEXTSEL_A::TIM6TRGO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIM1TRGO`"]
    #[inline(always)]
    pub fn is_tim1trgo(&self) -> bool {
        **self == JEXTSEL_A::TIM1TRGO
    }
    #[doc = "Checks if the value of the field is `TIM1CC4`"]
    #[inline(always)]
    pub fn is_tim1cc4(&self) -> bool {
        **self == JEXTSEL_A::TIM1CC4
    }
    #[doc = "Checks if the value of the field is `TIM2TRGO`"]
    #[inline(always)]
    pub fn is_tim2trgo(&self) -> bool {
        **self == JEXTSEL_A::TIM2TRGO
    }
    #[doc = "Checks if the value of the field is `TIM2CC1`"]
    #[inline(always)]
    pub fn is_tim2cc1(&self) -> bool {
        **self == JEXTSEL_A::TIM2CC1
    }
    #[doc = "Checks if the value of the field is `TIM3CC4`"]
    #[inline(always)]
    pub fn is_tim3cc4(&self) -> bool {
        **self == JEXTSEL_A::TIM3CC4
    }
    #[doc = "Checks if the value of the field is `TIM4TRGO`"]
    #[inline(always)]
    pub fn is_tim4trgo(&self) -> bool {
        **self == JEXTSEL_A::TIM4TRGO
    }
    #[doc = "Checks if the value of the field is `TIM8CC4`"]
    #[inline(always)]
    pub fn is_tim8cc4(&self) -> bool {
        **self == JEXTSEL_A::TIM8CC4
    }
    #[doc = "Checks if the value of the field is `TIM1TRGO2`"]
    #[inline(always)]
    pub fn is_tim1trgo2(&self) -> bool {
        **self == JEXTSEL_A::TIM1TRGO2
    }
    #[doc = "Checks if the value of the field is `TIM8TRGO`"]
    #[inline(always)]
    pub fn is_tim8trgo(&self) -> bool {
        **self == JEXTSEL_A::TIM8TRGO
    }
    #[doc = "Checks if the value of the field is `TIM8TRGO2`"]
    #[inline(always)]
    pub fn is_tim8trgo2(&self) -> bool {
        **self == JEXTSEL_A::TIM8TRGO2
    }
    #[doc = "Checks if the value of the field is `TIM3CC3`"]
    #[inline(always)]
    pub fn is_tim3cc3(&self) -> bool {
        **self == JEXTSEL_A::TIM3CC3
    }
    #[doc = "Checks if the value of the field is `TIM5TRGO`"]
    #[inline(always)]
    pub fn is_tim5trgo(&self) -> bool {
        **self == JEXTSEL_A::TIM5TRGO
    }
    #[doc = "Checks if the value of the field is `TIM3CC1`"]
    #[inline(always)]
    pub fn is_tim3cc1(&self) -> bool {
        **self == JEXTSEL_A::TIM3CC1
    }
    #[doc = "Checks if the value of the field is `TIM6TRGO`"]
    #[inline(always)]
    pub fn is_tim6trgo(&self) -> bool {
        **self == JEXTSEL_A::TIM6TRGO
    }
}
impl core::ops::Deref for JEXTSEL_R {
    type Target = crate::FieldReader<u8, JEXTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEXTSEL` writer - External event select for injected group"]
pub struct JEXTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JEXTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn tim1trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM1TRGO)
    }
    #[doc = "Timer 1 CC4 event"]
    #[inline(always)]
    pub fn tim1cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM1CC4)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn tim2trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM2TRGO)
    }
    #[doc = "Timer 2 CC1 event"]
    #[inline(always)]
    pub fn tim2cc1(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM2CC1)
    }
    #[doc = "Timer 3 CC4 event"]
    #[inline(always)]
    pub fn tim3cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM3CC4)
    }
    #[doc = "Timer 4 TRGO event"]
    #[inline(always)]
    pub fn tim4trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM4TRGO)
    }
    #[doc = "Timer 8 CC4 event"]
    #[inline(always)]
    pub fn tim8cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM8CC4)
    }
    #[doc = "Timer 1 TRGO(2) event"]
    #[inline(always)]
    pub fn tim1trgo2(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM1TRGO2)
    }
    #[doc = "Timer 8 TRGO event"]
    #[inline(always)]
    pub fn tim8trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM8TRGO)
    }
    #[doc = "Timer 8 TRGO(2) event"]
    #[inline(always)]
    pub fn tim8trgo2(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM8TRGO2)
    }
    #[doc = "Timer 3 CC3 event"]
    #[inline(always)]
    pub fn tim3cc3(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM3CC3)
    }
    #[doc = "Timer 5 TRGO event"]
    #[inline(always)]
    pub fn tim5trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM5TRGO)
    }
    #[doc = "Timer 3 CC1 event"]
    #[inline(always)]
    pub fn tim3cc1(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM3CC1)
    }
    #[doc = "Timer 6 TRGO event"]
    #[inline(always)]
    pub fn tim6trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM6TRGO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Data alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN_A {
    #[doc = "0: Right alignment"]
    RIGHT = 0,
    #[doc = "1: Left alignment"]
    LEFT = 1,
}
impl From<ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGN` reader - Data alignment"]
pub struct ALIGN_R(crate::FieldReader<bool, ALIGN_A>);
impl ALIGN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALIGN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGN_A {
        match self.bits {
            false => ALIGN_A::RIGHT,
            true => ALIGN_A::LEFT,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        **self == ALIGN_A::RIGHT
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        **self == ALIGN_A::LEFT
    }
}
impl core::ops::Deref for ALIGN_R {
    type Target = crate::FieldReader<bool, ALIGN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALIGN` writer - Data alignment"]
pub struct ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIGN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALIGN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(ALIGN_A::RIGHT)
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(ALIGN_A::LEFT)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "End of conversion selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOCS_A {
    #[doc = "0: The EOC bit is set at the end of each sequence of regular conversions"]
    EACHSEQUENCE = 0,
    #[doc = "1: The EOC bit is set at the end of each regular conversion"]
    EACHCONVERSION = 1,
}
impl From<EOCS_A> for bool {
    #[inline(always)]
    fn from(variant: EOCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOCS` reader - End of conversion selection"]
pub struct EOCS_R(crate::FieldReader<bool, EOCS_A>);
impl EOCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOCS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCS_A {
        match self.bits {
            false => EOCS_A::EACHSEQUENCE,
            true => EOCS_A::EACHCONVERSION,
        }
    }
    #[doc = "Checks if the value of the field is `EACHSEQUENCE`"]
    #[inline(always)]
    pub fn is_each_sequence(&self) -> bool {
        **self == EOCS_A::EACHSEQUENCE
    }
    #[doc = "Checks if the value of the field is `EACHCONVERSION`"]
    #[inline(always)]
    pub fn is_each_conversion(&self) -> bool {
        **self == EOCS_A::EACHCONVERSION
    }
}
impl core::ops::Deref for EOCS_R {
    type Target = crate::FieldReader<bool, EOCS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOCS` writer - End of conversion selection"]
pub struct EOCS_W<'a> {
    w: &'a mut W,
}
impl<'a> EOCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOCS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The EOC bit is set at the end of each sequence of regular conversions"]
    #[inline(always)]
    pub fn each_sequence(self) -> &'a mut W {
        self.variant(EOCS_A::EACHSEQUENCE)
    }
    #[doc = "The EOC bit is set at the end of each regular conversion"]
    #[inline(always)]
    pub fn each_conversion(self) -> &'a mut W {
        self.variant(EOCS_A::EACHCONVERSION)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "DMA disable selection (for single ADC mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDS_A {
    #[doc = "0: No new DMA request is issued after the last transfer"]
    SINGLE = 0,
    #[doc = "1: DMA requests are issued as long as data are converted and DMA=1"]
    CONTINUOUS = 1,
}
impl From<DDS_A> for bool {
    #[inline(always)]
    fn from(variant: DDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDS` reader - DMA disable selection (for single ADC mode)"]
pub struct DDS_R(crate::FieldReader<bool, DDS_A>);
impl DDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDS_A {
        match self.bits {
            false => DDS_A::SINGLE,
            true => DDS_A::CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == DDS_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == DDS_A::CONTINUOUS
    }
}
impl core::ops::Deref for DDS_R {
    type Target = crate::FieldReader<bool, DDS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDS` writer - DMA disable selection (for single ADC mode)"]
pub struct DDS_W<'a> {
    w: &'a mut W,
}
impl<'a> DDS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No new DMA request is issued after the last transfer"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(DDS_A::SINGLE)
    }
    #[doc = "DMA requests are issued as long as data are converted and DMA=1"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(DDS_A::CONTINUOUS)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Direct memory access mode (for single ADC mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    #[doc = "0: DMA mode disabled"]
    DISABLED = 0,
    #[doc = "1: DMA mode enabled"]
    ENABLED = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - Direct memory access mode (for single ADC mode)"]
pub struct DMA_R(crate::FieldReader<bool, DMA_A>);
impl DMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::DISABLED,
            true => DMA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMA_A::ENABLED
    }
}
impl core::ops::Deref for DMA_R {
    type Target = crate::FieldReader<bool, DMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA` writer - Direct memory access mode (for single ADC mode)"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_A::DISABLED)
    }
    #[doc = "DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA_A::ENABLED)
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
#[doc = "Continuous conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONT_A {
    #[doc = "0: Single conversion mode"]
    SINGLE = 0,
    #[doc = "1: Continuous conversion mode"]
    CONTINUOUS = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONT` reader - Continuous conversion"]
pub struct CONT_R(crate::FieldReader<bool, CONT_A>);
impl CONT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::SINGLE,
            true => CONT_A::CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == CONT_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == CONT_A::CONTINUOUS
    }
}
impl core::ops::Deref for CONT_R {
    type Target = crate::FieldReader<bool, CONT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONT` writer - Continuous conversion"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(CONT_A::SINGLE)
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONT_A::CONTINUOUS)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "A/D Converter ON / OFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADON_A {
    #[doc = "0: Disable ADC conversion and go to power down mode"]
    DISABLED = 0,
    #[doc = "1: Enable ADC"]
    ENABLED = 1,
}
impl From<ADON_A> for bool {
    #[inline(always)]
    fn from(variant: ADON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADON` reader - A/D Converter ON / OFF"]
pub struct ADON_R(crate::FieldReader<bool, ADON_A>);
impl ADON_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADON_A {
        match self.bits {
            false => ADON_A::DISABLED,
            true => ADON_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ADON_A::ENABLED
    }
}
impl core::ops::Deref for ADON_R {
    type Target = crate::FieldReader<bool, ADON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADON` writer - A/D Converter ON / OFF"]
pub struct ADON_W<'a> {
    w: &'a mut W,
}
impl<'a> ADON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable ADC conversion and go to power down mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADON_A::DISABLED)
    }
    #[doc = "Enable ADC"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADON_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Start conversion of regular channels"]
    #[inline(always)]
    pub fn swstart(&self) -> SWSTART_R {
        SWSTART_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - External trigger enable for regular channels"]
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - External event select for regular group"]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Start conversion of injected channels"]
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - External trigger enable for injected channels"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - External event select for injected group"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - End of conversion selection"]
    #[inline(always)]
    pub fn eocs(&self) -> EOCS_R {
        EOCS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DMA disable selection (for single ADC mode)"]
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Direct memory access mode (for single ADC mode)"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Continuous conversion"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - A/D Converter ON / OFF"]
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Start conversion of regular channels"]
    #[inline(always)]
    pub fn swstart(&mut self) -> SWSTART_W {
        SWSTART_W { w: self }
    }
    #[doc = "Bits 28:29 - External trigger enable for regular channels"]
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W {
        EXTEN_W { w: self }
    }
    #[doc = "Bits 24:27 - External event select for regular group"]
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W {
        EXTSEL_W { w: self }
    }
    #[doc = "Bit 22 - Start conversion of injected channels"]
    #[inline(always)]
    pub fn jswstart(&mut self) -> JSWSTART_W {
        JSWSTART_W { w: self }
    }
    #[doc = "Bits 20:21 - External trigger enable for injected channels"]
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W {
        JEXTEN_W { w: self }
    }
    #[doc = "Bits 16:19 - External event select for injected group"]
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W {
        JEXTSEL_W { w: self }
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W {
        ALIGN_W { w: self }
    }
    #[doc = "Bit 10 - End of conversion selection"]
    #[inline(always)]
    pub fn eocs(&mut self) -> EOCS_W {
        EOCS_W { w: self }
    }
    #[doc = "Bit 9 - DMA disable selection (for single ADC mode)"]
    #[inline(always)]
    pub fn dds(&mut self) -> DDS_W {
        DDS_W { w: self }
    }
    #[doc = "Bit 8 - Direct memory access mode (for single ADC mode)"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 1 - Continuous conversion"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    #[doc = "Bit 0 - A/D Converter ON / OFF"]
    #[inline(always)]
    pub fn adon(&mut self) -> ADON_W {
        ADON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
