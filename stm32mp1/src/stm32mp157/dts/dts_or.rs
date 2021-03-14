#[doc = "Reader of register DTS_OR"]
pub type R = crate::R<u32, super::DTS_OR>;
#[doc = "Writer for register DTS_OR"]
pub type W = crate::W<u32, super::DTS_OR>;
#[doc = "Register DTS_OR `reset()`'s with value 0"]
impl crate::ResetValue for super::DTS_OR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TS_Op0`"]
pub type TS_OP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op0`"]
pub struct TS_OP0_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op1`"]
pub type TS_OP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op1`"]
pub struct TS_OP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op2`"]
pub type TS_OP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op2`"]
pub struct TS_OP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op3`"]
pub type TS_OP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op3`"]
pub struct TS_OP3_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op4`"]
pub type TS_OP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op4`"]
pub struct TS_OP4_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op5`"]
pub type TS_OP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op5`"]
pub struct TS_OP5_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op6`"]
pub type TS_OP6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op6`"]
pub struct TS_OP6_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op7`"]
pub type TS_OP7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op7`"]
pub struct TS_OP7_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op8`"]
pub type TS_OP8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op8`"]
pub struct TS_OP8_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op9`"]
pub type TS_OP9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op9`"]
pub struct TS_OP9_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op10`"]
pub type TS_OP10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op10`"]
pub struct TS_OP10_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op11`"]
pub type TS_OP11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op11`"]
pub struct TS_OP11_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op12`"]
pub type TS_OP12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op12`"]
pub struct TS_OP12_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op13`"]
pub type TS_OP13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op13`"]
pub struct TS_OP13_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op14`"]
pub type TS_OP14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op14`"]
pub struct TS_OP14_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op15`"]
pub type TS_OP15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op15`"]
pub struct TS_OP15_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op16`"]
pub type TS_OP16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op16`"]
pub struct TS_OP16_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op17`"]
pub type TS_OP17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op17`"]
pub struct TS_OP17_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op18`"]
pub type TS_OP18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op18`"]
pub struct TS_OP18_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op19`"]
pub type TS_OP19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op19`"]
pub struct TS_OP19_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op20`"]
pub type TS_OP20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op20`"]
pub struct TS_OP20_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op21`"]
pub type TS_OP21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op21`"]
pub struct TS_OP21_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op22`"]
pub type TS_OP22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op22`"]
pub struct TS_OP22_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op23`"]
pub type TS_OP23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op23`"]
pub struct TS_OP23_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op24`"]
pub type TS_OP24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op24`"]
pub struct TS_OP24_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op25`"]
pub type TS_OP25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op25`"]
pub struct TS_OP25_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op26`"]
pub type TS_OP26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op26`"]
pub struct TS_OP26_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op27`"]
pub type TS_OP27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op27`"]
pub struct TS_OP27_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op28`"]
pub type TS_OP28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op28`"]
pub struct TS_OP28_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op29`"]
pub type TS_OP29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op29`"]
pub struct TS_OP29_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op30`"]
pub type TS_OP30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op30`"]
pub struct TS_OP30_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `TS_Op31`"]
pub type TS_OP31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS_Op31`"]
pub struct TS_OP31_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_OP31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - TS_Op0"]
    #[inline(always)]
    pub fn ts_op0(&self) -> TS_OP0_R {
        TS_OP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TS_Op1"]
    #[inline(always)]
    pub fn ts_op1(&self) -> TS_OP1_R {
        TS_OP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TS_Op2"]
    #[inline(always)]
    pub fn ts_op2(&self) -> TS_OP2_R {
        TS_OP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TS_Op3"]
    #[inline(always)]
    pub fn ts_op3(&self) -> TS_OP3_R {
        TS_OP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TS_Op4"]
    #[inline(always)]
    pub fn ts_op4(&self) -> TS_OP4_R {
        TS_OP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TS_Op5"]
    #[inline(always)]
    pub fn ts_op5(&self) -> TS_OP5_R {
        TS_OP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TS_Op6"]
    #[inline(always)]
    pub fn ts_op6(&self) -> TS_OP6_R {
        TS_OP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TS_Op7"]
    #[inline(always)]
    pub fn ts_op7(&self) -> TS_OP7_R {
        TS_OP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TS_Op8"]
    #[inline(always)]
    pub fn ts_op8(&self) -> TS_OP8_R {
        TS_OP8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TS_Op9"]
    #[inline(always)]
    pub fn ts_op9(&self) -> TS_OP9_R {
        TS_OP9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TS_Op10"]
    #[inline(always)]
    pub fn ts_op10(&self) -> TS_OP10_R {
        TS_OP10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TS_Op11"]
    #[inline(always)]
    pub fn ts_op11(&self) -> TS_OP11_R {
        TS_OP11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TS_Op12"]
    #[inline(always)]
    pub fn ts_op12(&self) -> TS_OP12_R {
        TS_OP12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TS_Op13"]
    #[inline(always)]
    pub fn ts_op13(&self) -> TS_OP13_R {
        TS_OP13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TS_Op14"]
    #[inline(always)]
    pub fn ts_op14(&self) -> TS_OP14_R {
        TS_OP14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TS_Op15"]
    #[inline(always)]
    pub fn ts_op15(&self) -> TS_OP15_R {
        TS_OP15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TS_Op16"]
    #[inline(always)]
    pub fn ts_op16(&self) -> TS_OP16_R {
        TS_OP16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TS_Op17"]
    #[inline(always)]
    pub fn ts_op17(&self) -> TS_OP17_R {
        TS_OP17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TS_Op18"]
    #[inline(always)]
    pub fn ts_op18(&self) -> TS_OP18_R {
        TS_OP18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TS_Op19"]
    #[inline(always)]
    pub fn ts_op19(&self) -> TS_OP19_R {
        TS_OP19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TS_Op20"]
    #[inline(always)]
    pub fn ts_op20(&self) -> TS_OP20_R {
        TS_OP20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TS_Op21"]
    #[inline(always)]
    pub fn ts_op21(&self) -> TS_OP21_R {
        TS_OP21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - TS_Op22"]
    #[inline(always)]
    pub fn ts_op22(&self) -> TS_OP22_R {
        TS_OP22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TS_Op23"]
    #[inline(always)]
    pub fn ts_op23(&self) -> TS_OP23_R {
        TS_OP23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TS_Op24"]
    #[inline(always)]
    pub fn ts_op24(&self) -> TS_OP24_R {
        TS_OP24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TS_Op25"]
    #[inline(always)]
    pub fn ts_op25(&self) -> TS_OP25_R {
        TS_OP25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TS_Op26"]
    #[inline(always)]
    pub fn ts_op26(&self) -> TS_OP26_R {
        TS_OP26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TS_Op27"]
    #[inline(always)]
    pub fn ts_op27(&self) -> TS_OP27_R {
        TS_OP27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TS_Op28"]
    #[inline(always)]
    pub fn ts_op28(&self) -> TS_OP28_R {
        TS_OP28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TS_Op29"]
    #[inline(always)]
    pub fn ts_op29(&self) -> TS_OP29_R {
        TS_OP29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TS_Op30"]
    #[inline(always)]
    pub fn ts_op30(&self) -> TS_OP30_R {
        TS_OP30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - TS_Op31"]
    #[inline(always)]
    pub fn ts_op31(&self) -> TS_OP31_R {
        TS_OP31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TS_Op0"]
    #[inline(always)]
    pub fn ts_op0(&mut self) -> TS_OP0_W {
        TS_OP0_W { w: self }
    }
    #[doc = "Bit 1 - TS_Op1"]
    #[inline(always)]
    pub fn ts_op1(&mut self) -> TS_OP1_W {
        TS_OP1_W { w: self }
    }
    #[doc = "Bit 2 - TS_Op2"]
    #[inline(always)]
    pub fn ts_op2(&mut self) -> TS_OP2_W {
        TS_OP2_W { w: self }
    }
    #[doc = "Bit 3 - TS_Op3"]
    #[inline(always)]
    pub fn ts_op3(&mut self) -> TS_OP3_W {
        TS_OP3_W { w: self }
    }
    #[doc = "Bit 4 - TS_Op4"]
    #[inline(always)]
    pub fn ts_op4(&mut self) -> TS_OP4_W {
        TS_OP4_W { w: self }
    }
    #[doc = "Bit 5 - TS_Op5"]
    #[inline(always)]
    pub fn ts_op5(&mut self) -> TS_OP5_W {
        TS_OP5_W { w: self }
    }
    #[doc = "Bit 6 - TS_Op6"]
    #[inline(always)]
    pub fn ts_op6(&mut self) -> TS_OP6_W {
        TS_OP6_W { w: self }
    }
    #[doc = "Bit 7 - TS_Op7"]
    #[inline(always)]
    pub fn ts_op7(&mut self) -> TS_OP7_W {
        TS_OP7_W { w: self }
    }
    #[doc = "Bit 8 - TS_Op8"]
    #[inline(always)]
    pub fn ts_op8(&mut self) -> TS_OP8_W {
        TS_OP8_W { w: self }
    }
    #[doc = "Bit 9 - TS_Op9"]
    #[inline(always)]
    pub fn ts_op9(&mut self) -> TS_OP9_W {
        TS_OP9_W { w: self }
    }
    #[doc = "Bit 10 - TS_Op10"]
    #[inline(always)]
    pub fn ts_op10(&mut self) -> TS_OP10_W {
        TS_OP10_W { w: self }
    }
    #[doc = "Bit 11 - TS_Op11"]
    #[inline(always)]
    pub fn ts_op11(&mut self) -> TS_OP11_W {
        TS_OP11_W { w: self }
    }
    #[doc = "Bit 12 - TS_Op12"]
    #[inline(always)]
    pub fn ts_op12(&mut self) -> TS_OP12_W {
        TS_OP12_W { w: self }
    }
    #[doc = "Bit 13 - TS_Op13"]
    #[inline(always)]
    pub fn ts_op13(&mut self) -> TS_OP13_W {
        TS_OP13_W { w: self }
    }
    #[doc = "Bit 14 - TS_Op14"]
    #[inline(always)]
    pub fn ts_op14(&mut self) -> TS_OP14_W {
        TS_OP14_W { w: self }
    }
    #[doc = "Bit 15 - TS_Op15"]
    #[inline(always)]
    pub fn ts_op15(&mut self) -> TS_OP15_W {
        TS_OP15_W { w: self }
    }
    #[doc = "Bit 16 - TS_Op16"]
    #[inline(always)]
    pub fn ts_op16(&mut self) -> TS_OP16_W {
        TS_OP16_W { w: self }
    }
    #[doc = "Bit 17 - TS_Op17"]
    #[inline(always)]
    pub fn ts_op17(&mut self) -> TS_OP17_W {
        TS_OP17_W { w: self }
    }
    #[doc = "Bit 18 - TS_Op18"]
    #[inline(always)]
    pub fn ts_op18(&mut self) -> TS_OP18_W {
        TS_OP18_W { w: self }
    }
    #[doc = "Bit 19 - TS_Op19"]
    #[inline(always)]
    pub fn ts_op19(&mut self) -> TS_OP19_W {
        TS_OP19_W { w: self }
    }
    #[doc = "Bit 20 - TS_Op20"]
    #[inline(always)]
    pub fn ts_op20(&mut self) -> TS_OP20_W {
        TS_OP20_W { w: self }
    }
    #[doc = "Bit 21 - TS_Op21"]
    #[inline(always)]
    pub fn ts_op21(&mut self) -> TS_OP21_W {
        TS_OP21_W { w: self }
    }
    #[doc = "Bit 22 - TS_Op22"]
    #[inline(always)]
    pub fn ts_op22(&mut self) -> TS_OP22_W {
        TS_OP22_W { w: self }
    }
    #[doc = "Bit 23 - TS_Op23"]
    #[inline(always)]
    pub fn ts_op23(&mut self) -> TS_OP23_W {
        TS_OP23_W { w: self }
    }
    #[doc = "Bit 24 - TS_Op24"]
    #[inline(always)]
    pub fn ts_op24(&mut self) -> TS_OP24_W {
        TS_OP24_W { w: self }
    }
    #[doc = "Bit 25 - TS_Op25"]
    #[inline(always)]
    pub fn ts_op25(&mut self) -> TS_OP25_W {
        TS_OP25_W { w: self }
    }
    #[doc = "Bit 26 - TS_Op26"]
    #[inline(always)]
    pub fn ts_op26(&mut self) -> TS_OP26_W {
        TS_OP26_W { w: self }
    }
    #[doc = "Bit 27 - TS_Op27"]
    #[inline(always)]
    pub fn ts_op27(&mut self) -> TS_OP27_W {
        TS_OP27_W { w: self }
    }
    #[doc = "Bit 28 - TS_Op28"]
    #[inline(always)]
    pub fn ts_op28(&mut self) -> TS_OP28_W {
        TS_OP28_W { w: self }
    }
    #[doc = "Bit 29 - TS_Op29"]
    #[inline(always)]
    pub fn ts_op29(&mut self) -> TS_OP29_W {
        TS_OP29_W { w: self }
    }
    #[doc = "Bit 30 - TS_Op30"]
    #[inline(always)]
    pub fn ts_op30(&mut self) -> TS_OP30_W {
        TS_OP30_W { w: self }
    }
    #[doc = "Bit 31 - TS_Op31"]
    #[inline(always)]
    pub fn ts_op31(&mut self) -> TS_OP31_W {
        TS_OP31_W { w: self }
    }
}
