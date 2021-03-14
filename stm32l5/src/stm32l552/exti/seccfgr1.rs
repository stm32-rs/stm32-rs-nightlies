#[doc = "Reader of register SECCFGR1"]
pub type R = crate::R<u32, super::SECCFGR1>;
#[doc = "Writer for register SECCFGR1"]
pub type W = crate::W<u32, super::SECCFGR1>;
#[doc = "Register SECCFGR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SECCFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEC0`"]
pub type SEC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC0`"]
pub struct SEC0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC1`"]
pub type SEC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC1`"]
pub struct SEC1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC2`"]
pub type SEC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC2`"]
pub struct SEC2_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC3`"]
pub type SEC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC3`"]
pub struct SEC3_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC4`"]
pub type SEC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC4`"]
pub struct SEC4_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC5`"]
pub type SEC5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC5`"]
pub struct SEC5_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC6`"]
pub type SEC6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC6`"]
pub struct SEC6_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC7`"]
pub type SEC7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC7`"]
pub struct SEC7_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC8`"]
pub type SEC8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC8`"]
pub struct SEC8_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC9`"]
pub type SEC9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC9`"]
pub struct SEC9_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC10`"]
pub type SEC10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC10`"]
pub struct SEC10_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC11`"]
pub type SEC11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC11`"]
pub struct SEC11_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC12`"]
pub type SEC12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC12`"]
pub struct SEC12_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC13`"]
pub type SEC13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC13`"]
pub struct SEC13_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC14`"]
pub type SEC14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC14`"]
pub struct SEC14_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC15`"]
pub type SEC15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC15`"]
pub struct SEC15_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC16`"]
pub type SEC16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC16`"]
pub struct SEC16_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC17`"]
pub type SEC17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC17`"]
pub struct SEC17_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC18`"]
pub type SEC18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC18`"]
pub struct SEC18_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC19`"]
pub type SEC19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC19`"]
pub struct SEC19_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC20`"]
pub type SEC20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC20`"]
pub struct SEC20_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC21`"]
pub type SEC21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC21`"]
pub struct SEC21_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC22`"]
pub type SEC22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC22`"]
pub struct SEC22_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC23`"]
pub type SEC23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC23`"]
pub struct SEC23_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC24`"]
pub type SEC24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC24`"]
pub struct SEC24_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC25`"]
pub type SEC25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC25`"]
pub struct SEC25_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC26`"]
pub type SEC26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC26`"]
pub struct SEC26_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC27`"]
pub type SEC27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC27`"]
pub struct SEC27_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC28`"]
pub type SEC28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC28`"]
pub struct SEC28_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC29`"]
pub type SEC29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC29`"]
pub struct SEC29_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC30`"]
pub type SEC30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC30`"]
pub struct SEC30_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `SEC31`"]
pub type SEC31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEC31`"]
pub struct SEC31_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec0(&self) -> SEC0_R {
        SEC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec2(&self) -> SEC2_R {
        SEC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec3(&self) -> SEC3_R {
        SEC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec4(&self) -> SEC4_R {
        SEC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec5(&self) -> SEC5_R {
        SEC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec6(&self) -> SEC6_R {
        SEC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec7(&self) -> SEC7_R {
        SEC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec8(&self) -> SEC8_R {
        SEC8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec9(&self) -> SEC9_R {
        SEC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec11(&self) -> SEC11_R {
        SEC11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec12(&self) -> SEC12_R {
        SEC12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec13(&self) -> SEC13_R {
        SEC13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec14(&self) -> SEC14_R {
        SEC14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec15(&self) -> SEC15_R {
        SEC15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec16(&self) -> SEC16_R {
        SEC16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec17(&self) -> SEC17_R {
        SEC17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec18(&self) -> SEC18_R {
        SEC18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec19(&self) -> SEC19_R {
        SEC19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec20(&self) -> SEC20_R {
        SEC20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec21(&self) -> SEC21_R {
        SEC21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec22(&self) -> SEC22_R {
        SEC22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec23(&self) -> SEC23_R {
        SEC23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec24(&self) -> SEC24_R {
        SEC24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec25(&self) -> SEC25_R {
        SEC25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec26(&self) -> SEC26_R {
        SEC26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec27(&self) -> SEC27_R {
        SEC27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec28(&self) -> SEC28_R {
        SEC28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec29(&self) -> SEC29_R {
        SEC29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec30(&self) -> SEC30_R {
        SEC30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec31(&self) -> SEC31_R {
        SEC31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec0(&mut self) -> SEC0_W {
        SEC0_W { w: self }
    }
    #[doc = "Bit 1 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec1(&mut self) -> SEC1_W {
        SEC1_W { w: self }
    }
    #[doc = "Bit 2 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec2(&mut self) -> SEC2_W {
        SEC2_W { w: self }
    }
    #[doc = "Bit 3 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec3(&mut self) -> SEC3_W {
        SEC3_W { w: self }
    }
    #[doc = "Bit 4 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec4(&mut self) -> SEC4_W {
        SEC4_W { w: self }
    }
    #[doc = "Bit 5 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec5(&mut self) -> SEC5_W {
        SEC5_W { w: self }
    }
    #[doc = "Bit 6 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec6(&mut self) -> SEC6_W {
        SEC6_W { w: self }
    }
    #[doc = "Bit 7 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec7(&mut self) -> SEC7_W {
        SEC7_W { w: self }
    }
    #[doc = "Bit 8 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec8(&mut self) -> SEC8_W {
        SEC8_W { w: self }
    }
    #[doc = "Bit 9 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec9(&mut self) -> SEC9_W {
        SEC9_W { w: self }
    }
    #[doc = "Bit 10 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec10(&mut self) -> SEC10_W {
        SEC10_W { w: self }
    }
    #[doc = "Bit 11 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec11(&mut self) -> SEC11_W {
        SEC11_W { w: self }
    }
    #[doc = "Bit 12 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec12(&mut self) -> SEC12_W {
        SEC12_W { w: self }
    }
    #[doc = "Bit 13 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec13(&mut self) -> SEC13_W {
        SEC13_W { w: self }
    }
    #[doc = "Bit 14 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec14(&mut self) -> SEC14_W {
        SEC14_W { w: self }
    }
    #[doc = "Bit 15 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec15(&mut self) -> SEC15_W {
        SEC15_W { w: self }
    }
    #[doc = "Bit 16 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec16(&mut self) -> SEC16_W {
        SEC16_W { w: self }
    }
    #[doc = "Bit 17 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec17(&mut self) -> SEC17_W {
        SEC17_W { w: self }
    }
    #[doc = "Bit 18 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec18(&mut self) -> SEC18_W {
        SEC18_W { w: self }
    }
    #[doc = "Bit 19 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec19(&mut self) -> SEC19_W {
        SEC19_W { w: self }
    }
    #[doc = "Bit 20 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec20(&mut self) -> SEC20_W {
        SEC20_W { w: self }
    }
    #[doc = "Bit 21 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec21(&mut self) -> SEC21_W {
        SEC21_W { w: self }
    }
    #[doc = "Bit 22 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec22(&mut self) -> SEC22_W {
        SEC22_W { w: self }
    }
    #[doc = "Bit 23 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec23(&mut self) -> SEC23_W {
        SEC23_W { w: self }
    }
    #[doc = "Bit 24 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec24(&mut self) -> SEC24_W {
        SEC24_W { w: self }
    }
    #[doc = "Bit 25 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec25(&mut self) -> SEC25_W {
        SEC25_W { w: self }
    }
    #[doc = "Bit 26 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec26(&mut self) -> SEC26_W {
        SEC26_W { w: self }
    }
    #[doc = "Bit 27 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec27(&mut self) -> SEC27_W {
        SEC27_W { w: self }
    }
    #[doc = "Bit 28 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec28(&mut self) -> SEC28_W {
        SEC28_W { w: self }
    }
    #[doc = "Bit 29 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec29(&mut self) -> SEC29_W {
        SEC29_W { w: self }
    }
    #[doc = "Bit 30 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec30(&mut self) -> SEC30_W {
        SEC30_W { w: self }
    }
    #[doc = "Bit 31 - Security enable on event input x"]
    #[inline(always)]
    pub fn sec31(&mut self) -> SEC31_W {
        SEC31_W { w: self }
    }
}
