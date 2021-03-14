#[doc = "Reader of register MPCBB1_VCTR25"]
pub type R = crate::R<u32, super::MPCBB1_VCTR25>;
#[doc = "Writer for register MPCBB1_VCTR25"]
pub type W = crate::W<u32, super::MPCBB1_VCTR25>;
#[doc = "Register MPCBB1_VCTR25 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB1_VCTR25 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B800`"]
pub type B800_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B800`"]
pub struct B800_W<'a> {
    w: &'a mut W,
}
impl<'a> B800_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B801`"]
pub type B801_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B801`"]
pub struct B801_W<'a> {
    w: &'a mut W,
}
impl<'a> B801_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B802`"]
pub type B802_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B802`"]
pub struct B802_W<'a> {
    w: &'a mut W,
}
impl<'a> B802_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B803`"]
pub type B803_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B803`"]
pub struct B803_W<'a> {
    w: &'a mut W,
}
impl<'a> B803_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B804`"]
pub type B804_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B804`"]
pub struct B804_W<'a> {
    w: &'a mut W,
}
impl<'a> B804_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B805`"]
pub type B805_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B805`"]
pub struct B805_W<'a> {
    w: &'a mut W,
}
impl<'a> B805_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B806`"]
pub type B806_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B806`"]
pub struct B806_W<'a> {
    w: &'a mut W,
}
impl<'a> B806_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B807`"]
pub type B807_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B807`"]
pub struct B807_W<'a> {
    w: &'a mut W,
}
impl<'a> B807_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B808`"]
pub type B808_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B808`"]
pub struct B808_W<'a> {
    w: &'a mut W,
}
impl<'a> B808_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B809`"]
pub type B809_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B809`"]
pub struct B809_W<'a> {
    w: &'a mut W,
}
impl<'a> B809_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B810`"]
pub type B810_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B810`"]
pub struct B810_W<'a> {
    w: &'a mut W,
}
impl<'a> B810_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B811`"]
pub type B811_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B811`"]
pub struct B811_W<'a> {
    w: &'a mut W,
}
impl<'a> B811_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B812`"]
pub type B812_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B812`"]
pub struct B812_W<'a> {
    w: &'a mut W,
}
impl<'a> B812_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B813`"]
pub type B813_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B813`"]
pub struct B813_W<'a> {
    w: &'a mut W,
}
impl<'a> B813_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B814`"]
pub type B814_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B814`"]
pub struct B814_W<'a> {
    w: &'a mut W,
}
impl<'a> B814_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B815`"]
pub type B815_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B815`"]
pub struct B815_W<'a> {
    w: &'a mut W,
}
impl<'a> B815_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B816`"]
pub type B816_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B816`"]
pub struct B816_W<'a> {
    w: &'a mut W,
}
impl<'a> B816_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B817`"]
pub type B817_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B817`"]
pub struct B817_W<'a> {
    w: &'a mut W,
}
impl<'a> B817_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B818`"]
pub type B818_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B818`"]
pub struct B818_W<'a> {
    w: &'a mut W,
}
impl<'a> B818_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B819`"]
pub type B819_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B819`"]
pub struct B819_W<'a> {
    w: &'a mut W,
}
impl<'a> B819_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B820`"]
pub type B820_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B820`"]
pub struct B820_W<'a> {
    w: &'a mut W,
}
impl<'a> B820_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B821`"]
pub type B821_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B821`"]
pub struct B821_W<'a> {
    w: &'a mut W,
}
impl<'a> B821_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B822`"]
pub type B822_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B822`"]
pub struct B822_W<'a> {
    w: &'a mut W,
}
impl<'a> B822_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B823`"]
pub type B823_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B823`"]
pub struct B823_W<'a> {
    w: &'a mut W,
}
impl<'a> B823_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B824`"]
pub type B824_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B824`"]
pub struct B824_W<'a> {
    w: &'a mut W,
}
impl<'a> B824_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B825`"]
pub type B825_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B825`"]
pub struct B825_W<'a> {
    w: &'a mut W,
}
impl<'a> B825_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B826`"]
pub type B826_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B826`"]
pub struct B826_W<'a> {
    w: &'a mut W,
}
impl<'a> B826_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B827`"]
pub type B827_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B827`"]
pub struct B827_W<'a> {
    w: &'a mut W,
}
impl<'a> B827_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B828`"]
pub type B828_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B828`"]
pub struct B828_W<'a> {
    w: &'a mut W,
}
impl<'a> B828_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B829`"]
pub type B829_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B829`"]
pub struct B829_W<'a> {
    w: &'a mut W,
}
impl<'a> B829_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B830`"]
pub type B830_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B830`"]
pub struct B830_W<'a> {
    w: &'a mut W,
}
impl<'a> B830_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B831`"]
pub type B831_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B831`"]
pub struct B831_W<'a> {
    w: &'a mut W,
}
impl<'a> B831_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B800"]
    #[inline(always)]
    pub fn b800(&self) -> B800_R {
        B800_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B801"]
    #[inline(always)]
    pub fn b801(&self) -> B801_R {
        B801_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B802"]
    #[inline(always)]
    pub fn b802(&self) -> B802_R {
        B802_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B803"]
    #[inline(always)]
    pub fn b803(&self) -> B803_R {
        B803_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B804"]
    #[inline(always)]
    pub fn b804(&self) -> B804_R {
        B804_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B805"]
    #[inline(always)]
    pub fn b805(&self) -> B805_R {
        B805_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B806"]
    #[inline(always)]
    pub fn b806(&self) -> B806_R {
        B806_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B807"]
    #[inline(always)]
    pub fn b807(&self) -> B807_R {
        B807_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B808"]
    #[inline(always)]
    pub fn b808(&self) -> B808_R {
        B808_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B809"]
    #[inline(always)]
    pub fn b809(&self) -> B809_R {
        B809_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B810"]
    #[inline(always)]
    pub fn b810(&self) -> B810_R {
        B810_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B811"]
    #[inline(always)]
    pub fn b811(&self) -> B811_R {
        B811_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B812"]
    #[inline(always)]
    pub fn b812(&self) -> B812_R {
        B812_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B813"]
    #[inline(always)]
    pub fn b813(&self) -> B813_R {
        B813_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B814"]
    #[inline(always)]
    pub fn b814(&self) -> B814_R {
        B814_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B815"]
    #[inline(always)]
    pub fn b815(&self) -> B815_R {
        B815_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B816"]
    #[inline(always)]
    pub fn b816(&self) -> B816_R {
        B816_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B817"]
    #[inline(always)]
    pub fn b817(&self) -> B817_R {
        B817_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B818"]
    #[inline(always)]
    pub fn b818(&self) -> B818_R {
        B818_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B819"]
    #[inline(always)]
    pub fn b819(&self) -> B819_R {
        B819_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B820"]
    #[inline(always)]
    pub fn b820(&self) -> B820_R {
        B820_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B821"]
    #[inline(always)]
    pub fn b821(&self) -> B821_R {
        B821_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B822"]
    #[inline(always)]
    pub fn b822(&self) -> B822_R {
        B822_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B823"]
    #[inline(always)]
    pub fn b823(&self) -> B823_R {
        B823_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B824"]
    #[inline(always)]
    pub fn b824(&self) -> B824_R {
        B824_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B825"]
    #[inline(always)]
    pub fn b825(&self) -> B825_R {
        B825_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B826"]
    #[inline(always)]
    pub fn b826(&self) -> B826_R {
        B826_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B827"]
    #[inline(always)]
    pub fn b827(&self) -> B827_R {
        B827_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B828"]
    #[inline(always)]
    pub fn b828(&self) -> B828_R {
        B828_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B829"]
    #[inline(always)]
    pub fn b829(&self) -> B829_R {
        B829_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B830"]
    #[inline(always)]
    pub fn b830(&self) -> B830_R {
        B830_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B831"]
    #[inline(always)]
    pub fn b831(&self) -> B831_R {
        B831_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B800"]
    #[inline(always)]
    pub fn b800(&mut self) -> B800_W {
        B800_W { w: self }
    }
    #[doc = "Bit 1 - B801"]
    #[inline(always)]
    pub fn b801(&mut self) -> B801_W {
        B801_W { w: self }
    }
    #[doc = "Bit 2 - B802"]
    #[inline(always)]
    pub fn b802(&mut self) -> B802_W {
        B802_W { w: self }
    }
    #[doc = "Bit 3 - B803"]
    #[inline(always)]
    pub fn b803(&mut self) -> B803_W {
        B803_W { w: self }
    }
    #[doc = "Bit 4 - B804"]
    #[inline(always)]
    pub fn b804(&mut self) -> B804_W {
        B804_W { w: self }
    }
    #[doc = "Bit 5 - B805"]
    #[inline(always)]
    pub fn b805(&mut self) -> B805_W {
        B805_W { w: self }
    }
    #[doc = "Bit 6 - B806"]
    #[inline(always)]
    pub fn b806(&mut self) -> B806_W {
        B806_W { w: self }
    }
    #[doc = "Bit 7 - B807"]
    #[inline(always)]
    pub fn b807(&mut self) -> B807_W {
        B807_W { w: self }
    }
    #[doc = "Bit 8 - B808"]
    #[inline(always)]
    pub fn b808(&mut self) -> B808_W {
        B808_W { w: self }
    }
    #[doc = "Bit 9 - B809"]
    #[inline(always)]
    pub fn b809(&mut self) -> B809_W {
        B809_W { w: self }
    }
    #[doc = "Bit 10 - B810"]
    #[inline(always)]
    pub fn b810(&mut self) -> B810_W {
        B810_W { w: self }
    }
    #[doc = "Bit 11 - B811"]
    #[inline(always)]
    pub fn b811(&mut self) -> B811_W {
        B811_W { w: self }
    }
    #[doc = "Bit 12 - B812"]
    #[inline(always)]
    pub fn b812(&mut self) -> B812_W {
        B812_W { w: self }
    }
    #[doc = "Bit 13 - B813"]
    #[inline(always)]
    pub fn b813(&mut self) -> B813_W {
        B813_W { w: self }
    }
    #[doc = "Bit 14 - B814"]
    #[inline(always)]
    pub fn b814(&mut self) -> B814_W {
        B814_W { w: self }
    }
    #[doc = "Bit 15 - B815"]
    #[inline(always)]
    pub fn b815(&mut self) -> B815_W {
        B815_W { w: self }
    }
    #[doc = "Bit 16 - B816"]
    #[inline(always)]
    pub fn b816(&mut self) -> B816_W {
        B816_W { w: self }
    }
    #[doc = "Bit 17 - B817"]
    #[inline(always)]
    pub fn b817(&mut self) -> B817_W {
        B817_W { w: self }
    }
    #[doc = "Bit 18 - B818"]
    #[inline(always)]
    pub fn b818(&mut self) -> B818_W {
        B818_W { w: self }
    }
    #[doc = "Bit 19 - B819"]
    #[inline(always)]
    pub fn b819(&mut self) -> B819_W {
        B819_W { w: self }
    }
    #[doc = "Bit 20 - B820"]
    #[inline(always)]
    pub fn b820(&mut self) -> B820_W {
        B820_W { w: self }
    }
    #[doc = "Bit 21 - B821"]
    #[inline(always)]
    pub fn b821(&mut self) -> B821_W {
        B821_W { w: self }
    }
    #[doc = "Bit 22 - B822"]
    #[inline(always)]
    pub fn b822(&mut self) -> B822_W {
        B822_W { w: self }
    }
    #[doc = "Bit 23 - B823"]
    #[inline(always)]
    pub fn b823(&mut self) -> B823_W {
        B823_W { w: self }
    }
    #[doc = "Bit 24 - B824"]
    #[inline(always)]
    pub fn b824(&mut self) -> B824_W {
        B824_W { w: self }
    }
    #[doc = "Bit 25 - B825"]
    #[inline(always)]
    pub fn b825(&mut self) -> B825_W {
        B825_W { w: self }
    }
    #[doc = "Bit 26 - B826"]
    #[inline(always)]
    pub fn b826(&mut self) -> B826_W {
        B826_W { w: self }
    }
    #[doc = "Bit 27 - B827"]
    #[inline(always)]
    pub fn b827(&mut self) -> B827_W {
        B827_W { w: self }
    }
    #[doc = "Bit 28 - B828"]
    #[inline(always)]
    pub fn b828(&mut self) -> B828_W {
        B828_W { w: self }
    }
    #[doc = "Bit 29 - B829"]
    #[inline(always)]
    pub fn b829(&mut self) -> B829_W {
        B829_W { w: self }
    }
    #[doc = "Bit 30 - B830"]
    #[inline(always)]
    pub fn b830(&mut self) -> B830_W {
        B830_W { w: self }
    }
    #[doc = "Bit 31 - B831"]
    #[inline(always)]
    pub fn b831(&mut self) -> B831_W {
        B831_W { w: self }
    }
}
