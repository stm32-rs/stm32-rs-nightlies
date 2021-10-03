#[doc = "Register `FLTINR1` reader"]
pub struct R(crate::R<FLTINR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTINR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTINR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTINR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTINR1` writer"]
pub struct W(crate::W<FLTINR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTINR1_SPEC>;
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
impl From<crate::W<FLTINR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTINR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FLT4LCK"]
pub type FLT4LCK_A = FLT1LCK_A;
#[doc = "Field `FLT4LCK` reader - FLT4LCK"]
pub type FLT4LCK_R = FLT1LCK_R;
#[doc = "FLT4LCK"]
pub type FLT4LCK_AW = FLT1LCK_AW;
#[doc = "Field `FLT4LCK` writer - FLT4LCK"]
pub struct FLT4LCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4LCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT4LCK_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Lock corresponding fault bits"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(FLT4LCK_AW::LOCK)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "FLT4F"]
pub type FLT4F_A = FLT1F_A;
#[doc = "Field `FLT4F` reader - FLT4F"]
pub type FLT4F_R = FLT1F_R;
#[doc = "Field `FLT4F` writer - FLT4F"]
pub struct FLT4F_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT4F_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No filter, FLTx acts asynchronously"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT4F_A::DISABLED)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=2"]
    #[inline(always)]
    pub fn div1_n2(self) -> &'a mut W {
        self.variant(FLT4F_A::DIV1_N2)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=4"]
    #[inline(always)]
    pub fn div1_n4(self) -> &'a mut W {
        self.variant(FLT4F_A::DIV1_N4)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=8"]
    #[inline(always)]
    pub fn div1_n8(self) -> &'a mut W {
        self.variant(FLT4F_A::DIV1_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=6"]
    #[inline(always)]
    pub fn div2_n6(self) -> &'a mut W {
        self.variant(FLT4F_A::DIV2_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=8"]
    #[inline(always)]
    pub fn div2_n8(self) -> &'a mut W {
        self.variant(FLT4F_A::DIV2_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=6"]
    #[inline(always)]
    pub fn div4_n6(self) -> &'a mut W {
        self.variant(FLT4F_A::DIV4_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=8"]
    #[inline(always)]
    pub fn div4_n8(self) -> &'a mut W {
        self.variant(FLT4F_A::DIV4_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=6"]
    #[inline(always)]
    pub fn div8_n6(self) -> &'a mut W {
        self.variant(FLT4F_A::DIV8_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=8"]
    #[inline(always)]
    pub fn div8_n8(self) -> &'a mut W {
        self.variant(FLT4F_A::DIV8_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=5"]
    #[inline(always)]
    pub fn div16_n5(self) -> &'a mut W {
        self.variant(FLT4F_A::DIV16_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=6"]
    #[inline(always)]
    pub fn div16_n6(self) -> &'a mut W {
        self.variant(FLT4F_A::DIV16_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=8"]
    #[inline(always)]
    pub fn div16_n8(self) -> &'a mut W {
        self.variant(FLT4F_A::DIV16_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=5"]
    #[inline(always)]
    pub fn div32_n5(self) -> &'a mut W {
        self.variant(FLT4F_A::DIV32_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=6"]
    #[inline(always)]
    pub fn div32_n6(self) -> &'a mut W {
        self.variant(FLT4F_A::DIV32_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=8"]
    #[inline(always)]
    pub fn div32_n8(self) -> &'a mut W {
        self.variant(FLT4F_A::DIV32_N8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 27)) | ((value as u32 & 0x0f) << 27);
        self.w
    }
}
#[doc = "FLT4SRC"]
pub type FLT4SRC_A = FLT1SRC_A;
#[doc = "Field `FLT4SRC` reader - FLT4SRC"]
pub type FLT4SRC_R = FLT1SRC_R;
#[doc = "Field `FLT4SRC` writer - FLT4SRC"]
pub struct FLT4SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT4SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input is FLTx input pin"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FLT4SRC_A::INPUT)
    }
    #[doc = "Fault input is FLTn_Int signal"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(FLT4SRC_A::INTERNAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "FLT4P"]
pub type FLT4P_A = FLT1P_A;
#[doc = "Field `FLT4P` reader - FLT4P"]
pub type FLT4P_R = FLT1P_R;
#[doc = "Field `FLT4P` writer - FLT4P"]
pub struct FLT4P_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT4P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(FLT4P_A::ACTIVELOW)
    }
    #[doc = "Fault input is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(FLT4P_A::ACTIVEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "FLT4E"]
pub type FLT4E_A = FLT1E_A;
#[doc = "Field `FLT4E` reader - FLT4E"]
pub type FLT4E_R = FLT1E_R;
#[doc = "Field `FLT4E` writer - FLT4E"]
pub struct FLT4E_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT4E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT4E_A::DISABLED)
    }
    #[doc = "Fault input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLT4E_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "FLT3LCK"]
pub type FLT3LCK_A = FLT1LCK_A;
#[doc = "Field `FLT3LCK` reader - FLT3LCK"]
pub type FLT3LCK_R = FLT1LCK_R;
#[doc = "FLT3LCK"]
pub type FLT3LCK_AW = FLT1LCK_AW;
#[doc = "Field `FLT3LCK` writer - FLT3LCK"]
pub struct FLT3LCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3LCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT3LCK_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Lock corresponding fault bits"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(FLT3LCK_AW::LOCK)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "FLT3F"]
pub type FLT3F_A = FLT1F_A;
#[doc = "Field `FLT3F` reader - FLT3F"]
pub type FLT3F_R = FLT1F_R;
#[doc = "Field `FLT3F` writer - FLT3F"]
pub struct FLT3F_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT3F_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No filter, FLTx acts asynchronously"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT3F_A::DISABLED)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=2"]
    #[inline(always)]
    pub fn div1_n2(self) -> &'a mut W {
        self.variant(FLT3F_A::DIV1_N2)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=4"]
    #[inline(always)]
    pub fn div1_n4(self) -> &'a mut W {
        self.variant(FLT3F_A::DIV1_N4)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=8"]
    #[inline(always)]
    pub fn div1_n8(self) -> &'a mut W {
        self.variant(FLT3F_A::DIV1_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=6"]
    #[inline(always)]
    pub fn div2_n6(self) -> &'a mut W {
        self.variant(FLT3F_A::DIV2_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=8"]
    #[inline(always)]
    pub fn div2_n8(self) -> &'a mut W {
        self.variant(FLT3F_A::DIV2_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=6"]
    #[inline(always)]
    pub fn div4_n6(self) -> &'a mut W {
        self.variant(FLT3F_A::DIV4_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=8"]
    #[inline(always)]
    pub fn div4_n8(self) -> &'a mut W {
        self.variant(FLT3F_A::DIV4_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=6"]
    #[inline(always)]
    pub fn div8_n6(self) -> &'a mut W {
        self.variant(FLT3F_A::DIV8_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=8"]
    #[inline(always)]
    pub fn div8_n8(self) -> &'a mut W {
        self.variant(FLT3F_A::DIV8_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=5"]
    #[inline(always)]
    pub fn div16_n5(self) -> &'a mut W {
        self.variant(FLT3F_A::DIV16_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=6"]
    #[inline(always)]
    pub fn div16_n6(self) -> &'a mut W {
        self.variant(FLT3F_A::DIV16_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=8"]
    #[inline(always)]
    pub fn div16_n8(self) -> &'a mut W {
        self.variant(FLT3F_A::DIV16_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=5"]
    #[inline(always)]
    pub fn div32_n5(self) -> &'a mut W {
        self.variant(FLT3F_A::DIV32_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=6"]
    #[inline(always)]
    pub fn div32_n6(self) -> &'a mut W {
        self.variant(FLT3F_A::DIV32_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=8"]
    #[inline(always)]
    pub fn div32_n8(self) -> &'a mut W {
        self.variant(FLT3F_A::DIV32_N8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 19)) | ((value as u32 & 0x0f) << 19);
        self.w
    }
}
#[doc = "FLT3SRC"]
pub type FLT3SRC_A = FLT1SRC_A;
#[doc = "Field `FLT3SRC` reader - FLT3SRC"]
pub type FLT3SRC_R = FLT1SRC_R;
#[doc = "Field `FLT3SRC` writer - FLT3SRC"]
pub struct FLT3SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT3SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input is FLTx input pin"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FLT3SRC_A::INPUT)
    }
    #[doc = "Fault input is FLTn_Int signal"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(FLT3SRC_A::INTERNAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "FLT3P"]
pub type FLT3P_A = FLT1P_A;
#[doc = "Field `FLT3P` reader - FLT3P"]
pub type FLT3P_R = FLT1P_R;
#[doc = "Field `FLT3P` writer - FLT3P"]
pub struct FLT3P_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT3P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(FLT3P_A::ACTIVELOW)
    }
    #[doc = "Fault input is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(FLT3P_A::ACTIVEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "FLT3E"]
pub type FLT3E_A = FLT1E_A;
#[doc = "Field `FLT3E` reader - FLT3E"]
pub type FLT3E_R = FLT1E_R;
#[doc = "Field `FLT3E` writer - FLT3E"]
pub struct FLT3E_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT3E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT3E_A::DISABLED)
    }
    #[doc = "Fault input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLT3E_A::ENABLED)
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
#[doc = "FLT2LCK"]
pub type FLT2LCK_A = FLT1LCK_A;
#[doc = "Field `FLT2LCK` reader - FLT2LCK"]
pub type FLT2LCK_R = FLT1LCK_R;
#[doc = "FLT2LCK"]
pub type FLT2LCK_AW = FLT1LCK_AW;
#[doc = "Field `FLT2LCK` writer - FLT2LCK"]
pub struct FLT2LCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2LCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT2LCK_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Lock corresponding fault bits"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(FLT2LCK_AW::LOCK)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "FLT2F"]
pub type FLT2F_A = FLT1F_A;
#[doc = "Field `FLT2F` reader - FLT2F"]
pub type FLT2F_R = FLT1F_R;
#[doc = "Field `FLT2F` writer - FLT2F"]
pub struct FLT2F_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT2F_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No filter, FLTx acts asynchronously"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT2F_A::DISABLED)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=2"]
    #[inline(always)]
    pub fn div1_n2(self) -> &'a mut W {
        self.variant(FLT2F_A::DIV1_N2)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=4"]
    #[inline(always)]
    pub fn div1_n4(self) -> &'a mut W {
        self.variant(FLT2F_A::DIV1_N4)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=8"]
    #[inline(always)]
    pub fn div1_n8(self) -> &'a mut W {
        self.variant(FLT2F_A::DIV1_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=6"]
    #[inline(always)]
    pub fn div2_n6(self) -> &'a mut W {
        self.variant(FLT2F_A::DIV2_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=8"]
    #[inline(always)]
    pub fn div2_n8(self) -> &'a mut W {
        self.variant(FLT2F_A::DIV2_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=6"]
    #[inline(always)]
    pub fn div4_n6(self) -> &'a mut W {
        self.variant(FLT2F_A::DIV4_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=8"]
    #[inline(always)]
    pub fn div4_n8(self) -> &'a mut W {
        self.variant(FLT2F_A::DIV4_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=6"]
    #[inline(always)]
    pub fn div8_n6(self) -> &'a mut W {
        self.variant(FLT2F_A::DIV8_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=8"]
    #[inline(always)]
    pub fn div8_n8(self) -> &'a mut W {
        self.variant(FLT2F_A::DIV8_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=5"]
    #[inline(always)]
    pub fn div16_n5(self) -> &'a mut W {
        self.variant(FLT2F_A::DIV16_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=6"]
    #[inline(always)]
    pub fn div16_n6(self) -> &'a mut W {
        self.variant(FLT2F_A::DIV16_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=8"]
    #[inline(always)]
    pub fn div16_n8(self) -> &'a mut W {
        self.variant(FLT2F_A::DIV16_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=5"]
    #[inline(always)]
    pub fn div32_n5(self) -> &'a mut W {
        self.variant(FLT2F_A::DIV32_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=6"]
    #[inline(always)]
    pub fn div32_n6(self) -> &'a mut W {
        self.variant(FLT2F_A::DIV32_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=8"]
    #[inline(always)]
    pub fn div32_n8(self) -> &'a mut W {
        self.variant(FLT2F_A::DIV32_N8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | ((value as u32 & 0x0f) << 11);
        self.w
    }
}
#[doc = "FLT2SRC"]
pub type FLT2SRC_A = FLT1SRC_A;
#[doc = "Field `FLT2SRC` reader - FLT2SRC"]
pub type FLT2SRC_R = FLT1SRC_R;
#[doc = "Field `FLT2SRC` writer - FLT2SRC"]
pub struct FLT2SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT2SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input is FLTx input pin"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FLT2SRC_A::INPUT)
    }
    #[doc = "Fault input is FLTn_Int signal"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(FLT2SRC_A::INTERNAL)
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
#[doc = "FLT2P"]
pub type FLT2P_A = FLT1P_A;
#[doc = "Field `FLT2P` reader - FLT2P"]
pub type FLT2P_R = FLT1P_R;
#[doc = "Field `FLT2P` writer - FLT2P"]
pub struct FLT2P_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT2P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(FLT2P_A::ACTIVELOW)
    }
    #[doc = "Fault input is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(FLT2P_A::ACTIVEHIGH)
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
#[doc = "FLT2E"]
pub type FLT2E_A = FLT1E_A;
#[doc = "Field `FLT2E` reader - FLT2E"]
pub type FLT2E_R = FLT1E_R;
#[doc = "Field `FLT2E` writer - FLT2E"]
pub struct FLT2E_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT2E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT2E_A::DISABLED)
    }
    #[doc = "Fault input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLT2E_A::ENABLED)
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
#[doc = "FLT1LCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT1LCK_A {
    #[doc = "0: Fault bits are read/write"]
    UNLOCKED = 0,
    #[doc = "1: Fault bits are read-only"]
    LOCKED = 1,
}
impl From<FLT1LCK_A> for bool {
    #[inline(always)]
    fn from(variant: FLT1LCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1LCK` reader - FLT1LCK"]
pub struct FLT1LCK_R(crate::FieldReader<bool, FLT1LCK_A>);
impl FLT1LCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT1LCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT1LCK_A {
        match self.bits {
            false => FLT1LCK_A::UNLOCKED,
            true => FLT1LCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == FLT1LCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == FLT1LCK_A::LOCKED
    }
}
impl core::ops::Deref for FLT1LCK_R {
    type Target = crate::FieldReader<bool, FLT1LCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "FLT1LCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT1LCK_AW {
    #[doc = "1: Lock corresponding fault bits"]
    LOCK = 1,
}
impl From<FLT1LCK_AW> for bool {
    #[inline(always)]
    fn from(variant: FLT1LCK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1LCK` writer - FLT1LCK"]
pub struct FLT1LCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1LCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT1LCK_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Lock corresponding fault bits"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(FLT1LCK_AW::LOCK)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "FLT1F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLT1F_A {
    #[doc = "0: No filter, FLTx acts asynchronously"]
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
impl From<FLT1F_A> for u8 {
    #[inline(always)]
    fn from(variant: FLT1F_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLT1F` reader - FLT1F"]
pub struct FLT1F_R(crate::FieldReader<u8, FLT1F_A>);
impl FLT1F_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLT1F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT1F_A {
        match self.bits {
            0 => FLT1F_A::DISABLED,
            1 => FLT1F_A::DIV1_N2,
            2 => FLT1F_A::DIV1_N4,
            3 => FLT1F_A::DIV1_N8,
            4 => FLT1F_A::DIV2_N6,
            5 => FLT1F_A::DIV2_N8,
            6 => FLT1F_A::DIV4_N6,
            7 => FLT1F_A::DIV4_N8,
            8 => FLT1F_A::DIV8_N6,
            9 => FLT1F_A::DIV8_N8,
            10 => FLT1F_A::DIV16_N5,
            11 => FLT1F_A::DIV16_N6,
            12 => FLT1F_A::DIV16_N8,
            13 => FLT1F_A::DIV32_N5,
            14 => FLT1F_A::DIV32_N6,
            15 => FLT1F_A::DIV32_N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FLT1F_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `DIV1_N2`"]
    #[inline(always)]
    pub fn is_div1_n2(&self) -> bool {
        **self == FLT1F_A::DIV1_N2
    }
    #[doc = "Checks if the value of the field is `DIV1_N4`"]
    #[inline(always)]
    pub fn is_div1_n4(&self) -> bool {
        **self == FLT1F_A::DIV1_N4
    }
    #[doc = "Checks if the value of the field is `DIV1_N8`"]
    #[inline(always)]
    pub fn is_div1_n8(&self) -> bool {
        **self == FLT1F_A::DIV1_N8
    }
    #[doc = "Checks if the value of the field is `DIV2_N6`"]
    #[inline(always)]
    pub fn is_div2_n6(&self) -> bool {
        **self == FLT1F_A::DIV2_N6
    }
    #[doc = "Checks if the value of the field is `DIV2_N8`"]
    #[inline(always)]
    pub fn is_div2_n8(&self) -> bool {
        **self == FLT1F_A::DIV2_N8
    }
    #[doc = "Checks if the value of the field is `DIV4_N6`"]
    #[inline(always)]
    pub fn is_div4_n6(&self) -> bool {
        **self == FLT1F_A::DIV4_N6
    }
    #[doc = "Checks if the value of the field is `DIV4_N8`"]
    #[inline(always)]
    pub fn is_div4_n8(&self) -> bool {
        **self == FLT1F_A::DIV4_N8
    }
    #[doc = "Checks if the value of the field is `DIV8_N6`"]
    #[inline(always)]
    pub fn is_div8_n6(&self) -> bool {
        **self == FLT1F_A::DIV8_N6
    }
    #[doc = "Checks if the value of the field is `DIV8_N8`"]
    #[inline(always)]
    pub fn is_div8_n8(&self) -> bool {
        **self == FLT1F_A::DIV8_N8
    }
    #[doc = "Checks if the value of the field is `DIV16_N5`"]
    #[inline(always)]
    pub fn is_div16_n5(&self) -> bool {
        **self == FLT1F_A::DIV16_N5
    }
    #[doc = "Checks if the value of the field is `DIV16_N6`"]
    #[inline(always)]
    pub fn is_div16_n6(&self) -> bool {
        **self == FLT1F_A::DIV16_N6
    }
    #[doc = "Checks if the value of the field is `DIV16_N8`"]
    #[inline(always)]
    pub fn is_div16_n8(&self) -> bool {
        **self == FLT1F_A::DIV16_N8
    }
    #[doc = "Checks if the value of the field is `DIV32_N5`"]
    #[inline(always)]
    pub fn is_div32_n5(&self) -> bool {
        **self == FLT1F_A::DIV32_N5
    }
    #[doc = "Checks if the value of the field is `DIV32_N6`"]
    #[inline(always)]
    pub fn is_div32_n6(&self) -> bool {
        **self == FLT1F_A::DIV32_N6
    }
    #[doc = "Checks if the value of the field is `DIV32_N8`"]
    #[inline(always)]
    pub fn is_div32_n8(&self) -> bool {
        **self == FLT1F_A::DIV32_N8
    }
}
impl core::ops::Deref for FLT1F_R {
    type Target = crate::FieldReader<u8, FLT1F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1F` writer - FLT1F"]
pub struct FLT1F_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT1F_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No filter, FLTx acts asynchronously"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT1F_A::DISABLED)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=2"]
    #[inline(always)]
    pub fn div1_n2(self) -> &'a mut W {
        self.variant(FLT1F_A::DIV1_N2)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=4"]
    #[inline(always)]
    pub fn div1_n4(self) -> &'a mut W {
        self.variant(FLT1F_A::DIV1_N4)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=8"]
    #[inline(always)]
    pub fn div1_n8(self) -> &'a mut W {
        self.variant(FLT1F_A::DIV1_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=6"]
    #[inline(always)]
    pub fn div2_n6(self) -> &'a mut W {
        self.variant(FLT1F_A::DIV2_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=8"]
    #[inline(always)]
    pub fn div2_n8(self) -> &'a mut W {
        self.variant(FLT1F_A::DIV2_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=6"]
    #[inline(always)]
    pub fn div4_n6(self) -> &'a mut W {
        self.variant(FLT1F_A::DIV4_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=8"]
    #[inline(always)]
    pub fn div4_n8(self) -> &'a mut W {
        self.variant(FLT1F_A::DIV4_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=6"]
    #[inline(always)]
    pub fn div8_n6(self) -> &'a mut W {
        self.variant(FLT1F_A::DIV8_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=8"]
    #[inline(always)]
    pub fn div8_n8(self) -> &'a mut W {
        self.variant(FLT1F_A::DIV8_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=5"]
    #[inline(always)]
    pub fn div16_n5(self) -> &'a mut W {
        self.variant(FLT1F_A::DIV16_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=6"]
    #[inline(always)]
    pub fn div16_n6(self) -> &'a mut W {
        self.variant(FLT1F_A::DIV16_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=8"]
    #[inline(always)]
    pub fn div16_n8(self) -> &'a mut W {
        self.variant(FLT1F_A::DIV16_N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=5"]
    #[inline(always)]
    pub fn div32_n5(self) -> &'a mut W {
        self.variant(FLT1F_A::DIV32_N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=6"]
    #[inline(always)]
    pub fn div32_n6(self) -> &'a mut W {
        self.variant(FLT1F_A::DIV32_N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=8"]
    #[inline(always)]
    pub fn div32_n8(self) -> &'a mut W {
        self.variant(FLT1F_A::DIV32_N8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | ((value as u32 & 0x0f) << 3);
        self.w
    }
}
#[doc = "FLT1SRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT1SRC_A {
    #[doc = "0: Fault input is FLTx input pin"]
    INPUT = 0,
    #[doc = "1: Fault input is FLTn_Int signal"]
    INTERNAL = 1,
}
impl From<FLT1SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FLT1SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1SRC` reader - FLT1SRC"]
pub struct FLT1SRC_R(crate::FieldReader<bool, FLT1SRC_A>);
impl FLT1SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT1SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT1SRC_A {
        match self.bits {
            false => FLT1SRC_A::INPUT,
            true => FLT1SRC_A::INTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == FLT1SRC_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        **self == FLT1SRC_A::INTERNAL
    }
}
impl core::ops::Deref for FLT1SRC_R {
    type Target = crate::FieldReader<bool, FLT1SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1SRC` writer - FLT1SRC"]
pub struct FLT1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT1SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input is FLTx input pin"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FLT1SRC_A::INPUT)
    }
    #[doc = "Fault input is FLTn_Int signal"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(FLT1SRC_A::INTERNAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "FLT1P\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT1P_A {
    #[doc = "0: Fault input is active low"]
    ACTIVELOW = 0,
    #[doc = "1: Fault input is active high"]
    ACTIVEHIGH = 1,
}
impl From<FLT1P_A> for bool {
    #[inline(always)]
    fn from(variant: FLT1P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1P` reader - FLT1P"]
pub struct FLT1P_R(crate::FieldReader<bool, FLT1P_A>);
impl FLT1P_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT1P_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT1P_A {
        match self.bits {
            false => FLT1P_A::ACTIVELOW,
            true => FLT1P_A::ACTIVEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVELOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        **self == FLT1P_A::ACTIVELOW
    }
    #[doc = "Checks if the value of the field is `ACTIVEHIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        **self == FLT1P_A::ACTIVEHIGH
    }
}
impl core::ops::Deref for FLT1P_R {
    type Target = crate::FieldReader<bool, FLT1P_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1P` writer - FLT1P"]
pub struct FLT1P_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT1P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(FLT1P_A::ACTIVELOW)
    }
    #[doc = "Fault input is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(FLT1P_A::ACTIVEHIGH)
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
#[doc = "FLT1E\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT1E_A {
    #[doc = "0: Fault input disabled"]
    DISABLED = 0,
    #[doc = "1: Fault input enabled"]
    ENABLED = 1,
}
impl From<FLT1E_A> for bool {
    #[inline(always)]
    fn from(variant: FLT1E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1E` reader - FLT1E"]
pub struct FLT1E_R(crate::FieldReader<bool, FLT1E_A>);
impl FLT1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT1E_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT1E_A {
        match self.bits {
            false => FLT1E_A::DISABLED,
            true => FLT1E_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FLT1E_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FLT1E_A::ENABLED
    }
}
impl core::ops::Deref for FLT1E_R {
    type Target = crate::FieldReader<bool, FLT1E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1E` writer - FLT1E"]
pub struct FLT1E_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT1E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT1E_A::DISABLED)
    }
    #[doc = "Fault input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLT1E_A::ENABLED)
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
    #[doc = "Bit 31 - FLT4LCK"]
    #[inline(always)]
    pub fn flt4lck(&self) -> FLT4LCK_R {
        FLT4LCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 27:30 - FLT4F"]
    #[inline(always)]
    pub fn flt4f(&self) -> FLT4F_R {
        FLT4F_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - FLT4SRC"]
    #[inline(always)]
    pub fn flt4src(&self) -> FLT4SRC_R {
        FLT4SRC_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - FLT4P"]
    #[inline(always)]
    pub fn flt4p(&self) -> FLT4P_R {
        FLT4P_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - FLT4E"]
    #[inline(always)]
    pub fn flt4e(&self) -> FLT4E_R {
        FLT4E_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - FLT3LCK"]
    #[inline(always)]
    pub fn flt3lck(&self) -> FLT3LCK_R {
        FLT3LCK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 19:22 - FLT3F"]
    #[inline(always)]
    pub fn flt3f(&self) -> FLT3F_R {
        FLT3F_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - FLT3SRC"]
    #[inline(always)]
    pub fn flt3src(&self) -> FLT3SRC_R {
        FLT3SRC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FLT3P"]
    #[inline(always)]
    pub fn flt3p(&self) -> FLT3P_R {
        FLT3P_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FLT3E"]
    #[inline(always)]
    pub fn flt3e(&self) -> FLT3E_R {
        FLT3E_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - FLT2LCK"]
    #[inline(always)]
    pub fn flt2lck(&self) -> FLT2LCK_R {
        FLT2LCK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 11:14 - FLT2F"]
    #[inline(always)]
    pub fn flt2f(&self) -> FLT2F_R {
        FLT2F_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - FLT2SRC"]
    #[inline(always)]
    pub fn flt2src(&self) -> FLT2SRC_R {
        FLT2SRC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FLT2P"]
    #[inline(always)]
    pub fn flt2p(&self) -> FLT2P_R {
        FLT2P_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FLT2E"]
    #[inline(always)]
    pub fn flt2e(&self) -> FLT2E_R {
        FLT2E_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FLT1LCK"]
    #[inline(always)]
    pub fn flt1lck(&self) -> FLT1LCK_R {
        FLT1LCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - FLT1F"]
    #[inline(always)]
    pub fn flt1f(&self) -> FLT1F_R {
        FLT1F_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 2 - FLT1SRC"]
    #[inline(always)]
    pub fn flt1src(&self) -> FLT1SRC_R {
        FLT1SRC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - FLT1P"]
    #[inline(always)]
    pub fn flt1p(&self) -> FLT1P_R {
        FLT1P_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - FLT1E"]
    #[inline(always)]
    pub fn flt1e(&self) -> FLT1E_R {
        FLT1E_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - FLT4LCK"]
    #[inline(always)]
    pub fn flt4lck(&mut self) -> FLT4LCK_W {
        FLT4LCK_W { w: self }
    }
    #[doc = "Bits 27:30 - FLT4F"]
    #[inline(always)]
    pub fn flt4f(&mut self) -> FLT4F_W {
        FLT4F_W { w: self }
    }
    #[doc = "Bit 26 - FLT4SRC"]
    #[inline(always)]
    pub fn flt4src(&mut self) -> FLT4SRC_W {
        FLT4SRC_W { w: self }
    }
    #[doc = "Bit 25 - FLT4P"]
    #[inline(always)]
    pub fn flt4p(&mut self) -> FLT4P_W {
        FLT4P_W { w: self }
    }
    #[doc = "Bit 24 - FLT4E"]
    #[inline(always)]
    pub fn flt4e(&mut self) -> FLT4E_W {
        FLT4E_W { w: self }
    }
    #[doc = "Bit 23 - FLT3LCK"]
    #[inline(always)]
    pub fn flt3lck(&mut self) -> FLT3LCK_W {
        FLT3LCK_W { w: self }
    }
    #[doc = "Bits 19:22 - FLT3F"]
    #[inline(always)]
    pub fn flt3f(&mut self) -> FLT3F_W {
        FLT3F_W { w: self }
    }
    #[doc = "Bit 18 - FLT3SRC"]
    #[inline(always)]
    pub fn flt3src(&mut self) -> FLT3SRC_W {
        FLT3SRC_W { w: self }
    }
    #[doc = "Bit 17 - FLT3P"]
    #[inline(always)]
    pub fn flt3p(&mut self) -> FLT3P_W {
        FLT3P_W { w: self }
    }
    #[doc = "Bit 16 - FLT3E"]
    #[inline(always)]
    pub fn flt3e(&mut self) -> FLT3E_W {
        FLT3E_W { w: self }
    }
    #[doc = "Bit 15 - FLT2LCK"]
    #[inline(always)]
    pub fn flt2lck(&mut self) -> FLT2LCK_W {
        FLT2LCK_W { w: self }
    }
    #[doc = "Bits 11:14 - FLT2F"]
    #[inline(always)]
    pub fn flt2f(&mut self) -> FLT2F_W {
        FLT2F_W { w: self }
    }
    #[doc = "Bit 10 - FLT2SRC"]
    #[inline(always)]
    pub fn flt2src(&mut self) -> FLT2SRC_W {
        FLT2SRC_W { w: self }
    }
    #[doc = "Bit 9 - FLT2P"]
    #[inline(always)]
    pub fn flt2p(&mut self) -> FLT2P_W {
        FLT2P_W { w: self }
    }
    #[doc = "Bit 8 - FLT2E"]
    #[inline(always)]
    pub fn flt2e(&mut self) -> FLT2E_W {
        FLT2E_W { w: self }
    }
    #[doc = "Bit 7 - FLT1LCK"]
    #[inline(always)]
    pub fn flt1lck(&mut self) -> FLT1LCK_W {
        FLT1LCK_W { w: self }
    }
    #[doc = "Bits 3:6 - FLT1F"]
    #[inline(always)]
    pub fn flt1f(&mut self) -> FLT1F_W {
        FLT1F_W { w: self }
    }
    #[doc = "Bit 2 - FLT1SRC"]
    #[inline(always)]
    pub fn flt1src(&mut self) -> FLT1SRC_W {
        FLT1SRC_W { w: self }
    }
    #[doc = "Bit 1 - FLT1P"]
    #[inline(always)]
    pub fn flt1p(&mut self) -> FLT1P_W {
        FLT1P_W { w: self }
    }
    #[doc = "Bit 0 - FLT1E"]
    #[inline(always)]
    pub fn flt1e(&mut self) -> FLT1E_W {
        FLT1E_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRTIM Fault Input Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltinr1](index.html) module"]
pub struct FLTINR1_SPEC;
impl crate::RegisterSpec for FLTINR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltinr1::R](R) reader structure"]
impl crate::Readable for FLTINR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltinr1::W](W) writer structure"]
impl crate::Writable for FLTINR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLTINR1 to value 0"]
impl crate::Resettable for FLTINR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
