#[doc = "Reader of register ETZPC_DECPROT_LOCK0"]
pub type R = crate::R<u32, super::ETZPC_DECPROT_LOCK0>;
#[doc = "Writer for register ETZPC_DECPROT_LOCK0"]
pub type W = crate::W<u32, super::ETZPC_DECPROT_LOCK0>;
#[doc = "Register ETZPC_DECPROT_LOCK0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ETZPC_DECPROT_LOCK0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOCK0`"]
pub type LOCK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK0`"]
pub struct LOCK0_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK1`"]
pub type LOCK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK1`"]
pub struct LOCK1_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK2`"]
pub type LOCK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK2`"]
pub struct LOCK2_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK3`"]
pub type LOCK3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK3`"]
pub struct LOCK3_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK4`"]
pub type LOCK4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK4`"]
pub struct LOCK4_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK5`"]
pub type LOCK5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK5`"]
pub struct LOCK5_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK6`"]
pub type LOCK6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK6`"]
pub struct LOCK6_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK7`"]
pub type LOCK7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK7`"]
pub struct LOCK7_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK8`"]
pub type LOCK8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK8`"]
pub struct LOCK8_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK9`"]
pub type LOCK9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK9`"]
pub struct LOCK9_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK10`"]
pub type LOCK10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK10`"]
pub struct LOCK10_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK11`"]
pub type LOCK11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK11`"]
pub struct LOCK11_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK12`"]
pub type LOCK12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK12`"]
pub struct LOCK12_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK13`"]
pub type LOCK13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK13`"]
pub struct LOCK13_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK14`"]
pub type LOCK14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK14`"]
pub struct LOCK14_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK15`"]
pub type LOCK15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK15`"]
pub struct LOCK15_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK16`"]
pub type LOCK16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK16`"]
pub struct LOCK16_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK17`"]
pub type LOCK17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK17`"]
pub struct LOCK17_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK18`"]
pub type LOCK18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK18`"]
pub struct LOCK18_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK19`"]
pub type LOCK19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK19`"]
pub struct LOCK19_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK20`"]
pub type LOCK20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK20`"]
pub struct LOCK20_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK21`"]
pub type LOCK21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK21`"]
pub struct LOCK21_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK22`"]
pub type LOCK22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK22`"]
pub struct LOCK22_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK23`"]
pub type LOCK23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK23`"]
pub struct LOCK23_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK24`"]
pub type LOCK24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK24`"]
pub struct LOCK24_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK25`"]
pub type LOCK25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK25`"]
pub struct LOCK25_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK26`"]
pub type LOCK26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK26`"]
pub struct LOCK26_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK27`"]
pub type LOCK27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK27`"]
pub struct LOCK27_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK28`"]
pub type LOCK28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK28`"]
pub struct LOCK28_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK29`"]
pub type LOCK29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK29`"]
pub struct LOCK29_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK30`"]
pub type LOCK30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK30`"]
pub struct LOCK30_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `LOCK31`"]
pub type LOCK31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK31`"]
pub struct LOCK31_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - LOCK0"]
    #[inline(always)]
    pub fn lock0(&self) -> LOCK0_R {
        LOCK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LOCK1"]
    #[inline(always)]
    pub fn lock1(&self) -> LOCK1_R {
        LOCK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LOCK2"]
    #[inline(always)]
    pub fn lock2(&self) -> LOCK2_R {
        LOCK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LOCK3"]
    #[inline(always)]
    pub fn lock3(&self) -> LOCK3_R {
        LOCK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LOCK4"]
    #[inline(always)]
    pub fn lock4(&self) -> LOCK4_R {
        LOCK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LOCK5"]
    #[inline(always)]
    pub fn lock5(&self) -> LOCK5_R {
        LOCK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LOCK6"]
    #[inline(always)]
    pub fn lock6(&self) -> LOCK6_R {
        LOCK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LOCK7"]
    #[inline(always)]
    pub fn lock7(&self) -> LOCK7_R {
        LOCK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LOCK8"]
    #[inline(always)]
    pub fn lock8(&self) -> LOCK8_R {
        LOCK8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LOCK9"]
    #[inline(always)]
    pub fn lock9(&self) -> LOCK9_R {
        LOCK9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LOCK10"]
    #[inline(always)]
    pub fn lock10(&self) -> LOCK10_R {
        LOCK10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LOCK11"]
    #[inline(always)]
    pub fn lock11(&self) -> LOCK11_R {
        LOCK11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LOCK12"]
    #[inline(always)]
    pub fn lock12(&self) -> LOCK12_R {
        LOCK12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LOCK13"]
    #[inline(always)]
    pub fn lock13(&self) -> LOCK13_R {
        LOCK13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LOCK14"]
    #[inline(always)]
    pub fn lock14(&self) -> LOCK14_R {
        LOCK14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LOCK15"]
    #[inline(always)]
    pub fn lock15(&self) -> LOCK15_R {
        LOCK15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LOCK16"]
    #[inline(always)]
    pub fn lock16(&self) -> LOCK16_R {
        LOCK16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - LOCK17"]
    #[inline(always)]
    pub fn lock17(&self) -> LOCK17_R {
        LOCK17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - LOCK18"]
    #[inline(always)]
    pub fn lock18(&self) -> LOCK18_R {
        LOCK18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - LOCK19"]
    #[inline(always)]
    pub fn lock19(&self) -> LOCK19_R {
        LOCK19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LOCK20"]
    #[inline(always)]
    pub fn lock20(&self) -> LOCK20_R {
        LOCK20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LOCK21"]
    #[inline(always)]
    pub fn lock21(&self) -> LOCK21_R {
        LOCK21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - LOCK22"]
    #[inline(always)]
    pub fn lock22(&self) -> LOCK22_R {
        LOCK22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - LOCK23"]
    #[inline(always)]
    pub fn lock23(&self) -> LOCK23_R {
        LOCK23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - LOCK24"]
    #[inline(always)]
    pub fn lock24(&self) -> LOCK24_R {
        LOCK24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - LOCK25"]
    #[inline(always)]
    pub fn lock25(&self) -> LOCK25_R {
        LOCK25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - LOCK26"]
    #[inline(always)]
    pub fn lock26(&self) -> LOCK26_R {
        LOCK26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LOCK27"]
    #[inline(always)]
    pub fn lock27(&self) -> LOCK27_R {
        LOCK27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - LOCK28"]
    #[inline(always)]
    pub fn lock28(&self) -> LOCK28_R {
        LOCK28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - LOCK29"]
    #[inline(always)]
    pub fn lock29(&self) -> LOCK29_R {
        LOCK29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - LOCK30"]
    #[inline(always)]
    pub fn lock30(&self) -> LOCK30_R {
        LOCK30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LOCK31"]
    #[inline(always)]
    pub fn lock31(&self) -> LOCK31_R {
        LOCK31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LOCK0"]
    #[inline(always)]
    pub fn lock0(&mut self) -> LOCK0_W {
        LOCK0_W { w: self }
    }
    #[doc = "Bit 1 - LOCK1"]
    #[inline(always)]
    pub fn lock1(&mut self) -> LOCK1_W {
        LOCK1_W { w: self }
    }
    #[doc = "Bit 2 - LOCK2"]
    #[inline(always)]
    pub fn lock2(&mut self) -> LOCK2_W {
        LOCK2_W { w: self }
    }
    #[doc = "Bit 3 - LOCK3"]
    #[inline(always)]
    pub fn lock3(&mut self) -> LOCK3_W {
        LOCK3_W { w: self }
    }
    #[doc = "Bit 4 - LOCK4"]
    #[inline(always)]
    pub fn lock4(&mut self) -> LOCK4_W {
        LOCK4_W { w: self }
    }
    #[doc = "Bit 5 - LOCK5"]
    #[inline(always)]
    pub fn lock5(&mut self) -> LOCK5_W {
        LOCK5_W { w: self }
    }
    #[doc = "Bit 6 - LOCK6"]
    #[inline(always)]
    pub fn lock6(&mut self) -> LOCK6_W {
        LOCK6_W { w: self }
    }
    #[doc = "Bit 7 - LOCK7"]
    #[inline(always)]
    pub fn lock7(&mut self) -> LOCK7_W {
        LOCK7_W { w: self }
    }
    #[doc = "Bit 8 - LOCK8"]
    #[inline(always)]
    pub fn lock8(&mut self) -> LOCK8_W {
        LOCK8_W { w: self }
    }
    #[doc = "Bit 9 - LOCK9"]
    #[inline(always)]
    pub fn lock9(&mut self) -> LOCK9_W {
        LOCK9_W { w: self }
    }
    #[doc = "Bit 10 - LOCK10"]
    #[inline(always)]
    pub fn lock10(&mut self) -> LOCK10_W {
        LOCK10_W { w: self }
    }
    #[doc = "Bit 11 - LOCK11"]
    #[inline(always)]
    pub fn lock11(&mut self) -> LOCK11_W {
        LOCK11_W { w: self }
    }
    #[doc = "Bit 12 - LOCK12"]
    #[inline(always)]
    pub fn lock12(&mut self) -> LOCK12_W {
        LOCK12_W { w: self }
    }
    #[doc = "Bit 13 - LOCK13"]
    #[inline(always)]
    pub fn lock13(&mut self) -> LOCK13_W {
        LOCK13_W { w: self }
    }
    #[doc = "Bit 14 - LOCK14"]
    #[inline(always)]
    pub fn lock14(&mut self) -> LOCK14_W {
        LOCK14_W { w: self }
    }
    #[doc = "Bit 15 - LOCK15"]
    #[inline(always)]
    pub fn lock15(&mut self) -> LOCK15_W {
        LOCK15_W { w: self }
    }
    #[doc = "Bit 16 - LOCK16"]
    #[inline(always)]
    pub fn lock16(&mut self) -> LOCK16_W {
        LOCK16_W { w: self }
    }
    #[doc = "Bit 17 - LOCK17"]
    #[inline(always)]
    pub fn lock17(&mut self) -> LOCK17_W {
        LOCK17_W { w: self }
    }
    #[doc = "Bit 18 - LOCK18"]
    #[inline(always)]
    pub fn lock18(&mut self) -> LOCK18_W {
        LOCK18_W { w: self }
    }
    #[doc = "Bit 19 - LOCK19"]
    #[inline(always)]
    pub fn lock19(&mut self) -> LOCK19_W {
        LOCK19_W { w: self }
    }
    #[doc = "Bit 20 - LOCK20"]
    #[inline(always)]
    pub fn lock20(&mut self) -> LOCK20_W {
        LOCK20_W { w: self }
    }
    #[doc = "Bit 21 - LOCK21"]
    #[inline(always)]
    pub fn lock21(&mut self) -> LOCK21_W {
        LOCK21_W { w: self }
    }
    #[doc = "Bit 22 - LOCK22"]
    #[inline(always)]
    pub fn lock22(&mut self) -> LOCK22_W {
        LOCK22_W { w: self }
    }
    #[doc = "Bit 23 - LOCK23"]
    #[inline(always)]
    pub fn lock23(&mut self) -> LOCK23_W {
        LOCK23_W { w: self }
    }
    #[doc = "Bit 24 - LOCK24"]
    #[inline(always)]
    pub fn lock24(&mut self) -> LOCK24_W {
        LOCK24_W { w: self }
    }
    #[doc = "Bit 25 - LOCK25"]
    #[inline(always)]
    pub fn lock25(&mut self) -> LOCK25_W {
        LOCK25_W { w: self }
    }
    #[doc = "Bit 26 - LOCK26"]
    #[inline(always)]
    pub fn lock26(&mut self) -> LOCK26_W {
        LOCK26_W { w: self }
    }
    #[doc = "Bit 27 - LOCK27"]
    #[inline(always)]
    pub fn lock27(&mut self) -> LOCK27_W {
        LOCK27_W { w: self }
    }
    #[doc = "Bit 28 - LOCK28"]
    #[inline(always)]
    pub fn lock28(&mut self) -> LOCK28_W {
        LOCK28_W { w: self }
    }
    #[doc = "Bit 29 - LOCK29"]
    #[inline(always)]
    pub fn lock29(&mut self) -> LOCK29_W {
        LOCK29_W { w: self }
    }
    #[doc = "Bit 30 - LOCK30"]
    #[inline(always)]
    pub fn lock30(&mut self) -> LOCK30_W {
        LOCK30_W { w: self }
    }
    #[doc = "Bit 31 - LOCK31"]
    #[inline(always)]
    pub fn lock31(&mut self) -> LOCK31_W {
        LOCK31_W { w: self }
    }
}
