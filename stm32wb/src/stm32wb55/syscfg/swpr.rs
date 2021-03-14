#[doc = "Writer for register SWPR"]
pub type W = crate::W<u32, super::SWPR>;
#[doc = "Register SWPR `reset()`'s with value 0"]
impl crate::ResetValue for super::SWPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `P31WP`"]
pub struct P31WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P31WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P30WP`"]
pub struct P30WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P30WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P29WP`"]
pub struct P29WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P29WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P28WP`"]
pub struct P28WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P28WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P27WP`"]
pub struct P27WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P27WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P26WP`"]
pub struct P26WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P26WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P25WP`"]
pub struct P25WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P25WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P24WP`"]
pub struct P24WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P24WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P23WP`"]
pub struct P23WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P23WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P22WP`"]
pub struct P22WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P22WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P21WP`"]
pub struct P21WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P21WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P20WP`"]
pub struct P20WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P20WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P19WP`"]
pub struct P19WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P19WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P18WP`"]
pub struct P18WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P18WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P17WP`"]
pub struct P17WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P17WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P16WP`"]
pub struct P16WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P16WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P15WP`"]
pub struct P15WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P15WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P14WP`"]
pub struct P14WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P14WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P13WP`"]
pub struct P13WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P13WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P12WP`"]
pub struct P12WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P12WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P11WP`"]
pub struct P11WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P11WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P10WP`"]
pub struct P10WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P10WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P9WP`"]
pub struct P9WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P9WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P8WP`"]
pub struct P8WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P8WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P7WP`"]
pub struct P7WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P7WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P6WP`"]
pub struct P6WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P6WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P5WP`"]
pub struct P5WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P5WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P4WP`"]
pub struct P4WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P4WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P3WP`"]
pub struct P3WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P3WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P2WP`"]
pub struct P2WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P2WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P1WP`"]
pub struct P1WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P1WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Write proxy for field `P0WP`"]
pub struct P0WP_W<'a> {
    w: &'a mut W,
}
impl<'a> P0WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl W {
    #[doc = "Bit 31 - SRAM2 page 31 write protection"]
    #[inline(always)]
    pub fn p31wp(&mut self) -> P31WP_W {
        P31WP_W { w: self }
    }
    #[doc = "Bit 30 - P30WP"]
    #[inline(always)]
    pub fn p30wp(&mut self) -> P30WP_W {
        P30WP_W { w: self }
    }
    #[doc = "Bit 29 - P29WP"]
    #[inline(always)]
    pub fn p29wp(&mut self) -> P29WP_W {
        P29WP_W { w: self }
    }
    #[doc = "Bit 28 - P28WP"]
    #[inline(always)]
    pub fn p28wp(&mut self) -> P28WP_W {
        P28WP_W { w: self }
    }
    #[doc = "Bit 27 - P27WP"]
    #[inline(always)]
    pub fn p27wp(&mut self) -> P27WP_W {
        P27WP_W { w: self }
    }
    #[doc = "Bit 26 - P26WP"]
    #[inline(always)]
    pub fn p26wp(&mut self) -> P26WP_W {
        P26WP_W { w: self }
    }
    #[doc = "Bit 25 - P25WP"]
    #[inline(always)]
    pub fn p25wp(&mut self) -> P25WP_W {
        P25WP_W { w: self }
    }
    #[doc = "Bit 24 - P24WP"]
    #[inline(always)]
    pub fn p24wp(&mut self) -> P24WP_W {
        P24WP_W { w: self }
    }
    #[doc = "Bit 23 - P23WP"]
    #[inline(always)]
    pub fn p23wp(&mut self) -> P23WP_W {
        P23WP_W { w: self }
    }
    #[doc = "Bit 22 - P22WP"]
    #[inline(always)]
    pub fn p22wp(&mut self) -> P22WP_W {
        P22WP_W { w: self }
    }
    #[doc = "Bit 21 - P21WP"]
    #[inline(always)]
    pub fn p21wp(&mut self) -> P21WP_W {
        P21WP_W { w: self }
    }
    #[doc = "Bit 20 - P20WP"]
    #[inline(always)]
    pub fn p20wp(&mut self) -> P20WP_W {
        P20WP_W { w: self }
    }
    #[doc = "Bit 19 - P19WP"]
    #[inline(always)]
    pub fn p19wp(&mut self) -> P19WP_W {
        P19WP_W { w: self }
    }
    #[doc = "Bit 18 - P18WP"]
    #[inline(always)]
    pub fn p18wp(&mut self) -> P18WP_W {
        P18WP_W { w: self }
    }
    #[doc = "Bit 17 - P17WP"]
    #[inline(always)]
    pub fn p17wp(&mut self) -> P17WP_W {
        P17WP_W { w: self }
    }
    #[doc = "Bit 16 - P16WP"]
    #[inline(always)]
    pub fn p16wp(&mut self) -> P16WP_W {
        P16WP_W { w: self }
    }
    #[doc = "Bit 15 - P15WP"]
    #[inline(always)]
    pub fn p15wp(&mut self) -> P15WP_W {
        P15WP_W { w: self }
    }
    #[doc = "Bit 14 - P14WP"]
    #[inline(always)]
    pub fn p14wp(&mut self) -> P14WP_W {
        P14WP_W { w: self }
    }
    #[doc = "Bit 13 - P13WP"]
    #[inline(always)]
    pub fn p13wp(&mut self) -> P13WP_W {
        P13WP_W { w: self }
    }
    #[doc = "Bit 12 - P12WP"]
    #[inline(always)]
    pub fn p12wp(&mut self) -> P12WP_W {
        P12WP_W { w: self }
    }
    #[doc = "Bit 11 - P11WP"]
    #[inline(always)]
    pub fn p11wp(&mut self) -> P11WP_W {
        P11WP_W { w: self }
    }
    #[doc = "Bit 10 - P10WP"]
    #[inline(always)]
    pub fn p10wp(&mut self) -> P10WP_W {
        P10WP_W { w: self }
    }
    #[doc = "Bit 9 - P9WP"]
    #[inline(always)]
    pub fn p9wp(&mut self) -> P9WP_W {
        P9WP_W { w: self }
    }
    #[doc = "Bit 8 - P8WP"]
    #[inline(always)]
    pub fn p8wp(&mut self) -> P8WP_W {
        P8WP_W { w: self }
    }
    #[doc = "Bit 7 - P7WP"]
    #[inline(always)]
    pub fn p7wp(&mut self) -> P7WP_W {
        P7WP_W { w: self }
    }
    #[doc = "Bit 6 - P6WP"]
    #[inline(always)]
    pub fn p6wp(&mut self) -> P6WP_W {
        P6WP_W { w: self }
    }
    #[doc = "Bit 5 - P5WP"]
    #[inline(always)]
    pub fn p5wp(&mut self) -> P5WP_W {
        P5WP_W { w: self }
    }
    #[doc = "Bit 4 - P4WP"]
    #[inline(always)]
    pub fn p4wp(&mut self) -> P4WP_W {
        P4WP_W { w: self }
    }
    #[doc = "Bit 3 - P3WP"]
    #[inline(always)]
    pub fn p3wp(&mut self) -> P3WP_W {
        P3WP_W { w: self }
    }
    #[doc = "Bit 2 - P2WP"]
    #[inline(always)]
    pub fn p2wp(&mut self) -> P2WP_W {
        P2WP_W { w: self }
    }
    #[doc = "Bit 1 - P1WP"]
    #[inline(always)]
    pub fn p1wp(&mut self) -> P1WP_W {
        P1WP_W { w: self }
    }
    #[doc = "Bit 0 - P0WP"]
    #[inline(always)]
    pub fn p0wp(&mut self) -> P0WP_W {
        P0WP_W { w: self }
    }
}
