#[doc = "Register `CRH` reader"]
pub struct R(crate::R<CRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRH` writer"]
pub struct W(crate::W<CRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRH_SPEC>;
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
impl From<crate::W<CRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port n.8 mode bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE8_A {
    #[doc = "0: Input mode (reset state)"]
    INPUT = 0,
    #[doc = "1: Output mode 10 MHz"]
    OUTPUT = 1,
    #[doc = "2: Output mode 2 MHz"]
    OUTPUT2 = 2,
    #[doc = "3: Output mode 50 MHz"]
    OUTPUT50 = 3,
}
impl From<MODE8_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE8_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE8` reader - Port n.8 mode bits"]
pub struct MODE8_R(crate::FieldReader<u8, MODE8_A>);
impl MODE8_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE8_A {
        match self.bits {
            0 => MODE8_A::INPUT,
            1 => MODE8_A::OUTPUT,
            2 => MODE8_A::OUTPUT2,
            3 => MODE8_A::OUTPUT50,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == MODE8_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == MODE8_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT2`"]
    #[inline(always)]
    pub fn is_output2(&self) -> bool {
        **self == MODE8_A::OUTPUT2
    }
    #[doc = "Checks if the value of the field is `OUTPUT50`"]
    #[inline(always)]
    pub fn is_output50(&self) -> bool {
        **self == MODE8_A::OUTPUT50
    }
}
impl core::ops::Deref for MODE8_R {
    type Target = crate::FieldReader<u8, MODE8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE8` writer - Port n.8 mode bits"]
pub struct MODE8_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE8_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE8_A::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE8_A::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE8_A::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE8_A::OUTPUT50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Port n.8 configuration bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CNF8_A {
    #[doc = "0: Analog mode / Push-Pull mode"]
    PUSHPULL = 0,
    #[doc = "1: Floating input (reset state) / Open Drain-Mode"]
    OPENDRAIN = 1,
    #[doc = "2: Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    ALTPUSHPULL = 2,
    #[doc = "3: Alternate Function Open-Drain Mode"]
    ALTOPENDRAIN = 3,
}
impl From<CNF8_A> for u8 {
    #[inline(always)]
    fn from(variant: CNF8_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CNF8` reader - Port n.8 configuration bits"]
pub struct CNF8_R(crate::FieldReader<u8, CNF8_A>);
impl CNF8_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNF8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNF8_A {
        match self.bits {
            0 => CNF8_A::PUSHPULL,
            1 => CNF8_A::OPENDRAIN,
            2 => CNF8_A::ALTPUSHPULL,
            3 => CNF8_A::ALTOPENDRAIN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        **self == CNF8_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OPENDRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        **self == CNF8_A::OPENDRAIN
    }
    #[doc = "Checks if the value of the field is `ALTPUSHPULL`"]
    #[inline(always)]
    pub fn is_alt_push_pull(&self) -> bool {
        **self == CNF8_A::ALTPUSHPULL
    }
    #[doc = "Checks if the value of the field is `ALTOPENDRAIN`"]
    #[inline(always)]
    pub fn is_alt_open_drain(&self) -> bool {
        **self == CNF8_A::ALTOPENDRAIN
    }
}
impl core::ops::Deref for CNF8_R {
    type Target = crate::FieldReader<u8, CNF8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNF8` writer - Port n.8 configuration bits"]
pub struct CNF8_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNF8_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(CNF8_A::PUSHPULL)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(CNF8_A::OPENDRAIN)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut W {
        self.variant(CNF8_A::ALTPUSHPULL)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut W {
        self.variant(CNF8_A::ALTOPENDRAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Port n.9 mode bits"]
pub type MODE9_A = MODE8_A;
#[doc = "Field `MODE9` reader - Port n.9 mode bits"]
pub type MODE9_R = MODE8_R;
#[doc = "Field `MODE9` writer - Port n.9 mode bits"]
pub struct MODE9_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE9_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE9_A::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE9_A::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE9_A::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE9_A::OUTPUT50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Port n.9 configuration bits"]
pub type CNF9_A = CNF8_A;
#[doc = "Field `CNF9` reader - Port n.9 configuration bits"]
pub type CNF9_R = CNF8_R;
#[doc = "Field `CNF9` writer - Port n.9 configuration bits"]
pub struct CNF9_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNF9_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(CNF9_A::PUSHPULL)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(CNF9_A::OPENDRAIN)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut W {
        self.variant(CNF9_A::ALTPUSHPULL)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut W {
        self.variant(CNF9_A::ALTOPENDRAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Port n.10 mode bits"]
pub type MODE10_A = MODE8_A;
#[doc = "Field `MODE10` reader - Port n.10 mode bits"]
pub type MODE10_R = MODE8_R;
#[doc = "Field `MODE10` writer - Port n.10 mode bits"]
pub struct MODE10_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE10_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE10_A::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE10_A::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE10_A::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE10_A::OUTPUT50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Port n.10 configuration bits"]
pub type CNF10_A = CNF8_A;
#[doc = "Field `CNF10` reader - Port n.10 configuration bits"]
pub type CNF10_R = CNF8_R;
#[doc = "Field `CNF10` writer - Port n.10 configuration bits"]
pub struct CNF10_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNF10_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(CNF10_A::PUSHPULL)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(CNF10_A::OPENDRAIN)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut W {
        self.variant(CNF10_A::ALTPUSHPULL)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut W {
        self.variant(CNF10_A::ALTOPENDRAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Port n.11 mode bits"]
pub type MODE11_A = MODE8_A;
#[doc = "Field `MODE11` reader - Port n.11 mode bits"]
pub type MODE11_R = MODE8_R;
#[doc = "Field `MODE11` writer - Port n.11 mode bits"]
pub struct MODE11_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE11_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE11_A::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE11_A::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE11_A::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE11_A::OUTPUT50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Port n.11 configuration bits"]
pub type CNF11_A = CNF8_A;
#[doc = "Field `CNF11` reader - Port n.11 configuration bits"]
pub type CNF11_R = CNF8_R;
#[doc = "Field `CNF11` writer - Port n.11 configuration bits"]
pub struct CNF11_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNF11_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(CNF11_A::PUSHPULL)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(CNF11_A::OPENDRAIN)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut W {
        self.variant(CNF11_A::ALTPUSHPULL)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut W {
        self.variant(CNF11_A::ALTOPENDRAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Port n.12 mode bits"]
pub type MODE12_A = MODE8_A;
#[doc = "Field `MODE12` reader - Port n.12 mode bits"]
pub type MODE12_R = MODE8_R;
#[doc = "Field `MODE12` writer - Port n.12 mode bits"]
pub struct MODE12_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE12_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE12_A::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE12_A::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE12_A::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE12_A::OUTPUT50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Port n.12 configuration bits"]
pub type CNF12_A = CNF8_A;
#[doc = "Field `CNF12` reader - Port n.12 configuration bits"]
pub type CNF12_R = CNF8_R;
#[doc = "Field `CNF12` writer - Port n.12 configuration bits"]
pub struct CNF12_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNF12_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(CNF12_A::PUSHPULL)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(CNF12_A::OPENDRAIN)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut W {
        self.variant(CNF12_A::ALTPUSHPULL)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut W {
        self.variant(CNF12_A::ALTOPENDRAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Port n.13 mode bits"]
pub type MODE13_A = MODE8_A;
#[doc = "Field `MODE13` reader - Port n.13 mode bits"]
pub type MODE13_R = MODE8_R;
#[doc = "Field `MODE13` writer - Port n.13 mode bits"]
pub struct MODE13_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE13_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE13_A::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE13_A::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE13_A::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE13_A::OUTPUT50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Port n.13 configuration bits"]
pub type CNF13_A = CNF8_A;
#[doc = "Field `CNF13` reader - Port n.13 configuration bits"]
pub type CNF13_R = CNF8_R;
#[doc = "Field `CNF13` writer - Port n.13 configuration bits"]
pub struct CNF13_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNF13_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(CNF13_A::PUSHPULL)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(CNF13_A::OPENDRAIN)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut W {
        self.variant(CNF13_A::ALTPUSHPULL)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut W {
        self.variant(CNF13_A::ALTOPENDRAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Port n.14 mode bits"]
pub type MODE14_A = MODE8_A;
#[doc = "Field `MODE14` reader - Port n.14 mode bits"]
pub type MODE14_R = MODE8_R;
#[doc = "Field `MODE14` writer - Port n.14 mode bits"]
pub struct MODE14_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE14_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE14_A::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE14_A::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE14_A::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE14_A::OUTPUT50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Port n.14 configuration bits"]
pub type CNF14_A = CNF8_A;
#[doc = "Field `CNF14` reader - Port n.14 configuration bits"]
pub type CNF14_R = CNF8_R;
#[doc = "Field `CNF14` writer - Port n.14 configuration bits"]
pub struct CNF14_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNF14_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(CNF14_A::PUSHPULL)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(CNF14_A::OPENDRAIN)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut W {
        self.variant(CNF14_A::ALTPUSHPULL)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut W {
        self.variant(CNF14_A::ALTOPENDRAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Port n.15 mode bits"]
pub type MODE15_A = MODE8_A;
#[doc = "Field `MODE15` reader - Port n.15 mode bits"]
pub type MODE15_R = MODE8_R;
#[doc = "Field `MODE15` writer - Port n.15 mode bits"]
pub struct MODE15_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE15_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE15_A::INPUT)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE15_A::OUTPUT)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE15_A::OUTPUT2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE15_A::OUTPUT50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Port n.15 configuration bits"]
pub type CNF15_A = CNF8_A;
#[doc = "Field `CNF15` reader - Port n.15 configuration bits"]
pub type CNF15_R = CNF8_R;
#[doc = "Field `CNF15` writer - Port n.15 configuration bits"]
pub struct CNF15_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNF15_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(CNF15_A::PUSHPULL)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(CNF15_A::OPENDRAIN)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut W {
        self.variant(CNF15_A::ALTPUSHPULL)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut W {
        self.variant(CNF15_A::ALTOPENDRAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Port n.8 mode bits"]
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port n.8 configuration bits"]
    #[inline(always)]
    pub fn cnf8(&self) -> CNF8_R {
        CNF8_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port n.9 mode bits"]
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port n.9 configuration bits"]
    #[inline(always)]
    pub fn cnf9(&self) -> CNF9_R {
        CNF9_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port n.10 mode bits"]
    #[inline(always)]
    pub fn mode10(&self) -> MODE10_R {
        MODE10_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port n.10 configuration bits"]
    #[inline(always)]
    pub fn cnf10(&self) -> CNF10_R {
        CNF10_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port n.11 mode bits"]
    #[inline(always)]
    pub fn mode11(&self) -> MODE11_R {
        MODE11_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port n.11 configuration bits"]
    #[inline(always)]
    pub fn cnf11(&self) -> CNF11_R {
        CNF11_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port n.12 mode bits"]
    #[inline(always)]
    pub fn mode12(&self) -> MODE12_R {
        MODE12_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port n.12 configuration bits"]
    #[inline(always)]
    pub fn cnf12(&self) -> CNF12_R {
        CNF12_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port n.13 mode bits"]
    #[inline(always)]
    pub fn mode13(&self) -> MODE13_R {
        MODE13_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port n.13 configuration bits"]
    #[inline(always)]
    pub fn cnf13(&self) -> CNF13_R {
        CNF13_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Port n.14 mode bits"]
    #[inline(always)]
    pub fn mode14(&self) -> MODE14_R {
        MODE14_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port n.14 configuration bits"]
    #[inline(always)]
    pub fn cnf14(&self) -> CNF14_R {
        CNF14_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port n.15 mode bits"]
    #[inline(always)]
    pub fn mode15(&self) -> MODE15_R {
        MODE15_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Port n.15 configuration bits"]
    #[inline(always)]
    pub fn cnf15(&self) -> CNF15_R {
        CNF15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port n.8 mode bits"]
    #[inline(always)]
    pub fn mode8(&mut self) -> MODE8_W {
        MODE8_W { w: self }
    }
    #[doc = "Bits 2:3 - Port n.8 configuration bits"]
    #[inline(always)]
    pub fn cnf8(&mut self) -> CNF8_W {
        CNF8_W { w: self }
    }
    #[doc = "Bits 4:5 - Port n.9 mode bits"]
    #[inline(always)]
    pub fn mode9(&mut self) -> MODE9_W {
        MODE9_W { w: self }
    }
    #[doc = "Bits 6:7 - Port n.9 configuration bits"]
    #[inline(always)]
    pub fn cnf9(&mut self) -> CNF9_W {
        CNF9_W { w: self }
    }
    #[doc = "Bits 8:9 - Port n.10 mode bits"]
    #[inline(always)]
    pub fn mode10(&mut self) -> MODE10_W {
        MODE10_W { w: self }
    }
    #[doc = "Bits 10:11 - Port n.10 configuration bits"]
    #[inline(always)]
    pub fn cnf10(&mut self) -> CNF10_W {
        CNF10_W { w: self }
    }
    #[doc = "Bits 12:13 - Port n.11 mode bits"]
    #[inline(always)]
    pub fn mode11(&mut self) -> MODE11_W {
        MODE11_W { w: self }
    }
    #[doc = "Bits 14:15 - Port n.11 configuration bits"]
    #[inline(always)]
    pub fn cnf11(&mut self) -> CNF11_W {
        CNF11_W { w: self }
    }
    #[doc = "Bits 16:17 - Port n.12 mode bits"]
    #[inline(always)]
    pub fn mode12(&mut self) -> MODE12_W {
        MODE12_W { w: self }
    }
    #[doc = "Bits 18:19 - Port n.12 configuration bits"]
    #[inline(always)]
    pub fn cnf12(&mut self) -> CNF12_W {
        CNF12_W { w: self }
    }
    #[doc = "Bits 20:21 - Port n.13 mode bits"]
    #[inline(always)]
    pub fn mode13(&mut self) -> MODE13_W {
        MODE13_W { w: self }
    }
    #[doc = "Bits 22:23 - Port n.13 configuration bits"]
    #[inline(always)]
    pub fn cnf13(&mut self) -> CNF13_W {
        CNF13_W { w: self }
    }
    #[doc = "Bits 24:25 - Port n.14 mode bits"]
    #[inline(always)]
    pub fn mode14(&mut self) -> MODE14_W {
        MODE14_W { w: self }
    }
    #[doc = "Bits 26:27 - Port n.14 configuration bits"]
    #[inline(always)]
    pub fn cnf14(&mut self) -> CNF14_W {
        CNF14_W { w: self }
    }
    #[doc = "Bits 28:29 - Port n.15 mode bits"]
    #[inline(always)]
    pub fn mode15(&mut self) -> MODE15_W {
        MODE15_W { w: self }
    }
    #[doc = "Bits 30:31 - Port n.15 configuration bits"]
    #[inline(always)]
    pub fn cnf15(&mut self) -> CNF15_W {
        CNF15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port configuration register high (GPIOn_CRL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crh](index.html) module"]
pub struct CRH_SPEC;
impl crate::RegisterSpec for CRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crh::R](R) reader structure"]
impl crate::Readable for CRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crh::W](W) writer structure"]
impl crate::Writable for CRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRH to value 0x4444_4444"]
impl crate::Resettable for CRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4444_4444
    }
}
