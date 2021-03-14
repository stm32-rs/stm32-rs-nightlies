#[doc = "Reader of register MPCBB1_VCTR7"]
pub type R = crate::R<u32, super::MPCBB1_VCTR7>;
#[doc = "Writer for register MPCBB1_VCTR7"]
pub type W = crate::W<u32, super::MPCBB1_VCTR7>;
#[doc = "Register MPCBB1_VCTR7 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB1_VCTR7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B224`"]
pub type B224_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B224`"]
pub struct B224_W<'a> {
    w: &'a mut W,
}
impl<'a> B224_W<'a> {
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
#[doc = "Reader of field `B225`"]
pub type B225_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B225`"]
pub struct B225_W<'a> {
    w: &'a mut W,
}
impl<'a> B225_W<'a> {
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
#[doc = "Reader of field `B226`"]
pub type B226_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B226`"]
pub struct B226_W<'a> {
    w: &'a mut W,
}
impl<'a> B226_W<'a> {
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
#[doc = "Reader of field `B227`"]
pub type B227_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B227`"]
pub struct B227_W<'a> {
    w: &'a mut W,
}
impl<'a> B227_W<'a> {
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
#[doc = "Reader of field `B228`"]
pub type B228_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B228`"]
pub struct B228_W<'a> {
    w: &'a mut W,
}
impl<'a> B228_W<'a> {
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
#[doc = "Reader of field `B229`"]
pub type B229_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B229`"]
pub struct B229_W<'a> {
    w: &'a mut W,
}
impl<'a> B229_W<'a> {
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
#[doc = "Reader of field `B230`"]
pub type B230_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B230`"]
pub struct B230_W<'a> {
    w: &'a mut W,
}
impl<'a> B230_W<'a> {
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
#[doc = "Reader of field `B231`"]
pub type B231_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B231`"]
pub struct B231_W<'a> {
    w: &'a mut W,
}
impl<'a> B231_W<'a> {
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
#[doc = "Reader of field `B232`"]
pub type B232_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B232`"]
pub struct B232_W<'a> {
    w: &'a mut W,
}
impl<'a> B232_W<'a> {
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
#[doc = "Reader of field `B233`"]
pub type B233_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B233`"]
pub struct B233_W<'a> {
    w: &'a mut W,
}
impl<'a> B233_W<'a> {
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
#[doc = "Reader of field `B234`"]
pub type B234_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B234`"]
pub struct B234_W<'a> {
    w: &'a mut W,
}
impl<'a> B234_W<'a> {
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
#[doc = "Reader of field `B235`"]
pub type B235_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B235`"]
pub struct B235_W<'a> {
    w: &'a mut W,
}
impl<'a> B235_W<'a> {
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
#[doc = "Reader of field `B236`"]
pub type B236_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B236`"]
pub struct B236_W<'a> {
    w: &'a mut W,
}
impl<'a> B236_W<'a> {
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
#[doc = "Reader of field `B237`"]
pub type B237_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B237`"]
pub struct B237_W<'a> {
    w: &'a mut W,
}
impl<'a> B237_W<'a> {
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
#[doc = "Reader of field `B238`"]
pub type B238_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B238`"]
pub struct B238_W<'a> {
    w: &'a mut W,
}
impl<'a> B238_W<'a> {
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
#[doc = "Reader of field `B239`"]
pub type B239_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B239`"]
pub struct B239_W<'a> {
    w: &'a mut W,
}
impl<'a> B239_W<'a> {
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
#[doc = "Reader of field `B240`"]
pub type B240_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B240`"]
pub struct B240_W<'a> {
    w: &'a mut W,
}
impl<'a> B240_W<'a> {
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
#[doc = "Reader of field `B241`"]
pub type B241_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B241`"]
pub struct B241_W<'a> {
    w: &'a mut W,
}
impl<'a> B241_W<'a> {
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
#[doc = "Reader of field `B242`"]
pub type B242_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B242`"]
pub struct B242_W<'a> {
    w: &'a mut W,
}
impl<'a> B242_W<'a> {
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
#[doc = "Reader of field `B243`"]
pub type B243_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B243`"]
pub struct B243_W<'a> {
    w: &'a mut W,
}
impl<'a> B243_W<'a> {
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
#[doc = "Reader of field `B244`"]
pub type B244_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B244`"]
pub struct B244_W<'a> {
    w: &'a mut W,
}
impl<'a> B244_W<'a> {
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
#[doc = "Reader of field `B245`"]
pub type B245_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B245`"]
pub struct B245_W<'a> {
    w: &'a mut W,
}
impl<'a> B245_W<'a> {
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
#[doc = "Reader of field `B246`"]
pub type B246_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B246`"]
pub struct B246_W<'a> {
    w: &'a mut W,
}
impl<'a> B246_W<'a> {
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
#[doc = "Reader of field `B247`"]
pub type B247_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B247`"]
pub struct B247_W<'a> {
    w: &'a mut W,
}
impl<'a> B247_W<'a> {
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
#[doc = "Reader of field `B248`"]
pub type B248_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B248`"]
pub struct B248_W<'a> {
    w: &'a mut W,
}
impl<'a> B248_W<'a> {
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
#[doc = "Reader of field `B249`"]
pub type B249_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B249`"]
pub struct B249_W<'a> {
    w: &'a mut W,
}
impl<'a> B249_W<'a> {
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
#[doc = "Reader of field `B250`"]
pub type B250_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B250`"]
pub struct B250_W<'a> {
    w: &'a mut W,
}
impl<'a> B250_W<'a> {
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
#[doc = "Reader of field `B251`"]
pub type B251_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B251`"]
pub struct B251_W<'a> {
    w: &'a mut W,
}
impl<'a> B251_W<'a> {
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
#[doc = "Reader of field `B252`"]
pub type B252_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B252`"]
pub struct B252_W<'a> {
    w: &'a mut W,
}
impl<'a> B252_W<'a> {
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
#[doc = "Reader of field `B253`"]
pub type B253_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B253`"]
pub struct B253_W<'a> {
    w: &'a mut W,
}
impl<'a> B253_W<'a> {
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
#[doc = "Reader of field `B254`"]
pub type B254_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B254`"]
pub struct B254_W<'a> {
    w: &'a mut W,
}
impl<'a> B254_W<'a> {
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
#[doc = "Reader of field `B255`"]
pub type B255_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B255`"]
pub struct B255_W<'a> {
    w: &'a mut W,
}
impl<'a> B255_W<'a> {
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
    #[doc = "Bit 0 - B224"]
    #[inline(always)]
    pub fn b224(&self) -> B224_R {
        B224_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B225"]
    #[inline(always)]
    pub fn b225(&self) -> B225_R {
        B225_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B226"]
    #[inline(always)]
    pub fn b226(&self) -> B226_R {
        B226_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B227"]
    #[inline(always)]
    pub fn b227(&self) -> B227_R {
        B227_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B228"]
    #[inline(always)]
    pub fn b228(&self) -> B228_R {
        B228_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B229"]
    #[inline(always)]
    pub fn b229(&self) -> B229_R {
        B229_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B230"]
    #[inline(always)]
    pub fn b230(&self) -> B230_R {
        B230_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B231"]
    #[inline(always)]
    pub fn b231(&self) -> B231_R {
        B231_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B232"]
    #[inline(always)]
    pub fn b232(&self) -> B232_R {
        B232_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B233"]
    #[inline(always)]
    pub fn b233(&self) -> B233_R {
        B233_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B234"]
    #[inline(always)]
    pub fn b234(&self) -> B234_R {
        B234_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B235"]
    #[inline(always)]
    pub fn b235(&self) -> B235_R {
        B235_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B236"]
    #[inline(always)]
    pub fn b236(&self) -> B236_R {
        B236_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B237"]
    #[inline(always)]
    pub fn b237(&self) -> B237_R {
        B237_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B238"]
    #[inline(always)]
    pub fn b238(&self) -> B238_R {
        B238_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B239"]
    #[inline(always)]
    pub fn b239(&self) -> B239_R {
        B239_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B240"]
    #[inline(always)]
    pub fn b240(&self) -> B240_R {
        B240_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B241"]
    #[inline(always)]
    pub fn b241(&self) -> B241_R {
        B241_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B242"]
    #[inline(always)]
    pub fn b242(&self) -> B242_R {
        B242_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B243"]
    #[inline(always)]
    pub fn b243(&self) -> B243_R {
        B243_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B244"]
    #[inline(always)]
    pub fn b244(&self) -> B244_R {
        B244_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B245"]
    #[inline(always)]
    pub fn b245(&self) -> B245_R {
        B245_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B246"]
    #[inline(always)]
    pub fn b246(&self) -> B246_R {
        B246_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B247"]
    #[inline(always)]
    pub fn b247(&self) -> B247_R {
        B247_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B248"]
    #[inline(always)]
    pub fn b248(&self) -> B248_R {
        B248_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B249"]
    #[inline(always)]
    pub fn b249(&self) -> B249_R {
        B249_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B250"]
    #[inline(always)]
    pub fn b250(&self) -> B250_R {
        B250_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B251"]
    #[inline(always)]
    pub fn b251(&self) -> B251_R {
        B251_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B252"]
    #[inline(always)]
    pub fn b252(&self) -> B252_R {
        B252_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B253"]
    #[inline(always)]
    pub fn b253(&self) -> B253_R {
        B253_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B254"]
    #[inline(always)]
    pub fn b254(&self) -> B254_R {
        B254_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B255"]
    #[inline(always)]
    pub fn b255(&self) -> B255_R {
        B255_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B224"]
    #[inline(always)]
    pub fn b224(&mut self) -> B224_W {
        B224_W { w: self }
    }
    #[doc = "Bit 1 - B225"]
    #[inline(always)]
    pub fn b225(&mut self) -> B225_W {
        B225_W { w: self }
    }
    #[doc = "Bit 2 - B226"]
    #[inline(always)]
    pub fn b226(&mut self) -> B226_W {
        B226_W { w: self }
    }
    #[doc = "Bit 3 - B227"]
    #[inline(always)]
    pub fn b227(&mut self) -> B227_W {
        B227_W { w: self }
    }
    #[doc = "Bit 4 - B228"]
    #[inline(always)]
    pub fn b228(&mut self) -> B228_W {
        B228_W { w: self }
    }
    #[doc = "Bit 5 - B229"]
    #[inline(always)]
    pub fn b229(&mut self) -> B229_W {
        B229_W { w: self }
    }
    #[doc = "Bit 6 - B230"]
    #[inline(always)]
    pub fn b230(&mut self) -> B230_W {
        B230_W { w: self }
    }
    #[doc = "Bit 7 - B231"]
    #[inline(always)]
    pub fn b231(&mut self) -> B231_W {
        B231_W { w: self }
    }
    #[doc = "Bit 8 - B232"]
    #[inline(always)]
    pub fn b232(&mut self) -> B232_W {
        B232_W { w: self }
    }
    #[doc = "Bit 9 - B233"]
    #[inline(always)]
    pub fn b233(&mut self) -> B233_W {
        B233_W { w: self }
    }
    #[doc = "Bit 10 - B234"]
    #[inline(always)]
    pub fn b234(&mut self) -> B234_W {
        B234_W { w: self }
    }
    #[doc = "Bit 11 - B235"]
    #[inline(always)]
    pub fn b235(&mut self) -> B235_W {
        B235_W { w: self }
    }
    #[doc = "Bit 12 - B236"]
    #[inline(always)]
    pub fn b236(&mut self) -> B236_W {
        B236_W { w: self }
    }
    #[doc = "Bit 13 - B237"]
    #[inline(always)]
    pub fn b237(&mut self) -> B237_W {
        B237_W { w: self }
    }
    #[doc = "Bit 14 - B238"]
    #[inline(always)]
    pub fn b238(&mut self) -> B238_W {
        B238_W { w: self }
    }
    #[doc = "Bit 15 - B239"]
    #[inline(always)]
    pub fn b239(&mut self) -> B239_W {
        B239_W { w: self }
    }
    #[doc = "Bit 16 - B240"]
    #[inline(always)]
    pub fn b240(&mut self) -> B240_W {
        B240_W { w: self }
    }
    #[doc = "Bit 17 - B241"]
    #[inline(always)]
    pub fn b241(&mut self) -> B241_W {
        B241_W { w: self }
    }
    #[doc = "Bit 18 - B242"]
    #[inline(always)]
    pub fn b242(&mut self) -> B242_W {
        B242_W { w: self }
    }
    #[doc = "Bit 19 - B243"]
    #[inline(always)]
    pub fn b243(&mut self) -> B243_W {
        B243_W { w: self }
    }
    #[doc = "Bit 20 - B244"]
    #[inline(always)]
    pub fn b244(&mut self) -> B244_W {
        B244_W { w: self }
    }
    #[doc = "Bit 21 - B245"]
    #[inline(always)]
    pub fn b245(&mut self) -> B245_W {
        B245_W { w: self }
    }
    #[doc = "Bit 22 - B246"]
    #[inline(always)]
    pub fn b246(&mut self) -> B246_W {
        B246_W { w: self }
    }
    #[doc = "Bit 23 - B247"]
    #[inline(always)]
    pub fn b247(&mut self) -> B247_W {
        B247_W { w: self }
    }
    #[doc = "Bit 24 - B248"]
    #[inline(always)]
    pub fn b248(&mut self) -> B248_W {
        B248_W { w: self }
    }
    #[doc = "Bit 25 - B249"]
    #[inline(always)]
    pub fn b249(&mut self) -> B249_W {
        B249_W { w: self }
    }
    #[doc = "Bit 26 - B250"]
    #[inline(always)]
    pub fn b250(&mut self) -> B250_W {
        B250_W { w: self }
    }
    #[doc = "Bit 27 - B251"]
    #[inline(always)]
    pub fn b251(&mut self) -> B251_W {
        B251_W { w: self }
    }
    #[doc = "Bit 28 - B252"]
    #[inline(always)]
    pub fn b252(&mut self) -> B252_W {
        B252_W { w: self }
    }
    #[doc = "Bit 29 - B253"]
    #[inline(always)]
    pub fn b253(&mut self) -> B253_W {
        B253_W { w: self }
    }
    #[doc = "Bit 30 - B254"]
    #[inline(always)]
    pub fn b254(&mut self) -> B254_W {
        B254_W { w: self }
    }
    #[doc = "Bit 31 - B255"]
    #[inline(always)]
    pub fn b255(&mut self) -> B255_W {
        B255_W { w: self }
    }
}
