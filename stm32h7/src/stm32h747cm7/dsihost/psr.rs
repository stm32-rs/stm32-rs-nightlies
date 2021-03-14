#[doc = "Reader of register PSR"]
pub type R = crate::R<u32, super::PSR>;
#[doc = "Writer for register PSR"]
pub type W = crate::W<u32, super::PSR>;
#[doc = "Register PSR `reset()`'s with value 0"]
impl crate::ResetValue for super::PSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PE4`"]
pub type PE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PE4`"]
pub struct PE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PE4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `PE3`"]
pub type PE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PE3`"]
pub struct PE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PE3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `PE2`"]
pub type PE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PE2`"]
pub struct PE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `PE1`"]
pub type PE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PE1`"]
pub struct PE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PE1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `PE0`"]
pub type PE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PE0`"]
pub struct PE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PE0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `AE15`"]
pub type AE15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE15`"]
pub struct AE15_W<'a> {
    w: &'a mut W,
}
impl<'a> AE15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `AE14`"]
pub type AE14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE14`"]
pub struct AE14_W<'a> {
    w: &'a mut W,
}
impl<'a> AE14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `AE13`"]
pub type AE13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE13`"]
pub struct AE13_W<'a> {
    w: &'a mut W,
}
impl<'a> AE13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `AE12`"]
pub type AE12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE12`"]
pub struct AE12_W<'a> {
    w: &'a mut W,
}
impl<'a> AE12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `AE11`"]
pub type AE11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE11`"]
pub struct AE11_W<'a> {
    w: &'a mut W,
}
impl<'a> AE11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `AE10`"]
pub type AE10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE10`"]
pub struct AE10_W<'a> {
    w: &'a mut W,
}
impl<'a> AE10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `AE9`"]
pub type AE9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE9`"]
pub struct AE9_W<'a> {
    w: &'a mut W,
}
impl<'a> AE9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `AE8`"]
pub type AE8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE8`"]
pub struct AE8_W<'a> {
    w: &'a mut W,
}
impl<'a> AE8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `AE7`"]
pub type AE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE7`"]
pub struct AE7_W<'a> {
    w: &'a mut W,
}
impl<'a> AE7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `AE6`"]
pub type AE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE6`"]
pub struct AE6_W<'a> {
    w: &'a mut W,
}
impl<'a> AE6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `AE5`"]
pub type AE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE5`"]
pub struct AE5_W<'a> {
    w: &'a mut W,
}
impl<'a> AE5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `AE4`"]
pub type AE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE4`"]
pub struct AE4_W<'a> {
    w: &'a mut W,
}
impl<'a> AE4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `AE3`"]
pub type AE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE3`"]
pub struct AE3_W<'a> {
    w: &'a mut W,
}
impl<'a> AE3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `AE2`"]
pub type AE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE2`"]
pub struct AE2_W<'a> {
    w: &'a mut W,
}
impl<'a> AE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `AE1`"]
pub type AE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE1`"]
pub struct AE1_W<'a> {
    w: &'a mut W,
}
impl<'a> AE1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `AE0`"]
pub type AE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE0`"]
pub struct AE0_W<'a> {
    w: &'a mut W,
}
impl<'a> AE0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 20 - PHY error 4"]
    #[inline(always)]
    pub fn pe4(&self) -> PE4_R {
        PE4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PHY error 3"]
    #[inline(always)]
    pub fn pe3(&self) -> PE3_R {
        PE3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PHY error 2"]
    #[inline(always)]
    pub fn pe2(&self) -> PE2_R {
        PE2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PHY error 1"]
    #[inline(always)]
    pub fn pe1(&self) -> PE1_R {
        PE1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PHY error 0"]
    #[inline(always)]
    pub fn pe0(&self) -> PE0_R {
        PE0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Acknowledge error 15"]
    #[inline(always)]
    pub fn ae15(&self) -> AE15_R {
        AE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Acknowledge error 14"]
    #[inline(always)]
    pub fn ae14(&self) -> AE14_R {
        AE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Acknowledge error 13"]
    #[inline(always)]
    pub fn ae13(&self) -> AE13_R {
        AE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Acknowledge error 12"]
    #[inline(always)]
    pub fn ae12(&self) -> AE12_R {
        AE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Acknowledge error 11"]
    #[inline(always)]
    pub fn ae11(&self) -> AE11_R {
        AE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Acknowledge error 10"]
    #[inline(always)]
    pub fn ae10(&self) -> AE10_R {
        AE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Acknowledge error 9"]
    #[inline(always)]
    pub fn ae9(&self) -> AE9_R {
        AE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Acknowledge error 8"]
    #[inline(always)]
    pub fn ae8(&self) -> AE8_R {
        AE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Acknowledge error 7"]
    #[inline(always)]
    pub fn ae7(&self) -> AE7_R {
        AE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Acknowledge error 6"]
    #[inline(always)]
    pub fn ae6(&self) -> AE6_R {
        AE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Acknowledge error 5"]
    #[inline(always)]
    pub fn ae5(&self) -> AE5_R {
        AE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Acknowledge error 4"]
    #[inline(always)]
    pub fn ae4(&self) -> AE4_R {
        AE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Acknowledge error 3"]
    #[inline(always)]
    pub fn ae3(&self) -> AE3_R {
        AE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Acknowledge error 2"]
    #[inline(always)]
    pub fn ae2(&self) -> AE2_R {
        AE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Acknowledge error 1"]
    #[inline(always)]
    pub fn ae1(&self) -> AE1_R {
        AE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Acknowledge error 0"]
    #[inline(always)]
    pub fn ae0(&self) -> AE0_R {
        AE0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - PHY error 4"]
    #[inline(always)]
    pub fn pe4(&mut self) -> PE4_W {
        PE4_W { w: self }
    }
    #[doc = "Bit 19 - PHY error 3"]
    #[inline(always)]
    pub fn pe3(&mut self) -> PE3_W {
        PE3_W { w: self }
    }
    #[doc = "Bit 18 - PHY error 2"]
    #[inline(always)]
    pub fn pe2(&mut self) -> PE2_W {
        PE2_W { w: self }
    }
    #[doc = "Bit 17 - PHY error 1"]
    #[inline(always)]
    pub fn pe1(&mut self) -> PE1_W {
        PE1_W { w: self }
    }
    #[doc = "Bit 16 - PHY error 0"]
    #[inline(always)]
    pub fn pe0(&mut self) -> PE0_W {
        PE0_W { w: self }
    }
    #[doc = "Bit 15 - Acknowledge error 15"]
    #[inline(always)]
    pub fn ae15(&mut self) -> AE15_W {
        AE15_W { w: self }
    }
    #[doc = "Bit 14 - Acknowledge error 14"]
    #[inline(always)]
    pub fn ae14(&mut self) -> AE14_W {
        AE14_W { w: self }
    }
    #[doc = "Bit 13 - Acknowledge error 13"]
    #[inline(always)]
    pub fn ae13(&mut self) -> AE13_W {
        AE13_W { w: self }
    }
    #[doc = "Bit 12 - Acknowledge error 12"]
    #[inline(always)]
    pub fn ae12(&mut self) -> AE12_W {
        AE12_W { w: self }
    }
    #[doc = "Bit 11 - Acknowledge error 11"]
    #[inline(always)]
    pub fn ae11(&mut self) -> AE11_W {
        AE11_W { w: self }
    }
    #[doc = "Bit 10 - Acknowledge error 10"]
    #[inline(always)]
    pub fn ae10(&mut self) -> AE10_W {
        AE10_W { w: self }
    }
    #[doc = "Bit 9 - Acknowledge error 9"]
    #[inline(always)]
    pub fn ae9(&mut self) -> AE9_W {
        AE9_W { w: self }
    }
    #[doc = "Bit 8 - Acknowledge error 8"]
    #[inline(always)]
    pub fn ae8(&mut self) -> AE8_W {
        AE8_W { w: self }
    }
    #[doc = "Bit 7 - Acknowledge error 7"]
    #[inline(always)]
    pub fn ae7(&mut self) -> AE7_W {
        AE7_W { w: self }
    }
    #[doc = "Bit 6 - Acknowledge error 6"]
    #[inline(always)]
    pub fn ae6(&mut self) -> AE6_W {
        AE6_W { w: self }
    }
    #[doc = "Bit 5 - Acknowledge error 5"]
    #[inline(always)]
    pub fn ae5(&mut self) -> AE5_W {
        AE5_W { w: self }
    }
    #[doc = "Bit 4 - Acknowledge error 4"]
    #[inline(always)]
    pub fn ae4(&mut self) -> AE4_W {
        AE4_W { w: self }
    }
    #[doc = "Bit 3 - Acknowledge error 3"]
    #[inline(always)]
    pub fn ae3(&mut self) -> AE3_W {
        AE3_W { w: self }
    }
    #[doc = "Bit 2 - Acknowledge error 2"]
    #[inline(always)]
    pub fn ae2(&mut self) -> AE2_W {
        AE2_W { w: self }
    }
    #[doc = "Bit 1 - Acknowledge error 1"]
    #[inline(always)]
    pub fn ae1(&mut self) -> AE1_W {
        AE1_W { w: self }
    }
    #[doc = "Bit 0 - Acknowledge error 0"]
    #[inline(always)]
    pub fn ae0(&mut self) -> AE0_W {
        AE0_W { w: self }
    }
}
