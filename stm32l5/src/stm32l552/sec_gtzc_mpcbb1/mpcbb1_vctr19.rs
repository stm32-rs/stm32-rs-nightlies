#[doc = "Reader of register MPCBB1_VCTR19"]
pub type R = crate::R<u32, super::MPCBB1_VCTR19>;
#[doc = "Writer for register MPCBB1_VCTR19"]
pub type W = crate::W<u32, super::MPCBB1_VCTR19>;
#[doc = "Register MPCBB1_VCTR19 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB1_VCTR19 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B608`"]
pub type B608_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B608`"]
pub struct B608_W<'a> {
    w: &'a mut W,
}
impl<'a> B608_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B609`"]
pub type B609_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B609`"]
pub struct B609_W<'a> {
    w: &'a mut W,
}
impl<'a> B609_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B610`"]
pub type B610_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B610`"]
pub struct B610_W<'a> {
    w: &'a mut W,
}
impl<'a> B610_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B611`"]
pub type B611_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B611`"]
pub struct B611_W<'a> {
    w: &'a mut W,
}
impl<'a> B611_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B612`"]
pub type B612_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B612`"]
pub struct B612_W<'a> {
    w: &'a mut W,
}
impl<'a> B612_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B613`"]
pub type B613_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B613`"]
pub struct B613_W<'a> {
    w: &'a mut W,
}
impl<'a> B613_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B614`"]
pub type B614_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B614`"]
pub struct B614_W<'a> {
    w: &'a mut W,
}
impl<'a> B614_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B615`"]
pub type B615_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B615`"]
pub struct B615_W<'a> {
    w: &'a mut W,
}
impl<'a> B615_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B616`"]
pub type B616_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B616`"]
pub struct B616_W<'a> {
    w: &'a mut W,
}
impl<'a> B616_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B617`"]
pub type B617_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B617`"]
pub struct B617_W<'a> {
    w: &'a mut W,
}
impl<'a> B617_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B618`"]
pub type B618_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B618`"]
pub struct B618_W<'a> {
    w: &'a mut W,
}
impl<'a> B618_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B619`"]
pub type B619_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B619`"]
pub struct B619_W<'a> {
    w: &'a mut W,
}
impl<'a> B619_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B620`"]
pub type B620_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B620`"]
pub struct B620_W<'a> {
    w: &'a mut W,
}
impl<'a> B620_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B621`"]
pub type B621_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B621`"]
pub struct B621_W<'a> {
    w: &'a mut W,
}
impl<'a> B621_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B622`"]
pub type B622_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B622`"]
pub struct B622_W<'a> {
    w: &'a mut W,
}
impl<'a> B622_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B623`"]
pub type B623_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B623`"]
pub struct B623_W<'a> {
    w: &'a mut W,
}
impl<'a> B623_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B624`"]
pub type B624_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B624`"]
pub struct B624_W<'a> {
    w: &'a mut W,
}
impl<'a> B624_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B625`"]
pub type B625_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B625`"]
pub struct B625_W<'a> {
    w: &'a mut W,
}
impl<'a> B625_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B626`"]
pub type B626_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B626`"]
pub struct B626_W<'a> {
    w: &'a mut W,
}
impl<'a> B626_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B627`"]
pub type B627_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B627`"]
pub struct B627_W<'a> {
    w: &'a mut W,
}
impl<'a> B627_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B628`"]
pub type B628_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B628`"]
pub struct B628_W<'a> {
    w: &'a mut W,
}
impl<'a> B628_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B629`"]
pub type B629_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B629`"]
pub struct B629_W<'a> {
    w: &'a mut W,
}
impl<'a> B629_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B630`"]
pub type B630_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B630`"]
pub struct B630_W<'a> {
    w: &'a mut W,
}
impl<'a> B630_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B631`"]
pub type B631_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B631`"]
pub struct B631_W<'a> {
    w: &'a mut W,
}
impl<'a> B631_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B632`"]
pub type B632_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B632`"]
pub struct B632_W<'a> {
    w: &'a mut W,
}
impl<'a> B632_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B633`"]
pub type B633_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B633`"]
pub struct B633_W<'a> {
    w: &'a mut W,
}
impl<'a> B633_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B634`"]
pub type B634_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B634`"]
pub struct B634_W<'a> {
    w: &'a mut W,
}
impl<'a> B634_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B635`"]
pub type B635_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B635`"]
pub struct B635_W<'a> {
    w: &'a mut W,
}
impl<'a> B635_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B636`"]
pub type B636_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B636`"]
pub struct B636_W<'a> {
    w: &'a mut W,
}
impl<'a> B636_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B637`"]
pub type B637_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B637`"]
pub struct B637_W<'a> {
    w: &'a mut W,
}
impl<'a> B637_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B638`"]
pub type B638_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B638`"]
pub struct B638_W<'a> {
    w: &'a mut W,
}
impl<'a> B638_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B639`"]
pub type B639_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B639`"]
pub struct B639_W<'a> {
    w: &'a mut W,
}
impl<'a> B639_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B608"]
    #[inline(always)]
    pub fn b608(&self) -> B608_R {
        B608_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B609"]
    #[inline(always)]
    pub fn b609(&self) -> B609_R {
        B609_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B610"]
    #[inline(always)]
    pub fn b610(&self) -> B610_R {
        B610_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B611"]
    #[inline(always)]
    pub fn b611(&self) -> B611_R {
        B611_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B612"]
    #[inline(always)]
    pub fn b612(&self) -> B612_R {
        B612_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B613"]
    #[inline(always)]
    pub fn b613(&self) -> B613_R {
        B613_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B614"]
    #[inline(always)]
    pub fn b614(&self) -> B614_R {
        B614_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B615"]
    #[inline(always)]
    pub fn b615(&self) -> B615_R {
        B615_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B616"]
    #[inline(always)]
    pub fn b616(&self) -> B616_R {
        B616_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B617"]
    #[inline(always)]
    pub fn b617(&self) -> B617_R {
        B617_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B618"]
    #[inline(always)]
    pub fn b618(&self) -> B618_R {
        B618_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B619"]
    #[inline(always)]
    pub fn b619(&self) -> B619_R {
        B619_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B620"]
    #[inline(always)]
    pub fn b620(&self) -> B620_R {
        B620_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B621"]
    #[inline(always)]
    pub fn b621(&self) -> B621_R {
        B621_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B622"]
    #[inline(always)]
    pub fn b622(&self) -> B622_R {
        B622_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B623"]
    #[inline(always)]
    pub fn b623(&self) -> B623_R {
        B623_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B624"]
    #[inline(always)]
    pub fn b624(&self) -> B624_R {
        B624_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B625"]
    #[inline(always)]
    pub fn b625(&self) -> B625_R {
        B625_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B626"]
    #[inline(always)]
    pub fn b626(&self) -> B626_R {
        B626_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B627"]
    #[inline(always)]
    pub fn b627(&self) -> B627_R {
        B627_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B628"]
    #[inline(always)]
    pub fn b628(&self) -> B628_R {
        B628_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B629"]
    #[inline(always)]
    pub fn b629(&self) -> B629_R {
        B629_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B630"]
    #[inline(always)]
    pub fn b630(&self) -> B630_R {
        B630_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B631"]
    #[inline(always)]
    pub fn b631(&self) -> B631_R {
        B631_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B632"]
    #[inline(always)]
    pub fn b632(&self) -> B632_R {
        B632_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B633"]
    #[inline(always)]
    pub fn b633(&self) -> B633_R {
        B633_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B634"]
    #[inline(always)]
    pub fn b634(&self) -> B634_R {
        B634_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B635"]
    #[inline(always)]
    pub fn b635(&self) -> B635_R {
        B635_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B636"]
    #[inline(always)]
    pub fn b636(&self) -> B636_R {
        B636_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B637"]
    #[inline(always)]
    pub fn b637(&self) -> B637_R {
        B637_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B638"]
    #[inline(always)]
    pub fn b638(&self) -> B638_R {
        B638_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B639"]
    #[inline(always)]
    pub fn b639(&self) -> B639_R {
        B639_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B608"]
    #[inline(always)]
    pub fn b608(&mut self) -> B608_W {
        B608_W { w: self }
    }
    #[doc = "Bit 1 - B609"]
    #[inline(always)]
    pub fn b609(&mut self) -> B609_W {
        B609_W { w: self }
    }
    #[doc = "Bit 2 - B610"]
    #[inline(always)]
    pub fn b610(&mut self) -> B610_W {
        B610_W { w: self }
    }
    #[doc = "Bit 3 - B611"]
    #[inline(always)]
    pub fn b611(&mut self) -> B611_W {
        B611_W { w: self }
    }
    #[doc = "Bit 4 - B612"]
    #[inline(always)]
    pub fn b612(&mut self) -> B612_W {
        B612_W { w: self }
    }
    #[doc = "Bit 5 - B613"]
    #[inline(always)]
    pub fn b613(&mut self) -> B613_W {
        B613_W { w: self }
    }
    #[doc = "Bit 6 - B614"]
    #[inline(always)]
    pub fn b614(&mut self) -> B614_W {
        B614_W { w: self }
    }
    #[doc = "Bit 7 - B615"]
    #[inline(always)]
    pub fn b615(&mut self) -> B615_W {
        B615_W { w: self }
    }
    #[doc = "Bit 8 - B616"]
    #[inline(always)]
    pub fn b616(&mut self) -> B616_W {
        B616_W { w: self }
    }
    #[doc = "Bit 9 - B617"]
    #[inline(always)]
    pub fn b617(&mut self) -> B617_W {
        B617_W { w: self }
    }
    #[doc = "Bit 10 - B618"]
    #[inline(always)]
    pub fn b618(&mut self) -> B618_W {
        B618_W { w: self }
    }
    #[doc = "Bit 11 - B619"]
    #[inline(always)]
    pub fn b619(&mut self) -> B619_W {
        B619_W { w: self }
    }
    #[doc = "Bit 12 - B620"]
    #[inline(always)]
    pub fn b620(&mut self) -> B620_W {
        B620_W { w: self }
    }
    #[doc = "Bit 13 - B621"]
    #[inline(always)]
    pub fn b621(&mut self) -> B621_W {
        B621_W { w: self }
    }
    #[doc = "Bit 14 - B622"]
    #[inline(always)]
    pub fn b622(&mut self) -> B622_W {
        B622_W { w: self }
    }
    #[doc = "Bit 15 - B623"]
    #[inline(always)]
    pub fn b623(&mut self) -> B623_W {
        B623_W { w: self }
    }
    #[doc = "Bit 16 - B624"]
    #[inline(always)]
    pub fn b624(&mut self) -> B624_W {
        B624_W { w: self }
    }
    #[doc = "Bit 17 - B625"]
    #[inline(always)]
    pub fn b625(&mut self) -> B625_W {
        B625_W { w: self }
    }
    #[doc = "Bit 18 - B626"]
    #[inline(always)]
    pub fn b626(&mut self) -> B626_W {
        B626_W { w: self }
    }
    #[doc = "Bit 19 - B627"]
    #[inline(always)]
    pub fn b627(&mut self) -> B627_W {
        B627_W { w: self }
    }
    #[doc = "Bit 20 - B628"]
    #[inline(always)]
    pub fn b628(&mut self) -> B628_W {
        B628_W { w: self }
    }
    #[doc = "Bit 21 - B629"]
    #[inline(always)]
    pub fn b629(&mut self) -> B629_W {
        B629_W { w: self }
    }
    #[doc = "Bit 22 - B630"]
    #[inline(always)]
    pub fn b630(&mut self) -> B630_W {
        B630_W { w: self }
    }
    #[doc = "Bit 23 - B631"]
    #[inline(always)]
    pub fn b631(&mut self) -> B631_W {
        B631_W { w: self }
    }
    #[doc = "Bit 24 - B632"]
    #[inline(always)]
    pub fn b632(&mut self) -> B632_W {
        B632_W { w: self }
    }
    #[doc = "Bit 25 - B633"]
    #[inline(always)]
    pub fn b633(&mut self) -> B633_W {
        B633_W { w: self }
    }
    #[doc = "Bit 26 - B634"]
    #[inline(always)]
    pub fn b634(&mut self) -> B634_W {
        B634_W { w: self }
    }
    #[doc = "Bit 27 - B635"]
    #[inline(always)]
    pub fn b635(&mut self) -> B635_W {
        B635_W { w: self }
    }
    #[doc = "Bit 28 - B636"]
    #[inline(always)]
    pub fn b636(&mut self) -> B636_W {
        B636_W { w: self }
    }
    #[doc = "Bit 29 - B637"]
    #[inline(always)]
    pub fn b637(&mut self) -> B637_W {
        B637_W { w: self }
    }
    #[doc = "Bit 30 - B638"]
    #[inline(always)]
    pub fn b638(&mut self) -> B638_W {
        B638_W { w: self }
    }
    #[doc = "Bit 31 - B639"]
    #[inline(always)]
    pub fn b639(&mut self) -> B639_W {
        B639_W { w: self }
    }
}
