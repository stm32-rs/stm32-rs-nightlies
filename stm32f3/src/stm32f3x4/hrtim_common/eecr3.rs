#[doc = "Register `EECR3` reader"]
pub struct R(crate::R<EECR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EECR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EECR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EECR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EECR3` writer"]
pub struct W(crate::W<EECR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EECR3_SPEC>;
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
impl From<crate::W<EECR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EECR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EEVSD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EEVSD_A {
    #[doc = "0: f_EEVS=f_HRTIM"]
    DIV1 = 0,
    #[doc = "1: f_EEVS=f_HRTIM/2"]
    DIV2 = 1,
    #[doc = "2: f_EEVS=f_HRTIM/4"]
    DIV4 = 2,
    #[doc = "3: f_EEVS=f_HRTIM/8"]
    DIV8 = 3,
}
impl From<EEVSD_A> for u8 {
    #[inline(always)]
    fn from(variant: EEVSD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EEVSD` reader - EEVSD"]
pub struct EEVSD_R(crate::FieldReader<u8, EEVSD_A>);
impl EEVSD_R {
    pub(crate) fn new(bits: u8) -> Self {
        EEVSD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEVSD_A {
        match self.bits {
            0 => EEVSD_A::DIV1,
            1 => EEVSD_A::DIV2,
            2 => EEVSD_A::DIV4,
            3 => EEVSD_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == EEVSD_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == EEVSD_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == EEVSD_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == EEVSD_A::DIV8
    }
}
impl core::ops::Deref for EEVSD_R {
    type Target = crate::FieldReader<u8, EEVSD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEVSD` writer - EEVSD"]
pub struct EEVSD_W<'a> {
    w: &'a mut W,
}
impl<'a> EEVSD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEVSD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "f_EEVS=f_HRTIM"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(EEVSD_A::DIV1)
    }
    #[doc = "f_EEVS=f_HRTIM/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(EEVSD_A::DIV2)
    }
    #[doc = "f_EEVS=f_HRTIM/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(EEVSD_A::DIV4)
    }
    #[doc = "f_EEVS=f_HRTIM/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(EEVSD_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "EE10F"]
pub type EE10F_A = EE6F_A;
#[doc = "Field `EE10F` reader - EE10F"]
pub type EE10F_R = EE6F_R;
#[doc = "Field `EE10F` writer - EE10F"]
pub struct EE10F_W<'a> {
    w: &'a mut W,
}
impl<'a> EE10F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE10F_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE10F_A::DISABLED)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=2"]
    #[inline(always)]
    pub fn div1_n2(self) -> &'a mut W {
        self.variant(EE10F_A::DIV1_N2)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=4"]
    #[inline(always)]
    pub fn div1_n4(self) -> &'a mut W {
        self.variant(EE10F_A::DIV1_N4)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=8"]
    #[inline(always)]
    pub fn div1_n8(self) -> &'a mut W {
        self.variant(EE10F_A::DIV1_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=6"]
    #[inline(always)]
    pub fn div2_n6(self) -> &'a mut W {
        self.variant(EE10F_A::DIV2_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=8"]
    #[inline(always)]
    pub fn div2_n8(self) -> &'a mut W {
        self.variant(EE10F_A::DIV2_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=6"]
    #[inline(always)]
    pub fn div4_n6(self) -> &'a mut W {
        self.variant(EE10F_A::DIV4_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=8"]
    #[inline(always)]
    pub fn div4_n8(self) -> &'a mut W {
        self.variant(EE10F_A::DIV4_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=6"]
    #[inline(always)]
    pub fn div8_n6(self) -> &'a mut W {
        self.variant(EE10F_A::DIV8_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=8"]
    #[inline(always)]
    pub fn div8_n8(self) -> &'a mut W {
        self.variant(EE10F_A::DIV8_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=5"]
    #[inline(always)]
    pub fn div16_n5(self) -> &'a mut W {
        self.variant(EE10F_A::DIV16_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=6"]
    #[inline(always)]
    pub fn div16_n6(self) -> &'a mut W {
        self.variant(EE10F_A::DIV16_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=8"]
    #[inline(always)]
    pub fn div16_n8(self) -> &'a mut W {
        self.variant(EE10F_A::DIV16_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=5"]
    #[inline(always)]
    pub fn div32_n5(self) -> &'a mut W {
        self.variant(EE10F_A::DIV32_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=6"]
    #[inline(always)]
    pub fn div32_n6(self) -> &'a mut W {
        self.variant(EE10F_A::DIV32_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=8"]
    #[inline(always)]
    pub fn div32_n8(self) -> &'a mut W {
        self.variant(EE10F_A::DIV32_N8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "EE9F"]
pub type EE9F_A = EE6F_A;
#[doc = "Field `EE9F` reader - EE9F"]
pub type EE9F_R = EE6F_R;
#[doc = "Field `EE9F` writer - EE9F"]
pub struct EE9F_W<'a> {
    w: &'a mut W,
}
impl<'a> EE9F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE9F_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE9F_A::DISABLED)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=2"]
    #[inline(always)]
    pub fn div1_n2(self) -> &'a mut W {
        self.variant(EE9F_A::DIV1_N2)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=4"]
    #[inline(always)]
    pub fn div1_n4(self) -> &'a mut W {
        self.variant(EE9F_A::DIV1_N4)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=8"]
    #[inline(always)]
    pub fn div1_n8(self) -> &'a mut W {
        self.variant(EE9F_A::DIV1_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=6"]
    #[inline(always)]
    pub fn div2_n6(self) -> &'a mut W {
        self.variant(EE9F_A::DIV2_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=8"]
    #[inline(always)]
    pub fn div2_n8(self) -> &'a mut W {
        self.variant(EE9F_A::DIV2_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=6"]
    #[inline(always)]
    pub fn div4_n6(self) -> &'a mut W {
        self.variant(EE9F_A::DIV4_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=8"]
    #[inline(always)]
    pub fn div4_n8(self) -> &'a mut W {
        self.variant(EE9F_A::DIV4_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=6"]
    #[inline(always)]
    pub fn div8_n6(self) -> &'a mut W {
        self.variant(EE9F_A::DIV8_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=8"]
    #[inline(always)]
    pub fn div8_n8(self) -> &'a mut W {
        self.variant(EE9F_A::DIV8_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=5"]
    #[inline(always)]
    pub fn div16_n5(self) -> &'a mut W {
        self.variant(EE9F_A::DIV16_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=6"]
    #[inline(always)]
    pub fn div16_n6(self) -> &'a mut W {
        self.variant(EE9F_A::DIV16_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=8"]
    #[inline(always)]
    pub fn div16_n8(self) -> &'a mut W {
        self.variant(EE9F_A::DIV16_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=5"]
    #[inline(always)]
    pub fn div32_n5(self) -> &'a mut W {
        self.variant(EE9F_A::DIV32_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=6"]
    #[inline(always)]
    pub fn div32_n6(self) -> &'a mut W {
        self.variant(EE9F_A::DIV32_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=8"]
    #[inline(always)]
    pub fn div32_n8(self) -> &'a mut W {
        self.variant(EE9F_A::DIV32_N8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
#[doc = "EE8F"]
pub type EE8F_A = EE6F_A;
#[doc = "Field `EE8F` reader - EE8F"]
pub type EE8F_R = EE6F_R;
#[doc = "Field `EE8F` writer - EE8F"]
pub struct EE8F_W<'a> {
    w: &'a mut W,
}
impl<'a> EE8F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE8F_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE8F_A::DISABLED)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=2"]
    #[inline(always)]
    pub fn div1_n2(self) -> &'a mut W {
        self.variant(EE8F_A::DIV1_N2)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=4"]
    #[inline(always)]
    pub fn div1_n4(self) -> &'a mut W {
        self.variant(EE8F_A::DIV1_N4)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=8"]
    #[inline(always)]
    pub fn div1_n8(self) -> &'a mut W {
        self.variant(EE8F_A::DIV1_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=6"]
    #[inline(always)]
    pub fn div2_n6(self) -> &'a mut W {
        self.variant(EE8F_A::DIV2_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=8"]
    #[inline(always)]
    pub fn div2_n8(self) -> &'a mut W {
        self.variant(EE8F_A::DIV2_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=6"]
    #[inline(always)]
    pub fn div4_n6(self) -> &'a mut W {
        self.variant(EE8F_A::DIV4_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=8"]
    #[inline(always)]
    pub fn div4_n8(self) -> &'a mut W {
        self.variant(EE8F_A::DIV4_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=6"]
    #[inline(always)]
    pub fn div8_n6(self) -> &'a mut W {
        self.variant(EE8F_A::DIV8_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=8"]
    #[inline(always)]
    pub fn div8_n8(self) -> &'a mut W {
        self.variant(EE8F_A::DIV8_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=5"]
    #[inline(always)]
    pub fn div16_n5(self) -> &'a mut W {
        self.variant(EE8F_A::DIV16_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=6"]
    #[inline(always)]
    pub fn div16_n6(self) -> &'a mut W {
        self.variant(EE8F_A::DIV16_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=8"]
    #[inline(always)]
    pub fn div16_n8(self) -> &'a mut W {
        self.variant(EE8F_A::DIV16_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=5"]
    #[inline(always)]
    pub fn div32_n5(self) -> &'a mut W {
        self.variant(EE8F_A::DIV32_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=6"]
    #[inline(always)]
    pub fn div32_n6(self) -> &'a mut W {
        self.variant(EE8F_A::DIV32_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=8"]
    #[inline(always)]
    pub fn div32_n8(self) -> &'a mut W {
        self.variant(EE8F_A::DIV32_N8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "EE7F"]
pub type EE7F_A = EE6F_A;
#[doc = "Field `EE7F` reader - EE7F"]
pub type EE7F_R = EE6F_R;
#[doc = "Field `EE7F` writer - EE7F"]
pub struct EE7F_W<'a> {
    w: &'a mut W,
}
impl<'a> EE7F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE7F_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE7F_A::DISABLED)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=2"]
    #[inline(always)]
    pub fn div1_n2(self) -> &'a mut W {
        self.variant(EE7F_A::DIV1_N2)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=4"]
    #[inline(always)]
    pub fn div1_n4(self) -> &'a mut W {
        self.variant(EE7F_A::DIV1_N4)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=8"]
    #[inline(always)]
    pub fn div1_n8(self) -> &'a mut W {
        self.variant(EE7F_A::DIV1_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=6"]
    #[inline(always)]
    pub fn div2_n6(self) -> &'a mut W {
        self.variant(EE7F_A::DIV2_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=8"]
    #[inline(always)]
    pub fn div2_n8(self) -> &'a mut W {
        self.variant(EE7F_A::DIV2_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=6"]
    #[inline(always)]
    pub fn div4_n6(self) -> &'a mut W {
        self.variant(EE7F_A::DIV4_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=8"]
    #[inline(always)]
    pub fn div4_n8(self) -> &'a mut W {
        self.variant(EE7F_A::DIV4_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=6"]
    #[inline(always)]
    pub fn div8_n6(self) -> &'a mut W {
        self.variant(EE7F_A::DIV8_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=8"]
    #[inline(always)]
    pub fn div8_n8(self) -> &'a mut W {
        self.variant(EE7F_A::DIV8_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=5"]
    #[inline(always)]
    pub fn div16_n5(self) -> &'a mut W {
        self.variant(EE7F_A::DIV16_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=6"]
    #[inline(always)]
    pub fn div16_n6(self) -> &'a mut W {
        self.variant(EE7F_A::DIV16_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=8"]
    #[inline(always)]
    pub fn div16_n8(self) -> &'a mut W {
        self.variant(EE7F_A::DIV16_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=5"]
    #[inline(always)]
    pub fn div32_n5(self) -> &'a mut W {
        self.variant(EE7F_A::DIV32_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=6"]
    #[inline(always)]
    pub fn div32_n6(self) -> &'a mut W {
        self.variant(EE7F_A::DIV32_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=8"]
    #[inline(always)]
    pub fn div32_n8(self) -> &'a mut W {
        self.variant(EE7F_A::DIV32_N8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | ((value as u32 & 0x0f) << 6);
        self.w
    }
}
#[doc = "EE6F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EE6F_A {
    #[doc = "0: Filter disabled"]
    DISABLED = 0,
    #[doc = "1: f_SAMPLING=f_HRTIM, N=2"]
    DIV1_N2 = 1,
    #[doc = "2: f_SAMPLING=f_HRTIM, N=4"]
    DIV1_N4 = 2,
    #[doc = "3: f_SAMPLING=f_HRTIM, N=8"]
    DIV1_N8 = 3,
    #[doc = "4: f_SAMPLING=f_HRTIM/2, N=6"]
    DIV2_N6 = 4,
    #[doc = "5: f_SAMPLING=f_HRTIM/2, N=8"]
    DIV2_N8 = 5,
    #[doc = "6: f_SAMPLING=f_HRTIM/4, N=6"]
    DIV4_N6 = 6,
    #[doc = "7: f_SAMPLING=f_HRTIM/4, N=8"]
    DIV4_N8 = 7,
    #[doc = "8: f_SAMPLING=f_HRTIM/8, N=6"]
    DIV8_N6 = 8,
    #[doc = "9: f_SAMPLING=f_HRTIM/8, N=8"]
    DIV8_N8 = 9,
    #[doc = "10: f_SAMPLING=f_HRTIM/16, N=5"]
    DIV16_N5 = 10,
    #[doc = "11: f_SAMPLING=f_HRTIM/16, N=6"]
    DIV16_N6 = 11,
    #[doc = "12: f_SAMPLING=f_HRTIM/16, N=8"]
    DIV16_N8 = 12,
    #[doc = "13: f_SAMPLING=f_HRTIM/32, N=5"]
    DIV32_N5 = 13,
    #[doc = "14: f_SAMPLING=f_HRTIM/32, N=6"]
    DIV32_N6 = 14,
    #[doc = "15: f_SAMPLING=f_HRTIM/32, N=8"]
    DIV32_N8 = 15,
}
impl From<EE6F_A> for u8 {
    #[inline(always)]
    fn from(variant: EE6F_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EE6F` reader - EE6F"]
pub struct EE6F_R(crate::FieldReader<u8, EE6F_A>);
impl EE6F_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE6F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EE6F_A {
        match self.bits {
            0 => EE6F_A::DISABLED,
            1 => EE6F_A::DIV1_N2,
            2 => EE6F_A::DIV1_N4,
            3 => EE6F_A::DIV1_N8,
            4 => EE6F_A::DIV2_N6,
            5 => EE6F_A::DIV2_N8,
            6 => EE6F_A::DIV4_N6,
            7 => EE6F_A::DIV4_N8,
            8 => EE6F_A::DIV8_N6,
            9 => EE6F_A::DIV8_N8,
            10 => EE6F_A::DIV16_N5,
            11 => EE6F_A::DIV16_N6,
            12 => EE6F_A::DIV16_N8,
            13 => EE6F_A::DIV32_N5,
            14 => EE6F_A::DIV32_N6,
            15 => EE6F_A::DIV32_N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EE6F_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `DIV1_N2`"]
    #[inline(always)]
    pub fn is_div1_n2(&self) -> bool {
        **self == EE6F_A::DIV1_N2
    }
    #[doc = "Checks if the value of the field is `DIV1_N4`"]
    #[inline(always)]
    pub fn is_div1_n4(&self) -> bool {
        **self == EE6F_A::DIV1_N4
    }
    #[doc = "Checks if the value of the field is `DIV1_N8`"]
    #[inline(always)]
    pub fn is_div1_n8(&self) -> bool {
        **self == EE6F_A::DIV1_N8
    }
    #[doc = "Checks if the value of the field is `DIV2_N6`"]
    #[inline(always)]
    pub fn is_div2_n6(&self) -> bool {
        **self == EE6F_A::DIV2_N6
    }
    #[doc = "Checks if the value of the field is `DIV2_N8`"]
    #[inline(always)]
    pub fn is_div2_n8(&self) -> bool {
        **self == EE6F_A::DIV2_N8
    }
    #[doc = "Checks if the value of the field is `DIV4_N6`"]
    #[inline(always)]
    pub fn is_div4_n6(&self) -> bool {
        **self == EE6F_A::DIV4_N6
    }
    #[doc = "Checks if the value of the field is `DIV4_N8`"]
    #[inline(always)]
    pub fn is_div4_n8(&self) -> bool {
        **self == EE6F_A::DIV4_N8
    }
    #[doc = "Checks if the value of the field is `DIV8_N6`"]
    #[inline(always)]
    pub fn is_div8_n6(&self) -> bool {
        **self == EE6F_A::DIV8_N6
    }
    #[doc = "Checks if the value of the field is `DIV8_N8`"]
    #[inline(always)]
    pub fn is_div8_n8(&self) -> bool {
        **self == EE6F_A::DIV8_N8
    }
    #[doc = "Checks if the value of the field is `DIV16_N5`"]
    #[inline(always)]
    pub fn is_div16_n5(&self) -> bool {
        **self == EE6F_A::DIV16_N5
    }
    #[doc = "Checks if the value of the field is `DIV16_N6`"]
    #[inline(always)]
    pub fn is_div16_n6(&self) -> bool {
        **self == EE6F_A::DIV16_N6
    }
    #[doc = "Checks if the value of the field is `DIV16_N8`"]
    #[inline(always)]
    pub fn is_div16_n8(&self) -> bool {
        **self == EE6F_A::DIV16_N8
    }
    #[doc = "Checks if the value of the field is `DIV32_N5`"]
    #[inline(always)]
    pub fn is_div32_n5(&self) -> bool {
        **self == EE6F_A::DIV32_N5
    }
    #[doc = "Checks if the value of the field is `DIV32_N6`"]
    #[inline(always)]
    pub fn is_div32_n6(&self) -> bool {
        **self == EE6F_A::DIV32_N6
    }
    #[doc = "Checks if the value of the field is `DIV32_N8`"]
    #[inline(always)]
    pub fn is_div32_n8(&self) -> bool {
        **self == EE6F_A::DIV32_N8
    }
}
impl core::ops::Deref for EE6F_R {
    type Target = crate::FieldReader<u8, EE6F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE6F` writer - EE6F"]
pub struct EE6F_W<'a> {
    w: &'a mut W,
}
impl<'a> EE6F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE6F_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE6F_A::DISABLED)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=2"]
    #[inline(always)]
    pub fn div1_n2(self) -> &'a mut W {
        self.variant(EE6F_A::DIV1_N2)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=4"]
    #[inline(always)]
    pub fn div1_n4(self) -> &'a mut W {
        self.variant(EE6F_A::DIV1_N4)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=8"]
    #[inline(always)]
    pub fn div1_n8(self) -> &'a mut W {
        self.variant(EE6F_A::DIV1_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=6"]
    #[inline(always)]
    pub fn div2_n6(self) -> &'a mut W {
        self.variant(EE6F_A::DIV2_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=8"]
    #[inline(always)]
    pub fn div2_n8(self) -> &'a mut W {
        self.variant(EE6F_A::DIV2_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=6"]
    #[inline(always)]
    pub fn div4_n6(self) -> &'a mut W {
        self.variant(EE6F_A::DIV4_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=8"]
    #[inline(always)]
    pub fn div4_n8(self) -> &'a mut W {
        self.variant(EE6F_A::DIV4_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=6"]
    #[inline(always)]
    pub fn div8_n6(self) -> &'a mut W {
        self.variant(EE6F_A::DIV8_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=8"]
    #[inline(always)]
    pub fn div8_n8(self) -> &'a mut W {
        self.variant(EE6F_A::DIV8_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=5"]
    #[inline(always)]
    pub fn div16_n5(self) -> &'a mut W {
        self.variant(EE6F_A::DIV16_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=6"]
    #[inline(always)]
    pub fn div16_n6(self) -> &'a mut W {
        self.variant(EE6F_A::DIV16_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=8"]
    #[inline(always)]
    pub fn div16_n8(self) -> &'a mut W {
        self.variant(EE6F_A::DIV16_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=5"]
    #[inline(always)]
    pub fn div32_n5(self) -> &'a mut W {
        self.variant(EE6F_A::DIV32_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=6"]
    #[inline(always)]
    pub fn div32_n6(self) -> &'a mut W {
        self.variant(EE6F_A::DIV32_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=8"]
    #[inline(always)]
    pub fn div32_n8(self) -> &'a mut W {
        self.variant(EE6F_A::DIV32_N8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - EEVSD"]
    #[inline(always)]
    pub fn eevsd(&self) -> EEVSD_R {
        EEVSD_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - EE10F"]
    #[inline(always)]
    pub fn ee10f(&self) -> EE10F_R {
        EE10F_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - EE9F"]
    #[inline(always)]
    pub fn ee9f(&self) -> EE9F_R {
        EE9F_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EE8F"]
    #[inline(always)]
    pub fn ee8f(&self) -> EE8F_R {
        EE8F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - EE7F"]
    #[inline(always)]
    pub fn ee7f(&self) -> EE7F_R {
        EE7F_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EE6F"]
    #[inline(always)]
    pub fn ee6f(&self) -> EE6F_R {
        EE6F_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - EEVSD"]
    #[inline(always)]
    pub fn eevsd(&mut self) -> EEVSD_W {
        EEVSD_W { w: self }
    }
    #[doc = "Bits 24:27 - EE10F"]
    #[inline(always)]
    pub fn ee10f(&mut self) -> EE10F_W {
        EE10F_W { w: self }
    }
    #[doc = "Bits 18:21 - EE9F"]
    #[inline(always)]
    pub fn ee9f(&mut self) -> EE9F_W {
        EE9F_W { w: self }
    }
    #[doc = "Bits 12:15 - EE8F"]
    #[inline(always)]
    pub fn ee8f(&mut self) -> EE8F_W {
        EE8F_W { w: self }
    }
    #[doc = "Bits 6:9 - EE7F"]
    #[inline(always)]
    pub fn ee7f(&mut self) -> EE7F_W {
        EE7F_W { w: self }
    }
    #[doc = "Bits 0:3 - EE6F"]
    #[inline(always)]
    pub fn ee6f(&mut self) -> EE6F_W {
        EE6F_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer External Event Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eecr3](index.html) module"]
pub struct EECR3_SPEC;
impl crate::RegisterSpec for EECR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eecr3::R](R) reader structure"]
impl crate::Readable for EECR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eecr3::W](W) writer structure"]
impl crate::Writable for EECR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EECR3 to value 0"]
impl crate::Resettable for EECR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
