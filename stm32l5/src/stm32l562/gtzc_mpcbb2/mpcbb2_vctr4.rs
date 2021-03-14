#[doc = "Reader of register MPCBB2_VCTR4"]
pub type R = crate::R<u32, super::MPCBB2_VCTR4>;
#[doc = "Writer for register MPCBB2_VCTR4"]
pub type W = crate::W<u32, super::MPCBB2_VCTR4>;
#[doc = "Register MPCBB2_VCTR4 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB2_VCTR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B128`"]
pub type B128_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B128`"]
pub struct B128_W<'a> {
    w: &'a mut W,
}
impl<'a> B128_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B129`"]
pub type B129_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B129`"]
pub struct B129_W<'a> {
    w: &'a mut W,
}
impl<'a> B129_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B130`"]
pub type B130_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B130`"]
pub struct B130_W<'a> {
    w: &'a mut W,
}
impl<'a> B130_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B131`"]
pub type B131_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B131`"]
pub struct B131_W<'a> {
    w: &'a mut W,
}
impl<'a> B131_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B132`"]
pub type B132_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B132`"]
pub struct B132_W<'a> {
    w: &'a mut W,
}
impl<'a> B132_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B133`"]
pub type B133_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B133`"]
pub struct B133_W<'a> {
    w: &'a mut W,
}
impl<'a> B133_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B134`"]
pub type B134_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B134`"]
pub struct B134_W<'a> {
    w: &'a mut W,
}
impl<'a> B134_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B135`"]
pub type B135_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B135`"]
pub struct B135_W<'a> {
    w: &'a mut W,
}
impl<'a> B135_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B136`"]
pub type B136_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B136`"]
pub struct B136_W<'a> {
    w: &'a mut W,
}
impl<'a> B136_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B137`"]
pub type B137_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B137`"]
pub struct B137_W<'a> {
    w: &'a mut W,
}
impl<'a> B137_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B138`"]
pub type B138_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B138`"]
pub struct B138_W<'a> {
    w: &'a mut W,
}
impl<'a> B138_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B139`"]
pub type B139_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B139`"]
pub struct B139_W<'a> {
    w: &'a mut W,
}
impl<'a> B139_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B140`"]
pub type B140_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B140`"]
pub struct B140_W<'a> {
    w: &'a mut W,
}
impl<'a> B140_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B141`"]
pub type B141_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B141`"]
pub struct B141_W<'a> {
    w: &'a mut W,
}
impl<'a> B141_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B142`"]
pub type B142_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B142`"]
pub struct B142_W<'a> {
    w: &'a mut W,
}
impl<'a> B142_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B143`"]
pub type B143_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B143`"]
pub struct B143_W<'a> {
    w: &'a mut W,
}
impl<'a> B143_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B144`"]
pub type B144_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B144`"]
pub struct B144_W<'a> {
    w: &'a mut W,
}
impl<'a> B144_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B145`"]
pub type B145_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B145`"]
pub struct B145_W<'a> {
    w: &'a mut W,
}
impl<'a> B145_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B146`"]
pub type B146_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B146`"]
pub struct B146_W<'a> {
    w: &'a mut W,
}
impl<'a> B146_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B147`"]
pub type B147_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B147`"]
pub struct B147_W<'a> {
    w: &'a mut W,
}
impl<'a> B147_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B148`"]
pub type B148_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B148`"]
pub struct B148_W<'a> {
    w: &'a mut W,
}
impl<'a> B148_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B149`"]
pub type B149_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B149`"]
pub struct B149_W<'a> {
    w: &'a mut W,
}
impl<'a> B149_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B150`"]
pub type B150_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B150`"]
pub struct B150_W<'a> {
    w: &'a mut W,
}
impl<'a> B150_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B151`"]
pub type B151_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B151`"]
pub struct B151_W<'a> {
    w: &'a mut W,
}
impl<'a> B151_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B152`"]
pub type B152_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B152`"]
pub struct B152_W<'a> {
    w: &'a mut W,
}
impl<'a> B152_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B153`"]
pub type B153_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B153`"]
pub struct B153_W<'a> {
    w: &'a mut W,
}
impl<'a> B153_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B154`"]
pub type B154_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B154`"]
pub struct B154_W<'a> {
    w: &'a mut W,
}
impl<'a> B154_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B155`"]
pub type B155_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B155`"]
pub struct B155_W<'a> {
    w: &'a mut W,
}
impl<'a> B155_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B156`"]
pub type B156_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B156`"]
pub struct B156_W<'a> {
    w: &'a mut W,
}
impl<'a> B156_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B157`"]
pub type B157_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B157`"]
pub struct B157_W<'a> {
    w: &'a mut W,
}
impl<'a> B157_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B158`"]
pub type B158_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B158`"]
pub struct B158_W<'a> {
    w: &'a mut W,
}
impl<'a> B158_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B159`"]
pub type B159_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B159`"]
pub struct B159_W<'a> {
    w: &'a mut W,
}
impl<'a> B159_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B128"]
    #[inline(always)]
    pub fn b128(&self) -> B128_R {
        B128_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B129"]
    #[inline(always)]
    pub fn b129(&self) -> B129_R {
        B129_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B130"]
    #[inline(always)]
    pub fn b130(&self) -> B130_R {
        B130_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B131"]
    #[inline(always)]
    pub fn b131(&self) -> B131_R {
        B131_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B132"]
    #[inline(always)]
    pub fn b132(&self) -> B132_R {
        B132_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B133"]
    #[inline(always)]
    pub fn b133(&self) -> B133_R {
        B133_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B134"]
    #[inline(always)]
    pub fn b134(&self) -> B134_R {
        B134_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B135"]
    #[inline(always)]
    pub fn b135(&self) -> B135_R {
        B135_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B136"]
    #[inline(always)]
    pub fn b136(&self) -> B136_R {
        B136_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B137"]
    #[inline(always)]
    pub fn b137(&self) -> B137_R {
        B137_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B138"]
    #[inline(always)]
    pub fn b138(&self) -> B138_R {
        B138_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B139"]
    #[inline(always)]
    pub fn b139(&self) -> B139_R {
        B139_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B140"]
    #[inline(always)]
    pub fn b140(&self) -> B140_R {
        B140_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B141"]
    #[inline(always)]
    pub fn b141(&self) -> B141_R {
        B141_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B142"]
    #[inline(always)]
    pub fn b142(&self) -> B142_R {
        B142_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B143"]
    #[inline(always)]
    pub fn b143(&self) -> B143_R {
        B143_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B144"]
    #[inline(always)]
    pub fn b144(&self) -> B144_R {
        B144_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B145"]
    #[inline(always)]
    pub fn b145(&self) -> B145_R {
        B145_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B146"]
    #[inline(always)]
    pub fn b146(&self) -> B146_R {
        B146_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B147"]
    #[inline(always)]
    pub fn b147(&self) -> B147_R {
        B147_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B148"]
    #[inline(always)]
    pub fn b148(&self) -> B148_R {
        B148_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B149"]
    #[inline(always)]
    pub fn b149(&self) -> B149_R {
        B149_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B150"]
    #[inline(always)]
    pub fn b150(&self) -> B150_R {
        B150_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B151"]
    #[inline(always)]
    pub fn b151(&self) -> B151_R {
        B151_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B152"]
    #[inline(always)]
    pub fn b152(&self) -> B152_R {
        B152_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B153"]
    #[inline(always)]
    pub fn b153(&self) -> B153_R {
        B153_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B154"]
    #[inline(always)]
    pub fn b154(&self) -> B154_R {
        B154_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B155"]
    #[inline(always)]
    pub fn b155(&self) -> B155_R {
        B155_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B156"]
    #[inline(always)]
    pub fn b156(&self) -> B156_R {
        B156_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B157"]
    #[inline(always)]
    pub fn b157(&self) -> B157_R {
        B157_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B158"]
    #[inline(always)]
    pub fn b158(&self) -> B158_R {
        B158_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B159"]
    #[inline(always)]
    pub fn b159(&self) -> B159_R {
        B159_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B128"]
    #[inline(always)]
    pub fn b128(&mut self) -> B128_W {
        B128_W { w: self }
    }
    #[doc = "Bit 1 - B129"]
    #[inline(always)]
    pub fn b129(&mut self) -> B129_W {
        B129_W { w: self }
    }
    #[doc = "Bit 2 - B130"]
    #[inline(always)]
    pub fn b130(&mut self) -> B130_W {
        B130_W { w: self }
    }
    #[doc = "Bit 3 - B131"]
    #[inline(always)]
    pub fn b131(&mut self) -> B131_W {
        B131_W { w: self }
    }
    #[doc = "Bit 4 - B132"]
    #[inline(always)]
    pub fn b132(&mut self) -> B132_W {
        B132_W { w: self }
    }
    #[doc = "Bit 5 - B133"]
    #[inline(always)]
    pub fn b133(&mut self) -> B133_W {
        B133_W { w: self }
    }
    #[doc = "Bit 6 - B134"]
    #[inline(always)]
    pub fn b134(&mut self) -> B134_W {
        B134_W { w: self }
    }
    #[doc = "Bit 7 - B135"]
    #[inline(always)]
    pub fn b135(&mut self) -> B135_W {
        B135_W { w: self }
    }
    #[doc = "Bit 8 - B136"]
    #[inline(always)]
    pub fn b136(&mut self) -> B136_W {
        B136_W { w: self }
    }
    #[doc = "Bit 9 - B137"]
    #[inline(always)]
    pub fn b137(&mut self) -> B137_W {
        B137_W { w: self }
    }
    #[doc = "Bit 10 - B138"]
    #[inline(always)]
    pub fn b138(&mut self) -> B138_W {
        B138_W { w: self }
    }
    #[doc = "Bit 11 - B139"]
    #[inline(always)]
    pub fn b139(&mut self) -> B139_W {
        B139_W { w: self }
    }
    #[doc = "Bit 12 - B140"]
    #[inline(always)]
    pub fn b140(&mut self) -> B140_W {
        B140_W { w: self }
    }
    #[doc = "Bit 13 - B141"]
    #[inline(always)]
    pub fn b141(&mut self) -> B141_W {
        B141_W { w: self }
    }
    #[doc = "Bit 14 - B142"]
    #[inline(always)]
    pub fn b142(&mut self) -> B142_W {
        B142_W { w: self }
    }
    #[doc = "Bit 15 - B143"]
    #[inline(always)]
    pub fn b143(&mut self) -> B143_W {
        B143_W { w: self }
    }
    #[doc = "Bit 16 - B144"]
    #[inline(always)]
    pub fn b144(&mut self) -> B144_W {
        B144_W { w: self }
    }
    #[doc = "Bit 17 - B145"]
    #[inline(always)]
    pub fn b145(&mut self) -> B145_W {
        B145_W { w: self }
    }
    #[doc = "Bit 18 - B146"]
    #[inline(always)]
    pub fn b146(&mut self) -> B146_W {
        B146_W { w: self }
    }
    #[doc = "Bit 19 - B147"]
    #[inline(always)]
    pub fn b147(&mut self) -> B147_W {
        B147_W { w: self }
    }
    #[doc = "Bit 20 - B148"]
    #[inline(always)]
    pub fn b148(&mut self) -> B148_W {
        B148_W { w: self }
    }
    #[doc = "Bit 21 - B149"]
    #[inline(always)]
    pub fn b149(&mut self) -> B149_W {
        B149_W { w: self }
    }
    #[doc = "Bit 22 - B150"]
    #[inline(always)]
    pub fn b150(&mut self) -> B150_W {
        B150_W { w: self }
    }
    #[doc = "Bit 23 - B151"]
    #[inline(always)]
    pub fn b151(&mut self) -> B151_W {
        B151_W { w: self }
    }
    #[doc = "Bit 24 - B152"]
    #[inline(always)]
    pub fn b152(&mut self) -> B152_W {
        B152_W { w: self }
    }
    #[doc = "Bit 25 - B153"]
    #[inline(always)]
    pub fn b153(&mut self) -> B153_W {
        B153_W { w: self }
    }
    #[doc = "Bit 26 - B154"]
    #[inline(always)]
    pub fn b154(&mut self) -> B154_W {
        B154_W { w: self }
    }
    #[doc = "Bit 27 - B155"]
    #[inline(always)]
    pub fn b155(&mut self) -> B155_W {
        B155_W { w: self }
    }
    #[doc = "Bit 28 - B156"]
    #[inline(always)]
    pub fn b156(&mut self) -> B156_W {
        B156_W { w: self }
    }
    #[doc = "Bit 29 - B157"]
    #[inline(always)]
    pub fn b157(&mut self) -> B157_W {
        B157_W { w: self }
    }
    #[doc = "Bit 30 - B158"]
    #[inline(always)]
    pub fn b158(&mut self) -> B158_W {
        B158_W { w: self }
    }
    #[doc = "Bit 31 - B159"]
    #[inline(always)]
    pub fn b159(&mut self) -> B159_W {
        B159_W { w: self }
    }
}
