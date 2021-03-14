#[doc = "Reader of register CRYP_IV1RR"]
pub type R = crate::R<u32, super::CRYP_IV1RR>;
#[doc = "Writer for register CRYP_IV1RR"]
pub type W = crate::W<u32, super::CRYP_IV1RR>;
#[doc = "Register CRYP_IV1RR `reset()`'s with value 0"]
impl crate::ResetValue for super::CRYP_IV1RR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IV127`"]
pub type IV127_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV127`"]
pub struct IV127_W<'a> {
    w: &'a mut W,
}
impl<'a> IV127_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV126`"]
pub type IV126_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV126`"]
pub struct IV126_W<'a> {
    w: &'a mut W,
}
impl<'a> IV126_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV125`"]
pub type IV125_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV125`"]
pub struct IV125_W<'a> {
    w: &'a mut W,
}
impl<'a> IV125_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV124`"]
pub type IV124_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV124`"]
pub struct IV124_W<'a> {
    w: &'a mut W,
}
impl<'a> IV124_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV123`"]
pub type IV123_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV123`"]
pub struct IV123_W<'a> {
    w: &'a mut W,
}
impl<'a> IV123_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV122`"]
pub type IV122_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV122`"]
pub struct IV122_W<'a> {
    w: &'a mut W,
}
impl<'a> IV122_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV121`"]
pub type IV121_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV121`"]
pub struct IV121_W<'a> {
    w: &'a mut W,
}
impl<'a> IV121_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV120`"]
pub type IV120_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV120`"]
pub struct IV120_W<'a> {
    w: &'a mut W,
}
impl<'a> IV120_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV119`"]
pub type IV119_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV119`"]
pub struct IV119_W<'a> {
    w: &'a mut W,
}
impl<'a> IV119_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV118`"]
pub type IV118_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV118`"]
pub struct IV118_W<'a> {
    w: &'a mut W,
}
impl<'a> IV118_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV117`"]
pub type IV117_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV117`"]
pub struct IV117_W<'a> {
    w: &'a mut W,
}
impl<'a> IV117_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV116`"]
pub type IV116_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV116`"]
pub struct IV116_W<'a> {
    w: &'a mut W,
}
impl<'a> IV116_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV115`"]
pub type IV115_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV115`"]
pub struct IV115_W<'a> {
    w: &'a mut W,
}
impl<'a> IV115_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV114`"]
pub type IV114_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV114`"]
pub struct IV114_W<'a> {
    w: &'a mut W,
}
impl<'a> IV114_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV113`"]
pub type IV113_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV113`"]
pub struct IV113_W<'a> {
    w: &'a mut W,
}
impl<'a> IV113_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV112`"]
pub type IV112_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV112`"]
pub struct IV112_W<'a> {
    w: &'a mut W,
}
impl<'a> IV112_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV111`"]
pub type IV111_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV111`"]
pub struct IV111_W<'a> {
    w: &'a mut W,
}
impl<'a> IV111_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV110`"]
pub type IV110_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV110`"]
pub struct IV110_W<'a> {
    w: &'a mut W,
}
impl<'a> IV110_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV109`"]
pub type IV109_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV109`"]
pub struct IV109_W<'a> {
    w: &'a mut W,
}
impl<'a> IV109_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV108`"]
pub type IV108_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV108`"]
pub struct IV108_W<'a> {
    w: &'a mut W,
}
impl<'a> IV108_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV107`"]
pub type IV107_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV107`"]
pub struct IV107_W<'a> {
    w: &'a mut W,
}
impl<'a> IV107_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV106`"]
pub type IV106_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV106`"]
pub struct IV106_W<'a> {
    w: &'a mut W,
}
impl<'a> IV106_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV105`"]
pub type IV105_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV105`"]
pub struct IV105_W<'a> {
    w: &'a mut W,
}
impl<'a> IV105_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV104`"]
pub type IV104_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV104`"]
pub struct IV104_W<'a> {
    w: &'a mut W,
}
impl<'a> IV104_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV103`"]
pub type IV103_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV103`"]
pub struct IV103_W<'a> {
    w: &'a mut W,
}
impl<'a> IV103_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV102`"]
pub type IV102_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV102`"]
pub struct IV102_W<'a> {
    w: &'a mut W,
}
impl<'a> IV102_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV101`"]
pub type IV101_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV101`"]
pub struct IV101_W<'a> {
    w: &'a mut W,
}
impl<'a> IV101_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV100`"]
pub type IV100_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV100`"]
pub struct IV100_W<'a> {
    w: &'a mut W,
}
impl<'a> IV100_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV99`"]
pub type IV99_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV99`"]
pub struct IV99_W<'a> {
    w: &'a mut W,
}
impl<'a> IV99_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV98`"]
pub type IV98_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV98`"]
pub struct IV98_W<'a> {
    w: &'a mut W,
}
impl<'a> IV98_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV97`"]
pub type IV97_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV97`"]
pub struct IV97_W<'a> {
    w: &'a mut W,
}
impl<'a> IV97_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV96`"]
pub type IV96_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV96`"]
pub struct IV96_W<'a> {
    w: &'a mut W,
}
impl<'a> IV96_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - IV127"]
    #[inline(always)]
    pub fn iv127(&self) -> IV127_R {
        IV127_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IV126"]
    #[inline(always)]
    pub fn iv126(&self) -> IV126_R {
        IV126_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IV125"]
    #[inline(always)]
    pub fn iv125(&self) -> IV125_R {
        IV125_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IV124"]
    #[inline(always)]
    pub fn iv124(&self) -> IV124_R {
        IV124_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IV123"]
    #[inline(always)]
    pub fn iv123(&self) -> IV123_R {
        IV123_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IV122"]
    #[inline(always)]
    pub fn iv122(&self) -> IV122_R {
        IV122_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IV121"]
    #[inline(always)]
    pub fn iv121(&self) -> IV121_R {
        IV121_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IV120"]
    #[inline(always)]
    pub fn iv120(&self) -> IV120_R {
        IV120_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - IV119"]
    #[inline(always)]
    pub fn iv119(&self) -> IV119_R {
        IV119_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - IV118"]
    #[inline(always)]
    pub fn iv118(&self) -> IV118_R {
        IV118_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - IV117"]
    #[inline(always)]
    pub fn iv117(&self) -> IV117_R {
        IV117_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - IV116"]
    #[inline(always)]
    pub fn iv116(&self) -> IV116_R {
        IV116_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - IV115"]
    #[inline(always)]
    pub fn iv115(&self) -> IV115_R {
        IV115_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - IV114"]
    #[inline(always)]
    pub fn iv114(&self) -> IV114_R {
        IV114_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - IV113"]
    #[inline(always)]
    pub fn iv113(&self) -> IV113_R {
        IV113_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IV112"]
    #[inline(always)]
    pub fn iv112(&self) -> IV112_R {
        IV112_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - IV111"]
    #[inline(always)]
    pub fn iv111(&self) -> IV111_R {
        IV111_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - IV110"]
    #[inline(always)]
    pub fn iv110(&self) -> IV110_R {
        IV110_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IV109"]
    #[inline(always)]
    pub fn iv109(&self) -> IV109_R {
        IV109_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - IV108"]
    #[inline(always)]
    pub fn iv108(&self) -> IV108_R {
        IV108_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IV107"]
    #[inline(always)]
    pub fn iv107(&self) -> IV107_R {
        IV107_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - IV106"]
    #[inline(always)]
    pub fn iv106(&self) -> IV106_R {
        IV106_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - IV105"]
    #[inline(always)]
    pub fn iv105(&self) -> IV105_R {
        IV105_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - IV104"]
    #[inline(always)]
    pub fn iv104(&self) -> IV104_R {
        IV104_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - IV103"]
    #[inline(always)]
    pub fn iv103(&self) -> IV103_R {
        IV103_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - IV102"]
    #[inline(always)]
    pub fn iv102(&self) -> IV102_R {
        IV102_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - IV101"]
    #[inline(always)]
    pub fn iv101(&self) -> IV101_R {
        IV101_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IV100"]
    #[inline(always)]
    pub fn iv100(&self) -> IV100_R {
        IV100_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - IV99"]
    #[inline(always)]
    pub fn iv99(&self) -> IV99_R {
        IV99_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - IV98"]
    #[inline(always)]
    pub fn iv98(&self) -> IV98_R {
        IV98_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - IV97"]
    #[inline(always)]
    pub fn iv97(&self) -> IV97_R {
        IV97_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - IV96"]
    #[inline(always)]
    pub fn iv96(&self) -> IV96_R {
        IV96_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IV127"]
    #[inline(always)]
    pub fn iv127(&mut self) -> IV127_W {
        IV127_W { w: self }
    }
    #[doc = "Bit 1 - IV126"]
    #[inline(always)]
    pub fn iv126(&mut self) -> IV126_W {
        IV126_W { w: self }
    }
    #[doc = "Bit 2 - IV125"]
    #[inline(always)]
    pub fn iv125(&mut self) -> IV125_W {
        IV125_W { w: self }
    }
    #[doc = "Bit 3 - IV124"]
    #[inline(always)]
    pub fn iv124(&mut self) -> IV124_W {
        IV124_W { w: self }
    }
    #[doc = "Bit 4 - IV123"]
    #[inline(always)]
    pub fn iv123(&mut self) -> IV123_W {
        IV123_W { w: self }
    }
    #[doc = "Bit 5 - IV122"]
    #[inline(always)]
    pub fn iv122(&mut self) -> IV122_W {
        IV122_W { w: self }
    }
    #[doc = "Bit 6 - IV121"]
    #[inline(always)]
    pub fn iv121(&mut self) -> IV121_W {
        IV121_W { w: self }
    }
    #[doc = "Bit 7 - IV120"]
    #[inline(always)]
    pub fn iv120(&mut self) -> IV120_W {
        IV120_W { w: self }
    }
    #[doc = "Bit 8 - IV119"]
    #[inline(always)]
    pub fn iv119(&mut self) -> IV119_W {
        IV119_W { w: self }
    }
    #[doc = "Bit 9 - IV118"]
    #[inline(always)]
    pub fn iv118(&mut self) -> IV118_W {
        IV118_W { w: self }
    }
    #[doc = "Bit 10 - IV117"]
    #[inline(always)]
    pub fn iv117(&mut self) -> IV117_W {
        IV117_W { w: self }
    }
    #[doc = "Bit 11 - IV116"]
    #[inline(always)]
    pub fn iv116(&mut self) -> IV116_W {
        IV116_W { w: self }
    }
    #[doc = "Bit 12 - IV115"]
    #[inline(always)]
    pub fn iv115(&mut self) -> IV115_W {
        IV115_W { w: self }
    }
    #[doc = "Bit 13 - IV114"]
    #[inline(always)]
    pub fn iv114(&mut self) -> IV114_W {
        IV114_W { w: self }
    }
    #[doc = "Bit 14 - IV113"]
    #[inline(always)]
    pub fn iv113(&mut self) -> IV113_W {
        IV113_W { w: self }
    }
    #[doc = "Bit 15 - IV112"]
    #[inline(always)]
    pub fn iv112(&mut self) -> IV112_W {
        IV112_W { w: self }
    }
    #[doc = "Bit 16 - IV111"]
    #[inline(always)]
    pub fn iv111(&mut self) -> IV111_W {
        IV111_W { w: self }
    }
    #[doc = "Bit 17 - IV110"]
    #[inline(always)]
    pub fn iv110(&mut self) -> IV110_W {
        IV110_W { w: self }
    }
    #[doc = "Bit 18 - IV109"]
    #[inline(always)]
    pub fn iv109(&mut self) -> IV109_W {
        IV109_W { w: self }
    }
    #[doc = "Bit 19 - IV108"]
    #[inline(always)]
    pub fn iv108(&mut self) -> IV108_W {
        IV108_W { w: self }
    }
    #[doc = "Bit 20 - IV107"]
    #[inline(always)]
    pub fn iv107(&mut self) -> IV107_W {
        IV107_W { w: self }
    }
    #[doc = "Bit 21 - IV106"]
    #[inline(always)]
    pub fn iv106(&mut self) -> IV106_W {
        IV106_W { w: self }
    }
    #[doc = "Bit 22 - IV105"]
    #[inline(always)]
    pub fn iv105(&mut self) -> IV105_W {
        IV105_W { w: self }
    }
    #[doc = "Bit 23 - IV104"]
    #[inline(always)]
    pub fn iv104(&mut self) -> IV104_W {
        IV104_W { w: self }
    }
    #[doc = "Bit 24 - IV103"]
    #[inline(always)]
    pub fn iv103(&mut self) -> IV103_W {
        IV103_W { w: self }
    }
    #[doc = "Bit 25 - IV102"]
    #[inline(always)]
    pub fn iv102(&mut self) -> IV102_W {
        IV102_W { w: self }
    }
    #[doc = "Bit 26 - IV101"]
    #[inline(always)]
    pub fn iv101(&mut self) -> IV101_W {
        IV101_W { w: self }
    }
    #[doc = "Bit 27 - IV100"]
    #[inline(always)]
    pub fn iv100(&mut self) -> IV100_W {
        IV100_W { w: self }
    }
    #[doc = "Bit 28 - IV99"]
    #[inline(always)]
    pub fn iv99(&mut self) -> IV99_W {
        IV99_W { w: self }
    }
    #[doc = "Bit 29 - IV98"]
    #[inline(always)]
    pub fn iv98(&mut self) -> IV98_W {
        IV98_W { w: self }
    }
    #[doc = "Bit 30 - IV97"]
    #[inline(always)]
    pub fn iv97(&mut self) -> IV97_W {
        IV97_W { w: self }
    }
    #[doc = "Bit 31 - IV96"]
    #[inline(always)]
    pub fn iv96(&mut self) -> IV96_W {
        IV96_W { w: self }
    }
}
