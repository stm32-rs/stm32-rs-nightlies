#[doc = "Reader of register MPCBB1_VCTR5"]
pub type R = crate::R<u32, super::MPCBB1_VCTR5>;
#[doc = "Writer for register MPCBB1_VCTR5"]
pub type W = crate::W<u32, super::MPCBB1_VCTR5>;
#[doc = "Register MPCBB1_VCTR5 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB1_VCTR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B160`"]
pub type B160_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B160`"]
pub struct B160_W<'a> {
    w: &'a mut W,
}
impl<'a> B160_W<'a> {
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
#[doc = "Reader of field `B161`"]
pub type B161_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B161`"]
pub struct B161_W<'a> {
    w: &'a mut W,
}
impl<'a> B161_W<'a> {
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
#[doc = "Reader of field `B162`"]
pub type B162_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B162`"]
pub struct B162_W<'a> {
    w: &'a mut W,
}
impl<'a> B162_W<'a> {
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
#[doc = "Reader of field `B163`"]
pub type B163_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B163`"]
pub struct B163_W<'a> {
    w: &'a mut W,
}
impl<'a> B163_W<'a> {
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
#[doc = "Reader of field `B164`"]
pub type B164_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B164`"]
pub struct B164_W<'a> {
    w: &'a mut W,
}
impl<'a> B164_W<'a> {
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
#[doc = "Reader of field `B165`"]
pub type B165_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B165`"]
pub struct B165_W<'a> {
    w: &'a mut W,
}
impl<'a> B165_W<'a> {
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
#[doc = "Reader of field `B166`"]
pub type B166_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B166`"]
pub struct B166_W<'a> {
    w: &'a mut W,
}
impl<'a> B166_W<'a> {
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
#[doc = "Reader of field `B167`"]
pub type B167_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B167`"]
pub struct B167_W<'a> {
    w: &'a mut W,
}
impl<'a> B167_W<'a> {
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
#[doc = "Reader of field `B168`"]
pub type B168_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B168`"]
pub struct B168_W<'a> {
    w: &'a mut W,
}
impl<'a> B168_W<'a> {
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
#[doc = "Reader of field `B169`"]
pub type B169_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B169`"]
pub struct B169_W<'a> {
    w: &'a mut W,
}
impl<'a> B169_W<'a> {
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
#[doc = "Reader of field `B170`"]
pub type B170_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B170`"]
pub struct B170_W<'a> {
    w: &'a mut W,
}
impl<'a> B170_W<'a> {
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
#[doc = "Reader of field `B171`"]
pub type B171_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B171`"]
pub struct B171_W<'a> {
    w: &'a mut W,
}
impl<'a> B171_W<'a> {
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
#[doc = "Reader of field `B172`"]
pub type B172_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B172`"]
pub struct B172_W<'a> {
    w: &'a mut W,
}
impl<'a> B172_W<'a> {
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
#[doc = "Reader of field `B173`"]
pub type B173_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B173`"]
pub struct B173_W<'a> {
    w: &'a mut W,
}
impl<'a> B173_W<'a> {
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
#[doc = "Reader of field `B174`"]
pub type B174_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B174`"]
pub struct B174_W<'a> {
    w: &'a mut W,
}
impl<'a> B174_W<'a> {
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
#[doc = "Reader of field `B175`"]
pub type B175_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B175`"]
pub struct B175_W<'a> {
    w: &'a mut W,
}
impl<'a> B175_W<'a> {
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
#[doc = "Reader of field `B176`"]
pub type B176_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B176`"]
pub struct B176_W<'a> {
    w: &'a mut W,
}
impl<'a> B176_W<'a> {
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
#[doc = "Reader of field `B177`"]
pub type B177_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B177`"]
pub struct B177_W<'a> {
    w: &'a mut W,
}
impl<'a> B177_W<'a> {
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
#[doc = "Reader of field `B178`"]
pub type B178_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B178`"]
pub struct B178_W<'a> {
    w: &'a mut W,
}
impl<'a> B178_W<'a> {
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
#[doc = "Reader of field `B179`"]
pub type B179_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B179`"]
pub struct B179_W<'a> {
    w: &'a mut W,
}
impl<'a> B179_W<'a> {
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
#[doc = "Reader of field `B180`"]
pub type B180_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B180`"]
pub struct B180_W<'a> {
    w: &'a mut W,
}
impl<'a> B180_W<'a> {
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
#[doc = "Reader of field `B181`"]
pub type B181_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B181`"]
pub struct B181_W<'a> {
    w: &'a mut W,
}
impl<'a> B181_W<'a> {
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
#[doc = "Reader of field `B182`"]
pub type B182_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B182`"]
pub struct B182_W<'a> {
    w: &'a mut W,
}
impl<'a> B182_W<'a> {
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
#[doc = "Reader of field `B183`"]
pub type B183_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B183`"]
pub struct B183_W<'a> {
    w: &'a mut W,
}
impl<'a> B183_W<'a> {
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
#[doc = "Reader of field `B184`"]
pub type B184_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B184`"]
pub struct B184_W<'a> {
    w: &'a mut W,
}
impl<'a> B184_W<'a> {
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
#[doc = "Reader of field `B185`"]
pub type B185_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B185`"]
pub struct B185_W<'a> {
    w: &'a mut W,
}
impl<'a> B185_W<'a> {
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
#[doc = "Reader of field `B186`"]
pub type B186_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B186`"]
pub struct B186_W<'a> {
    w: &'a mut W,
}
impl<'a> B186_W<'a> {
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
#[doc = "Reader of field `B187`"]
pub type B187_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B187`"]
pub struct B187_W<'a> {
    w: &'a mut W,
}
impl<'a> B187_W<'a> {
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
#[doc = "Reader of field `B188`"]
pub type B188_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B188`"]
pub struct B188_W<'a> {
    w: &'a mut W,
}
impl<'a> B188_W<'a> {
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
#[doc = "Reader of field `B189`"]
pub type B189_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B189`"]
pub struct B189_W<'a> {
    w: &'a mut W,
}
impl<'a> B189_W<'a> {
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
#[doc = "Reader of field `B190`"]
pub type B190_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B190`"]
pub struct B190_W<'a> {
    w: &'a mut W,
}
impl<'a> B190_W<'a> {
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
#[doc = "Reader of field `B191`"]
pub type B191_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B191`"]
pub struct B191_W<'a> {
    w: &'a mut W,
}
impl<'a> B191_W<'a> {
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
    #[doc = "Bit 0 - B160"]
    #[inline(always)]
    pub fn b160(&self) -> B160_R {
        B160_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B161"]
    #[inline(always)]
    pub fn b161(&self) -> B161_R {
        B161_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B162"]
    #[inline(always)]
    pub fn b162(&self) -> B162_R {
        B162_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B163"]
    #[inline(always)]
    pub fn b163(&self) -> B163_R {
        B163_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B164"]
    #[inline(always)]
    pub fn b164(&self) -> B164_R {
        B164_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B165"]
    #[inline(always)]
    pub fn b165(&self) -> B165_R {
        B165_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B166"]
    #[inline(always)]
    pub fn b166(&self) -> B166_R {
        B166_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B167"]
    #[inline(always)]
    pub fn b167(&self) -> B167_R {
        B167_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B168"]
    #[inline(always)]
    pub fn b168(&self) -> B168_R {
        B168_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B169"]
    #[inline(always)]
    pub fn b169(&self) -> B169_R {
        B169_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B170"]
    #[inline(always)]
    pub fn b170(&self) -> B170_R {
        B170_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B171"]
    #[inline(always)]
    pub fn b171(&self) -> B171_R {
        B171_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B172"]
    #[inline(always)]
    pub fn b172(&self) -> B172_R {
        B172_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B173"]
    #[inline(always)]
    pub fn b173(&self) -> B173_R {
        B173_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B174"]
    #[inline(always)]
    pub fn b174(&self) -> B174_R {
        B174_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B175"]
    #[inline(always)]
    pub fn b175(&self) -> B175_R {
        B175_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B176"]
    #[inline(always)]
    pub fn b176(&self) -> B176_R {
        B176_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B177"]
    #[inline(always)]
    pub fn b177(&self) -> B177_R {
        B177_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B178"]
    #[inline(always)]
    pub fn b178(&self) -> B178_R {
        B178_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B179"]
    #[inline(always)]
    pub fn b179(&self) -> B179_R {
        B179_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B180"]
    #[inline(always)]
    pub fn b180(&self) -> B180_R {
        B180_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B181"]
    #[inline(always)]
    pub fn b181(&self) -> B181_R {
        B181_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B182"]
    #[inline(always)]
    pub fn b182(&self) -> B182_R {
        B182_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B183"]
    #[inline(always)]
    pub fn b183(&self) -> B183_R {
        B183_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B184"]
    #[inline(always)]
    pub fn b184(&self) -> B184_R {
        B184_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B185"]
    #[inline(always)]
    pub fn b185(&self) -> B185_R {
        B185_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B186"]
    #[inline(always)]
    pub fn b186(&self) -> B186_R {
        B186_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B187"]
    #[inline(always)]
    pub fn b187(&self) -> B187_R {
        B187_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B188"]
    #[inline(always)]
    pub fn b188(&self) -> B188_R {
        B188_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B189"]
    #[inline(always)]
    pub fn b189(&self) -> B189_R {
        B189_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B190"]
    #[inline(always)]
    pub fn b190(&self) -> B190_R {
        B190_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B191"]
    #[inline(always)]
    pub fn b191(&self) -> B191_R {
        B191_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B160"]
    #[inline(always)]
    pub fn b160(&mut self) -> B160_W {
        B160_W { w: self }
    }
    #[doc = "Bit 1 - B161"]
    #[inline(always)]
    pub fn b161(&mut self) -> B161_W {
        B161_W { w: self }
    }
    #[doc = "Bit 2 - B162"]
    #[inline(always)]
    pub fn b162(&mut self) -> B162_W {
        B162_W { w: self }
    }
    #[doc = "Bit 3 - B163"]
    #[inline(always)]
    pub fn b163(&mut self) -> B163_W {
        B163_W { w: self }
    }
    #[doc = "Bit 4 - B164"]
    #[inline(always)]
    pub fn b164(&mut self) -> B164_W {
        B164_W { w: self }
    }
    #[doc = "Bit 5 - B165"]
    #[inline(always)]
    pub fn b165(&mut self) -> B165_W {
        B165_W { w: self }
    }
    #[doc = "Bit 6 - B166"]
    #[inline(always)]
    pub fn b166(&mut self) -> B166_W {
        B166_W { w: self }
    }
    #[doc = "Bit 7 - B167"]
    #[inline(always)]
    pub fn b167(&mut self) -> B167_W {
        B167_W { w: self }
    }
    #[doc = "Bit 8 - B168"]
    #[inline(always)]
    pub fn b168(&mut self) -> B168_W {
        B168_W { w: self }
    }
    #[doc = "Bit 9 - B169"]
    #[inline(always)]
    pub fn b169(&mut self) -> B169_W {
        B169_W { w: self }
    }
    #[doc = "Bit 10 - B170"]
    #[inline(always)]
    pub fn b170(&mut self) -> B170_W {
        B170_W { w: self }
    }
    #[doc = "Bit 11 - B171"]
    #[inline(always)]
    pub fn b171(&mut self) -> B171_W {
        B171_W { w: self }
    }
    #[doc = "Bit 12 - B172"]
    #[inline(always)]
    pub fn b172(&mut self) -> B172_W {
        B172_W { w: self }
    }
    #[doc = "Bit 13 - B173"]
    #[inline(always)]
    pub fn b173(&mut self) -> B173_W {
        B173_W { w: self }
    }
    #[doc = "Bit 14 - B174"]
    #[inline(always)]
    pub fn b174(&mut self) -> B174_W {
        B174_W { w: self }
    }
    #[doc = "Bit 15 - B175"]
    #[inline(always)]
    pub fn b175(&mut self) -> B175_W {
        B175_W { w: self }
    }
    #[doc = "Bit 16 - B176"]
    #[inline(always)]
    pub fn b176(&mut self) -> B176_W {
        B176_W { w: self }
    }
    #[doc = "Bit 17 - B177"]
    #[inline(always)]
    pub fn b177(&mut self) -> B177_W {
        B177_W { w: self }
    }
    #[doc = "Bit 18 - B178"]
    #[inline(always)]
    pub fn b178(&mut self) -> B178_W {
        B178_W { w: self }
    }
    #[doc = "Bit 19 - B179"]
    #[inline(always)]
    pub fn b179(&mut self) -> B179_W {
        B179_W { w: self }
    }
    #[doc = "Bit 20 - B180"]
    #[inline(always)]
    pub fn b180(&mut self) -> B180_W {
        B180_W { w: self }
    }
    #[doc = "Bit 21 - B181"]
    #[inline(always)]
    pub fn b181(&mut self) -> B181_W {
        B181_W { w: self }
    }
    #[doc = "Bit 22 - B182"]
    #[inline(always)]
    pub fn b182(&mut self) -> B182_W {
        B182_W { w: self }
    }
    #[doc = "Bit 23 - B183"]
    #[inline(always)]
    pub fn b183(&mut self) -> B183_W {
        B183_W { w: self }
    }
    #[doc = "Bit 24 - B184"]
    #[inline(always)]
    pub fn b184(&mut self) -> B184_W {
        B184_W { w: self }
    }
    #[doc = "Bit 25 - B185"]
    #[inline(always)]
    pub fn b185(&mut self) -> B185_W {
        B185_W { w: self }
    }
    #[doc = "Bit 26 - B186"]
    #[inline(always)]
    pub fn b186(&mut self) -> B186_W {
        B186_W { w: self }
    }
    #[doc = "Bit 27 - B187"]
    #[inline(always)]
    pub fn b187(&mut self) -> B187_W {
        B187_W { w: self }
    }
    #[doc = "Bit 28 - B188"]
    #[inline(always)]
    pub fn b188(&mut self) -> B188_W {
        B188_W { w: self }
    }
    #[doc = "Bit 29 - B189"]
    #[inline(always)]
    pub fn b189(&mut self) -> B189_W {
        B189_W { w: self }
    }
    #[doc = "Bit 30 - B190"]
    #[inline(always)]
    pub fn b190(&mut self) -> B190_W {
        B190_W { w: self }
    }
    #[doc = "Bit 31 - B191"]
    #[inline(always)]
    pub fn b191(&mut self) -> B191_W {
        B191_W { w: self }
    }
}
