#[doc = "Register `PUPDR` reader"]
pub struct R(crate::R<PUPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUPDR` writer"]
pub struct W(crate::W<PUPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUPDR_SPEC>;
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
impl From<crate::W<PUPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPD15_A = PUPD0_A;
#[doc = "Field `PUPD15` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD15_R = PUPD0_R;
#[doc = "Field `PUPD15` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPD15_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPD15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPD15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPD15_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPD15_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPD15_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPD14_A = PUPD0_A;
#[doc = "Field `PUPD14` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD14_R = PUPD0_R;
#[doc = "Field `PUPD14` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPD14_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPD14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPD14_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPD14_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPD14_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPD14_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPD13_A = PUPD0_A;
#[doc = "Field `PUPD13` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD13_R = PUPD0_R;
#[doc = "Field `PUPD13` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPD13_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPD13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPD13_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPD13_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPD13_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPD13_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPD12_A = PUPD0_A;
#[doc = "Field `PUPD12` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD12_R = PUPD0_R;
#[doc = "Field `PUPD12` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPD12_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPD12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPD12_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPD12_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPD12_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPD12_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPD11_A = PUPD0_A;
#[doc = "Field `PUPD11` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD11_R = PUPD0_R;
#[doc = "Field `PUPD11` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPD11_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPD11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPD11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPD11_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPD11_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPD11_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPD10_A = PUPD0_A;
#[doc = "Field `PUPD10` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD10_R = PUPD0_R;
#[doc = "Field `PUPD10` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPD10_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPD10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPD10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPD10_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPD10_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPD10_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPD9_A = PUPD0_A;
#[doc = "Field `PUPD9` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD9_R = PUPD0_R;
#[doc = "Field `PUPD9` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPD9_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPD9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPD9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPD9_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPD9_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPD9_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPD8_A = PUPD0_A;
#[doc = "Field `PUPD8` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD8_R = PUPD0_R;
#[doc = "Field `PUPD8` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPD8_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPD8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPD8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPD8_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPD8_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPD8_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPD7_A = PUPD0_A;
#[doc = "Field `PUPD7` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD7_R = PUPD0_R;
#[doc = "Field `PUPD7` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPD7_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPD7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPD7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPD7_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPD7_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPD7_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPD6_A = PUPD0_A;
#[doc = "Field `PUPD6` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD6_R = PUPD0_R;
#[doc = "Field `PUPD6` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPD6_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPD6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPD6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPD6_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPD6_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPD6_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPD5_A = PUPD0_A;
#[doc = "Field `PUPD5` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD5_R = PUPD0_R;
#[doc = "Field `PUPD5` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPD5_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPD5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPD5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPD5_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPD5_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPD5_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPD4_A = PUPD0_A;
#[doc = "Field `PUPD4` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD4_R = PUPD0_R;
#[doc = "Field `PUPD4` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPD4_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPD4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPD4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPD4_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPD4_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPD4_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPD3_A = PUPD0_A;
#[doc = "Field `PUPD3` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD3_R = PUPD0_R;
#[doc = "Field `PUPD3` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPD3_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPD3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPD3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPD3_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPD3_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPD3_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPD2_A = PUPD0_A;
#[doc = "Field `PUPD2` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD2_R = PUPD0_R;
#[doc = "Field `PUPD2` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPD2_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPD2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPD2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPD2_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPD2_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPD2_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type PUPD1_A = PUPD0_A;
#[doc = "Field `PUPD1` reader - Port x configuration bits (y = 0..15)"]
pub type PUPD1_R = PUPD0_R;
#[doc = "Field `PUPD1` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPD1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPD1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPD1_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPD1_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPD1_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PUPD0_A {
    #[doc = "0: No pull-up, pull-down"]
    FLOATING = 0,
    #[doc = "1: Pull-up"]
    PULLUP = 1,
    #[doc = "2: Pull-down"]
    PULLDOWN = 2,
}
impl From<PUPD0_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPD0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PUPD0` reader - Port x configuration bits (y = 0..15)"]
pub struct PUPD0_R(crate::FieldReader<u8, PUPD0_A>);
impl PUPD0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PUPD0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PUPD0_A> {
        match self.bits {
            0 => Some(PUPD0_A::FLOATING),
            1 => Some(PUPD0_A::PULLUP),
            2 => Some(PUPD0_A::PULLDOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLOATING`"]
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        **self == PUPD0_A::FLOATING
    }
    #[doc = "Checks if the value of the field is `PULLUP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PUPD0_A::PULLUP
    }
    #[doc = "Checks if the value of the field is `PULLDOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PUPD0_A::PULLDOWN
    }
}
impl core::ops::Deref for PUPD0_R {
    type Target = crate::FieldReader<u8, PUPD0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUPD0` writer - Port x configuration bits (y = 0..15)"]
pub struct PUPD0_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPD0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUPD0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPD0_A::FLOATING)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPD0_A::PULLUP)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPD0_A::PULLDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd15(&self) -> PUPD15_R {
        PUPD15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd14(&self) -> PUPD14_R {
        PUPD14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd13(&self) -> PUPD13_R {
        PUPD13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd12(&self) -> PUPD12_R {
        PUPD12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd11(&self) -> PUPD11_R {
        PUPD11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd10(&self) -> PUPD10_R {
        PUPD10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd9(&self) -> PUPD9_R {
        PUPD9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd8(&self) -> PUPD8_R {
        PUPD8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd7(&self) -> PUPD7_R {
        PUPD7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd6(&self) -> PUPD6_R {
        PUPD6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd5(&self) -> PUPD5_R {
        PUPD5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd4(&self) -> PUPD4_R {
        PUPD4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd3(&self) -> PUPD3_R {
        PUPD3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd2(&self) -> PUPD2_R {
        PUPD2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd1(&self) -> PUPD1_R {
        PUPD1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd0(&self) -> PUPD0_R {
        PUPD0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd15(&mut self) -> PUPD15_W {
        PUPD15_W { w: self }
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd14(&mut self) -> PUPD14_W {
        PUPD14_W { w: self }
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd13(&mut self) -> PUPD13_W {
        PUPD13_W { w: self }
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd12(&mut self) -> PUPD12_W {
        PUPD12_W { w: self }
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd11(&mut self) -> PUPD11_W {
        PUPD11_W { w: self }
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd10(&mut self) -> PUPD10_W {
        PUPD10_W { w: self }
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd9(&mut self) -> PUPD9_W {
        PUPD9_W { w: self }
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd8(&mut self) -> PUPD8_W {
        PUPD8_W { w: self }
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd7(&mut self) -> PUPD7_W {
        PUPD7_W { w: self }
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd6(&mut self) -> PUPD6_W {
        PUPD6_W { w: self }
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd5(&mut self) -> PUPD5_W {
        PUPD5_W { w: self }
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd4(&mut self) -> PUPD4_W {
        PUPD4_W { w: self }
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd3(&mut self) -> PUPD3_W {
        PUPD3_W { w: self }
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd2(&mut self) -> PUPD2_W {
        PUPD2_W { w: self }
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd1(&mut self) -> PUPD1_W {
        PUPD1_W { w: self }
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd0(&mut self) -> PUPD0_W {
        PUPD0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pupdr](index.html) module"]
pub struct PUPDR_SPEC;
impl crate::RegisterSpec for PUPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pupdr::R](R) reader structure"]
impl crate::Readable for PUPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pupdr::W](W) writer structure"]
impl crate::Writable for PUPDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUPDR to value 0"]
impl crate::Resettable for PUPDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
