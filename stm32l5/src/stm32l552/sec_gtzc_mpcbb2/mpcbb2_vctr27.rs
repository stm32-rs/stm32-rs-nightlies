#[doc = "Reader of register MPCBB2_VCTR27"]
pub type R = crate::R<u32, super::MPCBB2_VCTR27>;
#[doc = "Writer for register MPCBB2_VCTR27"]
pub type W = crate::W<u32, super::MPCBB2_VCTR27>;
#[doc = "Register MPCBB2_VCTR27 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB2_VCTR27 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B864`"]
pub type B864_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B864`"]
pub struct B864_W<'a> {
    w: &'a mut W,
}
impl<'a> B864_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B865`"]
pub type B865_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B865`"]
pub struct B865_W<'a> {
    w: &'a mut W,
}
impl<'a> B865_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B866`"]
pub type B866_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B866`"]
pub struct B866_W<'a> {
    w: &'a mut W,
}
impl<'a> B866_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B867`"]
pub type B867_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B867`"]
pub struct B867_W<'a> {
    w: &'a mut W,
}
impl<'a> B867_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B868`"]
pub type B868_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B868`"]
pub struct B868_W<'a> {
    w: &'a mut W,
}
impl<'a> B868_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B869`"]
pub type B869_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B869`"]
pub struct B869_W<'a> {
    w: &'a mut W,
}
impl<'a> B869_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B870`"]
pub type B870_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B870`"]
pub struct B870_W<'a> {
    w: &'a mut W,
}
impl<'a> B870_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B871`"]
pub type B871_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B871`"]
pub struct B871_W<'a> {
    w: &'a mut W,
}
impl<'a> B871_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B872`"]
pub type B872_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B872`"]
pub struct B872_W<'a> {
    w: &'a mut W,
}
impl<'a> B872_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B873`"]
pub type B873_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B873`"]
pub struct B873_W<'a> {
    w: &'a mut W,
}
impl<'a> B873_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B874`"]
pub type B874_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B874`"]
pub struct B874_W<'a> {
    w: &'a mut W,
}
impl<'a> B874_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B875`"]
pub type B875_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B875`"]
pub struct B875_W<'a> {
    w: &'a mut W,
}
impl<'a> B875_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B876`"]
pub type B876_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B876`"]
pub struct B876_W<'a> {
    w: &'a mut W,
}
impl<'a> B876_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B877`"]
pub type B877_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B877`"]
pub struct B877_W<'a> {
    w: &'a mut W,
}
impl<'a> B877_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B878`"]
pub type B878_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B878`"]
pub struct B878_W<'a> {
    w: &'a mut W,
}
impl<'a> B878_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B879`"]
pub type B879_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B879`"]
pub struct B879_W<'a> {
    w: &'a mut W,
}
impl<'a> B879_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B880`"]
pub type B880_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B880`"]
pub struct B880_W<'a> {
    w: &'a mut W,
}
impl<'a> B880_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B881`"]
pub type B881_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B881`"]
pub struct B881_W<'a> {
    w: &'a mut W,
}
impl<'a> B881_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B882`"]
pub type B882_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B882`"]
pub struct B882_W<'a> {
    w: &'a mut W,
}
impl<'a> B882_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B883`"]
pub type B883_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B883`"]
pub struct B883_W<'a> {
    w: &'a mut W,
}
impl<'a> B883_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B884`"]
pub type B884_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B884`"]
pub struct B884_W<'a> {
    w: &'a mut W,
}
impl<'a> B884_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B885`"]
pub type B885_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B885`"]
pub struct B885_W<'a> {
    w: &'a mut W,
}
impl<'a> B885_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B886`"]
pub type B886_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B886`"]
pub struct B886_W<'a> {
    w: &'a mut W,
}
impl<'a> B886_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B887`"]
pub type B887_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B887`"]
pub struct B887_W<'a> {
    w: &'a mut W,
}
impl<'a> B887_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B888`"]
pub type B888_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B888`"]
pub struct B888_W<'a> {
    w: &'a mut W,
}
impl<'a> B888_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B889`"]
pub type B889_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B889`"]
pub struct B889_W<'a> {
    w: &'a mut W,
}
impl<'a> B889_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B890`"]
pub type B890_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B890`"]
pub struct B890_W<'a> {
    w: &'a mut W,
}
impl<'a> B890_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B891`"]
pub type B891_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B891`"]
pub struct B891_W<'a> {
    w: &'a mut W,
}
impl<'a> B891_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B892`"]
pub type B892_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B892`"]
pub struct B892_W<'a> {
    w: &'a mut W,
}
impl<'a> B892_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B893`"]
pub type B893_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B893`"]
pub struct B893_W<'a> {
    w: &'a mut W,
}
impl<'a> B893_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B894`"]
pub type B894_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B894`"]
pub struct B894_W<'a> {
    w: &'a mut W,
}
impl<'a> B894_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B895`"]
pub type B895_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B895`"]
pub struct B895_W<'a> {
    w: &'a mut W,
}
impl<'a> B895_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B864"]
    #[inline(always)]
    pub fn b864(&self) -> B864_R {
        B864_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B865"]
    #[inline(always)]
    pub fn b865(&self) -> B865_R {
        B865_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B866"]
    #[inline(always)]
    pub fn b866(&self) -> B866_R {
        B866_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B867"]
    #[inline(always)]
    pub fn b867(&self) -> B867_R {
        B867_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B868"]
    #[inline(always)]
    pub fn b868(&self) -> B868_R {
        B868_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B869"]
    #[inline(always)]
    pub fn b869(&self) -> B869_R {
        B869_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B870"]
    #[inline(always)]
    pub fn b870(&self) -> B870_R {
        B870_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B871"]
    #[inline(always)]
    pub fn b871(&self) -> B871_R {
        B871_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B872"]
    #[inline(always)]
    pub fn b872(&self) -> B872_R {
        B872_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B873"]
    #[inline(always)]
    pub fn b873(&self) -> B873_R {
        B873_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B874"]
    #[inline(always)]
    pub fn b874(&self) -> B874_R {
        B874_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B875"]
    #[inline(always)]
    pub fn b875(&self) -> B875_R {
        B875_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B876"]
    #[inline(always)]
    pub fn b876(&self) -> B876_R {
        B876_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B877"]
    #[inline(always)]
    pub fn b877(&self) -> B877_R {
        B877_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B878"]
    #[inline(always)]
    pub fn b878(&self) -> B878_R {
        B878_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B879"]
    #[inline(always)]
    pub fn b879(&self) -> B879_R {
        B879_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B880"]
    #[inline(always)]
    pub fn b880(&self) -> B880_R {
        B880_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B881"]
    #[inline(always)]
    pub fn b881(&self) -> B881_R {
        B881_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B882"]
    #[inline(always)]
    pub fn b882(&self) -> B882_R {
        B882_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B883"]
    #[inline(always)]
    pub fn b883(&self) -> B883_R {
        B883_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B884"]
    #[inline(always)]
    pub fn b884(&self) -> B884_R {
        B884_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B885"]
    #[inline(always)]
    pub fn b885(&self) -> B885_R {
        B885_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B886"]
    #[inline(always)]
    pub fn b886(&self) -> B886_R {
        B886_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B887"]
    #[inline(always)]
    pub fn b887(&self) -> B887_R {
        B887_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B888"]
    #[inline(always)]
    pub fn b888(&self) -> B888_R {
        B888_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B889"]
    #[inline(always)]
    pub fn b889(&self) -> B889_R {
        B889_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B890"]
    #[inline(always)]
    pub fn b890(&self) -> B890_R {
        B890_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B891"]
    #[inline(always)]
    pub fn b891(&self) -> B891_R {
        B891_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B892"]
    #[inline(always)]
    pub fn b892(&self) -> B892_R {
        B892_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B893"]
    #[inline(always)]
    pub fn b893(&self) -> B893_R {
        B893_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B894"]
    #[inline(always)]
    pub fn b894(&self) -> B894_R {
        B894_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B895"]
    #[inline(always)]
    pub fn b895(&self) -> B895_R {
        B895_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B864"]
    #[inline(always)]
    pub fn b864(&mut self) -> B864_W {
        B864_W { w: self }
    }
    #[doc = "Bit 1 - B865"]
    #[inline(always)]
    pub fn b865(&mut self) -> B865_W {
        B865_W { w: self }
    }
    #[doc = "Bit 2 - B866"]
    #[inline(always)]
    pub fn b866(&mut self) -> B866_W {
        B866_W { w: self }
    }
    #[doc = "Bit 3 - B867"]
    #[inline(always)]
    pub fn b867(&mut self) -> B867_W {
        B867_W { w: self }
    }
    #[doc = "Bit 4 - B868"]
    #[inline(always)]
    pub fn b868(&mut self) -> B868_W {
        B868_W { w: self }
    }
    #[doc = "Bit 5 - B869"]
    #[inline(always)]
    pub fn b869(&mut self) -> B869_W {
        B869_W { w: self }
    }
    #[doc = "Bit 6 - B870"]
    #[inline(always)]
    pub fn b870(&mut self) -> B870_W {
        B870_W { w: self }
    }
    #[doc = "Bit 7 - B871"]
    #[inline(always)]
    pub fn b871(&mut self) -> B871_W {
        B871_W { w: self }
    }
    #[doc = "Bit 8 - B872"]
    #[inline(always)]
    pub fn b872(&mut self) -> B872_W {
        B872_W { w: self }
    }
    #[doc = "Bit 9 - B873"]
    #[inline(always)]
    pub fn b873(&mut self) -> B873_W {
        B873_W { w: self }
    }
    #[doc = "Bit 10 - B874"]
    #[inline(always)]
    pub fn b874(&mut self) -> B874_W {
        B874_W { w: self }
    }
    #[doc = "Bit 11 - B875"]
    #[inline(always)]
    pub fn b875(&mut self) -> B875_W {
        B875_W { w: self }
    }
    #[doc = "Bit 12 - B876"]
    #[inline(always)]
    pub fn b876(&mut self) -> B876_W {
        B876_W { w: self }
    }
    #[doc = "Bit 13 - B877"]
    #[inline(always)]
    pub fn b877(&mut self) -> B877_W {
        B877_W { w: self }
    }
    #[doc = "Bit 14 - B878"]
    #[inline(always)]
    pub fn b878(&mut self) -> B878_W {
        B878_W { w: self }
    }
    #[doc = "Bit 15 - B879"]
    #[inline(always)]
    pub fn b879(&mut self) -> B879_W {
        B879_W { w: self }
    }
    #[doc = "Bit 16 - B880"]
    #[inline(always)]
    pub fn b880(&mut self) -> B880_W {
        B880_W { w: self }
    }
    #[doc = "Bit 17 - B881"]
    #[inline(always)]
    pub fn b881(&mut self) -> B881_W {
        B881_W { w: self }
    }
    #[doc = "Bit 18 - B882"]
    #[inline(always)]
    pub fn b882(&mut self) -> B882_W {
        B882_W { w: self }
    }
    #[doc = "Bit 19 - B883"]
    #[inline(always)]
    pub fn b883(&mut self) -> B883_W {
        B883_W { w: self }
    }
    #[doc = "Bit 20 - B884"]
    #[inline(always)]
    pub fn b884(&mut self) -> B884_W {
        B884_W { w: self }
    }
    #[doc = "Bit 21 - B885"]
    #[inline(always)]
    pub fn b885(&mut self) -> B885_W {
        B885_W { w: self }
    }
    #[doc = "Bit 22 - B886"]
    #[inline(always)]
    pub fn b886(&mut self) -> B886_W {
        B886_W { w: self }
    }
    #[doc = "Bit 23 - B887"]
    #[inline(always)]
    pub fn b887(&mut self) -> B887_W {
        B887_W { w: self }
    }
    #[doc = "Bit 24 - B888"]
    #[inline(always)]
    pub fn b888(&mut self) -> B888_W {
        B888_W { w: self }
    }
    #[doc = "Bit 25 - B889"]
    #[inline(always)]
    pub fn b889(&mut self) -> B889_W {
        B889_W { w: self }
    }
    #[doc = "Bit 26 - B890"]
    #[inline(always)]
    pub fn b890(&mut self) -> B890_W {
        B890_W { w: self }
    }
    #[doc = "Bit 27 - B891"]
    #[inline(always)]
    pub fn b891(&mut self) -> B891_W {
        B891_W { w: self }
    }
    #[doc = "Bit 28 - B892"]
    #[inline(always)]
    pub fn b892(&mut self) -> B892_W {
        B892_W { w: self }
    }
    #[doc = "Bit 29 - B893"]
    #[inline(always)]
    pub fn b893(&mut self) -> B893_W {
        B893_W { w: self }
    }
    #[doc = "Bit 30 - B894"]
    #[inline(always)]
    pub fn b894(&mut self) -> B894_W {
        B894_W { w: self }
    }
    #[doc = "Bit 31 - B895"]
    #[inline(always)]
    pub fn b895(&mut self) -> B895_W {
        B895_W { w: self }
    }
}
