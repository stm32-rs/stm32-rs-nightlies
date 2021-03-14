#[doc = "Reader of register MPCBB2_VCTR6"]
pub type R = crate::R<u32, super::MPCBB2_VCTR6>;
#[doc = "Writer for register MPCBB2_VCTR6"]
pub type W = crate::W<u32, super::MPCBB2_VCTR6>;
#[doc = "Register MPCBB2_VCTR6 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB2_VCTR6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B192`"]
pub type B192_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B192`"]
pub struct B192_W<'a> {
    w: &'a mut W,
}
impl<'a> B192_W<'a> {
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
#[doc = "Reader of field `B193`"]
pub type B193_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B193`"]
pub struct B193_W<'a> {
    w: &'a mut W,
}
impl<'a> B193_W<'a> {
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
#[doc = "Reader of field `B194`"]
pub type B194_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B194`"]
pub struct B194_W<'a> {
    w: &'a mut W,
}
impl<'a> B194_W<'a> {
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
#[doc = "Reader of field `B195`"]
pub type B195_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B195`"]
pub struct B195_W<'a> {
    w: &'a mut W,
}
impl<'a> B195_W<'a> {
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
#[doc = "Reader of field `B196`"]
pub type B196_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B196`"]
pub struct B196_W<'a> {
    w: &'a mut W,
}
impl<'a> B196_W<'a> {
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
#[doc = "Reader of field `B197`"]
pub type B197_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B197`"]
pub struct B197_W<'a> {
    w: &'a mut W,
}
impl<'a> B197_W<'a> {
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
#[doc = "Reader of field `B198`"]
pub type B198_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B198`"]
pub struct B198_W<'a> {
    w: &'a mut W,
}
impl<'a> B198_W<'a> {
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
#[doc = "Reader of field `B199`"]
pub type B199_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B199`"]
pub struct B199_W<'a> {
    w: &'a mut W,
}
impl<'a> B199_W<'a> {
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
#[doc = "Reader of field `B200`"]
pub type B200_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B200`"]
pub struct B200_W<'a> {
    w: &'a mut W,
}
impl<'a> B200_W<'a> {
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
#[doc = "Reader of field `B201`"]
pub type B201_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B201`"]
pub struct B201_W<'a> {
    w: &'a mut W,
}
impl<'a> B201_W<'a> {
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
#[doc = "Reader of field `B202`"]
pub type B202_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B202`"]
pub struct B202_W<'a> {
    w: &'a mut W,
}
impl<'a> B202_W<'a> {
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
#[doc = "Reader of field `B203`"]
pub type B203_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B203`"]
pub struct B203_W<'a> {
    w: &'a mut W,
}
impl<'a> B203_W<'a> {
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
#[doc = "Reader of field `B204`"]
pub type B204_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B204`"]
pub struct B204_W<'a> {
    w: &'a mut W,
}
impl<'a> B204_W<'a> {
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
#[doc = "Reader of field `B205`"]
pub type B205_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B205`"]
pub struct B205_W<'a> {
    w: &'a mut W,
}
impl<'a> B205_W<'a> {
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
#[doc = "Reader of field `B206`"]
pub type B206_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B206`"]
pub struct B206_W<'a> {
    w: &'a mut W,
}
impl<'a> B206_W<'a> {
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
#[doc = "Reader of field `B207`"]
pub type B207_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B207`"]
pub struct B207_W<'a> {
    w: &'a mut W,
}
impl<'a> B207_W<'a> {
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
#[doc = "Reader of field `B208`"]
pub type B208_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B208`"]
pub struct B208_W<'a> {
    w: &'a mut W,
}
impl<'a> B208_W<'a> {
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
#[doc = "Reader of field `B209`"]
pub type B209_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B209`"]
pub struct B209_W<'a> {
    w: &'a mut W,
}
impl<'a> B209_W<'a> {
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
#[doc = "Reader of field `B210`"]
pub type B210_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B210`"]
pub struct B210_W<'a> {
    w: &'a mut W,
}
impl<'a> B210_W<'a> {
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
#[doc = "Reader of field `B211`"]
pub type B211_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B211`"]
pub struct B211_W<'a> {
    w: &'a mut W,
}
impl<'a> B211_W<'a> {
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
#[doc = "Reader of field `B212`"]
pub type B212_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B212`"]
pub struct B212_W<'a> {
    w: &'a mut W,
}
impl<'a> B212_W<'a> {
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
#[doc = "Reader of field `B213`"]
pub type B213_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B213`"]
pub struct B213_W<'a> {
    w: &'a mut W,
}
impl<'a> B213_W<'a> {
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
#[doc = "Reader of field `B214`"]
pub type B214_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B214`"]
pub struct B214_W<'a> {
    w: &'a mut W,
}
impl<'a> B214_W<'a> {
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
#[doc = "Reader of field `B215`"]
pub type B215_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B215`"]
pub struct B215_W<'a> {
    w: &'a mut W,
}
impl<'a> B215_W<'a> {
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
#[doc = "Reader of field `B216`"]
pub type B216_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B216`"]
pub struct B216_W<'a> {
    w: &'a mut W,
}
impl<'a> B216_W<'a> {
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
#[doc = "Reader of field `B217`"]
pub type B217_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B217`"]
pub struct B217_W<'a> {
    w: &'a mut W,
}
impl<'a> B217_W<'a> {
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
#[doc = "Reader of field `B218`"]
pub type B218_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B218`"]
pub struct B218_W<'a> {
    w: &'a mut W,
}
impl<'a> B218_W<'a> {
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
#[doc = "Reader of field `B219`"]
pub type B219_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B219`"]
pub struct B219_W<'a> {
    w: &'a mut W,
}
impl<'a> B219_W<'a> {
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
#[doc = "Reader of field `B220`"]
pub type B220_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B220`"]
pub struct B220_W<'a> {
    w: &'a mut W,
}
impl<'a> B220_W<'a> {
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
#[doc = "Reader of field `B221`"]
pub type B221_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B221`"]
pub struct B221_W<'a> {
    w: &'a mut W,
}
impl<'a> B221_W<'a> {
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
#[doc = "Reader of field `B222`"]
pub type B222_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B222`"]
pub struct B222_W<'a> {
    w: &'a mut W,
}
impl<'a> B222_W<'a> {
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
#[doc = "Reader of field `B223`"]
pub type B223_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B223`"]
pub struct B223_W<'a> {
    w: &'a mut W,
}
impl<'a> B223_W<'a> {
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
    #[doc = "Bit 0 - B192"]
    #[inline(always)]
    pub fn b192(&self) -> B192_R {
        B192_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B193"]
    #[inline(always)]
    pub fn b193(&self) -> B193_R {
        B193_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B194"]
    #[inline(always)]
    pub fn b194(&self) -> B194_R {
        B194_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B195"]
    #[inline(always)]
    pub fn b195(&self) -> B195_R {
        B195_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B196"]
    #[inline(always)]
    pub fn b196(&self) -> B196_R {
        B196_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B197"]
    #[inline(always)]
    pub fn b197(&self) -> B197_R {
        B197_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B198"]
    #[inline(always)]
    pub fn b198(&self) -> B198_R {
        B198_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B199"]
    #[inline(always)]
    pub fn b199(&self) -> B199_R {
        B199_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B200"]
    #[inline(always)]
    pub fn b200(&self) -> B200_R {
        B200_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B201"]
    #[inline(always)]
    pub fn b201(&self) -> B201_R {
        B201_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B202"]
    #[inline(always)]
    pub fn b202(&self) -> B202_R {
        B202_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B203"]
    #[inline(always)]
    pub fn b203(&self) -> B203_R {
        B203_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B204"]
    #[inline(always)]
    pub fn b204(&self) -> B204_R {
        B204_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B205"]
    #[inline(always)]
    pub fn b205(&self) -> B205_R {
        B205_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B206"]
    #[inline(always)]
    pub fn b206(&self) -> B206_R {
        B206_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B207"]
    #[inline(always)]
    pub fn b207(&self) -> B207_R {
        B207_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B208"]
    #[inline(always)]
    pub fn b208(&self) -> B208_R {
        B208_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B209"]
    #[inline(always)]
    pub fn b209(&self) -> B209_R {
        B209_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B210"]
    #[inline(always)]
    pub fn b210(&self) -> B210_R {
        B210_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B211"]
    #[inline(always)]
    pub fn b211(&self) -> B211_R {
        B211_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B212"]
    #[inline(always)]
    pub fn b212(&self) -> B212_R {
        B212_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B213"]
    #[inline(always)]
    pub fn b213(&self) -> B213_R {
        B213_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B214"]
    #[inline(always)]
    pub fn b214(&self) -> B214_R {
        B214_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B215"]
    #[inline(always)]
    pub fn b215(&self) -> B215_R {
        B215_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B216"]
    #[inline(always)]
    pub fn b216(&self) -> B216_R {
        B216_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B217"]
    #[inline(always)]
    pub fn b217(&self) -> B217_R {
        B217_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B218"]
    #[inline(always)]
    pub fn b218(&self) -> B218_R {
        B218_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B219"]
    #[inline(always)]
    pub fn b219(&self) -> B219_R {
        B219_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B220"]
    #[inline(always)]
    pub fn b220(&self) -> B220_R {
        B220_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B221"]
    #[inline(always)]
    pub fn b221(&self) -> B221_R {
        B221_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B222"]
    #[inline(always)]
    pub fn b222(&self) -> B222_R {
        B222_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B223"]
    #[inline(always)]
    pub fn b223(&self) -> B223_R {
        B223_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B192"]
    #[inline(always)]
    pub fn b192(&mut self) -> B192_W {
        B192_W { w: self }
    }
    #[doc = "Bit 1 - B193"]
    #[inline(always)]
    pub fn b193(&mut self) -> B193_W {
        B193_W { w: self }
    }
    #[doc = "Bit 2 - B194"]
    #[inline(always)]
    pub fn b194(&mut self) -> B194_W {
        B194_W { w: self }
    }
    #[doc = "Bit 3 - B195"]
    #[inline(always)]
    pub fn b195(&mut self) -> B195_W {
        B195_W { w: self }
    }
    #[doc = "Bit 4 - B196"]
    #[inline(always)]
    pub fn b196(&mut self) -> B196_W {
        B196_W { w: self }
    }
    #[doc = "Bit 5 - B197"]
    #[inline(always)]
    pub fn b197(&mut self) -> B197_W {
        B197_W { w: self }
    }
    #[doc = "Bit 6 - B198"]
    #[inline(always)]
    pub fn b198(&mut self) -> B198_W {
        B198_W { w: self }
    }
    #[doc = "Bit 7 - B199"]
    #[inline(always)]
    pub fn b199(&mut self) -> B199_W {
        B199_W { w: self }
    }
    #[doc = "Bit 8 - B200"]
    #[inline(always)]
    pub fn b200(&mut self) -> B200_W {
        B200_W { w: self }
    }
    #[doc = "Bit 9 - B201"]
    #[inline(always)]
    pub fn b201(&mut self) -> B201_W {
        B201_W { w: self }
    }
    #[doc = "Bit 10 - B202"]
    #[inline(always)]
    pub fn b202(&mut self) -> B202_W {
        B202_W { w: self }
    }
    #[doc = "Bit 11 - B203"]
    #[inline(always)]
    pub fn b203(&mut self) -> B203_W {
        B203_W { w: self }
    }
    #[doc = "Bit 12 - B204"]
    #[inline(always)]
    pub fn b204(&mut self) -> B204_W {
        B204_W { w: self }
    }
    #[doc = "Bit 13 - B205"]
    #[inline(always)]
    pub fn b205(&mut self) -> B205_W {
        B205_W { w: self }
    }
    #[doc = "Bit 14 - B206"]
    #[inline(always)]
    pub fn b206(&mut self) -> B206_W {
        B206_W { w: self }
    }
    #[doc = "Bit 15 - B207"]
    #[inline(always)]
    pub fn b207(&mut self) -> B207_W {
        B207_W { w: self }
    }
    #[doc = "Bit 16 - B208"]
    #[inline(always)]
    pub fn b208(&mut self) -> B208_W {
        B208_W { w: self }
    }
    #[doc = "Bit 17 - B209"]
    #[inline(always)]
    pub fn b209(&mut self) -> B209_W {
        B209_W { w: self }
    }
    #[doc = "Bit 18 - B210"]
    #[inline(always)]
    pub fn b210(&mut self) -> B210_W {
        B210_W { w: self }
    }
    #[doc = "Bit 19 - B211"]
    #[inline(always)]
    pub fn b211(&mut self) -> B211_W {
        B211_W { w: self }
    }
    #[doc = "Bit 20 - B212"]
    #[inline(always)]
    pub fn b212(&mut self) -> B212_W {
        B212_W { w: self }
    }
    #[doc = "Bit 21 - B213"]
    #[inline(always)]
    pub fn b213(&mut self) -> B213_W {
        B213_W { w: self }
    }
    #[doc = "Bit 22 - B214"]
    #[inline(always)]
    pub fn b214(&mut self) -> B214_W {
        B214_W { w: self }
    }
    #[doc = "Bit 23 - B215"]
    #[inline(always)]
    pub fn b215(&mut self) -> B215_W {
        B215_W { w: self }
    }
    #[doc = "Bit 24 - B216"]
    #[inline(always)]
    pub fn b216(&mut self) -> B216_W {
        B216_W { w: self }
    }
    #[doc = "Bit 25 - B217"]
    #[inline(always)]
    pub fn b217(&mut self) -> B217_W {
        B217_W { w: self }
    }
    #[doc = "Bit 26 - B218"]
    #[inline(always)]
    pub fn b218(&mut self) -> B218_W {
        B218_W { w: self }
    }
    #[doc = "Bit 27 - B219"]
    #[inline(always)]
    pub fn b219(&mut self) -> B219_W {
        B219_W { w: self }
    }
    #[doc = "Bit 28 - B220"]
    #[inline(always)]
    pub fn b220(&mut self) -> B220_W {
        B220_W { w: self }
    }
    #[doc = "Bit 29 - B221"]
    #[inline(always)]
    pub fn b221(&mut self) -> B221_W {
        B221_W { w: self }
    }
    #[doc = "Bit 30 - B222"]
    #[inline(always)]
    pub fn b222(&mut self) -> B222_W {
        B222_W { w: self }
    }
    #[doc = "Bit 31 - B223"]
    #[inline(always)]
    pub fn b223(&mut self) -> B223_W {
        B223_W { w: self }
    }
}
