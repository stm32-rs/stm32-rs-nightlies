#[doc = "Reader of register ASCR1"]
pub type R = crate::R<u32, super::ASCR1>;
#[doc = "Writer for register ASCR1"]
pub type W = crate::W<u32, super::ASCR1>;
#[doc = "Register ASCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ASCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCM`"]
pub type SCM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCM`"]
pub struct SCM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH30GR11_4`"]
pub type CH30GR11_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH30GR11_4`"]
pub struct CH30GR11_4_W<'a> {
    w: &'a mut W,
}
impl<'a> CH30GR11_4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH29GR11_3`"]
pub type CH29GR11_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH29GR11_3`"]
pub struct CH29GR11_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH29GR11_3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH28GR11_2`"]
pub type CH28GR11_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH28GR11_2`"]
pub struct CH28GR11_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH28GR11_2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH27GR11_1`"]
pub type CH27GR11_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH27GR11_1`"]
pub struct CH27GR11_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH27GR11_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `VCOMP`"]
pub type VCOMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VCOMP`"]
pub struct VCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOMP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH25`"]
pub type CH25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH25`"]
pub struct CH25_W<'a> {
    w: &'a mut W,
}
impl<'a> CH25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH24`"]
pub type CH24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH24`"]
pub struct CH24_W<'a> {
    w: &'a mut W,
}
impl<'a> CH24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH23`"]
pub type CH23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH23`"]
pub struct CH23_W<'a> {
    w: &'a mut W,
}
impl<'a> CH23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH22`"]
pub type CH22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH22`"]
pub struct CH22_W<'a> {
    w: &'a mut W,
}
impl<'a> CH22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH21GR7_4`"]
pub type CH21GR7_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH21GR7_4`"]
pub struct CH21GR7_4_W<'a> {
    w: &'a mut W,
}
impl<'a> CH21GR7_4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH20GR7_3`"]
pub type CH20GR7_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH20GR7_3`"]
pub struct CH20GR7_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH20GR7_3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH19GR7_2`"]
pub type CH19GR7_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH19GR7_2`"]
pub struct CH19GR7_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH19GR7_2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH18GR7_1`"]
pub type CH18GR7_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH18GR7_1`"]
pub struct CH18GR7_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH18GR7_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH31GR7_1`"]
pub type CH31GR7_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH31GR7_1`"]
pub struct CH31GR7_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH31GR7_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH15GR9_2`"]
pub type CH15GR9_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH15GR9_2`"]
pub struct CH15GR9_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH15GR9_2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH14GR9_1`"]
pub type CH14GR9_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH14GR9_1`"]
pub struct CH14GR9_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH14GR9_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH13GR8_4`"]
pub type CH13GR8_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH13GR8_4`"]
pub struct CH13GR8_4_W<'a> {
    w: &'a mut W,
}
impl<'a> CH13GR8_4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH12GR8_3`"]
pub type CH12GR8_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH12GR8_3`"]
pub struct CH12GR8_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH12GR8_3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH11GR8_2`"]
pub type CH11GR8_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH11GR8_2`"]
pub struct CH11GR8_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH11GR8_2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH10GR8_1`"]
pub type CH10GR8_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH10GR8_1`"]
pub struct CH10GR8_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH10GR8_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH9GR3_2`"]
pub type CH9GR3_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH9GR3_2`"]
pub struct CH9GR3_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH9GR3_2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH8GR3_1`"]
pub type CH8GR3_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH8GR3_1`"]
pub struct CH8GR3_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH8GR3_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH7GR2_2`"]
pub type CH7GR2_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH7GR2_2`"]
pub struct CH7GR2_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7GR2_2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH6GR2_1`"]
pub type CH6GR2_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH6GR2_1`"]
pub struct CH6GR2_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6GR2_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `COMP1_SW1`"]
pub type COMP1_SW1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP1_SW1`"]
pub struct COMP1_SW1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_SW1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH31GR11_5`"]
pub type CH31GR11_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH31GR11_5`"]
pub struct CH31GR11_5_W<'a> {
    w: &'a mut W,
}
impl<'a> CH31GR11_5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH3GR1_4`"]
pub type CH3GR1_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3GR1_4`"]
pub struct CH3GR1_4_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3GR1_4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH2GR1_3`"]
pub type CH2GR1_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2GR1_3`"]
pub struct CH2GR1_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2GR1_3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH1GR1_2`"]
pub type CH1GR1_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1GR1_2`"]
pub struct CH1GR1_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1GR1_2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `CH0GR1_1`"]
pub type CH0GR1_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0GR1_1`"]
pub struct CH0GR1_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0GR1_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 31 - Switch control mode"]
    #[inline(always)]
    pub fn scm(&self) -> SCM_R {
        SCM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Analog switch control"]
    #[inline(always)]
    pub fn ch30gr11_4(&self) -> CH30GR11_4_R {
        CH30GR11_4_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Analog switch control"]
    #[inline(always)]
    pub fn ch29gr11_3(&self) -> CH29GR11_3_R {
        CH29GR11_3_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Analog switch control"]
    #[inline(always)]
    pub fn ch28gr11_2(&self) -> CH28GR11_2_R {
        CH28GR11_2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Analog switch control"]
    #[inline(always)]
    pub fn ch27gr11_1(&self) -> CH27GR11_1_R {
        CH27GR11_1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - ADC analog switch selection for internal node to comparator 1"]
    #[inline(always)]
    pub fn vcomp(&self) -> VCOMP_R {
        VCOMP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Analog I/O switch control of channel CH25"]
    #[inline(always)]
    pub fn ch25(&self) -> CH25_R {
        CH25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Analog I/O switch control of channel CH24"]
    #[inline(always)]
    pub fn ch24(&self) -> CH24_R {
        CH24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Analog I/O switch control of channel CH23"]
    #[inline(always)]
    pub fn ch23(&self) -> CH23_R {
        CH23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Analog I/O switch control of channel CH22"]
    #[inline(always)]
    pub fn ch22(&self) -> CH22_R {
        CH22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Analog switch control"]
    #[inline(always)]
    pub fn ch21gr7_4(&self) -> CH21GR7_4_R {
        CH21GR7_4_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Analog switch control"]
    #[inline(always)]
    pub fn ch20gr7_3(&self) -> CH20GR7_3_R {
        CH20GR7_3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Analog switch control"]
    #[inline(always)]
    pub fn ch19gr7_2(&self) -> CH19GR7_2_R {
        CH19GR7_2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Analog switch control"]
    #[inline(always)]
    pub fn ch18gr7_1(&self) -> CH18GR7_1_R {
        CH18GR7_1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Analog switch control"]
    #[inline(always)]
    pub fn ch31gr7_1(&self) -> CH31GR7_1_R {
        CH31GR7_1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Analog switch control"]
    #[inline(always)]
    pub fn ch15gr9_2(&self) -> CH15GR9_2_R {
        CH15GR9_2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Analog switch control"]
    #[inline(always)]
    pub fn ch14gr9_1(&self) -> CH14GR9_1_R {
        CH14GR9_1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Analog switch control"]
    #[inline(always)]
    pub fn ch13gr8_4(&self) -> CH13GR8_4_R {
        CH13GR8_4_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Analog switch control"]
    #[inline(always)]
    pub fn ch12gr8_3(&self) -> CH12GR8_3_R {
        CH12GR8_3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Analog switch control"]
    #[inline(always)]
    pub fn ch11gr8_2(&self) -> CH11GR8_2_R {
        CH11GR8_2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Analog switch control"]
    #[inline(always)]
    pub fn ch10gr8_1(&self) -> CH10GR8_1_R {
        CH10GR8_1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Analog switch control"]
    #[inline(always)]
    pub fn ch9gr3_2(&self) -> CH9GR3_2_R {
        CH9GR3_2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Analog switch control"]
    #[inline(always)]
    pub fn ch8gr3_1(&self) -> CH8GR3_1_R {
        CH8GR3_1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Analog switch control"]
    #[inline(always)]
    pub fn ch7gr2_2(&self) -> CH7GR2_2_R {
        CH7GR2_2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Analog switch control"]
    #[inline(always)]
    pub fn ch6gr2_1(&self) -> CH6GR2_1_R {
        CH6GR2_1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comparator 1 analog switch"]
    #[inline(always)]
    pub fn comp1_sw1(&self) -> COMP1_SW1_R {
        COMP1_SW1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Analog switch control"]
    #[inline(always)]
    pub fn ch31gr11_5(&self) -> CH31GR11_5_R {
        CH31GR11_5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Analog switch control"]
    #[inline(always)]
    pub fn ch3gr1_4(&self) -> CH3GR1_4_R {
        CH3GR1_4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Analog switch control"]
    #[inline(always)]
    pub fn ch2gr1_3(&self) -> CH2GR1_3_R {
        CH2GR1_3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Analog switch control"]
    #[inline(always)]
    pub fn ch1gr1_2(&self) -> CH1GR1_2_R {
        CH1GR1_2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Analog switch control"]
    #[inline(always)]
    pub fn ch0gr1_1(&self) -> CH0GR1_1_R {
        CH0GR1_1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Switch control mode"]
    #[inline(always)]
    pub fn scm(&mut self) -> SCM_W {
        SCM_W { w: self }
    }
    #[doc = "Bit 30 - Analog switch control"]
    #[inline(always)]
    pub fn ch30gr11_4(&mut self) -> CH30GR11_4_W {
        CH30GR11_4_W { w: self }
    }
    #[doc = "Bit 29 - Analog switch control"]
    #[inline(always)]
    pub fn ch29gr11_3(&mut self) -> CH29GR11_3_W {
        CH29GR11_3_W { w: self }
    }
    #[doc = "Bit 28 - Analog switch control"]
    #[inline(always)]
    pub fn ch28gr11_2(&mut self) -> CH28GR11_2_W {
        CH28GR11_2_W { w: self }
    }
    #[doc = "Bit 27 - Analog switch control"]
    #[inline(always)]
    pub fn ch27gr11_1(&mut self) -> CH27GR11_1_W {
        CH27GR11_1_W { w: self }
    }
    #[doc = "Bit 26 - ADC analog switch selection for internal node to comparator 1"]
    #[inline(always)]
    pub fn vcomp(&mut self) -> VCOMP_W {
        VCOMP_W { w: self }
    }
    #[doc = "Bit 25 - Analog I/O switch control of channel CH25"]
    #[inline(always)]
    pub fn ch25(&mut self) -> CH25_W {
        CH25_W { w: self }
    }
    #[doc = "Bit 24 - Analog I/O switch control of channel CH24"]
    #[inline(always)]
    pub fn ch24(&mut self) -> CH24_W {
        CH24_W { w: self }
    }
    #[doc = "Bit 23 - Analog I/O switch control of channel CH23"]
    #[inline(always)]
    pub fn ch23(&mut self) -> CH23_W {
        CH23_W { w: self }
    }
    #[doc = "Bit 22 - Analog I/O switch control of channel CH22"]
    #[inline(always)]
    pub fn ch22(&mut self) -> CH22_W {
        CH22_W { w: self }
    }
    #[doc = "Bit 21 - Analog switch control"]
    #[inline(always)]
    pub fn ch21gr7_4(&mut self) -> CH21GR7_4_W {
        CH21GR7_4_W { w: self }
    }
    #[doc = "Bit 20 - Analog switch control"]
    #[inline(always)]
    pub fn ch20gr7_3(&mut self) -> CH20GR7_3_W {
        CH20GR7_3_W { w: self }
    }
    #[doc = "Bit 19 - Analog switch control"]
    #[inline(always)]
    pub fn ch19gr7_2(&mut self) -> CH19GR7_2_W {
        CH19GR7_2_W { w: self }
    }
    #[doc = "Bit 18 - Analog switch control"]
    #[inline(always)]
    pub fn ch18gr7_1(&mut self) -> CH18GR7_1_W {
        CH18GR7_1_W { w: self }
    }
    #[doc = "Bit 16 - Analog switch control"]
    #[inline(always)]
    pub fn ch31gr7_1(&mut self) -> CH31GR7_1_W {
        CH31GR7_1_W { w: self }
    }
    #[doc = "Bit 15 - Analog switch control"]
    #[inline(always)]
    pub fn ch15gr9_2(&mut self) -> CH15GR9_2_W {
        CH15GR9_2_W { w: self }
    }
    #[doc = "Bit 14 - Analog switch control"]
    #[inline(always)]
    pub fn ch14gr9_1(&mut self) -> CH14GR9_1_W {
        CH14GR9_1_W { w: self }
    }
    #[doc = "Bit 13 - Analog switch control"]
    #[inline(always)]
    pub fn ch13gr8_4(&mut self) -> CH13GR8_4_W {
        CH13GR8_4_W { w: self }
    }
    #[doc = "Bit 12 - Analog switch control"]
    #[inline(always)]
    pub fn ch12gr8_3(&mut self) -> CH12GR8_3_W {
        CH12GR8_3_W { w: self }
    }
    #[doc = "Bit 11 - Analog switch control"]
    #[inline(always)]
    pub fn ch11gr8_2(&mut self) -> CH11GR8_2_W {
        CH11GR8_2_W { w: self }
    }
    #[doc = "Bit 10 - Analog switch control"]
    #[inline(always)]
    pub fn ch10gr8_1(&mut self) -> CH10GR8_1_W {
        CH10GR8_1_W { w: self }
    }
    #[doc = "Bit 9 - Analog switch control"]
    #[inline(always)]
    pub fn ch9gr3_2(&mut self) -> CH9GR3_2_W {
        CH9GR3_2_W { w: self }
    }
    #[doc = "Bit 8 - Analog switch control"]
    #[inline(always)]
    pub fn ch8gr3_1(&mut self) -> CH8GR3_1_W {
        CH8GR3_1_W { w: self }
    }
    #[doc = "Bit 7 - Analog switch control"]
    #[inline(always)]
    pub fn ch7gr2_2(&mut self) -> CH7GR2_2_W {
        CH7GR2_2_W { w: self }
    }
    #[doc = "Bit 6 - Analog switch control"]
    #[inline(always)]
    pub fn ch6gr2_1(&mut self) -> CH6GR2_1_W {
        CH6GR2_1_W { w: self }
    }
    #[doc = "Bit 5 - Comparator 1 analog switch"]
    #[inline(always)]
    pub fn comp1_sw1(&mut self) -> COMP1_SW1_W {
        COMP1_SW1_W { w: self }
    }
    #[doc = "Bit 4 - Analog switch control"]
    #[inline(always)]
    pub fn ch31gr11_5(&mut self) -> CH31GR11_5_W {
        CH31GR11_5_W { w: self }
    }
    #[doc = "Bit 3 - Analog switch control"]
    #[inline(always)]
    pub fn ch3gr1_4(&mut self) -> CH3GR1_4_W {
        CH3GR1_4_W { w: self }
    }
    #[doc = "Bit 2 - Analog switch control"]
    #[inline(always)]
    pub fn ch2gr1_3(&mut self) -> CH2GR1_3_W {
        CH2GR1_3_W { w: self }
    }
    #[doc = "Bit 1 - Analog switch control"]
    #[inline(always)]
    pub fn ch1gr1_2(&mut self) -> CH1GR1_2_W {
        CH1GR1_2_W { w: self }
    }
    #[doc = "Bit 0 - Analog switch control"]
    #[inline(always)]
    pub fn ch0gr1_1(&mut self) -> CH0GR1_1_W {
        CH0GR1_1_W { w: self }
    }
}
