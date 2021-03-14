#[doc = "Reader of register PRIVCFGR1"]
pub type R = crate::R<u32, super::PRIVCFGR1>;
#[doc = "Writer for register PRIVCFGR1"]
pub type W = crate::W<u32, super::PRIVCFGR1>;
#[doc = "Register PRIVCFGR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRIVCFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRIV0`"]
pub type PRIV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV0`"]
pub struct PRIV0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV1`"]
pub type PRIV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV1`"]
pub struct PRIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV2`"]
pub type PRIV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV2`"]
pub struct PRIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV3`"]
pub type PRIV3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV3`"]
pub struct PRIV3_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV4`"]
pub type PRIV4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV4`"]
pub struct PRIV4_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV5`"]
pub type PRIV5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV5`"]
pub struct PRIV5_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV6`"]
pub type PRIV6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV6`"]
pub struct PRIV6_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV7`"]
pub type PRIV7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV7`"]
pub struct PRIV7_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV8`"]
pub type PRIV8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV8`"]
pub struct PRIV8_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV9`"]
pub type PRIV9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV9`"]
pub struct PRIV9_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV10`"]
pub type PRIV10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV10`"]
pub struct PRIV10_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV11`"]
pub type PRIV11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV11`"]
pub struct PRIV11_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV12`"]
pub type PRIV12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV12`"]
pub struct PRIV12_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV13`"]
pub type PRIV13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV13`"]
pub struct PRIV13_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV14`"]
pub type PRIV14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV14`"]
pub struct PRIV14_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV15`"]
pub type PRIV15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV15`"]
pub struct PRIV15_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV16`"]
pub type PRIV16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV16`"]
pub struct PRIV16_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV17`"]
pub type PRIV17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV17`"]
pub struct PRIV17_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV18`"]
pub type PRIV18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV18`"]
pub struct PRIV18_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV19`"]
pub type PRIV19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV19`"]
pub struct PRIV19_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV20`"]
pub type PRIV20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV20`"]
pub struct PRIV20_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV21`"]
pub type PRIV21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV21`"]
pub struct PRIV21_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV22`"]
pub type PRIV22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV22`"]
pub struct PRIV22_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV23`"]
pub type PRIV23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV23`"]
pub struct PRIV23_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV24`"]
pub type PRIV24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV24`"]
pub struct PRIV24_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV25`"]
pub type PRIV25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV25`"]
pub struct PRIV25_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV26`"]
pub type PRIV26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV26`"]
pub struct PRIV26_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV27`"]
pub type PRIV27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV27`"]
pub struct PRIV27_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV28`"]
pub type PRIV28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV28`"]
pub struct PRIV28_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV29`"]
pub type PRIV29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV29`"]
pub struct PRIV29_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV30`"]
pub type PRIV30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV30`"]
pub struct PRIV30_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `PRIV31`"]
pub type PRIV31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV31`"]
pub struct PRIV31_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    pub fn priv0(&self) -> PRIV0_R {
        PRIV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv1(&self) -> PRIV1_R {
        PRIV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv2(&self) -> PRIV2_R {
        PRIV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv3(&self) -> PRIV3_R {
        PRIV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv4(&self) -> PRIV4_R {
        PRIV4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv5(&self) -> PRIV5_R {
        PRIV5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv6(&self) -> PRIV6_R {
        PRIV6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv7(&self) -> PRIV7_R {
        PRIV7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv8(&self) -> PRIV8_R {
        PRIV8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv9(&self) -> PRIV9_R {
        PRIV9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv10(&self) -> PRIV10_R {
        PRIV10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv11(&self) -> PRIV11_R {
        PRIV11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv12(&self) -> PRIV12_R {
        PRIV12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv13(&self) -> PRIV13_R {
        PRIV13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv14(&self) -> PRIV14_R {
        PRIV14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv15(&self) -> PRIV15_R {
        PRIV15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv16(&self) -> PRIV16_R {
        PRIV16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv17(&self) -> PRIV17_R {
        PRIV17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv18(&self) -> PRIV18_R {
        PRIV18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv19(&self) -> PRIV19_R {
        PRIV19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv20(&self) -> PRIV20_R {
        PRIV20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv21(&self) -> PRIV21_R {
        PRIV21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv22(&self) -> PRIV22_R {
        PRIV22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv23(&self) -> PRIV23_R {
        PRIV23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv24(&self) -> PRIV24_R {
        PRIV24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv25(&self) -> PRIV25_R {
        PRIV25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv26(&self) -> PRIV26_R {
        PRIV26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv27(&self) -> PRIV27_R {
        PRIV27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv28(&self) -> PRIV28_R {
        PRIV28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv29(&self) -> PRIV29_R {
        PRIV29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv30(&self) -> PRIV30_R {
        PRIV30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv31(&self) -> PRIV31_R {
        PRIV31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv0(&mut self) -> PRIV0_W {
        PRIV0_W { w: self }
    }
    #[doc = "Bit 1 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv1(&mut self) -> PRIV1_W {
        PRIV1_W { w: self }
    }
    #[doc = "Bit 2 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv2(&mut self) -> PRIV2_W {
        PRIV2_W { w: self }
    }
    #[doc = "Bit 3 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv3(&mut self) -> PRIV3_W {
        PRIV3_W { w: self }
    }
    #[doc = "Bit 4 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv4(&mut self) -> PRIV4_W {
        PRIV4_W { w: self }
    }
    #[doc = "Bit 5 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv5(&mut self) -> PRIV5_W {
        PRIV5_W { w: self }
    }
    #[doc = "Bit 6 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv6(&mut self) -> PRIV6_W {
        PRIV6_W { w: self }
    }
    #[doc = "Bit 7 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv7(&mut self) -> PRIV7_W {
        PRIV7_W { w: self }
    }
    #[doc = "Bit 8 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv8(&mut self) -> PRIV8_W {
        PRIV8_W { w: self }
    }
    #[doc = "Bit 9 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv9(&mut self) -> PRIV9_W {
        PRIV9_W { w: self }
    }
    #[doc = "Bit 10 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv10(&mut self) -> PRIV10_W {
        PRIV10_W { w: self }
    }
    #[doc = "Bit 11 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv11(&mut self) -> PRIV11_W {
        PRIV11_W { w: self }
    }
    #[doc = "Bit 12 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv12(&mut self) -> PRIV12_W {
        PRIV12_W { w: self }
    }
    #[doc = "Bit 13 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv13(&mut self) -> PRIV13_W {
        PRIV13_W { w: self }
    }
    #[doc = "Bit 14 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv14(&mut self) -> PRIV14_W {
        PRIV14_W { w: self }
    }
    #[doc = "Bit 15 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv15(&mut self) -> PRIV15_W {
        PRIV15_W { w: self }
    }
    #[doc = "Bit 16 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv16(&mut self) -> PRIV16_W {
        PRIV16_W { w: self }
    }
    #[doc = "Bit 17 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv17(&mut self) -> PRIV17_W {
        PRIV17_W { w: self }
    }
    #[doc = "Bit 18 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv18(&mut self) -> PRIV18_W {
        PRIV18_W { w: self }
    }
    #[doc = "Bit 19 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv19(&mut self) -> PRIV19_W {
        PRIV19_W { w: self }
    }
    #[doc = "Bit 20 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv20(&mut self) -> PRIV20_W {
        PRIV20_W { w: self }
    }
    #[doc = "Bit 21 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv21(&mut self) -> PRIV21_W {
        PRIV21_W { w: self }
    }
    #[doc = "Bit 22 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv22(&mut self) -> PRIV22_W {
        PRIV22_W { w: self }
    }
    #[doc = "Bit 23 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv23(&mut self) -> PRIV23_W {
        PRIV23_W { w: self }
    }
    #[doc = "Bit 24 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv24(&mut self) -> PRIV24_W {
        PRIV24_W { w: self }
    }
    #[doc = "Bit 25 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv25(&mut self) -> PRIV25_W {
        PRIV25_W { w: self }
    }
    #[doc = "Bit 26 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv26(&mut self) -> PRIV26_W {
        PRIV26_W { w: self }
    }
    #[doc = "Bit 27 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv27(&mut self) -> PRIV27_W {
        PRIV27_W { w: self }
    }
    #[doc = "Bit 28 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv28(&mut self) -> PRIV28_W {
        PRIV28_W { w: self }
    }
    #[doc = "Bit 29 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv29(&mut self) -> PRIV29_W {
        PRIV29_W { w: self }
    }
    #[doc = "Bit 30 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv30(&mut self) -> PRIV30_W {
        PRIV30_W { w: self }
    }
    #[doc = "Bit 31 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv31(&mut self) -> PRIV31_W {
        PRIV31_W { w: self }
    }
}
