#[doc = "Register `AFRL` reader"]
pub struct R(crate::R<AFRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFRL` writer"]
pub struct W(crate::W<AFRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFRL_SPEC>;
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
impl From<crate::W<AFRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AFR0_A {
    #[doc = "0: AF0"]
    AF0 = 0,
    #[doc = "1: AF1"]
    AF1 = 1,
    #[doc = "2: AF2"]
    AF2 = 2,
    #[doc = "3: AF3"]
    AF3 = 3,
    #[doc = "4: AF4"]
    AF4 = 4,
    #[doc = "5: AF5"]
    AF5 = 5,
    #[doc = "6: AF6"]
    AF6 = 6,
    #[doc = "7: AF7"]
    AF7 = 7,
    #[doc = "8: AF8"]
    AF8 = 8,
    #[doc = "9: AF9"]
    AF9 = 9,
    #[doc = "10: AF10"]
    AF10 = 10,
    #[doc = "11: AF11"]
    AF11 = 11,
    #[doc = "12: AF12"]
    AF12 = 12,
    #[doc = "13: AF13"]
    AF13 = 13,
    #[doc = "14: AF14"]
    AF14 = 14,
    #[doc = "15: AF15"]
    AF15 = 15,
}
impl From<AFR0_A> for u8 {
    #[inline(always)]
    fn from(variant: AFR0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AFR0` reader - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub struct AFR0_R(crate::FieldReader<u8, AFR0_A>);
impl AFR0_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFR0_A {
        match self.bits {
            0 => AFR0_A::AF0,
            1 => AFR0_A::AF1,
            2 => AFR0_A::AF2,
            3 => AFR0_A::AF3,
            4 => AFR0_A::AF4,
            5 => AFR0_A::AF5,
            6 => AFR0_A::AF6,
            7 => AFR0_A::AF7,
            8 => AFR0_A::AF8,
            9 => AFR0_A::AF9,
            10 => AFR0_A::AF10,
            11 => AFR0_A::AF11,
            12 => AFR0_A::AF12,
            13 => AFR0_A::AF13,
            14 => AFR0_A::AF14,
            15 => AFR0_A::AF15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AF0`"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        **self == AFR0_A::AF0
    }
    #[doc = "Checks if the value of the field is `AF1`"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        **self == AFR0_A::AF1
    }
    #[doc = "Checks if the value of the field is `AF2`"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        **self == AFR0_A::AF2
    }
    #[doc = "Checks if the value of the field is `AF3`"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        **self == AFR0_A::AF3
    }
    #[doc = "Checks if the value of the field is `AF4`"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        **self == AFR0_A::AF4
    }
    #[doc = "Checks if the value of the field is `AF5`"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        **self == AFR0_A::AF5
    }
    #[doc = "Checks if the value of the field is `AF6`"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        **self == AFR0_A::AF6
    }
    #[doc = "Checks if the value of the field is `AF7`"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        **self == AFR0_A::AF7
    }
    #[doc = "Checks if the value of the field is `AF8`"]
    #[inline(always)]
    pub fn is_af8(&self) -> bool {
        **self == AFR0_A::AF8
    }
    #[doc = "Checks if the value of the field is `AF9`"]
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        **self == AFR0_A::AF9
    }
    #[doc = "Checks if the value of the field is `AF10`"]
    #[inline(always)]
    pub fn is_af10(&self) -> bool {
        **self == AFR0_A::AF10
    }
    #[doc = "Checks if the value of the field is `AF11`"]
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        **self == AFR0_A::AF11
    }
    #[doc = "Checks if the value of the field is `AF12`"]
    #[inline(always)]
    pub fn is_af12(&self) -> bool {
        **self == AFR0_A::AF12
    }
    #[doc = "Checks if the value of the field is `AF13`"]
    #[inline(always)]
    pub fn is_af13(&self) -> bool {
        **self == AFR0_A::AF13
    }
    #[doc = "Checks if the value of the field is `AF14`"]
    #[inline(always)]
    pub fn is_af14(&self) -> bool {
        **self == AFR0_A::AF14
    }
    #[doc = "Checks if the value of the field is `AF15`"]
    #[inline(always)]
    pub fn is_af15(&self) -> bool {
        **self == AFR0_A::AF15
    }
}
impl core::ops::Deref for AFR0_R {
    type Target = crate::FieldReader<u8, AFR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFR0` writer - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub struct AFR0_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFR0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFR0_A::AF0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFR0_A::AF1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFR0_A::AF2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFR0_A::AF3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFR0_A::AF4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFR0_A::AF5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFR0_A::AF6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFR0_A::AF7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFR0_A::AF8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFR0_A::AF9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFR0_A::AF10)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFR0_A::AF11)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFR0_A::AF12)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFR0_A::AF13)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFR0_A::AF14)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFR0_A::AF15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub type AFR1_A = AFR0_A;
