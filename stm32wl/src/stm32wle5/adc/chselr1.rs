#[doc = "Register `CHSELR1` reader"]
pub struct R(crate::R<CHSELR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSELR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSELR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSELR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHSELR1` writer"]
pub struct W(crate::W<CHSELR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHSELR1_SPEC>;
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
impl From<crate::W<CHSELR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHSELR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SQ1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SQ1_A {
    #[doc = "0: Channel 0 selected for the Nth conversion"]
    CH0 = 0,
    #[doc = "1: Channel 1 selected for the Nth conversion"]
    CH1 = 1,
    #[doc = "2: Channel 2 selected for the Nth conversion"]
    CH2 = 2,
    #[doc = "3: Channel 3 selected for the Nth conversion"]
    CH3 = 3,
    #[doc = "4: Channel 4 selected for the Nth conversion"]
    CH4 = 4,
    #[doc = "5: Channel 5 selected for the Nth conversion"]
    CH5 = 5,
    #[doc = "6: Channel 6 selected for the Nth conversion"]
    CH6 = 6,
    #[doc = "7: Channel 7 selected for the Nth conversion"]
    CH7 = 7,
    #[doc = "8: Channel 8 selected for the Nth conversion"]
    CH8 = 8,
    #[doc = "9: Channel 9 selected for the Nth conversion"]
    CH9 = 9,
    #[doc = "10: Channel 10 selected for the Nth conversion"]
    CH10 = 10,
    #[doc = "11: Channel 11 selected for the Nth conversion"]
    CH11 = 11,
    #[doc = "12: Channel 12 selected for the Nth conversion"]
    CH12 = 12,
    #[doc = "13: Channel 13 selected for the Nth conversion"]
    CH13 = 13,
    #[doc = "14: Channel 14 selected for the Nth conversion"]
    CH14 = 14,
    #[doc = "15: End of sequence"]
    EOS = 15,
}
impl From<SQ1_A> for u8 {
    #[inline(always)]
    fn from(variant: SQ1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SQ1` reader - SQ1"]
pub struct SQ1_R(crate::FieldReader<u8, SQ1_A>);
impl SQ1_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SQ1_A {
        match self.bits {
            0 => SQ1_A::CH0,
            1 => SQ1_A::CH1,
            2 => SQ1_A::CH2,
            3 => SQ1_A::CH3,
            4 => SQ1_A::CH4,
            5 => SQ1_A::CH5,
            6 => SQ1_A::CH6,
            7 => SQ1_A::CH7,
            8 => SQ1_A::CH8,
            9 => SQ1_A::CH9,
            10 => SQ1_A::CH10,
            11 => SQ1_A::CH11,
            12 => SQ1_A::CH12,
            13 => SQ1_A::CH13,
            14 => SQ1_A::CH14,
            15 => SQ1_A::EOS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        **self == SQ1_A::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        **self == SQ1_A::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        **self == SQ1_A::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        **self == SQ1_A::CH3
    }
    #[doc = "Checks if the value of the field is `CH4`"]
    #[inline(always)]
    pub fn is_ch4(&self) -> bool {
        **self == SQ1_A::CH4
    }
    #[doc = "Checks if the value of the field is `CH5`"]
    #[inline(always)]
    pub fn is_ch5(&self) -> bool {
        **self == SQ1_A::CH5
    }
    #[doc = "Checks if the value of the field is `CH6`"]
    #[inline(always)]
    pub fn is_ch6(&self) -> bool {
        **self == SQ1_A::CH6
    }
    #[doc = "Checks if the value of the field is `CH7`"]
    #[inline(always)]
    pub fn is_ch7(&self) -> bool {
        **self == SQ1_A::CH7
    }
    #[doc = "Checks if the value of the field is `CH8`"]
    #[inline(always)]
    pub fn is_ch8(&self) -> bool {
        **self == SQ1_A::CH8
    }
    #[doc = "Checks if the value of the field is `CH9`"]
    #[inline(always)]
    pub fn is_ch9(&self) -> bool {
        **self == SQ1_A::CH9
    }
    #[doc = "Checks if the value of the field is `CH10`"]
    #[inline(always)]
    pub fn is_ch10(&self) -> bool {
        **self == SQ1_A::CH10
    }
    #[doc = "Checks if the value of the field is `CH11`"]
    #[inline(always)]
    pub fn is_ch11(&self) -> bool {
        **self == SQ1_A::CH11
    }
    #[doc = "Checks if the value of the field is `CH12`"]
    #[inline(always)]
    pub fn is_ch12(&self) -> bool {
        **self == SQ1_A::CH12
    }
    #[doc = "Checks if the value of the field is `CH13`"]
    #[inline(always)]
    pub fn is_ch13(&self) -> bool {
        **self == SQ1_A::CH13
    }
    #[doc = "Checks if the value of the field is `CH14`"]
    #[inline(always)]
    pub fn is_ch14(&self) -> bool {
        **self == SQ1_A::CH14
    }
    #[doc = "Checks if the value of the field is `EOS`"]
    #[inline(always)]
    pub fn is_eos(&self) -> bool {
        **self == SQ1_A::EOS
    }
}
impl core::ops::Deref for SQ1_R {
    type Target = crate::FieldReader<u8, SQ1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ1` writer - SQ1"]
pub struct SQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SQ1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Channel 0 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(SQ1_A::CH0)
    }
    #[doc = "Channel 1 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(SQ1_A::CH1)
    }
    #[doc = "Channel 2 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(SQ1_A::CH2)
    }
    #[doc = "Channel 3 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(SQ1_A::CH3)
    }
    #[doc = "Channel 4 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut W {
        self.variant(SQ1_A::CH4)
    }
    #[doc = "Channel 5 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut W {
        self.variant(SQ1_A::CH5)
    }
    #[doc = "Channel 6 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut W {
        self.variant(SQ1_A::CH6)
    }
    #[doc = "Channel 7 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut W {
        self.variant(SQ1_A::CH7)
    }
    #[doc = "Channel 8 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch8(self) -> &'a mut W {
        self.variant(SQ1_A::CH8)
    }
    #[doc = "Channel 9 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch9(self) -> &'a mut W {
        self.variant(SQ1_A::CH9)
    }
    #[doc = "Channel 10 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch10(self) -> &'a mut W {
        self.variant(SQ1_A::CH10)
    }
    #[doc = "Channel 11 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch11(self) -> &'a mut W {
        self.variant(SQ1_A::CH11)
    }
    #[doc = "Channel 12 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch12(self) -> &'a mut W {
        self.variant(SQ1_A::CH12)
    }
    #[doc = "Channel 13 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch13(self) -> &'a mut W {
        self.variant(SQ1_A::CH13)
    }
    #[doc = "Channel 14 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch14(self) -> &'a mut W {
        self.variant(SQ1_A::CH14)
    }
    #[doc = "End of sequence"]
    #[inline(always)]
    pub fn eos(self) -> &'a mut W {
        self.variant(SQ1_A::EOS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "SQ2"]
pub type SQ2_A = SQ1_A;
#[doc = "Field `SQ2` reader - SQ2"]
pub type SQ2_R = SQ1_R;
#[doc = "Field `SQ2` writer - SQ2"]
pub struct SQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SQ2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Channel 0 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(SQ2_A::CH0)
    }
    #[doc = "Channel 1 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(SQ2_A::CH1)
    }
    #[doc = "Channel 2 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(SQ2_A::CH2)
    }
    #[doc = "Channel 3 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(SQ2_A::CH3)
    }
    #[doc = "Channel 4 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut W {
        self.variant(SQ2_A::CH4)
    }
    #[doc = "Channel 5 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut W {
        self.variant(SQ2_A::CH5)
    }
    #[doc = "Channel 6 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut W {
        self.variant(SQ2_A::CH6)
    }
    #[doc = "Channel 7 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut W {
        self.variant(SQ2_A::CH7)
    }
    #[doc = "Channel 8 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch8(self) -> &'a mut W {
        self.variant(SQ2_A::CH8)
    }
    #[doc = "Channel 9 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch9(self) -> &'a mut W {
        self.variant(SQ2_A::CH9)
    }
    #[doc = "Channel 10 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch10(self) -> &'a mut W {
        self.variant(SQ2_A::CH10)
    }
    #[doc = "Channel 11 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch11(self) -> &'a mut W {
        self.variant(SQ2_A::CH11)
    }
    #[doc = "Channel 12 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch12(self) -> &'a mut W {
        self.variant(SQ2_A::CH12)
    }
    #[doc = "Channel 13 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch13(self) -> &'a mut W {
        self.variant(SQ2_A::CH13)
    }
    #[doc = "Channel 14 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch14(self) -> &'a mut W {
        self.variant(SQ2_A::CH14)
    }
    #[doc = "End of sequence"]
    #[inline(always)]
    pub fn eos(self) -> &'a mut W {
        self.variant(SQ2_A::EOS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "SQ3"]
pub type SQ3_A = SQ1_A;
#[doc = "Field `SQ3` reader - SQ3"]
pub type SQ3_R = SQ1_R;
#[doc = "Field `SQ3` writer - SQ3"]
pub struct SQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SQ3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Channel 0 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(SQ3_A::CH0)
    }
    #[doc = "Channel 1 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(SQ3_A::CH1)
    }
    #[doc = "Channel 2 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(SQ3_A::CH2)
    }
    #[doc = "Channel 3 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(SQ3_A::CH3)
    }
    #[doc = "Channel 4 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut W {
        self.variant(SQ3_A::CH4)
    }
    #[doc = "Channel 5 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut W {
        self.variant(SQ3_A::CH5)
    }
    #[doc = "Channel 6 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut W {
        self.variant(SQ3_A::CH6)
    }
    #[doc = "Channel 7 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut W {
        self.variant(SQ3_A::CH7)
    }
    #[doc = "Channel 8 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch8(self) -> &'a mut W {
        self.variant(SQ3_A::CH8)
    }
    #[doc = "Channel 9 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch9(self) -> &'a mut W {
        self.variant(SQ3_A::CH9)
    }
    #[doc = "Channel 10 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch10(self) -> &'a mut W {
        self.variant(SQ3_A::CH10)
    }
    #[doc = "Channel 11 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch11(self) -> &'a mut W {
        self.variant(SQ3_A::CH11)
    }
    #[doc = "Channel 12 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch12(self) -> &'a mut W {
        self.variant(SQ3_A::CH12)
    }
    #[doc = "Channel 13 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch13(self) -> &'a mut W {
        self.variant(SQ3_A::CH13)
    }
    #[doc = "Channel 14 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch14(self) -> &'a mut W {
        self.variant(SQ3_A::CH14)
    }
    #[doc = "End of sequence"]
    #[inline(always)]
    pub fn eos(self) -> &'a mut W {
        self.variant(SQ3_A::EOS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "SQ4"]
pub type SQ4_A = SQ1_A;
#[doc = "Field `SQ4` reader - SQ4"]
pub type SQ4_R = SQ1_R;
#[doc = "Field `SQ4` writer - SQ4"]
pub struct SQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SQ4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Channel 0 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(SQ4_A::CH0)
    }
    #[doc = "Channel 1 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(SQ4_A::CH1)
    }
    #[doc = "Channel 2 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(SQ4_A::CH2)
    }
    #[doc = "Channel 3 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(SQ4_A::CH3)
    }
    #[doc = "Channel 4 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut W {
        self.variant(SQ4_A::CH4)
    }
    #[doc = "Channel 5 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut W {
        self.variant(SQ4_A::CH5)
    }
    #[doc = "Channel 6 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut W {
        self.variant(SQ4_A::CH6)
    }
    #[doc = "Channel 7 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut W {
        self.variant(SQ4_A::CH7)
    }
    #[doc = "Channel 8 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch8(self) -> &'a mut W {
        self.variant(SQ4_A::CH8)
    }
    #[doc = "Channel 9 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch9(self) -> &'a mut W {
        self.variant(SQ4_A::CH9)
    }
    #[doc = "Channel 10 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch10(self) -> &'a mut W {
        self.variant(SQ4_A::CH10)
    }
    #[doc = "Channel 11 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch11(self) -> &'a mut W {
        self.variant(SQ4_A::CH11)
    }
    #[doc = "Channel 12 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch12(self) -> &'a mut W {
        self.variant(SQ4_A::CH12)
    }
    #[doc = "Channel 13 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch13(self) -> &'a mut W {
        self.variant(SQ4_A::CH13)
    }
    #[doc = "Channel 14 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch14(self) -> &'a mut W {
        self.variant(SQ4_A::CH14)
    }
    #[doc = "End of sequence"]
    #[inline(always)]
    pub fn eos(self) -> &'a mut W {
        self.variant(SQ4_A::EOS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "SQ5"]
pub type SQ5_A = SQ1_A;
#[doc = "Field `SQ5` reader - SQ5"]
pub type SQ5_R = SQ1_R;
#[doc = "Field `SQ5` writer - SQ5"]
pub struct SQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SQ5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Channel 0 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(SQ5_A::CH0)
    }
    #[doc = "Channel 1 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(SQ5_A::CH1)
    }
    #[doc = "Channel 2 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(SQ5_A::CH2)
    }
    #[doc = "Channel 3 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(SQ5_A::CH3)
    }
    #[doc = "Channel 4 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut W {
        self.variant(SQ5_A::CH4)
    }
    #[doc = "Channel 5 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut W {
        self.variant(SQ5_A::CH5)
    }
    #[doc = "Channel 6 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut W {
        self.variant(SQ5_A::CH6)
    }
    #[doc = "Channel 7 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut W {
        self.variant(SQ5_A::CH7)
    }
    #[doc = "Channel 8 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch8(self) -> &'a mut W {
        self.variant(SQ5_A::CH8)
    }
    #[doc = "Channel 9 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch9(self) -> &'a mut W {
        self.variant(SQ5_A::CH9)
    }
    #[doc = "Channel 10 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch10(self) -> &'a mut W {
        self.variant(SQ5_A::CH10)
    }
    #[doc = "Channel 11 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch11(self) -> &'a mut W {
        self.variant(SQ5_A::CH11)
    }
    #[doc = "Channel 12 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch12(self) -> &'a mut W {
        self.variant(SQ5_A::CH12)
    }
    #[doc = "Channel 13 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch13(self) -> &'a mut W {
        self.variant(SQ5_A::CH13)
    }
    #[doc = "Channel 14 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch14(self) -> &'a mut W {
        self.variant(SQ5_A::CH14)
    }
    #[doc = "End of sequence"]
    #[inline(always)]
    pub fn eos(self) -> &'a mut W {
        self.variant(SQ5_A::EOS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "SQ6"]
pub type SQ6_A = SQ1_A;
#[doc = "Field `SQ6` reader - SQ6"]
pub type SQ6_R = SQ1_R;
#[doc = "Field `SQ6` writer - SQ6"]
pub struct SQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SQ6_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Channel 0 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(SQ6_A::CH0)
    }
    #[doc = "Channel 1 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(SQ6_A::CH1)
    }
    #[doc = "Channel 2 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(SQ6_A::CH2)
    }
    #[doc = "Channel 3 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(SQ6_A::CH3)
    }
    #[doc = "Channel 4 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut W {
        self.variant(SQ6_A::CH4)
    }
    #[doc = "Channel 5 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut W {
        self.variant(SQ6_A::CH5)
    }
    #[doc = "Channel 6 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut W {
        self.variant(SQ6_A::CH6)
    }
    #[doc = "Channel 7 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut W {
        self.variant(SQ6_A::CH7)
    }
    #[doc = "Channel 8 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch8(self) -> &'a mut W {
        self.variant(SQ6_A::CH8)
    }
    #[doc = "Channel 9 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch9(self) -> &'a mut W {
        self.variant(SQ6_A::CH9)
    }
    #[doc = "Channel 10 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch10(self) -> &'a mut W {
        self.variant(SQ6_A::CH10)
    }
    #[doc = "Channel 11 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch11(self) -> &'a mut W {
        self.variant(SQ6_A::CH11)
    }
    #[doc = "Channel 12 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch12(self) -> &'a mut W {
        self.variant(SQ6_A::CH12)
    }
    #[doc = "Channel 13 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch13(self) -> &'a mut W {
        self.variant(SQ6_A::CH13)
    }
    #[doc = "Channel 14 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch14(self) -> &'a mut W {
        self.variant(SQ6_A::CH14)
    }
    #[doc = "End of sequence"]
    #[inline(always)]
    pub fn eos(self) -> &'a mut W {
        self.variant(SQ6_A::EOS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "SQ7"]
pub type SQ7_A = SQ1_A;
#[doc = "Field `SQ7` reader - SQ7"]
pub type SQ7_R = SQ1_R;
#[doc = "Field `SQ7` writer - SQ7"]
pub struct SQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SQ7_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Channel 0 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(SQ7_A::CH0)
    }
    #[doc = "Channel 1 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(SQ7_A::CH1)
    }
    #[doc = "Channel 2 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(SQ7_A::CH2)
    }
    #[doc = "Channel 3 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(SQ7_A::CH3)
    }
    #[doc = "Channel 4 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut W {
        self.variant(SQ7_A::CH4)
    }
    #[doc = "Channel 5 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut W {
        self.variant(SQ7_A::CH5)
    }
    #[doc = "Channel 6 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut W {
        self.variant(SQ7_A::CH6)
    }
    #[doc = "Channel 7 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut W {
        self.variant(SQ7_A::CH7)
    }
    #[doc = "Channel 8 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch8(self) -> &'a mut W {
        self.variant(SQ7_A::CH8)
    }
    #[doc = "Channel 9 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch9(self) -> &'a mut W {
        self.variant(SQ7_A::CH9)
    }
    #[doc = "Channel 10 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch10(self) -> &'a mut W {
        self.variant(SQ7_A::CH10)
    }
    #[doc = "Channel 11 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch11(self) -> &'a mut W {
        self.variant(SQ7_A::CH11)
    }
    #[doc = "Channel 12 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch12(self) -> &'a mut W {
        self.variant(SQ7_A::CH12)
    }
    #[doc = "Channel 13 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch13(self) -> &'a mut W {
        self.variant(SQ7_A::CH13)
    }
    #[doc = "Channel 14 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch14(self) -> &'a mut W {
        self.variant(SQ7_A::CH14)
    }
    #[doc = "End of sequence"]
    #[inline(always)]
    pub fn eos(self) -> &'a mut W {
        self.variant(SQ7_A::EOS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "SQ8"]
pub type SQ8_A = SQ1_A;
#[doc = "Field `SQ8` reader - SQ8"]
pub type SQ8_R = SQ1_R;
#[doc = "Field `SQ8` writer - SQ8"]
pub struct SQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SQ8_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Channel 0 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(SQ8_A::CH0)
    }
    #[doc = "Channel 1 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(SQ8_A::CH1)
    }
    #[doc = "Channel 2 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(SQ8_A::CH2)
    }
    #[doc = "Channel 3 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(SQ8_A::CH3)
    }
    #[doc = "Channel 4 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut W {
        self.variant(SQ8_A::CH4)
    }
    #[doc = "Channel 5 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut W {
        self.variant(SQ8_A::CH5)
    }
    #[doc = "Channel 6 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut W {
        self.variant(SQ8_A::CH6)
    }
    #[doc = "Channel 7 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut W {
        self.variant(SQ8_A::CH7)
    }
    #[doc = "Channel 8 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch8(self) -> &'a mut W {
        self.variant(SQ8_A::CH8)
    }
    #[doc = "Channel 9 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch9(self) -> &'a mut W {
        self.variant(SQ8_A::CH9)
    }
    #[doc = "Channel 10 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch10(self) -> &'a mut W {
        self.variant(SQ8_A::CH10)
    }
    #[doc = "Channel 11 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch11(self) -> &'a mut W {
        self.variant(SQ8_A::CH11)
    }
    #[doc = "Channel 12 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch12(self) -> &'a mut W {
        self.variant(SQ8_A::CH12)
    }
    #[doc = "Channel 13 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch13(self) -> &'a mut W {
        self.variant(SQ8_A::CH13)
    }
    #[doc = "Channel 14 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch14(self) -> &'a mut W {
        self.variant(SQ8_A::CH14)
    }
    #[doc = "End of sequence"]
    #[inline(always)]
    pub fn eos(self) -> &'a mut W {
        self.variant(SQ8_A::EOS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - SQ1"]
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SQ2"]
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SQ3"]
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SQ4"]
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SQ5"]
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SQ6"]
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - SQ7"]
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - SQ8"]
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SQ1"]
    #[inline(always)]
    pub fn sq1(&mut self) -> SQ1_W {
        SQ1_W { w: self }
    }
    #[doc = "Bits 4:7 - SQ2"]
    #[inline(always)]
    pub fn sq2(&mut self) -> SQ2_W {
        SQ2_W { w: self }
    }
    #[doc = "Bits 8:11 - SQ3"]
    #[inline(always)]
    pub fn sq3(&mut self) -> SQ3_W {
        SQ3_W { w: self }
    }
    #[doc = "Bits 12:15 - SQ4"]
    #[inline(always)]
    pub fn sq4(&mut self) -> SQ4_W {
        SQ4_W { w: self }
    }
    #[doc = "Bits 16:19 - SQ5"]
    #[inline(always)]
    pub fn sq5(&mut self) -> SQ5_W {
        SQ5_W { w: self }
    }
    #[doc = "Bits 20:23 - SQ6"]
    #[inline(always)]
    pub fn sq6(&mut self) -> SQ6_W {
        SQ6_W { w: self }
    }
    #[doc = "Bits 24:27 - SQ7"]
    #[inline(always)]
    pub fn sq7(&mut self) -> SQ7_W {
        SQ7_W { w: self }
    }
    #[doc = "Bits 28:31 - SQ8"]
    #[inline(always)]
    pub fn sq8(&mut self) -> SQ8_W {
        SQ8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chselr1](index.html) module"]
pub struct CHSELR1_SPEC;
impl crate::RegisterSpec for CHSELR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chselr1::R](R) reader structure"]
impl crate::Readable for CHSELR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chselr1::W](W) writer structure"]
impl crate::Writable for CHSELR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHSELR1 to value 0"]
impl crate::Resettable for CHSELR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
