#[doc = "Reader of register SWPR"]
pub type R = crate::R<u32, super::SWPR>;
#[doc = "Writer for register SWPR"]
pub type W = crate::W<u32, super::SWPR>;
#[doc = "Register SWPR `reset()`'s with value 0"]
impl crate::ResetValue for super::SWPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P31WP`"]
pub type P31WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P31WP`"]
pub struct P31WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P31WP_W<'a> {
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
#[doc = "Reader of field `P30WP`"]
pub type P30WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P30WP`"]
pub struct P30WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P30WP_W<'a> {
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
#[doc = "Reader of field `P29WP`"]
pub type P29WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P29WP`"]
pub struct P29WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P29WP_W<'a> {
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
#[doc = "Reader of field `P28WP`"]
pub type P28WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P28WP`"]
pub struct P28WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P28WP_W<'a> {
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
#[doc = "Reader of field `P27WP`"]
pub type P27WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P27WP`"]
pub struct P27WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P27WP_W<'a> {
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
#[doc = "Reader of field `P26WP`"]
pub type P26WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P26WP`"]
pub struct P26WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P26WP_W<'a> {
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
#[doc = "Reader of field `P25WP`"]
pub type P25WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P25WP`"]
pub struct P25WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P25WP_W<'a> {
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
#[doc = "Reader of field `P24WP`"]
pub type P24WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P24WP`"]
pub struct P24WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P24WP_W<'a> {
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
#[doc = "Reader of field `P23WP`"]
pub type P23WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P23WP`"]
pub struct P23WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P23WP_W<'a> {
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
#[doc = "Reader of field `P22WP`"]
pub type P22WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P22WP`"]
pub struct P22WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P22WP_W<'a> {
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
#[doc = "Reader of field `P21WP`"]
pub type P21WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P21WP`"]
pub struct P21WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P21WP_W<'a> {
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
#[doc = "Reader of field `P20WP`"]
pub type P20WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P20WP`"]
pub struct P20WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P20WP_W<'a> {
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
#[doc = "Reader of field `P19WP`"]
pub type P19WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P19WP`"]
pub struct P19WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P19WP_W<'a> {
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
#[doc = "Reader of field `P18WP`"]
pub type P18WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P18WP`"]
pub struct P18WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P18WP_W<'a> {
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
#[doc = "Reader of field `P17WP`"]
pub type P17WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P17WP`"]
pub struct P17WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P17WP_W<'a> {
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
#[doc = "Reader of field `P16WP`"]
pub type P16WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P16WP`"]
pub struct P16WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P16WP_W<'a> {
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
#[doc = "Reader of field `P15WP`"]
pub type P15WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P15WP`"]
pub struct P15WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P15WP_W<'a> {
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
#[doc = "Reader of field `P14WP`"]
pub type P14WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P14WP`"]
pub struct P14WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P14WP_W<'a> {
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
#[doc = "Reader of field `P13WP`"]
pub type P13WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P13WP`"]
pub struct P13WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P13WP_W<'a> {
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
#[doc = "Reader of field `P12WP`"]
pub type P12WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P12WP`"]
pub struct P12WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P12WP_W<'a> {
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
#[doc = "Reader of field `P11WP`"]
pub type P11WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P11WP`"]
pub struct P11WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P11WP_W<'a> {
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
#[doc = "Reader of field `P10WP`"]
pub type P10WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P10WP`"]
pub struct P10WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P10WP_W<'a> {
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
#[doc = "Reader of field `P9WP`"]
pub type P9WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9WP`"]
pub struct P9WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P9WP_W<'a> {
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
#[doc = "Reader of field `P8WP`"]
pub type P8WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P8WP`"]
pub struct P8WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P8WP_W<'a> {
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
#[doc = "Reader of field `P7WP`"]
pub type P7WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P7WP`"]
pub struct P7WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P7WP_W<'a> {
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
#[doc = "Reader of field `P6WP`"]
pub type P6WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P6WP`"]
pub struct P6WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P6WP_W<'a> {
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
#[doc = "Reader of field `P5WP`"]
pub type P5WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P5WP`"]
pub struct P5WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P5WP_W<'a> {
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
#[doc = "Reader of field `P4WP`"]
pub type P4WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P4WP`"]
pub struct P4WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P4WP_W<'a> {
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
#[doc = "Reader of field `P3WP`"]
pub type P3WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3WP`"]
pub struct P3WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P3WP_W<'a> {
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
#[doc = "Reader of field `P2WP`"]
pub type P2WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P2WP`"]
pub struct P2WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P2WP_W<'a> {
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
#[doc = "Reader of field `P1WP`"]
pub type P1WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P1WP`"]
pub struct P1WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P1WP_W<'a> {
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
#[doc = "Reader of field `P0WP`"]
pub type P0WP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P0WP`"]
pub struct P0WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P0WP_W<'a> {
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
    #[doc = "Bit 31 - SRAM2 1Kbyte page 31 write protection"]
    #[inline(always)]
    pub fn p31wp(&self) -> P31WP_R {
        P31WP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - SRAM2 1Kbyte page 30 write protection"]
    #[inline(always)]
    pub fn p30wp(&self) -> P30WP_R {
        P30WP_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - SRAM2 1Kbyte page 29 write protection"]
    #[inline(always)]
    pub fn p29wp(&self) -> P29WP_R {
        P29WP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - SRAM2 1Kbyte page 28 write protection"]
    #[inline(always)]
    pub fn p28wp(&self) -> P28WP_R {
        P28WP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - SRAM2 1Kbyte page 27 write protection"]
    #[inline(always)]
    pub fn p27wp(&self) -> P27WP_R {
        P27WP_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SRAM2 1Kbyte page 26 write protection"]
    #[inline(always)]
    pub fn p26wp(&self) -> P26WP_R {
        P26WP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SRAM2 1Kbyte page 25 write protection"]
    #[inline(always)]
    pub fn p25wp(&self) -> P25WP_R {
        P25WP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - SRAM2 1Kbyte page 24 write protection"]
    #[inline(always)]
    pub fn p24wp(&self) -> P24WP_R {
        P24WP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - SRAM2 1Kbyte page 23 write protection"]
    #[inline(always)]
    pub fn p23wp(&self) -> P23WP_R {
        P23WP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SRAM2 1Kbyte page 22 write protection"]
    #[inline(always)]
    pub fn p22wp(&self) -> P22WP_R {
        P22WP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SRAM2 1Kbyte page 21 write protection"]
    #[inline(always)]
    pub fn p21wp(&self) -> P21WP_R {
        P21WP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SRAM2 1Kbyte page 20 write protection"]
    #[inline(always)]
    pub fn p20wp(&self) -> P20WP_R {
        P20WP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SRAM2 1Kbyte page 19 write protection"]
    #[inline(always)]
    pub fn p19wp(&self) -> P19WP_R {
        P19WP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SRAM2 1Kbyte page 18 write protection"]
    #[inline(always)]
    pub fn p18wp(&self) -> P18WP_R {
        P18WP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SRAM2 1Kbyte page 17 write protection"]
    #[inline(always)]
    pub fn p17wp(&self) -> P17WP_R {
        P17WP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SRAM2 1Kbyte page 16 write protection"]
    #[inline(always)]
    pub fn p16wp(&self) -> P16WP_R {
        P16WP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SRAM2 1Kbyte page 15 write protection"]
    #[inline(always)]
    pub fn p15wp(&self) -> P15WP_R {
        P15WP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SRAM2 1Kbyte page 14 write protection"]
    #[inline(always)]
    pub fn p14wp(&self) -> P14WP_R {
        P14WP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SRAM2 1Kbyte page 13 write protection"]
    #[inline(always)]
    pub fn p13wp(&self) -> P13WP_R {
        P13WP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SRAM2 1Kbyte page 12 write protection"]
    #[inline(always)]
    pub fn p12wp(&self) -> P12WP_R {
        P12WP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SRAM2 1Kbyte page 11 write protection"]
    #[inline(always)]
    pub fn p11wp(&self) -> P11WP_R {
        P11WP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SRAM2 1Kbyte page 10 write protection"]
    #[inline(always)]
    pub fn p10wp(&self) -> P10WP_R {
        P10WP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SRAM2 1Kbyte page 9 write protection"]
    #[inline(always)]
    pub fn p9wp(&self) -> P9WP_R {
        P9WP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SRAM2 1Kbyte page 8 write protection"]
    #[inline(always)]
    pub fn p8wp(&self) -> P8WP_R {
        P8WP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SRAM2 1Kbyte page 7 write protection"]
    #[inline(always)]
    pub fn p7wp(&self) -> P7WP_R {
        P7WP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SRAM2 1Kbyte page 6 write protection"]
    #[inline(always)]
    pub fn p6wp(&self) -> P6WP_R {
        P6WP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SRAM2 1Kbyte page 5 write protection"]
    #[inline(always)]
    pub fn p5wp(&self) -> P5WP_R {
        P5WP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SRAM2 1Kbyte page 4 write protection"]
    #[inline(always)]
    pub fn p4wp(&self) -> P4WP_R {
        P4WP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SRAM2 1Kbyte page 3 write protection"]
    #[inline(always)]
    pub fn p3wp(&self) -> P3WP_R {
        P3WP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SRAM2 1Kbyte page 2 write protection"]
    #[inline(always)]
    pub fn p2wp(&self) -> P2WP_R {
        P2WP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRAM2 1Kbyte page 1 write protection"]
    #[inline(always)]
    pub fn p1wp(&self) -> P1WP_R {
        P1WP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SRAM2 1Kbyte page 0 write protection"]
    #[inline(always)]
    pub fn p0wp(&self) -> P0WP_R {
        P0WP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - SRAM2 1Kbyte page 31 write protection"]
    #[inline(always)]
    pub fn p31wp(&mut self) -> P31WP_W {
        P31WP_W { w: self }
    }
    #[doc = "Bit 30 - SRAM2 1Kbyte page 30 write protection"]
    #[inline(always)]
    pub fn p30wp(&mut self) -> P30WP_W {
        P30WP_W { w: self }
    }
    #[doc = "Bit 29 - SRAM2 1Kbyte page 29 write protection"]
    #[inline(always)]
    pub fn p29wp(&mut self) -> P29WP_W {
        P29WP_W { w: self }
    }
    #[doc = "Bit 28 - SRAM2 1Kbyte page 28 write protection"]
    #[inline(always)]
    pub fn p28wp(&mut self) -> P28WP_W {
        P28WP_W { w: self }
    }
    #[doc = "Bit 27 - SRAM2 1Kbyte page 27 write protection"]
    #[inline(always)]
    pub fn p27wp(&mut self) -> P27WP_W {
        P27WP_W { w: self }
    }
    #[doc = "Bit 26 - SRAM2 1Kbyte page 26 write protection"]
    #[inline(always)]
    pub fn p26wp(&mut self) -> P26WP_W {
        P26WP_W { w: self }
    }
    #[doc = "Bit 25 - SRAM2 1Kbyte page 25 write protection"]
    #[inline(always)]
    pub fn p25wp(&mut self) -> P25WP_W {
        P25WP_W { w: self }
    }
    #[doc = "Bit 24 - SRAM2 1Kbyte page 24 write protection"]
    #[inline(always)]
    pub fn p24wp(&mut self) -> P24WP_W {
        P24WP_W { w: self }
    }
    #[doc = "Bit 23 - SRAM2 1Kbyte page 23 write protection"]
    #[inline(always)]
    pub fn p23wp(&mut self) -> P23WP_W {
        P23WP_W { w: self }
    }
    #[doc = "Bit 22 - SRAM2 1Kbyte page 22 write protection"]
    #[inline(always)]
    pub fn p22wp(&mut self) -> P22WP_W {
        P22WP_W { w: self }
    }
    #[doc = "Bit 21 - SRAM2 1Kbyte page 21 write protection"]
    #[inline(always)]
    pub fn p21wp(&mut self) -> P21WP_W {
        P21WP_W { w: self }
    }
    #[doc = "Bit 20 - SRAM2 1Kbyte page 20 write protection"]
    #[inline(always)]
    pub fn p20wp(&mut self) -> P20WP_W {
        P20WP_W { w: self }
    }
    #[doc = "Bit 19 - SRAM2 1Kbyte page 19 write protection"]
    #[inline(always)]
    pub fn p19wp(&mut self) -> P19WP_W {
        P19WP_W { w: self }
    }
    #[doc = "Bit 18 - SRAM2 1Kbyte page 18 write protection"]
    #[inline(always)]
    pub fn p18wp(&mut self) -> P18WP_W {
        P18WP_W { w: self }
    }
    #[doc = "Bit 17 - SRAM2 1Kbyte page 17 write protection"]
    #[inline(always)]
    pub fn p17wp(&mut self) -> P17WP_W {
        P17WP_W { w: self }
    }
    #[doc = "Bit 16 - SRAM2 1Kbyte page 16 write protection"]
    #[inline(always)]
    pub fn p16wp(&mut self) -> P16WP_W {
        P16WP_W { w: self }
    }
    #[doc = "Bit 15 - SRAM2 1Kbyte page 15 write protection"]
    #[inline(always)]
    pub fn p15wp(&mut self) -> P15WP_W {
        P15WP_W { w: self }
    }
    #[doc = "Bit 14 - SRAM2 1Kbyte page 14 write protection"]
    #[inline(always)]
    pub fn p14wp(&mut self) -> P14WP_W {
        P14WP_W { w: self }
    }
    #[doc = "Bit 13 - SRAM2 1Kbyte page 13 write protection"]
    #[inline(always)]
    pub fn p13wp(&mut self) -> P13WP_W {
        P13WP_W { w: self }
    }
    #[doc = "Bit 12 - SRAM2 1Kbyte page 12 write protection"]
    #[inline(always)]
    pub fn p12wp(&mut self) -> P12WP_W {
        P12WP_W { w: self }
    }
    #[doc = "Bit 11 - SRAM2 1Kbyte page 11 write protection"]
    #[inline(always)]
    pub fn p11wp(&mut self) -> P11WP_W {
        P11WP_W { w: self }
    }
    #[doc = "Bit 10 - SRAM2 1Kbyte page 10 write protection"]
    #[inline(always)]
    pub fn p10wp(&mut self) -> P10WP_W {
        P10WP_W { w: self }
    }
    #[doc = "Bit 9 - SRAM2 1Kbyte page 9 write protection"]
    #[inline(always)]
    pub fn p9wp(&mut self) -> P9WP_W {
        P9WP_W { w: self }
    }
    #[doc = "Bit 8 - SRAM2 1Kbyte page 8 write protection"]
    #[inline(always)]
    pub fn p8wp(&mut self) -> P8WP_W {
        P8WP_W { w: self }
    }
    #[doc = "Bit 7 - SRAM2 1Kbyte page 7 write protection"]
    #[inline(always)]
    pub fn p7wp(&mut self) -> P7WP_W {
        P7WP_W { w: self }
    }
    #[doc = "Bit 6 - SRAM2 1Kbyte page 6 write protection"]
    #[inline(always)]
    pub fn p6wp(&mut self) -> P6WP_W {
        P6WP_W { w: self }
    }
    #[doc = "Bit 5 - SRAM2 1Kbyte page 5 write protection"]
    #[inline(always)]
    pub fn p5wp(&mut self) -> P5WP_W {
        P5WP_W { w: self }
    }
    #[doc = "Bit 4 - SRAM2 1Kbyte page 4 write protection"]
    #[inline(always)]
    pub fn p4wp(&mut self) -> P4WP_W {
        P4WP_W { w: self }
    }
    #[doc = "Bit 3 - SRAM2 1Kbyte page 3 write protection"]
    #[inline(always)]
    pub fn p3wp(&mut self) -> P3WP_W {
        P3WP_W { w: self }
    }
    #[doc = "Bit 2 - SRAM2 1Kbyte page 2 write protection"]
    #[inline(always)]
    pub fn p2wp(&mut self) -> P2WP_W {
        P2WP_W { w: self }
    }
    #[doc = "Bit 1 - SRAM2 1Kbyte page 1 write protection"]
    #[inline(always)]
    pub fn p1wp(&mut self) -> P1WP_W {
        P1WP_W { w: self }
    }
    #[doc = "Bit 0 - SRAM2 1Kbyte page 0 write protection"]
    #[inline(always)]
    pub fn p0wp(&mut self) -> P0WP_W {
        P0WP_W { w: self }
    }
}