#[doc = "Field `AFR1` reader - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub type AFR1_R = AFR0_R;
#[doc = "Field `AFR1` writer - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub struct AFR1_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFR1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFR1_A::AF0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFR1_A::AF1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFR1_A::AF2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFR1_A::AF3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFR1_A::AF4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFR1_A::AF5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFR1_A::AF6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFR1_A::AF7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFR1_A::AF8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFR1_A::AF9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFR1_A::AF10)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFR1_A::AF11)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFR1_A::AF12)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFR1_A::AF13)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFR1_A::AF14)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFR1_A::AF15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub type AFR2_A = AFR0_A;
#[doc = "Field `AFR2` reader - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub type AFR2_R = AFR0_R;
#[doc = "Field `AFR2` writer - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub struct AFR2_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFR2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFR2_A::AF0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFR2_A::AF1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFR2_A::AF2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFR2_A::AF3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFR2_A::AF4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFR2_A::AF5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFR2_A::AF6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFR2_A::AF7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFR2_A::AF8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFR2_A::AF9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFR2_A::AF10)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFR2_A::AF11)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFR2_A::AF12)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFR2_A::AF13)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFR2_A::AF14)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFR2_A::AF15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub type AFR3_A = AFR0_A;
#[doc = "Field `AFR3` reader - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub type AFR3_R = AFR0_R;
#[doc = "Field `AFR3` writer - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub struct AFR3_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFR3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFR3_A::AF0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFR3_A::AF1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFR3_A::AF2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFR3_A::AF3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFR3_A::AF4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFR3_A::AF5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFR3_A::AF6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFR3_A::AF7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFR3_A::AF8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFR3_A::AF9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFR3_A::AF10)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFR3_A::AF11)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFR3_A::AF12)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFR3_A::AF13)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFR3_A::AF14)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFR3_A::AF15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub type AFR4_A = AFR0_A;
#[doc = "Field `AFR4` reader - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub type AFR4_R = AFR0_R;
#[doc = "Field `AFR4` writer - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub struct AFR4_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFR4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFR4_A::AF0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFR4_A::AF1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFR4_A::AF2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFR4_A::AF3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFR4_A::AF4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFR4_A::AF5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFR4_A::AF6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFR4_A::AF7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFR4_A::AF8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFR4_A::AF9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFR4_A::AF10)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFR4_A::AF11)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFR4_A::AF12)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFR4_A::AF13)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFR4_A::AF14)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFR4_A::AF15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub type AFR5_A = AFR0_A;
#[doc = "Field `AFR5` reader - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub type AFR5_R = AFR0_R;
#[doc = "Field `AFR5` writer - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub struct AFR5_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFR5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFR5_A::AF0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFR5_A::AF1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFR5_A::AF2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFR5_A::AF3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFR5_A::AF4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFR5_A::AF5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFR5_A::AF6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFR5_A::AF7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFR5_A::AF8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFR5_A::AF9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFR5_A::AF10)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFR5_A::AF11)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFR5_A::AF12)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFR5_A::AF13)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFR5_A::AF14)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFR5_A::AF15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub type AFR6_A = AFR0_A;
#[doc = "Field `AFR6` reader - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub type AFR6_R = AFR0_R;
#[doc = "Field `AFR6` writer - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub struct AFR6_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFR6_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFR6_A::AF0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFR6_A::AF1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFR6_A::AF2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFR6_A::AF3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFR6_A::AF4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFR6_A::AF5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFR6_A::AF6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFR6_A::AF7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFR6_A::AF8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFR6_A::AF9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFR6_A::AF10)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFR6_A::AF11)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFR6_A::AF12)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFR6_A::AF13)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFR6_A::AF14)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFR6_A::AF15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub type AFR7_A = AFR0_A;
#[doc = "Field `AFR7` reader - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub type AFR7_R = AFR0_R;
#[doc = "Field `AFR7` writer - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
pub struct AFR7_W<'a> {
    w: &'a mut W,
}
impl<'a> AFR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFR7_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "AF0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFR7_A::AF0)
    }
    #[doc = "AF1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFR7_A::AF1)
    }
    #[doc = "AF2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFR7_A::AF2)
    }
    #[doc = "AF3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFR7_A::AF3)
    }
    #[doc = "AF4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFR7_A::AF4)
    }
    #[doc = "AF5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFR7_A::AF5)
    }
    #[doc = "AF6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFR7_A::AF6)
    }
    #[doc = "AF7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFR7_A::AF7)
    }
    #[doc = "AF8"]
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFR7_A::AF8)
    }
    #[doc = "AF9"]
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFR7_A::AF9)
    }
    #[doc = "AF10"]
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFR7_A::AF10)
    }
    #[doc = "AF11"]
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFR7_A::AF11)
    }
    #[doc = "AF12"]
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFR7_A::AF12)
    }
    #[doc = "AF13"]
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFR7_A::AF13)
    }
    #[doc = "AF14"]
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFR7_A::AF14)
    }
    #[doc = "AF15"]
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFR7_A::AF15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr0(&self) -> AFR0_R {
        AFR0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr1(&self) -> AFR1_R {
        AFR1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr2(&self) -> AFR2_R {
        AFR2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr3(&self) -> AFR3_R {
        AFR3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr4(&self) -> AFR4_R {
        AFR4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr5(&self) -> AFR5_R {
        AFR5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr6(&self) -> AFR6_R {
        AFR6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr7(&self) -> AFR7_R {
        AFR7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr0(&mut self) -> AFR0_W {
        AFR0_W { w: self }
    }
    #[doc = "Bits 4:7 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr1(&mut self) -> AFR1_W {
        AFR1_W { w: self }
    }
    #[doc = "Bits 8:11 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr2(&mut self) -> AFR2_W {
        AFR2_W { w: self }
    }
    #[doc = "Bits 12:15 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr3(&mut self) -> AFR3_W {
        AFR3_W { w: self }
    }
    #[doc = "Bits 16:19 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr4(&mut self) -> AFR4_W {
        AFR4_W { w: self }
    }
    #[doc = "Bits 20:23 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr5(&mut self) -> AFR5_W {
        AFR5_W { w: self }
    }
    #[doc = "Bits 24:27 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr6(&mut self) -> AFR6_W {
        AFR6_W { w: self }
    }
    #[doc = "Bits 28:31 - 3:0\\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:"]
    #[inline(always)]
    pub fn afr7(&mut self) -> AFR7_W {
        AFR7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrl](index.html) module"]
pub struct AFRL_SPEC;
impl crate::RegisterSpec for AFRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afrl::R](R) reader structure"]
impl crate::Readable for AFRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afrl::W](W) writer structure"]
impl crate::Writable for AFRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::Resettable for AFRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
