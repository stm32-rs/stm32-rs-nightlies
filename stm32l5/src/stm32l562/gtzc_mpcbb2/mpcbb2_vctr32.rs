#[doc = "Reader of register MPCBB2_VCTR32"]
pub type R = crate::R<u32, super::MPCBB2_VCTR32>;
#[doc = "Writer for register MPCBB2_VCTR32"]
pub type W = crate::W<u32, super::MPCBB2_VCTR32>;
#[doc = "Register MPCBB2_VCTR32 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB2_VCTR32 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B1024`"]
pub type B1024_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1024`"]
pub struct B1024_W<'a> {
    w: &'a mut W,
}
impl<'a> B1024_W<'a> {
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
#[doc = "Reader of field `B1025`"]
pub type B1025_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1025`"]
pub struct B1025_W<'a> {
    w: &'a mut W,
}
impl<'a> B1025_W<'a> {
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
#[doc = "Reader of field `B1026`"]
pub type B1026_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1026`"]
pub struct B1026_W<'a> {
    w: &'a mut W,
}
impl<'a> B1026_W<'a> {
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
#[doc = "Reader of field `B1027`"]
pub type B1027_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1027`"]
pub struct B1027_W<'a> {
    w: &'a mut W,
}
impl<'a> B1027_W<'a> {
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
#[doc = "Reader of field `B1028`"]
pub type B1028_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1028`"]
pub struct B1028_W<'a> {
    w: &'a mut W,
}
impl<'a> B1028_W<'a> {
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
#[doc = "Reader of field `B1029`"]
pub type B1029_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1029`"]
pub struct B1029_W<'a> {
    w: &'a mut W,
}
impl<'a> B1029_W<'a> {
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
#[doc = "Reader of field `B1030`"]
pub type B1030_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1030`"]
pub struct B1030_W<'a> {
    w: &'a mut W,
}
impl<'a> B1030_W<'a> {
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
#[doc = "Reader of field `B1031`"]
pub type B1031_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1031`"]
pub struct B1031_W<'a> {
    w: &'a mut W,
}
impl<'a> B1031_W<'a> {
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
#[doc = "Reader of field `B1032`"]
pub type B1032_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1032`"]
pub struct B1032_W<'a> {
    w: &'a mut W,
}
impl<'a> B1032_W<'a> {
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
#[doc = "Reader of field `B1033`"]
pub type B1033_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1033`"]
pub struct B1033_W<'a> {
    w: &'a mut W,
}
impl<'a> B1033_W<'a> {
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
#[doc = "Reader of field `B1034`"]
pub type B1034_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1034`"]
pub struct B1034_W<'a> {
    w: &'a mut W,
}
impl<'a> B1034_W<'a> {
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
#[doc = "Reader of field `B1035`"]
pub type B1035_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1035`"]
pub struct B1035_W<'a> {
    w: &'a mut W,
}
impl<'a> B1035_W<'a> {
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
#[doc = "Reader of field `B1036`"]
pub type B1036_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1036`"]
pub struct B1036_W<'a> {
    w: &'a mut W,
}
impl<'a> B1036_W<'a> {
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
#[doc = "Reader of field `B1037`"]
pub type B1037_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1037`"]
pub struct B1037_W<'a> {
    w: &'a mut W,
}
impl<'a> B1037_W<'a> {
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
#[doc = "Reader of field `B1038`"]
pub type B1038_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1038`"]
pub struct B1038_W<'a> {
    w: &'a mut W,
}
impl<'a> B1038_W<'a> {
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
#[doc = "Reader of field `B1039`"]
pub type B1039_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1039`"]
pub struct B1039_W<'a> {
    w: &'a mut W,
}
impl<'a> B1039_W<'a> {
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
#[doc = "Reader of field `B1040`"]
pub type B1040_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1040`"]
pub struct B1040_W<'a> {
    w: &'a mut W,
}
impl<'a> B1040_W<'a> {
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
#[doc = "Reader of field `B1041`"]
pub type B1041_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1041`"]
pub struct B1041_W<'a> {
    w: &'a mut W,
}
impl<'a> B1041_W<'a> {
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
#[doc = "Reader of field `B1042`"]
pub type B1042_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1042`"]
pub struct B1042_W<'a> {
    w: &'a mut W,
}
impl<'a> B1042_W<'a> {
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
#[doc = "Reader of field `B1043`"]
pub type B1043_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1043`"]
pub struct B1043_W<'a> {
    w: &'a mut W,
}
impl<'a> B1043_W<'a> {
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
#[doc = "Reader of field `B1044`"]
pub type B1044_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1044`"]
pub struct B1044_W<'a> {
    w: &'a mut W,
}
impl<'a> B1044_W<'a> {
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
#[doc = "Reader of field `B1045`"]
pub type B1045_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1045`"]
pub struct B1045_W<'a> {
    w: &'a mut W,
}
impl<'a> B1045_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `B1046`"]
pub type B1046_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1046`"]
pub struct B1046_W<'a> {
    w: &'a mut W,
}
impl<'a> B1046_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `B1047`"]
pub type B1047_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1047`"]
pub struct B1047_W<'a> {
    w: &'a mut W,
}
impl<'a> B1047_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `B1048`"]
pub type B1048_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1048`"]
pub struct B1048_W<'a> {
    w: &'a mut W,
}
impl<'a> B1048_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `B1049`"]
pub type B1049_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1049`"]
pub struct B1049_W<'a> {
    w: &'a mut W,
}
impl<'a> B1049_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `B1050`"]
pub type B1050_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1050`"]
pub struct B1050_W<'a> {
    w: &'a mut W,
}
impl<'a> B1050_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `B1051`"]
pub type B1051_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1051`"]
pub struct B1051_W<'a> {
    w: &'a mut W,
}
impl<'a> B1051_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `B1052`"]
pub type B1052_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1052`"]
pub struct B1052_W<'a> {
    w: &'a mut W,
}
impl<'a> B1052_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `B1053`"]
pub type B1053_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1053`"]
pub struct B1053_W<'a> {
    w: &'a mut W,
}
impl<'a> B1053_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `B1054`"]
pub type B1054_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1054`"]
pub struct B1054_W<'a> {
    w: &'a mut W,
}
impl<'a> B1054_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `B1055`"]
pub type B1055_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1055`"]
pub struct B1055_W<'a> {
    w: &'a mut W,
}
impl<'a> B1055_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - B1024"]
    #[inline(always)]
    pub fn b1024(&self) -> B1024_R {
        B1024_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1025"]
    #[inline(always)]
    pub fn b1025(&self) -> B1025_R {
        B1025_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1026"]
    #[inline(always)]
    pub fn b1026(&self) -> B1026_R {
        B1026_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1027"]
    #[inline(always)]
    pub fn b1027(&self) -> B1027_R {
        B1027_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1028"]
    #[inline(always)]
    pub fn b1028(&self) -> B1028_R {
        B1028_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1029"]
    #[inline(always)]
    pub fn b1029(&self) -> B1029_R {
        B1029_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1030"]
    #[inline(always)]
    pub fn b1030(&self) -> B1030_R {
        B1030_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1031"]
    #[inline(always)]
    pub fn b1031(&self) -> B1031_R {
        B1031_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1032"]
    #[inline(always)]
    pub fn b1032(&self) -> B1032_R {
        B1032_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1033"]
    #[inline(always)]
    pub fn b1033(&self) -> B1033_R {
        B1033_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1034"]
    #[inline(always)]
    pub fn b1034(&self) -> B1034_R {
        B1034_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1035"]
    #[inline(always)]
    pub fn b1035(&self) -> B1035_R {
        B1035_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1036"]
    #[inline(always)]
    pub fn b1036(&self) -> B1036_R {
        B1036_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1037"]
    #[inline(always)]
    pub fn b1037(&self) -> B1037_R {
        B1037_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1038"]
    #[inline(always)]
    pub fn b1038(&self) -> B1038_R {
        B1038_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1039"]
    #[inline(always)]
    pub fn b1039(&self) -> B1039_R {
        B1039_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1040"]
    #[inline(always)]
    pub fn b1040(&self) -> B1040_R {
        B1040_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1041"]
    #[inline(always)]
    pub fn b1041(&self) -> B1041_R {
        B1041_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1042"]
    #[inline(always)]
    pub fn b1042(&self) -> B1042_R {
        B1042_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1043"]
    #[inline(always)]
    pub fn b1043(&self) -> B1043_R {
        B1043_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1044"]
    #[inline(always)]
    pub fn b1044(&self) -> B1044_R {
        B1044_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1045"]
    #[inline(always)]
    pub fn b1045(&self) -> B1045_R {
        B1045_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1046"]
    #[inline(always)]
    pub fn b1046(&self) -> B1046_R {
        B1046_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1047"]
    #[inline(always)]
    pub fn b1047(&self) -> B1047_R {
        B1047_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1048"]
    #[inline(always)]
    pub fn b1048(&self) -> B1048_R {
        B1048_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1049"]
    #[inline(always)]
    pub fn b1049(&self) -> B1049_R {
        B1049_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1050"]
    #[inline(always)]
    pub fn b1050(&self) -> B1050_R {
        B1050_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1051"]
    #[inline(always)]
    pub fn b1051(&self) -> B1051_R {
        B1051_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1052"]
    #[inline(always)]
    pub fn b1052(&self) -> B1052_R {
        B1052_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1053"]
    #[inline(always)]
    pub fn b1053(&self) -> B1053_R {
        B1053_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1054"]
    #[inline(always)]
    pub fn b1054(&self) -> B1054_R {
        B1054_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1055"]
    #[inline(always)]
    pub fn b1055(&self) -> B1055_R {
        B1055_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1024"]
    #[inline(always)]
    pub fn b1024(&mut self) -> B1024_W {
        B1024_W { w: self }
    }
    #[doc = "Bit 1 - B1025"]
    #[inline(always)]
    pub fn b1025(&mut self) -> B1025_W {
        B1025_W { w: self }
    }
    #[doc = "Bit 2 - B1026"]
    #[inline(always)]
    pub fn b1026(&mut self) -> B1026_W {
        B1026_W { w: self }
    }
    #[doc = "Bit 3 - B1027"]
    #[inline(always)]
    pub fn b1027(&mut self) -> B1027_W {
        B1027_W { w: self }
    }
    #[doc = "Bit 4 - B1028"]
    #[inline(always)]
    pub fn b1028(&mut self) -> B1028_W {
        B1028_W { w: self }
    }
    #[doc = "Bit 5 - B1029"]
    #[inline(always)]
    pub fn b1029(&mut self) -> B1029_W {
        B1029_W { w: self }
    }
    #[doc = "Bit 6 - B1030"]
    #[inline(always)]
    pub fn b1030(&mut self) -> B1030_W {
        B1030_W { w: self }
    }
    #[doc = "Bit 7 - B1031"]
    #[inline(always)]
    pub fn b1031(&mut self) -> B1031_W {
        B1031_W { w: self }
    }
    #[doc = "Bit 8 - B1032"]
    #[inline(always)]
    pub fn b1032(&mut self) -> B1032_W {
        B1032_W { w: self }
    }
    #[doc = "Bit 9 - B1033"]
    #[inline(always)]
    pub fn b1033(&mut self) -> B1033_W {
        B1033_W { w: self }
    }
    #[doc = "Bit 10 - B1034"]
    #[inline(always)]
    pub fn b1034(&mut self) -> B1034_W {
        B1034_W { w: self }
    }
    #[doc = "Bit 11 - B1035"]
    #[inline(always)]
    pub fn b1035(&mut self) -> B1035_W {
        B1035_W { w: self }
    }
    #[doc = "Bit 12 - B1036"]
    #[inline(always)]
    pub fn b1036(&mut self) -> B1036_W {
        B1036_W { w: self }
    }
    #[doc = "Bit 13 - B1037"]
    #[inline(always)]
    pub fn b1037(&mut self) -> B1037_W {
        B1037_W { w: self }
    }
    #[doc = "Bit 14 - B1038"]
    #[inline(always)]
    pub fn b1038(&mut self) -> B1038_W {
        B1038_W { w: self }
    }
    #[doc = "Bit 15 - B1039"]
    #[inline(always)]
    pub fn b1039(&mut self) -> B1039_W {
        B1039_W { w: self }
    }
    #[doc = "Bit 16 - B1040"]
    #[inline(always)]
    pub fn b1040(&mut self) -> B1040_W {
        B1040_W { w: self }
    }
    #[doc = "Bit 17 - B1041"]
    #[inline(always)]
    pub fn b1041(&mut self) -> B1041_W {
        B1041_W { w: self }
    }
    #[doc = "Bit 18 - B1042"]
    #[inline(always)]
    pub fn b1042(&mut self) -> B1042_W {
        B1042_W { w: self }
    }
    #[doc = "Bit 19 - B1043"]
    #[inline(always)]
    pub fn b1043(&mut self) -> B1043_W {
        B1043_W { w: self }
    }
    #[doc = "Bit 20 - B1044"]
    #[inline(always)]
    pub fn b1044(&mut self) -> B1044_W {
        B1044_W { w: self }
    }
    #[doc = "Bit 21 - B1045"]
    #[inline(always)]
    pub fn b1045(&mut self) -> B1045_W {
        B1045_W { w: self }
    }
    #[doc = "Bit 22 - B1046"]
    #[inline(always)]
    pub fn b1046(&mut self) -> B1046_W {
        B1046_W { w: self }
    }
    #[doc = "Bit 23 - B1047"]
    #[inline(always)]
    pub fn b1047(&mut self) -> B1047_W {
        B1047_W { w: self }
    }
    #[doc = "Bit 24 - B1048"]
    #[inline(always)]
    pub fn b1048(&mut self) -> B1048_W {
        B1048_W { w: self }
    }
    #[doc = "Bit 25 - B1049"]
    #[inline(always)]
    pub fn b1049(&mut self) -> B1049_W {
        B1049_W { w: self }
    }
    #[doc = "Bit 26 - B1050"]
    #[inline(always)]
    pub fn b1050(&mut self) -> B1050_W {
        B1050_W { w: self }
    }
    #[doc = "Bit 27 - B1051"]
    #[inline(always)]
    pub fn b1051(&mut self) -> B1051_W {
        B1051_W { w: self }
    }
    #[doc = "Bit 28 - B1052"]
    #[inline(always)]
    pub fn b1052(&mut self) -> B1052_W {
        B1052_W { w: self }
    }
    #[doc = "Bit 29 - B1053"]
    #[inline(always)]
    pub fn b1053(&mut self) -> B1053_W {
        B1053_W { w: self }
    }
    #[doc = "Bit 30 - B1054"]
    #[inline(always)]
    pub fn b1054(&mut self) -> B1054_W {
        B1054_W { w: self }
    }
    #[doc = "Bit 31 - B1055"]
    #[inline(always)]
    pub fn b1055(&mut self) -> B1055_W {
        B1055_W { w: self }
    }
}
