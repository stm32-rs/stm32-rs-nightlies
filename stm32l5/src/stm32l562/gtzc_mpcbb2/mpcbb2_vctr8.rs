#[doc = "Reader of register MPCBB2_VCTR8"]
pub type R = crate::R<u32, super::MPCBB2_VCTR8>;
#[doc = "Writer for register MPCBB2_VCTR8"]
pub type W = crate::W<u32, super::MPCBB2_VCTR8>;
#[doc = "Register MPCBB2_VCTR8 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB2_VCTR8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B256`"]
pub type B256_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B256`"]
pub struct B256_W<'a> {
    w: &'a mut W,
}
impl<'a> B256_W<'a> {
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
#[doc = "Reader of field `B257`"]
pub type B257_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B257`"]
pub struct B257_W<'a> {
    w: &'a mut W,
}
impl<'a> B257_W<'a> {
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
#[doc = "Reader of field `B258`"]
pub type B258_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B258`"]
pub struct B258_W<'a> {
    w: &'a mut W,
}
impl<'a> B258_W<'a> {
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
#[doc = "Reader of field `B259`"]
pub type B259_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B259`"]
pub struct B259_W<'a> {
    w: &'a mut W,
}
impl<'a> B259_W<'a> {
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
#[doc = "Reader of field `B260`"]
pub type B260_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B260`"]
pub struct B260_W<'a> {
    w: &'a mut W,
}
impl<'a> B260_W<'a> {
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
#[doc = "Reader of field `B261`"]
pub type B261_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B261`"]
pub struct B261_W<'a> {
    w: &'a mut W,
}
impl<'a> B261_W<'a> {
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
#[doc = "Reader of field `B262`"]
pub type B262_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B262`"]
pub struct B262_W<'a> {
    w: &'a mut W,
}
impl<'a> B262_W<'a> {
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
#[doc = "Reader of field `B263`"]
pub type B263_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B263`"]
pub struct B263_W<'a> {
    w: &'a mut W,
}
impl<'a> B263_W<'a> {
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
#[doc = "Reader of field `B264`"]
pub type B264_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B264`"]
pub struct B264_W<'a> {
    w: &'a mut W,
}
impl<'a> B264_W<'a> {
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
#[doc = "Reader of field `B265`"]
pub type B265_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B265`"]
pub struct B265_W<'a> {
    w: &'a mut W,
}
impl<'a> B265_W<'a> {
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
#[doc = "Reader of field `B266`"]
pub type B266_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B266`"]
pub struct B266_W<'a> {
    w: &'a mut W,
}
impl<'a> B266_W<'a> {
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
#[doc = "Reader of field `B267`"]
pub type B267_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B267`"]
pub struct B267_W<'a> {
    w: &'a mut W,
}
impl<'a> B267_W<'a> {
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
#[doc = "Reader of field `B268`"]
pub type B268_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B268`"]
pub struct B268_W<'a> {
    w: &'a mut W,
}
impl<'a> B268_W<'a> {
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
#[doc = "Reader of field `B269`"]
pub type B269_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B269`"]
pub struct B269_W<'a> {
    w: &'a mut W,
}
impl<'a> B269_W<'a> {
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
#[doc = "Reader of field `B270`"]
pub type B270_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B270`"]
pub struct B270_W<'a> {
    w: &'a mut W,
}
impl<'a> B270_W<'a> {
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
#[doc = "Reader of field `B271`"]
pub type B271_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B271`"]
pub struct B271_W<'a> {
    w: &'a mut W,
}
impl<'a> B271_W<'a> {
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
#[doc = "Reader of field `B272`"]
pub type B272_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B272`"]
pub struct B272_W<'a> {
    w: &'a mut W,
}
impl<'a> B272_W<'a> {
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
#[doc = "Reader of field `B273`"]
pub type B273_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B273`"]
pub struct B273_W<'a> {
    w: &'a mut W,
}
impl<'a> B273_W<'a> {
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
#[doc = "Reader of field `B274`"]
pub type B274_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B274`"]
pub struct B274_W<'a> {
    w: &'a mut W,
}
impl<'a> B274_W<'a> {
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
#[doc = "Reader of field `B275`"]
pub type B275_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B275`"]
pub struct B275_W<'a> {
    w: &'a mut W,
}
impl<'a> B275_W<'a> {
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
#[doc = "Reader of field `B276`"]
pub type B276_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B276`"]
pub struct B276_W<'a> {
    w: &'a mut W,
}
impl<'a> B276_W<'a> {
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
#[doc = "Reader of field `B277`"]
pub type B277_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B277`"]
pub struct B277_W<'a> {
    w: &'a mut W,
}
impl<'a> B277_W<'a> {
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
#[doc = "Reader of field `B278`"]
pub type B278_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B278`"]
pub struct B278_W<'a> {
    w: &'a mut W,
}
impl<'a> B278_W<'a> {
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
#[doc = "Reader of field `B279`"]
pub type B279_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B279`"]
pub struct B279_W<'a> {
    w: &'a mut W,
}
impl<'a> B279_W<'a> {
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
#[doc = "Reader of field `B280`"]
pub type B280_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B280`"]
pub struct B280_W<'a> {
    w: &'a mut W,
}
impl<'a> B280_W<'a> {
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
#[doc = "Reader of field `B281`"]
pub type B281_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B281`"]
pub struct B281_W<'a> {
    w: &'a mut W,
}
impl<'a> B281_W<'a> {
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
#[doc = "Reader of field `B282`"]
pub type B282_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B282`"]
pub struct B282_W<'a> {
    w: &'a mut W,
}
impl<'a> B282_W<'a> {
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
#[doc = "Reader of field `B283`"]
pub type B283_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B283`"]
pub struct B283_W<'a> {
    w: &'a mut W,
}
impl<'a> B283_W<'a> {
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
#[doc = "Reader of field `B284`"]
pub type B284_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B284`"]
pub struct B284_W<'a> {
    w: &'a mut W,
}
impl<'a> B284_W<'a> {
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
#[doc = "Reader of field `B285`"]
pub type B285_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B285`"]
pub struct B285_W<'a> {
    w: &'a mut W,
}
impl<'a> B285_W<'a> {
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
#[doc = "Reader of field `B286`"]
pub type B286_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B286`"]
pub struct B286_W<'a> {
    w: &'a mut W,
}
impl<'a> B286_W<'a> {
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
#[doc = "Reader of field `B287`"]
pub type B287_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B287`"]
pub struct B287_W<'a> {
    w: &'a mut W,
}
impl<'a> B287_W<'a> {
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
    #[doc = "Bit 0 - B256"]
    #[inline(always)]
    pub fn b256(&self) -> B256_R {
        B256_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B257"]
    #[inline(always)]
    pub fn b257(&self) -> B257_R {
        B257_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B258"]
    #[inline(always)]
    pub fn b258(&self) -> B258_R {
        B258_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B259"]
    #[inline(always)]
    pub fn b259(&self) -> B259_R {
        B259_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B260"]
    #[inline(always)]
    pub fn b260(&self) -> B260_R {
        B260_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B261"]
    #[inline(always)]
    pub fn b261(&self) -> B261_R {
        B261_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B262"]
    #[inline(always)]
    pub fn b262(&self) -> B262_R {
        B262_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B263"]
    #[inline(always)]
    pub fn b263(&self) -> B263_R {
        B263_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B264"]
    #[inline(always)]
    pub fn b264(&self) -> B264_R {
        B264_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B265"]
    #[inline(always)]
    pub fn b265(&self) -> B265_R {
        B265_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B266"]
    #[inline(always)]
    pub fn b266(&self) -> B266_R {
        B266_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B267"]
    #[inline(always)]
    pub fn b267(&self) -> B267_R {
        B267_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B268"]
    #[inline(always)]
    pub fn b268(&self) -> B268_R {
        B268_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B269"]
    #[inline(always)]
    pub fn b269(&self) -> B269_R {
        B269_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B270"]
    #[inline(always)]
    pub fn b270(&self) -> B270_R {
        B270_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B271"]
    #[inline(always)]
    pub fn b271(&self) -> B271_R {
        B271_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B272"]
    #[inline(always)]
    pub fn b272(&self) -> B272_R {
        B272_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B273"]
    #[inline(always)]
    pub fn b273(&self) -> B273_R {
        B273_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B274"]
    #[inline(always)]
    pub fn b274(&self) -> B274_R {
        B274_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B275"]
    #[inline(always)]
    pub fn b275(&self) -> B275_R {
        B275_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B276"]
    #[inline(always)]
    pub fn b276(&self) -> B276_R {
        B276_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B277"]
    #[inline(always)]
    pub fn b277(&self) -> B277_R {
        B277_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B278"]
    #[inline(always)]
    pub fn b278(&self) -> B278_R {
        B278_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B279"]
    #[inline(always)]
    pub fn b279(&self) -> B279_R {
        B279_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B280"]
    #[inline(always)]
    pub fn b280(&self) -> B280_R {
        B280_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B281"]
    #[inline(always)]
    pub fn b281(&self) -> B281_R {
        B281_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B282"]
    #[inline(always)]
    pub fn b282(&self) -> B282_R {
        B282_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B283"]
    #[inline(always)]
    pub fn b283(&self) -> B283_R {
        B283_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B284"]
    #[inline(always)]
    pub fn b284(&self) -> B284_R {
        B284_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B285"]
    #[inline(always)]
    pub fn b285(&self) -> B285_R {
        B285_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B286"]
    #[inline(always)]
    pub fn b286(&self) -> B286_R {
        B286_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B287"]
    #[inline(always)]
    pub fn b287(&self) -> B287_R {
        B287_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B256"]
    #[inline(always)]
    pub fn b256(&mut self) -> B256_W {
        B256_W { w: self }
    }
    #[doc = "Bit 1 - B257"]
    #[inline(always)]
    pub fn b257(&mut self) -> B257_W {
        B257_W { w: self }
    }
    #[doc = "Bit 2 - B258"]
    #[inline(always)]
    pub fn b258(&mut self) -> B258_W {
        B258_W { w: self }
    }
    #[doc = "Bit 3 - B259"]
    #[inline(always)]
    pub fn b259(&mut self) -> B259_W {
        B259_W { w: self }
    }
    #[doc = "Bit 4 - B260"]
    #[inline(always)]
    pub fn b260(&mut self) -> B260_W {
        B260_W { w: self }
    }
    #[doc = "Bit 5 - B261"]
    #[inline(always)]
    pub fn b261(&mut self) -> B261_W {
        B261_W { w: self }
    }
    #[doc = "Bit 6 - B262"]
    #[inline(always)]
    pub fn b262(&mut self) -> B262_W {
        B262_W { w: self }
    }
    #[doc = "Bit 7 - B263"]
    #[inline(always)]
    pub fn b263(&mut self) -> B263_W {
        B263_W { w: self }
    }
    #[doc = "Bit 8 - B264"]
    #[inline(always)]
    pub fn b264(&mut self) -> B264_W {
        B264_W { w: self }
    }
    #[doc = "Bit 9 - B265"]
    #[inline(always)]
    pub fn b265(&mut self) -> B265_W {
        B265_W { w: self }
    }
    #[doc = "Bit 10 - B266"]
    #[inline(always)]
    pub fn b266(&mut self) -> B266_W {
        B266_W { w: self }
    }
    #[doc = "Bit 11 - B267"]
    #[inline(always)]
    pub fn b267(&mut self) -> B267_W {
        B267_W { w: self }
    }
    #[doc = "Bit 12 - B268"]
    #[inline(always)]
    pub fn b268(&mut self) -> B268_W {
        B268_W { w: self }
    }
    #[doc = "Bit 13 - B269"]
    #[inline(always)]
    pub fn b269(&mut self) -> B269_W {
        B269_W { w: self }
    }
    #[doc = "Bit 14 - B270"]
    #[inline(always)]
    pub fn b270(&mut self) -> B270_W {
        B270_W { w: self }
    }
    #[doc = "Bit 15 - B271"]
    #[inline(always)]
    pub fn b271(&mut self) -> B271_W {
        B271_W { w: self }
    }
    #[doc = "Bit 16 - B272"]
    #[inline(always)]
    pub fn b272(&mut self) -> B272_W {
        B272_W { w: self }
    }
    #[doc = "Bit 17 - B273"]
    #[inline(always)]
    pub fn b273(&mut self) -> B273_W {
        B273_W { w: self }
    }
    #[doc = "Bit 18 - B274"]
    #[inline(always)]
    pub fn b274(&mut self) -> B274_W {
        B274_W { w: self }
    }
    #[doc = "Bit 19 - B275"]
    #[inline(always)]
    pub fn b275(&mut self) -> B275_W {
        B275_W { w: self }
    }
    #[doc = "Bit 20 - B276"]
    #[inline(always)]
    pub fn b276(&mut self) -> B276_W {
        B276_W { w: self }
    }
    #[doc = "Bit 21 - B277"]
    #[inline(always)]
    pub fn b277(&mut self) -> B277_W {
        B277_W { w: self }
    }
    #[doc = "Bit 22 - B278"]
    #[inline(always)]
    pub fn b278(&mut self) -> B278_W {
        B278_W { w: self }
    }
    #[doc = "Bit 23 - B279"]
    #[inline(always)]
    pub fn b279(&mut self) -> B279_W {
        B279_W { w: self }
    }
    #[doc = "Bit 24 - B280"]
    #[inline(always)]
    pub fn b280(&mut self) -> B280_W {
        B280_W { w: self }
    }
    #[doc = "Bit 25 - B281"]
    #[inline(always)]
    pub fn b281(&mut self) -> B281_W {
        B281_W { w: self }
    }
    #[doc = "Bit 26 - B282"]
    #[inline(always)]
    pub fn b282(&mut self) -> B282_W {
        B282_W { w: self }
    }
    #[doc = "Bit 27 - B283"]
    #[inline(always)]
    pub fn b283(&mut self) -> B283_W {
        B283_W { w: self }
    }
    #[doc = "Bit 28 - B284"]
    #[inline(always)]
    pub fn b284(&mut self) -> B284_W {
        B284_W { w: self }
    }
    #[doc = "Bit 29 - B285"]
    #[inline(always)]
    pub fn b285(&mut self) -> B285_W {
        B285_W { w: self }
    }
    #[doc = "Bit 30 - B286"]
    #[inline(always)]
    pub fn b286(&mut self) -> B286_W {
        B286_W { w: self }
    }
    #[doc = "Bit 31 - B287"]
    #[inline(always)]
    pub fn b287(&mut self) -> B287_W {
        B287_W { w: self }
    }
}
