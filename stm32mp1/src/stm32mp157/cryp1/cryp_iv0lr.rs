#[doc = "Reader of register CRYP_IV0LR"]
pub type R = crate::R<u32, super::CRYP_IV0LR>;
#[doc = "Writer for register CRYP_IV0LR"]
pub type W = crate::W<u32, super::CRYP_IV0LR>;
#[doc = "Register CRYP_IV0LR `reset()`'s with value 0"]
impl crate::ResetValue for super::CRYP_IV0LR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IV31`"]
pub type IV31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV31`"]
pub struct IV31_W<'a> {
    w: &'a mut W,
}
impl<'a> IV31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV30`"]
pub type IV30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV30`"]
pub struct IV30_W<'a> {
    w: &'a mut W,
}
impl<'a> IV30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV29`"]
pub type IV29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV29`"]
pub struct IV29_W<'a> {
    w: &'a mut W,
}
impl<'a> IV29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV28`"]
pub type IV28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV28`"]
pub struct IV28_W<'a> {
    w: &'a mut W,
}
impl<'a> IV28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV27`"]
pub type IV27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV27`"]
pub struct IV27_W<'a> {
    w: &'a mut W,
}
impl<'a> IV27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV26`"]
pub type IV26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV26`"]
pub struct IV26_W<'a> {
    w: &'a mut W,
}
impl<'a> IV26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV25`"]
pub type IV25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV25`"]
pub struct IV25_W<'a> {
    w: &'a mut W,
}
impl<'a> IV25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV24`"]
pub type IV24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV24`"]
pub struct IV24_W<'a> {
    w: &'a mut W,
}
impl<'a> IV24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV23`"]
pub type IV23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV23`"]
pub struct IV23_W<'a> {
    w: &'a mut W,
}
impl<'a> IV23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV22`"]
pub type IV22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV22`"]
pub struct IV22_W<'a> {
    w: &'a mut W,
}
impl<'a> IV22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV21`"]
pub type IV21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV21`"]
pub struct IV21_W<'a> {
    w: &'a mut W,
}
impl<'a> IV21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV20`"]
pub type IV20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV20`"]
pub struct IV20_W<'a> {
    w: &'a mut W,
}
impl<'a> IV20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV19`"]
pub type IV19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV19`"]
pub struct IV19_W<'a> {
    w: &'a mut W,
}
impl<'a> IV19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV18`"]
pub type IV18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV18`"]
pub struct IV18_W<'a> {
    w: &'a mut W,
}
impl<'a> IV18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV17`"]
pub type IV17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV17`"]
pub struct IV17_W<'a> {
    w: &'a mut W,
}
impl<'a> IV17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV16`"]
pub type IV16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV16`"]
pub struct IV16_W<'a> {
    w: &'a mut W,
}
impl<'a> IV16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV15`"]
pub type IV15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV15`"]
pub struct IV15_W<'a> {
    w: &'a mut W,
}
impl<'a> IV15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV14`"]
pub type IV14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV14`"]
pub struct IV14_W<'a> {
    w: &'a mut W,
}
impl<'a> IV14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV13`"]
pub type IV13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV13`"]
pub struct IV13_W<'a> {
    w: &'a mut W,
}
impl<'a> IV13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV12`"]
pub type IV12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV12`"]
pub struct IV12_W<'a> {
    w: &'a mut W,
}
impl<'a> IV12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV11`"]
pub type IV11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV11`"]
pub struct IV11_W<'a> {
    w: &'a mut W,
}
impl<'a> IV11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV10`"]
pub type IV10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV10`"]
pub struct IV10_W<'a> {
    w: &'a mut W,
}
impl<'a> IV10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV9`"]
pub type IV9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV9`"]
pub struct IV9_W<'a> {
    w: &'a mut W,
}
impl<'a> IV9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV8`"]
pub type IV8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV8`"]
pub struct IV8_W<'a> {
    w: &'a mut W,
}
impl<'a> IV8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV7`"]
pub type IV7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV7`"]
pub struct IV7_W<'a> {
    w: &'a mut W,
}
impl<'a> IV7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV6`"]
pub type IV6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV6`"]
pub struct IV6_W<'a> {
    w: &'a mut W,
}
impl<'a> IV6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV5`"]
pub type IV5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV5`"]
pub struct IV5_W<'a> {
    w: &'a mut W,
}
impl<'a> IV5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV4`"]
pub type IV4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV4`"]
pub struct IV4_W<'a> {
    w: &'a mut W,
}
impl<'a> IV4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV3`"]
pub type IV3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV3`"]
pub struct IV3_W<'a> {
    w: &'a mut W,
}
impl<'a> IV3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV2`"]
pub type IV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV2`"]
pub struct IV2_W<'a> {
    w: &'a mut W,
}
impl<'a> IV2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV1`"]
pub type IV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV1`"]
pub struct IV1_W<'a> {
    w: &'a mut W,
}
impl<'a> IV1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV0`"]
pub type IV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV0`"]
pub struct IV0_W<'a> {
    w: &'a mut W,
}
impl<'a> IV0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - IV31"]
    #[inline(always)]
    pub fn iv31(&self) -> IV31_R {
        IV31_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IV30"]
    #[inline(always)]
    pub fn iv30(&self) -> IV30_R {
        IV30_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IV29"]
    #[inline(always)]
    pub fn iv29(&self) -> IV29_R {
        IV29_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IV28"]
    #[inline(always)]
    pub fn iv28(&self) -> IV28_R {
        IV28_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IV27"]
    #[inline(always)]
    pub fn iv27(&self) -> IV27_R {
        IV27_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IV26"]
    #[inline(always)]
    pub fn iv26(&self) -> IV26_R {
        IV26_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IV25"]
    #[inline(always)]
    pub fn iv25(&self) -> IV25_R {
        IV25_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IV24"]
    #[inline(always)]
    pub fn iv24(&self) -> IV24_R {
        IV24_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - IV23"]
    #[inline(always)]
    pub fn iv23(&self) -> IV23_R {
        IV23_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - IV22"]
    #[inline(always)]
    pub fn iv22(&self) -> IV22_R {
        IV22_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - IV21"]
    #[inline(always)]
    pub fn iv21(&self) -> IV21_R {
        IV21_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - IV20"]
    #[inline(always)]
    pub fn iv20(&self) -> IV20_R {
        IV20_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - IV19"]
    #[inline(always)]
    pub fn iv19(&self) -> IV19_R {
        IV19_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - IV18"]
    #[inline(always)]
    pub fn iv18(&self) -> IV18_R {
        IV18_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - IV17"]
    #[inline(always)]
    pub fn iv17(&self) -> IV17_R {
        IV17_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IV16"]
    #[inline(always)]
    pub fn iv16(&self) -> IV16_R {
        IV16_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - IV15"]
    #[inline(always)]
    pub fn iv15(&self) -> IV15_R {
        IV15_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - IV14"]
    #[inline(always)]
    pub fn iv14(&self) -> IV14_R {
        IV14_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IV13"]
    #[inline(always)]
    pub fn iv13(&self) -> IV13_R {
        IV13_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - IV12"]
    #[inline(always)]
    pub fn iv12(&self) -> IV12_R {
        IV12_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IV11"]
    #[inline(always)]
    pub fn iv11(&self) -> IV11_R {
        IV11_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - IV10"]
    #[inline(always)]
    pub fn iv10(&self) -> IV10_R {
        IV10_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - IV9"]
    #[inline(always)]
    pub fn iv9(&self) -> IV9_R {
        IV9_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - IV8"]
    #[inline(always)]
    pub fn iv8(&self) -> IV8_R {
        IV8_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - IV7"]
    #[inline(always)]
    pub fn iv7(&self) -> IV7_R {
        IV7_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - IV6"]
    #[inline(always)]
    pub fn iv6(&self) -> IV6_R {
        IV6_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - IV5"]
    #[inline(always)]
    pub fn iv5(&self) -> IV5_R {
        IV5_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IV4"]
    #[inline(always)]
    pub fn iv4(&self) -> IV4_R {
        IV4_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - IV3"]
    #[inline(always)]
    pub fn iv3(&self) -> IV3_R {
        IV3_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - IV2"]
    #[inline(always)]
    pub fn iv2(&self) -> IV2_R {
        IV2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - IV1"]
    #[inline(always)]
    pub fn iv1(&self) -> IV1_R {
        IV1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - IV0"]
    #[inline(always)]
    pub fn iv0(&self) -> IV0_R {
        IV0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IV31"]
    #[inline(always)]
    pub fn iv31(&mut self) -> IV31_W {
        IV31_W { w: self }
    }
    #[doc = "Bit 1 - IV30"]
    #[inline(always)]
    pub fn iv30(&mut self) -> IV30_W {
        IV30_W { w: self }
    }
    #[doc = "Bit 2 - IV29"]
    #[inline(always)]
    pub fn iv29(&mut self) -> IV29_W {
        IV29_W { w: self }
    }
    #[doc = "Bit 3 - IV28"]
    #[inline(always)]
    pub fn iv28(&mut self) -> IV28_W {
        IV28_W { w: self }
    }
    #[doc = "Bit 4 - IV27"]
    #[inline(always)]
    pub fn iv27(&mut self) -> IV27_W {
        IV27_W { w: self }
    }
    #[doc = "Bit 5 - IV26"]
    #[inline(always)]
    pub fn iv26(&mut self) -> IV26_W {
        IV26_W { w: self }
    }
    #[doc = "Bit 6 - IV25"]
    #[inline(always)]
    pub fn iv25(&mut self) -> IV25_W {
        IV25_W { w: self }
    }
    #[doc = "Bit 7 - IV24"]
    #[inline(always)]
    pub fn iv24(&mut self) -> IV24_W {
        IV24_W { w: self }
    }
    #[doc = "Bit 8 - IV23"]
    #[inline(always)]
    pub fn iv23(&mut self) -> IV23_W {
        IV23_W { w: self }
    }
    #[doc = "Bit 9 - IV22"]
    #[inline(always)]
    pub fn iv22(&mut self) -> IV22_W {
        IV22_W { w: self }
    }
    #[doc = "Bit 10 - IV21"]
    #[inline(always)]
    pub fn iv21(&mut self) -> IV21_W {
        IV21_W { w: self }
    }
    #[doc = "Bit 11 - IV20"]
    #[inline(always)]
    pub fn iv20(&mut self) -> IV20_W {
        IV20_W { w: self }
    }
    #[doc = "Bit 12 - IV19"]
    #[inline(always)]
    pub fn iv19(&mut self) -> IV19_W {
        IV19_W { w: self }
    }
    #[doc = "Bit 13 - IV18"]
    #[inline(always)]
    pub fn iv18(&mut self) -> IV18_W {
        IV18_W { w: self }
    }
    #[doc = "Bit 14 - IV17"]
    #[inline(always)]
    pub fn iv17(&mut self) -> IV17_W {
        IV17_W { w: self }
    }
    #[doc = "Bit 15 - IV16"]
    #[inline(always)]
    pub fn iv16(&mut self) -> IV16_W {
        IV16_W { w: self }
    }
    #[doc = "Bit 16 - IV15"]
    #[inline(always)]
    pub fn iv15(&mut self) -> IV15_W {
        IV15_W { w: self }
    }
    #[doc = "Bit 17 - IV14"]
    #[inline(always)]
    pub fn iv14(&mut self) -> IV14_W {
        IV14_W { w: self }
    }
    #[doc = "Bit 18 - IV13"]
    #[inline(always)]
    pub fn iv13(&mut self) -> IV13_W {
        IV13_W { w: self }
    }
    #[doc = "Bit 19 - IV12"]
    #[inline(always)]
    pub fn iv12(&mut self) -> IV12_W {
        IV12_W { w: self }
    }
    #[doc = "Bit 20 - IV11"]
    #[inline(always)]
    pub fn iv11(&mut self) -> IV11_W {
        IV11_W { w: self }
    }
    #[doc = "Bit 21 - IV10"]
    #[inline(always)]
    pub fn iv10(&mut self) -> IV10_W {
        IV10_W { w: self }
    }
    #[doc = "Bit 22 - IV9"]
    #[inline(always)]
    pub fn iv9(&mut self) -> IV9_W {
        IV9_W { w: self }
    }
    #[doc = "Bit 23 - IV8"]
    #[inline(always)]
    pub fn iv8(&mut self) -> IV8_W {
        IV8_W { w: self }
    }
    #[doc = "Bit 24 - IV7"]
    #[inline(always)]
    pub fn iv7(&mut self) -> IV7_W {
        IV7_W { w: self }
    }
    #[doc = "Bit 25 - IV6"]
    #[inline(always)]
    pub fn iv6(&mut self) -> IV6_W {
        IV6_W { w: self }
    }
    #[doc = "Bit 26 - IV5"]
    #[inline(always)]
    pub fn iv5(&mut self) -> IV5_W {
        IV5_W { w: self }
    }
    #[doc = "Bit 27 - IV4"]
    #[inline(always)]
    pub fn iv4(&mut self) -> IV4_W {
        IV4_W { w: self }
    }
    #[doc = "Bit 28 - IV3"]
    #[inline(always)]
    pub fn iv3(&mut self) -> IV3_W {
        IV3_W { w: self }
    }
    #[doc = "Bit 29 - IV2"]
    #[inline(always)]
    pub fn iv2(&mut self) -> IV2_W {
        IV2_W { w: self }
    }
    #[doc = "Bit 30 - IV1"]
    #[inline(always)]
    pub fn iv1(&mut self) -> IV1_W {
        IV1_W { w: self }
    }
    #[doc = "Bit 31 - IV0"]
    #[inline(always)]
    pub fn iv0(&mut self) -> IV0_W {
        IV0_W { w: self }
    }
}
