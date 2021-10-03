#[doc = "Register `CRL` reader"]
pub struct R(crate::R<CRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRL` writer"]
pub struct W(crate::W<CRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRL_SPEC>;
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
impl From<crate::W<CRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port n.0 mode bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE0_A {
    #[doc = "0: Input mode (reset state)"]
    INPUT = 0,
    #[doc = "1: Output mode 10 MHz"]
    OUTPUT = 1,
    #[doc = "2: Output mode 2 MHz"]
    OUTPUT2 = 2,
    #[doc = "3: Output mode 50 MHz"]
    OUTPUT50 = 3,
}
impl From<MODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE0` reader - Port n.0 mode bits"]
pub struct MODE0_R(crate::FieldReader<u8, MODE0_A>);
impl MODE0_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE0_A {
        match self.bits {
            0 => MODE0_A::INPUT,
            1 => MODE0_A::OUTPUT,
            2 => MODE0_A::OUTPUT2,
            3 => MODE0_A::OUTPUT50,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == MODE0_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == MODE0_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT2`"]
    #[inline(always)]
    pub fn is_output2(&self) -> bool {
        **self == MODE0_A::OUTPUT2
    }
    #[doc = "Checks if the value of the field is `OUTPUT50`"]
    #[inline(always)]
    pub fn is_output50(&self) -> bool {
        **self == MODE0_A::OUTPUT50
    }
}
impl core::ops::Deref for MODE0_R {
    type Target = crate::FieldReader<u8, MODE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE0` writer - Port n.0 mode bits"]
pub struct MODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE0_A::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE0_A::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE0_A::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE0_A::OUTPUT50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Port n.0 configuration bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CNF0_A {
    #[doc = "0: Analog mode / Push-Pull mode"]
    PUSHPULL = 0,
    #[doc = "1: Floating input (reset state) / Open Drain-Mode"]
    OPENDRAIN = 1,
    #[doc = "2: Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    ALTPUSHPULL = 2,
    #[doc = "3: Alternate Function Open-Drain Mode"]
    ALTOPENDRAIN = 3,
}
impl From<CNF0_A> for u8 {
    #[inline(always)]
    fn from(variant: CNF0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CNF0` reader - Port n.0 configuration bits"]
pub struct CNF0_R(crate::FieldReader<u8, CNF0_A>);
impl CNF0_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNF0_A {
        match self.bits {
            0 => CNF0_A::PUSHPULL,
            1 => CNF0_A::OPENDRAIN,
            2 => CNF0_A::ALTPUSHPULL,
            3 => CNF0_A::ALTOPENDRAIN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        **self == CNF0_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OPENDRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        **self == CNF0_A::OPENDRAIN
    }
    #[doc = "Checks if the value of the field is `ALTPUSHPULL`"]
    #[inline(always)]
    pub fn is_alt_push_pull(&self) -> bool {
        **self == CNF0_A::ALTPUSHPULL
    }
    #[doc = "Checks if the value of the field is `ALTOPENDRAIN`"]
    #[inline(always)]
    pub fn is_alt_open_drain(&self) -> bool {
        **self == CNF0_A::ALTOPENDRAIN
    }
}
impl core::ops::Deref for CNF0_R {
    type Target = crate::FieldReader<u8, CNF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNF0` writer - Port n.0 configuration bits"]
pub struct CNF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNF0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(CNF0_A::PUSHPULL)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(CNF0_A::OPENDRAIN)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut W {
        self.variant(CNF0_A::ALTPUSHPULL)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut W {
        self.variant(CNF0_A::ALTOPENDRAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Port n.1 mode bits"]
pub type MODE1_A = MODE0_A;
#[doc = "Field `MODE1` reader - Port n.1 mode bits"]
pub type MODE1_R = MODE0_R;
#[doc = "Field `MODE1` writer - Port n.1 mode bits"]
pub struct MODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE1_A::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE1_A::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE1_A::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE1_A::OUTPUT50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Port n.1 configuration bits"]
pub type CNF1_A = CNF0_A;
#[doc = "Field `CNF1` reader - Port n.1 configuration bits"]
pub type CNF1_R = CNF0_R;
#[doc = "Field `CNF1` writer - Port n.1 configuration bits"]
pub struct CNF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNF1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(CNF1_A::PUSHPULL)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(CNF1_A::OPENDRAIN)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut W {
        self.variant(CNF1_A::ALTPUSHPULL)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut W {
        self.variant(CNF1_A::ALTOPENDRAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Port n.2 mode bits"]
pub type MODE2_A = MODE0_A;
#[doc = "Field `MODE2` reader - Port n.2 mode bits"]
pub type MODE2_R = MODE0_R;
#[doc = "Field `MODE2` writer - Port n.2 mode bits"]
pub struct MODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE2_A::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE2_A::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE2_A::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE2_A::OUTPUT50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Port n.2 configuration bits"]
pub type CNF2_A = CNF0_A;
#[doc = "Field `CNF2` reader - Port n.2 configuration bits"]
pub type CNF2_R = CNF0_R;
#[doc = "Field `CNF2` writer - Port n.2 configuration bits"]
pub struct CNF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNF2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(CNF2_A::PUSHPULL)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(CNF2_A::OPENDRAIN)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut W {
        self.variant(CNF2_A::ALTPUSHPULL)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut W {
        self.variant(CNF2_A::ALTOPENDRAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Port n.3 mode bits"]
pub type MODE3_A = MODE0_A;
#[doc = "Field `MODE3` reader - Port n.3 mode bits"]
pub type MODE3_R = MODE0_R;
#[doc = "Field `MODE3` writer - Port n.3 mode bits"]
pub struct MODE3_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE3_A::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE3_A::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE3_A::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE3_A::OUTPUT50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Port n.3 configuration bits"]
pub type CNF3_A = CNF0_A;
#[doc = "Field `CNF3` reader - Port n.3 configuration bits"]
pub type CNF3_R = CNF0_R;
#[doc = "Field `CNF3` writer - Port n.3 configuration bits"]
pub struct CNF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNF3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(CNF3_A::PUSHPULL)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(CNF3_A::OPENDRAIN)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut W {
        self.variant(CNF3_A::ALTPUSHPULL)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut W {
        self.variant(CNF3_A::ALTOPENDRAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Port n.4 mode bits"]
pub type MODE4_A = MODE0_A;
#[doc = "Field `MODE4` reader - Port n.4 mode bits"]
pub type MODE4_R = MODE0_R;
#[doc = "Field `MODE4` writer - Port n.4 mode bits"]
pub struct MODE4_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE4_A::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE4_A::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE4_A::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE4_A::OUTPUT50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Port n.4 configuration bits"]
pub type CNF4_A = CNF0_A;
#[doc = "Field `CNF4` reader - Port n.4 configuration bits"]
pub type CNF4_R = CNF0_R;
#[doc = "Field `CNF4` writer - Port n.4 configuration bits"]
pub struct CNF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNF4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(CNF4_A::PUSHPULL)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(CNF4_A::OPENDRAIN)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut W {
        self.variant(CNF4_A::ALTPUSHPULL)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut W {
        self.variant(CNF4_A::ALTOPENDRAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Port n.5 mode bits"]
pub type MODE5_A = MODE0_A;
#[doc = "Field `MODE5` reader - Port n.5 mode bits"]
pub type MODE5_R = MODE0_R;
#[doc = "Field `MODE5` writer - Port n.5 mode bits"]
pub struct MODE5_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE5_A::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE5_A::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE5_A::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE5_A::OUTPUT50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Port n.5 configuration bits"]
pub type CNF5_A = CNF0_A;
#[doc = "Field `CNF5` reader - Port n.5 configuration bits"]
pub type CNF5_R = CNF0_R;
#[doc = "Field `CNF5` writer - Port n.5 configuration bits"]
pub struct CNF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNF5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(CNF5_A::PUSHPULL)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(CNF5_A::OPENDRAIN)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut W {
        self.variant(CNF5_A::ALTPUSHPULL)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut W {
        self.variant(CNF5_A::ALTOPENDRAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Port n.6 mode bits"]
pub type MODE6_A = MODE0_A;
#[doc = "Field `MODE6` reader - Port n.6 mode bits"]
pub type MODE6_R = MODE0_R;
#[doc = "Field `MODE6` writer - Port n.6 mode bits"]
pub struct MODE6_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE6_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE6_A::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE6_A::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE6_A::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE6_A::OUTPUT50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Port n.6 configuration bits"]
pub type CNF6_A = CNF0_A;
#[doc = "Field `CNF6` reader - Port n.6 configuration bits"]
pub type CNF6_R = CNF0_R;
#[doc = "Field `CNF6` writer - Port n.6 configuration bits"]
pub struct CNF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNF6_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(CNF6_A::PUSHPULL)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(CNF6_A::OPENDRAIN)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut W {
        self.variant(CNF6_A::ALTPUSHPULL)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut W {
        self.variant(CNF6_A::ALTOPENDRAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Port n.7 mode bits"]
pub type MODE7_A = MODE0_A;
#[doc = "Field `MODE7` reader - Port n.7 mode bits"]
pub type MODE7_R = MODE0_R;
#[doc = "Field `MODE7` writer - Port n.7 mode bits"]
pub struct MODE7_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE7_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE7_A::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE7_A::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE7_A::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE7_A::OUTPUT50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Port n.7 configuration bits"]
pub type CNF7_A = CNF0_A;
#[doc = "Field `CNF7` reader - Port n.7 configuration bits"]
pub type CNF7_R = CNF0_R;
#[doc = "Field `CNF7` writer - Port n.7 configuration bits"]
pub struct CNF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNF7_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(CNF7_A::PUSHPULL)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(CNF7_A::OPENDRAIN)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut W {
        self.variant(CNF7_A::ALTPUSHPULL)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut W {
        self.variant(CNF7_A::ALTOPENDRAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Port n.0 mode bits"]
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port n.0 configuration bits"]
    #[inline(always)]
    pub fn cnf0(&self) -> CNF0_R {
        CNF0_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port n.1 mode bits"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port n.1 configuration bits"]
    #[inline(always)]
    pub fn cnf1(&self) -> CNF1_R {
        CNF1_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port n.2 mode bits"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port n.2 configuration bits"]
    #[inline(always)]
    pub fn cnf2(&self) -> CNF2_R {
        CNF2_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port n.3 mode bits"]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port n.3 configuration bits"]
    #[inline(always)]
    pub fn cnf3(&self) -> CNF3_R {
        CNF3_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port n.4 mode bits"]
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port n.4 configuration bits"]
    #[inline(always)]
    pub fn cnf4(&self) -> CNF4_R {
        CNF4_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port n.5 mode bits"]
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port n.5 configuration bits"]
    #[inline(always)]
    pub fn cnf5(&self) -> CNF5_R {
        CNF5_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Port n.6 mode bits"]
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port n.6 configuration bits"]
    #[inline(always)]
    pub fn cnf6(&self) -> CNF6_R {
        CNF6_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port n.7 mode bits"]
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Port n.7 configuration bits"]
    #[inline(always)]
    pub fn cnf7(&self) -> CNF7_R {
        CNF7_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port n.0 mode bits"]
    #[inline(always)]
    pub fn mode0(&mut self) -> MODE0_W {
        MODE0_W { w: self }
    }
    #[doc = "Bits 2:3 - Port n.0 configuration bits"]
    #[inline(always)]
    pub fn cnf0(&mut self) -> CNF0_W {
        CNF0_W { w: self }
    }
    #[doc = "Bits 4:5 - Port n.1 mode bits"]
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W {
        MODE1_W { w: self }
    }
    #[doc = "Bits 6:7 - Port n.1 configuration bits"]
    #[inline(always)]
    pub fn cnf1(&mut self) -> CNF1_W {
        CNF1_W { w: self }
    }
    #[doc = "Bits 8:9 - Port n.2 mode bits"]
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W {
        MODE2_W { w: self }
    }
    #[doc = "Bits 10:11 - Port n.2 configuration bits"]
    #[inline(always)]
    pub fn cnf2(&mut self) -> CNF2_W {
        CNF2_W { w: self }
    }
    #[doc = "Bits 12:13 - Port n.3 mode bits"]
    #[inline(always)]
    pub fn mode3(&mut self) -> MODE3_W {
        MODE3_W { w: self }
    }
    #[doc = "Bits 14:15 - Port n.3 configuration bits"]
    #[inline(always)]
    pub fn cnf3(&mut self) -> CNF3_W {
        CNF3_W { w: self }
    }
    #[doc = "Bits 16:17 - Port n.4 mode bits"]
    #[inline(always)]
    pub fn mode4(&mut self) -> MODE4_W {
        MODE4_W { w: self }
    }
    #[doc = "Bits 18:19 - Port n.4 configuration bits"]
    #[inline(always)]
    pub fn cnf4(&mut self) -> CNF4_W {
        CNF4_W { w: self }
    }
    #[doc = "Bits 20:21 - Port n.5 mode bits"]
    #[inline(always)]
    pub fn mode5(&mut self) -> MODE5_W {
        MODE5_W { w: self }
    }
    #[doc = "Bits 22:23 - Port n.5 configuration bits"]
    #[inline(always)]
    pub fn cnf5(&mut self) -> CNF5_W {
        CNF5_W { w: self }
    }
    #[doc = "Bits 24:25 - Port n.6 mode bits"]
    #[inline(always)]
    pub fn mode6(&mut self) -> MODE6_W {
        MODE6_W { w: self }
    }
    #[doc = "Bits 26:27 - Port n.6 configuration bits"]
    #[inline(always)]
    pub fn cnf6(&mut self) -> CNF6_W {
        CNF6_W { w: self }
    }
    #[doc = "Bits 28:29 - Port n.7 mode bits"]
    #[inline(always)]
    pub fn mode7(&mut self) -> MODE7_W {
        MODE7_W { w: self }
    }
    #[doc = "Bits 30:31 - Port n.7 configuration bits"]
    #[inline(always)]
    pub fn cnf7(&mut self) -> CNF7_W {
        CNF7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port configuration register low (GPIOn_CRL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crl](index.html) module"]
pub struct CRL_SPEC;
impl crate::RegisterSpec for CRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crl::R](R) reader structure"]
impl crate::Readable for CRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crl::W](W) writer structure"]
impl crate::Writable for CRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRL to value 0x4444_4444"]
impl crate::Resettable for CRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4444_4444
    }
}
