#[doc = "Reader of register IOASCR"]
pub type R = crate::R<u32, super::IOASCR>;
#[doc = "Writer for register IOASCR"]
pub type W = crate::W<u32, super::IOASCR>;
#[doc = "Register IOASCR `reset()`'s with value 0"]
impl crate::ResetValue for super::IOASCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `G8_IO4`"]
pub type G8_IO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G8_IO4`"]
pub struct G8_IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> G8_IO4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G8_IO3`"]
pub type G8_IO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G8_IO3`"]
pub struct G8_IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> G8_IO3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G8_IO2`"]
pub type G8_IO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G8_IO2`"]
pub struct G8_IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> G8_IO2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G8_IO1`"]
pub type G8_IO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G8_IO1`"]
pub struct G8_IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> G8_IO1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G7_IO4`"]
pub type G7_IO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G7_IO4`"]
pub struct G7_IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> G7_IO4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G7_IO3`"]
pub type G7_IO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G7_IO3`"]
pub struct G7_IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> G7_IO3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G7_IO2`"]
pub type G7_IO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G7_IO2`"]
pub struct G7_IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> G7_IO2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G7_IO1`"]
pub type G7_IO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G7_IO1`"]
pub struct G7_IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> G7_IO1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G6_IO4`"]
pub type G6_IO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G6_IO4`"]
pub struct G6_IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> G6_IO4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G6_IO3`"]
pub type G6_IO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G6_IO3`"]
pub struct G6_IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> G6_IO3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G6_IO2`"]
pub type G6_IO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G6_IO2`"]
pub struct G6_IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> G6_IO2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G6_IO1`"]
pub type G6_IO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G6_IO1`"]
pub struct G6_IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> G6_IO1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G5_IO4`"]
pub type G5_IO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G5_IO4`"]
pub struct G5_IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> G5_IO4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G5_IO3`"]
pub type G5_IO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G5_IO3`"]
pub struct G5_IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> G5_IO3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G5_IO2`"]
pub type G5_IO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G5_IO2`"]
pub struct G5_IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> G5_IO2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G5_IO1`"]
pub type G5_IO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G5_IO1`"]
pub struct G5_IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> G5_IO1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G4_IO4`"]
pub type G4_IO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G4_IO4`"]
pub struct G4_IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> G4_IO4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G4_IO3`"]
pub type G4_IO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G4_IO3`"]
pub struct G4_IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> G4_IO3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G4_IO2`"]
pub type G4_IO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G4_IO2`"]
pub struct G4_IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> G4_IO2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G4_IO1`"]
pub type G4_IO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G4_IO1`"]
pub struct G4_IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> G4_IO1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G3_IO4`"]
pub type G3_IO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G3_IO4`"]
pub struct G3_IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> G3_IO4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G3_IO3`"]
pub type G3_IO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G3_IO3`"]
pub struct G3_IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> G3_IO3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G3_IO2`"]
pub type G3_IO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G3_IO2`"]
pub struct G3_IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> G3_IO2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G3_IO1`"]
pub type G3_IO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G3_IO1`"]
pub struct G3_IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> G3_IO1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G2_IO4`"]
pub type G2_IO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G2_IO4`"]
pub struct G2_IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> G2_IO4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G2_IO3`"]
pub type G2_IO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G2_IO3`"]
pub struct G2_IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> G2_IO3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G2_IO2`"]
pub type G2_IO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G2_IO2`"]
pub struct G2_IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> G2_IO2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G2_IO1`"]
pub type G2_IO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G2_IO1`"]
pub struct G2_IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> G2_IO1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G1_IO4`"]
pub type G1_IO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G1_IO4`"]
pub struct G1_IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> G1_IO4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G1_IO3`"]
pub type G1_IO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G1_IO3`"]
pub struct G1_IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> G1_IO3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G1_IO2`"]
pub type G1_IO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G1_IO2`"]
pub struct G1_IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> G1_IO2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `G1_IO1`"]
pub type G1_IO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `G1_IO1`"]
pub struct G1_IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> G1_IO1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 31 - G8_IO4"]
    #[inline(always)]
    pub fn g8_io4(&self) -> G8_IO4_R {
        G8_IO4_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - G8_IO3"]
    #[inline(always)]
    pub fn g8_io3(&self) -> G8_IO3_R {
        G8_IO3_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - G8_IO2"]
    #[inline(always)]
    pub fn g8_io2(&self) -> G8_IO2_R {
        G8_IO2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - G8_IO1"]
    #[inline(always)]
    pub fn g8_io1(&self) -> G8_IO1_R {
        G8_IO1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - G7_IO4"]
    #[inline(always)]
    pub fn g7_io4(&self) -> G7_IO4_R {
        G7_IO4_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - G7_IO3"]
    #[inline(always)]
    pub fn g7_io3(&self) -> G7_IO3_R {
        G7_IO3_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - G7_IO2"]
    #[inline(always)]
    pub fn g7_io2(&self) -> G7_IO2_R {
        G7_IO2_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - G7_IO1"]
    #[inline(always)]
    pub fn g7_io1(&self) -> G7_IO1_R {
        G7_IO1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - G6_IO4"]
    #[inline(always)]
    pub fn g6_io4(&self) -> G6_IO4_R {
        G6_IO4_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - G6_IO3"]
    #[inline(always)]
    pub fn g6_io3(&self) -> G6_IO3_R {
        G6_IO3_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - G6_IO2"]
    #[inline(always)]
    pub fn g6_io2(&self) -> G6_IO2_R {
        G6_IO2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - G6_IO1"]
    #[inline(always)]
    pub fn g6_io1(&self) -> G6_IO1_R {
        G6_IO1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - G5_IO4"]
    #[inline(always)]
    pub fn g5_io4(&self) -> G5_IO4_R {
        G5_IO4_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - G5_IO3"]
    #[inline(always)]
    pub fn g5_io3(&self) -> G5_IO3_R {
        G5_IO3_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - G5_IO2"]
    #[inline(always)]
    pub fn g5_io2(&self) -> G5_IO2_R {
        G5_IO2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - G5_IO1"]
    #[inline(always)]
    pub fn g5_io1(&self) -> G5_IO1_R {
        G5_IO1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - G4_IO4"]
    #[inline(always)]
    pub fn g4_io4(&self) -> G4_IO4_R {
        G4_IO4_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - G4_IO3"]
    #[inline(always)]
    pub fn g4_io3(&self) -> G4_IO3_R {
        G4_IO3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - G4_IO2"]
    #[inline(always)]
    pub fn g4_io2(&self) -> G4_IO2_R {
        G4_IO2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - G4_IO1"]
    #[inline(always)]
    pub fn g4_io1(&self) -> G4_IO1_R {
        G4_IO1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - G3_IO4"]
    #[inline(always)]
    pub fn g3_io4(&self) -> G3_IO4_R {
        G3_IO4_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - G3_IO3"]
    #[inline(always)]
    pub fn g3_io3(&self) -> G3_IO3_R {
        G3_IO3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - G3_IO2"]
    #[inline(always)]
    pub fn g3_io2(&self) -> G3_IO2_R {
        G3_IO2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - G3_IO1"]
    #[inline(always)]
    pub fn g3_io1(&self) -> G3_IO1_R {
        G3_IO1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - G2_IO4"]
    #[inline(always)]
    pub fn g2_io4(&self) -> G2_IO4_R {
        G2_IO4_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - G2_IO3"]
    #[inline(always)]
    pub fn g2_io3(&self) -> G2_IO3_R {
        G2_IO3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - G2_IO2"]
    #[inline(always)]
    pub fn g2_io2(&self) -> G2_IO2_R {
        G2_IO2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - G2_IO1"]
    #[inline(always)]
    pub fn g2_io1(&self) -> G2_IO1_R {
        G2_IO1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - G1_IO4"]
    #[inline(always)]
    pub fn g1_io4(&self) -> G1_IO4_R {
        G1_IO4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - G1_IO3"]
    #[inline(always)]
    pub fn g1_io3(&self) -> G1_IO3_R {
        G1_IO3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - G1_IO2"]
    #[inline(always)]
    pub fn g1_io2(&self) -> G1_IO2_R {
        G1_IO2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - G1_IO1"]
    #[inline(always)]
    pub fn g1_io1(&self) -> G1_IO1_R {
        G1_IO1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - G8_IO4"]
    #[inline(always)]
    pub fn g8_io4(&mut self) -> G8_IO4_W {
        G8_IO4_W { w: self }
    }
    #[doc = "Bit 30 - G8_IO3"]
    #[inline(always)]
    pub fn g8_io3(&mut self) -> G8_IO3_W {
        G8_IO3_W { w: self }
    }
    #[doc = "Bit 29 - G8_IO2"]
    #[inline(always)]
    pub fn g8_io2(&mut self) -> G8_IO2_W {
        G8_IO2_W { w: self }
    }
    #[doc = "Bit 28 - G8_IO1"]
    #[inline(always)]
    pub fn g8_io1(&mut self) -> G8_IO1_W {
        G8_IO1_W { w: self }
    }
    #[doc = "Bit 27 - G7_IO4"]
    #[inline(always)]
    pub fn g7_io4(&mut self) -> G7_IO4_W {
        G7_IO4_W { w: self }
    }
    #[doc = "Bit 26 - G7_IO3"]
    #[inline(always)]
    pub fn g7_io3(&mut self) -> G7_IO3_W {
        G7_IO3_W { w: self }
    }
    #[doc = "Bit 25 - G7_IO2"]
    #[inline(always)]
    pub fn g7_io2(&mut self) -> G7_IO2_W {
        G7_IO2_W { w: self }
    }
    #[doc = "Bit 24 - G7_IO1"]
    #[inline(always)]
    pub fn g7_io1(&mut self) -> G7_IO1_W {
        G7_IO1_W { w: self }
    }
    #[doc = "Bit 23 - G6_IO4"]
    #[inline(always)]
    pub fn g6_io4(&mut self) -> G6_IO4_W {
        G6_IO4_W { w: self }
    }
    #[doc = "Bit 22 - G6_IO3"]
    #[inline(always)]
    pub fn g6_io3(&mut self) -> G6_IO3_W {
        G6_IO3_W { w: self }
    }
    #[doc = "Bit 21 - G6_IO2"]
    #[inline(always)]
    pub fn g6_io2(&mut self) -> G6_IO2_W {
        G6_IO2_W { w: self }
    }
    #[doc = "Bit 20 - G6_IO1"]
    #[inline(always)]
    pub fn g6_io1(&mut self) -> G6_IO1_W {
        G6_IO1_W { w: self }
    }
    #[doc = "Bit 19 - G5_IO4"]
    #[inline(always)]
    pub fn g5_io4(&mut self) -> G5_IO4_W {
        G5_IO4_W { w: self }
    }
    #[doc = "Bit 18 - G5_IO3"]
    #[inline(always)]
    pub fn g5_io3(&mut self) -> G5_IO3_W {
        G5_IO3_W { w: self }
    }
    #[doc = "Bit 17 - G5_IO2"]
    #[inline(always)]
    pub fn g5_io2(&mut self) -> G5_IO2_W {
        G5_IO2_W { w: self }
    }
    #[doc = "Bit 16 - G5_IO1"]
    #[inline(always)]
    pub fn g5_io1(&mut self) -> G5_IO1_W {
        G5_IO1_W { w: self }
    }
    #[doc = "Bit 15 - G4_IO4"]
    #[inline(always)]
    pub fn g4_io4(&mut self) -> G4_IO4_W {
        G4_IO4_W { w: self }
    }
    #[doc = "Bit 14 - G4_IO3"]
    #[inline(always)]
    pub fn g4_io3(&mut self) -> G4_IO3_W {
        G4_IO3_W { w: self }
    }
    #[doc = "Bit 13 - G4_IO2"]
    #[inline(always)]
    pub fn g4_io2(&mut self) -> G4_IO2_W {
        G4_IO2_W { w: self }
    }
    #[doc = "Bit 12 - G4_IO1"]
    #[inline(always)]
    pub fn g4_io1(&mut self) -> G4_IO1_W {
        G4_IO1_W { w: self }
    }
    #[doc = "Bit 11 - G3_IO4"]
    #[inline(always)]
    pub fn g3_io4(&mut self) -> G3_IO4_W {
        G3_IO4_W { w: self }
    }
    #[doc = "Bit 10 - G3_IO3"]
    #[inline(always)]
    pub fn g3_io3(&mut self) -> G3_IO3_W {
        G3_IO3_W { w: self }
    }
    #[doc = "Bit 9 - G3_IO2"]
    #[inline(always)]
    pub fn g3_io2(&mut self) -> G3_IO2_W {
        G3_IO2_W { w: self }
    }
    #[doc = "Bit 8 - G3_IO1"]
    #[inline(always)]
    pub fn g3_io1(&mut self) -> G3_IO1_W {
        G3_IO1_W { w: self }
    }
    #[doc = "Bit 7 - G2_IO4"]
    #[inline(always)]
    pub fn g2_io4(&mut self) -> G2_IO4_W {
        G2_IO4_W { w: self }
    }
    #[doc = "Bit 6 - G2_IO3"]
    #[inline(always)]
    pub fn g2_io3(&mut self) -> G2_IO3_W {
        G2_IO3_W { w: self }
    }
    #[doc = "Bit 5 - G2_IO2"]
    #[inline(always)]
    pub fn g2_io2(&mut self) -> G2_IO2_W {
        G2_IO2_W { w: self }
    }
    #[doc = "Bit 4 - G2_IO1"]
    #[inline(always)]
    pub fn g2_io1(&mut self) -> G2_IO1_W {
        G2_IO1_W { w: self }
    }
    #[doc = "Bit 3 - G1_IO4"]
    #[inline(always)]
    pub fn g1_io4(&mut self) -> G1_IO4_W {
        G1_IO4_W { w: self }
    }
    #[doc = "Bit 2 - G1_IO3"]
    #[inline(always)]
    pub fn g1_io3(&mut self) -> G1_IO3_W {
        G1_IO3_W { w: self }
    }
    #[doc = "Bit 1 - G1_IO2"]
    #[inline(always)]
    pub fn g1_io2(&mut self) -> G1_IO2_W {
        G1_IO2_W { w: self }
    }
    #[doc = "Bit 0 - G1_IO1"]
    #[inline(always)]
    pub fn g1_io1(&mut self) -> G1_IO1_W {
        G1_IO1_W { w: self }
    }
}
