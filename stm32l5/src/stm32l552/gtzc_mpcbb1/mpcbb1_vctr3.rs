#[doc = "Reader of register MPCBB1_VCTR3"]
pub type R = crate::R<u32, super::MPCBB1_VCTR3>;
#[doc = "Writer for register MPCBB1_VCTR3"]
pub type W = crate::W<u32, super::MPCBB1_VCTR3>;
#[doc = "Register MPCBB1_VCTR3 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB1_VCTR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B96`"]
pub type B96_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B96`"]
pub struct B96_W<'a> {
    w: &'a mut W,
}
impl<'a> B96_W<'a> {
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
#[doc = "Reader of field `B97`"]
pub type B97_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B97`"]
pub struct B97_W<'a> {
    w: &'a mut W,
}
impl<'a> B97_W<'a> {
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
#[doc = "Reader of field `B98`"]
pub type B98_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B98`"]
pub struct B98_W<'a> {
    w: &'a mut W,
}
impl<'a> B98_W<'a> {
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
#[doc = "Reader of field `B99`"]
pub type B99_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B99`"]
pub struct B99_W<'a> {
    w: &'a mut W,
}
impl<'a> B99_W<'a> {
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
#[doc = "Reader of field `B100`"]
pub type B100_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B100`"]
pub struct B100_W<'a> {
    w: &'a mut W,
}
impl<'a> B100_W<'a> {
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
#[doc = "Reader of field `B101`"]
pub type B101_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B101`"]
pub struct B101_W<'a> {
    w: &'a mut W,
}
impl<'a> B101_W<'a> {
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
#[doc = "Reader of field `B102`"]
pub type B102_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B102`"]
pub struct B102_W<'a> {
    w: &'a mut W,
}
impl<'a> B102_W<'a> {
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
#[doc = "Reader of field `B103`"]
pub type B103_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B103`"]
pub struct B103_W<'a> {
    w: &'a mut W,
}
impl<'a> B103_W<'a> {
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
#[doc = "Reader of field `B104`"]
pub type B104_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B104`"]
pub struct B104_W<'a> {
    w: &'a mut W,
}
impl<'a> B104_W<'a> {
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
#[doc = "Reader of field `B105`"]
pub type B105_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B105`"]
pub struct B105_W<'a> {
    w: &'a mut W,
}
impl<'a> B105_W<'a> {
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
#[doc = "Reader of field `B106`"]
pub type B106_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B106`"]
pub struct B106_W<'a> {
    w: &'a mut W,
}
impl<'a> B106_W<'a> {
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
#[doc = "Reader of field `B107`"]
pub type B107_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B107`"]
pub struct B107_W<'a> {
    w: &'a mut W,
}
impl<'a> B107_W<'a> {
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
#[doc = "Reader of field `B108`"]
pub type B108_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B108`"]
pub struct B108_W<'a> {
    w: &'a mut W,
}
impl<'a> B108_W<'a> {
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
#[doc = "Reader of field `B109`"]
pub type B109_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B109`"]
pub struct B109_W<'a> {
    w: &'a mut W,
}
impl<'a> B109_W<'a> {
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
#[doc = "Reader of field `B110`"]
pub type B110_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B110`"]
pub struct B110_W<'a> {
    w: &'a mut W,
}
impl<'a> B110_W<'a> {
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
#[doc = "Reader of field `B111`"]
pub type B111_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B111`"]
pub struct B111_W<'a> {
    w: &'a mut W,
}
impl<'a> B111_W<'a> {
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
#[doc = "Reader of field `B112`"]
pub type B112_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B112`"]
pub struct B112_W<'a> {
    w: &'a mut W,
}
impl<'a> B112_W<'a> {
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
#[doc = "Reader of field `B113`"]
pub type B113_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B113`"]
pub struct B113_W<'a> {
    w: &'a mut W,
}
impl<'a> B113_W<'a> {
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
#[doc = "Reader of field `B114`"]
pub type B114_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B114`"]
pub struct B114_W<'a> {
    w: &'a mut W,
}
impl<'a> B114_W<'a> {
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
#[doc = "Reader of field `B115`"]
pub type B115_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B115`"]
pub struct B115_W<'a> {
    w: &'a mut W,
}
impl<'a> B115_W<'a> {
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
#[doc = "Reader of field `B116`"]
pub type B116_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B116`"]
pub struct B116_W<'a> {
    w: &'a mut W,
}
impl<'a> B116_W<'a> {
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
#[doc = "Reader of field `B117`"]
pub type B117_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B117`"]
pub struct B117_W<'a> {
    w: &'a mut W,
}
impl<'a> B117_W<'a> {
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
#[doc = "Reader of field `B118`"]
pub type B118_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B118`"]
pub struct B118_W<'a> {
    w: &'a mut W,
}
impl<'a> B118_W<'a> {
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
#[doc = "Reader of field `B119`"]
pub type B119_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B119`"]
pub struct B119_W<'a> {
    w: &'a mut W,
}
impl<'a> B119_W<'a> {
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
#[doc = "Reader of field `B120`"]
pub type B120_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B120`"]
pub struct B120_W<'a> {
    w: &'a mut W,
}
impl<'a> B120_W<'a> {
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
#[doc = "Reader of field `B121`"]
pub type B121_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B121`"]
pub struct B121_W<'a> {
    w: &'a mut W,
}
impl<'a> B121_W<'a> {
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
#[doc = "Reader of field `B122`"]
pub type B122_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B122`"]
pub struct B122_W<'a> {
    w: &'a mut W,
}
impl<'a> B122_W<'a> {
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
#[doc = "Reader of field `B123`"]
pub type B123_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B123`"]
pub struct B123_W<'a> {
    w: &'a mut W,
}
impl<'a> B123_W<'a> {
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
#[doc = "Reader of field `B124`"]
pub type B124_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B124`"]
pub struct B124_W<'a> {
    w: &'a mut W,
}
impl<'a> B124_W<'a> {
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
#[doc = "Reader of field `B125`"]
pub type B125_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B125`"]
pub struct B125_W<'a> {
    w: &'a mut W,
}
impl<'a> B125_W<'a> {
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
#[doc = "Reader of field `B126`"]
pub type B126_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B126`"]
pub struct B126_W<'a> {
    w: &'a mut W,
}
impl<'a> B126_W<'a> {
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
#[doc = "Reader of field `B127`"]
pub type B127_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B127`"]
pub struct B127_W<'a> {
    w: &'a mut W,
}
impl<'a> B127_W<'a> {
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
    #[doc = "Bit 0 - B96"]
    #[inline(always)]
    pub fn b96(&self) -> B96_R {
        B96_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B97"]
    #[inline(always)]
    pub fn b97(&self) -> B97_R {
        B97_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B98"]
    #[inline(always)]
    pub fn b98(&self) -> B98_R {
        B98_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B99"]
    #[inline(always)]
    pub fn b99(&self) -> B99_R {
        B99_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B100"]
    #[inline(always)]
    pub fn b100(&self) -> B100_R {
        B100_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B101"]
    #[inline(always)]
    pub fn b101(&self) -> B101_R {
        B101_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B102"]
    #[inline(always)]
    pub fn b102(&self) -> B102_R {
        B102_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B103"]
    #[inline(always)]
    pub fn b103(&self) -> B103_R {
        B103_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B104"]
    #[inline(always)]
    pub fn b104(&self) -> B104_R {
        B104_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B105"]
    #[inline(always)]
    pub fn b105(&self) -> B105_R {
        B105_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B106"]
    #[inline(always)]
    pub fn b106(&self) -> B106_R {
        B106_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B107"]
    #[inline(always)]
    pub fn b107(&self) -> B107_R {
        B107_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B108"]
    #[inline(always)]
    pub fn b108(&self) -> B108_R {
        B108_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B109"]
    #[inline(always)]
    pub fn b109(&self) -> B109_R {
        B109_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B110"]
    #[inline(always)]
    pub fn b110(&self) -> B110_R {
        B110_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B111"]
    #[inline(always)]
    pub fn b111(&self) -> B111_R {
        B111_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B112"]
    #[inline(always)]
    pub fn b112(&self) -> B112_R {
        B112_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B113"]
    #[inline(always)]
    pub fn b113(&self) -> B113_R {
        B113_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B114"]
    #[inline(always)]
    pub fn b114(&self) -> B114_R {
        B114_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B115"]
    #[inline(always)]
    pub fn b115(&self) -> B115_R {
        B115_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B116"]
    #[inline(always)]
    pub fn b116(&self) -> B116_R {
        B116_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B117"]
    #[inline(always)]
    pub fn b117(&self) -> B117_R {
        B117_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B118"]
    #[inline(always)]
    pub fn b118(&self) -> B118_R {
        B118_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B119"]
    #[inline(always)]
    pub fn b119(&self) -> B119_R {
        B119_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B120"]
    #[inline(always)]
    pub fn b120(&self) -> B120_R {
        B120_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B121"]
    #[inline(always)]
    pub fn b121(&self) -> B121_R {
        B121_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B122"]
    #[inline(always)]
    pub fn b122(&self) -> B122_R {
        B122_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B123"]
    #[inline(always)]
    pub fn b123(&self) -> B123_R {
        B123_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B124"]
    #[inline(always)]
    pub fn b124(&self) -> B124_R {
        B124_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B125"]
    #[inline(always)]
    pub fn b125(&self) -> B125_R {
        B125_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B126"]
    #[inline(always)]
    pub fn b126(&self) -> B126_R {
        B126_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B127"]
    #[inline(always)]
    pub fn b127(&self) -> B127_R {
        B127_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B96"]
    #[inline(always)]
    pub fn b96(&mut self) -> B96_W {
        B96_W { w: self }
    }
    #[doc = "Bit 1 - B97"]
    #[inline(always)]
    pub fn b97(&mut self) -> B97_W {
        B97_W { w: self }
    }
    #[doc = "Bit 2 - B98"]
    #[inline(always)]
    pub fn b98(&mut self) -> B98_W {
        B98_W { w: self }
    }
    #[doc = "Bit 3 - B99"]
    #[inline(always)]
    pub fn b99(&mut self) -> B99_W {
        B99_W { w: self }
    }
    #[doc = "Bit 4 - B100"]
    #[inline(always)]
    pub fn b100(&mut self) -> B100_W {
        B100_W { w: self }
    }
    #[doc = "Bit 5 - B101"]
    #[inline(always)]
    pub fn b101(&mut self) -> B101_W {
        B101_W { w: self }
    }
    #[doc = "Bit 6 - B102"]
    #[inline(always)]
    pub fn b102(&mut self) -> B102_W {
        B102_W { w: self }
    }
    #[doc = "Bit 7 - B103"]
    #[inline(always)]
    pub fn b103(&mut self) -> B103_W {
        B103_W { w: self }
    }
    #[doc = "Bit 8 - B104"]
    #[inline(always)]
    pub fn b104(&mut self) -> B104_W {
        B104_W { w: self }
    }
    #[doc = "Bit 9 - B105"]
    #[inline(always)]
    pub fn b105(&mut self) -> B105_W {
        B105_W { w: self }
    }
    #[doc = "Bit 10 - B106"]
    #[inline(always)]
    pub fn b106(&mut self) -> B106_W {
        B106_W { w: self }
    }
    #[doc = "Bit 11 - B107"]
    #[inline(always)]
    pub fn b107(&mut self) -> B107_W {
        B107_W { w: self }
    }
    #[doc = "Bit 12 - B108"]
    #[inline(always)]
    pub fn b108(&mut self) -> B108_W {
        B108_W { w: self }
    }
    #[doc = "Bit 13 - B109"]
    #[inline(always)]
    pub fn b109(&mut self) -> B109_W {
        B109_W { w: self }
    }
    #[doc = "Bit 14 - B110"]
    #[inline(always)]
    pub fn b110(&mut self) -> B110_W {
        B110_W { w: self }
    }
    #[doc = "Bit 15 - B111"]
    #[inline(always)]
    pub fn b111(&mut self) -> B111_W {
        B111_W { w: self }
    }
    #[doc = "Bit 16 - B112"]
    #[inline(always)]
    pub fn b112(&mut self) -> B112_W {
        B112_W { w: self }
    }
    #[doc = "Bit 17 - B113"]
    #[inline(always)]
    pub fn b113(&mut self) -> B113_W {
        B113_W { w: self }
    }
    #[doc = "Bit 18 - B114"]
    #[inline(always)]
    pub fn b114(&mut self) -> B114_W {
        B114_W { w: self }
    }
    #[doc = "Bit 19 - B115"]
    #[inline(always)]
    pub fn b115(&mut self) -> B115_W {
        B115_W { w: self }
    }
    #[doc = "Bit 20 - B116"]
    #[inline(always)]
    pub fn b116(&mut self) -> B116_W {
        B116_W { w: self }
    }
    #[doc = "Bit 21 - B117"]
    #[inline(always)]
    pub fn b117(&mut self) -> B117_W {
        B117_W { w: self }
    }
    #[doc = "Bit 22 - B118"]
    #[inline(always)]
    pub fn b118(&mut self) -> B118_W {
        B118_W { w: self }
    }
    #[doc = "Bit 23 - B119"]
    #[inline(always)]
    pub fn b119(&mut self) -> B119_W {
        B119_W { w: self }
    }
    #[doc = "Bit 24 - B120"]
    #[inline(always)]
    pub fn b120(&mut self) -> B120_W {
        B120_W { w: self }
    }
    #[doc = "Bit 25 - B121"]
    #[inline(always)]
    pub fn b121(&mut self) -> B121_W {
        B121_W { w: self }
    }
    #[doc = "Bit 26 - B122"]
    #[inline(always)]
    pub fn b122(&mut self) -> B122_W {
        B122_W { w: self }
    }
    #[doc = "Bit 27 - B123"]
    #[inline(always)]
    pub fn b123(&mut self) -> B123_W {
        B123_W { w: self }
    }
    #[doc = "Bit 28 - B124"]
    #[inline(always)]
    pub fn b124(&mut self) -> B124_W {
        B124_W { w: self }
    }
    #[doc = "Bit 29 - B125"]
    #[inline(always)]
    pub fn b125(&mut self) -> B125_W {
        B125_W { w: self }
    }
    #[doc = "Bit 30 - B126"]
    #[inline(always)]
    pub fn b126(&mut self) -> B126_W {
        B126_W { w: self }
    }
    #[doc = "Bit 31 - B127"]
    #[inline(always)]
    pub fn b127(&mut self) -> B127_W {
        B127_W { w: self }
    }
}
