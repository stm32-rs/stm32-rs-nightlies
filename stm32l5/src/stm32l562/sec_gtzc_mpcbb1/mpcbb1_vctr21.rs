#[doc = "Reader of register MPCBB1_VCTR21"]
pub type R = crate::R<u32, super::MPCBB1_VCTR21>;
#[doc = "Writer for register MPCBB1_VCTR21"]
pub type W = crate::W<u32, super::MPCBB1_VCTR21>;
#[doc = "Register MPCBB1_VCTR21 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB1_VCTR21 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B672`"]
pub type B672_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B672`"]
pub struct B672_W<'a> {
    w: &'a mut W,
}
impl<'a> B672_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B673`"]
pub type B673_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B673`"]
pub struct B673_W<'a> {
    w: &'a mut W,
}
impl<'a> B673_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B674`"]
pub type B674_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B674`"]
pub struct B674_W<'a> {
    w: &'a mut W,
}
impl<'a> B674_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B675`"]
pub type B675_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B675`"]
pub struct B675_W<'a> {
    w: &'a mut W,
}
impl<'a> B675_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B676`"]
pub type B676_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B676`"]
pub struct B676_W<'a> {
    w: &'a mut W,
}
impl<'a> B676_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B677`"]
pub type B677_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B677`"]
pub struct B677_W<'a> {
    w: &'a mut W,
}
impl<'a> B677_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B678`"]
pub type B678_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B678`"]
pub struct B678_W<'a> {
    w: &'a mut W,
}
impl<'a> B678_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B679`"]
pub type B679_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B679`"]
pub struct B679_W<'a> {
    w: &'a mut W,
}
impl<'a> B679_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B680`"]
pub type B680_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B680`"]
pub struct B680_W<'a> {
    w: &'a mut W,
}
impl<'a> B680_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B681`"]
pub type B681_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B681`"]
pub struct B681_W<'a> {
    w: &'a mut W,
}
impl<'a> B681_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B682`"]
pub type B682_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B682`"]
pub struct B682_W<'a> {
    w: &'a mut W,
}
impl<'a> B682_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B683`"]
pub type B683_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B683`"]
pub struct B683_W<'a> {
    w: &'a mut W,
}
impl<'a> B683_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B684`"]
pub type B684_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B684`"]
pub struct B684_W<'a> {
    w: &'a mut W,
}
impl<'a> B684_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B685`"]
pub type B685_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B685`"]
pub struct B685_W<'a> {
    w: &'a mut W,
}
impl<'a> B685_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B686`"]
pub type B686_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B686`"]
pub struct B686_W<'a> {
    w: &'a mut W,
}
impl<'a> B686_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B687`"]
pub type B687_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B687`"]
pub struct B687_W<'a> {
    w: &'a mut W,
}
impl<'a> B687_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B688`"]
pub type B688_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B688`"]
pub struct B688_W<'a> {
    w: &'a mut W,
}
impl<'a> B688_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B689`"]
pub type B689_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B689`"]
pub struct B689_W<'a> {
    w: &'a mut W,
}
impl<'a> B689_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B690`"]
pub type B690_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B690`"]
pub struct B690_W<'a> {
    w: &'a mut W,
}
impl<'a> B690_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B691`"]
pub type B691_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B691`"]
pub struct B691_W<'a> {
    w: &'a mut W,
}
impl<'a> B691_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B692`"]
pub type B692_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B692`"]
pub struct B692_W<'a> {
    w: &'a mut W,
}
impl<'a> B692_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B693`"]
pub type B693_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B693`"]
pub struct B693_W<'a> {
    w: &'a mut W,
}
impl<'a> B693_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B694`"]
pub type B694_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B694`"]
pub struct B694_W<'a> {
    w: &'a mut W,
}
impl<'a> B694_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B695`"]
pub type B695_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B695`"]
pub struct B695_W<'a> {
    w: &'a mut W,
}
impl<'a> B695_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B696`"]
pub type B696_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B696`"]
pub struct B696_W<'a> {
    w: &'a mut W,
}
impl<'a> B696_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B697`"]
pub type B697_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B697`"]
pub struct B697_W<'a> {
    w: &'a mut W,
}
impl<'a> B697_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B698`"]
pub type B698_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B698`"]
pub struct B698_W<'a> {
    w: &'a mut W,
}
impl<'a> B698_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B699`"]
pub type B699_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B699`"]
pub struct B699_W<'a> {
    w: &'a mut W,
}
impl<'a> B699_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B700`"]
pub type B700_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B700`"]
pub struct B700_W<'a> {
    w: &'a mut W,
}
impl<'a> B700_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B701`"]
pub type B701_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B701`"]
pub struct B701_W<'a> {
    w: &'a mut W,
}
impl<'a> B701_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B702`"]
pub type B702_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B702`"]
pub struct B702_W<'a> {
    w: &'a mut W,
}
impl<'a> B702_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B703`"]
pub type B703_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B703`"]
pub struct B703_W<'a> {
    w: &'a mut W,
}
impl<'a> B703_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B672"]
    #[inline(always)]
    pub fn b672(&self) -> B672_R {
        B672_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B673"]
    #[inline(always)]
    pub fn b673(&self) -> B673_R {
        B673_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B674"]
    #[inline(always)]
    pub fn b674(&self) -> B674_R {
        B674_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B675"]
    #[inline(always)]
    pub fn b675(&self) -> B675_R {
        B675_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B676"]
    #[inline(always)]
    pub fn b676(&self) -> B676_R {
        B676_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B677"]
    #[inline(always)]
    pub fn b677(&self) -> B677_R {
        B677_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B678"]
    #[inline(always)]
    pub fn b678(&self) -> B678_R {
        B678_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B679"]
    #[inline(always)]
    pub fn b679(&self) -> B679_R {
        B679_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B680"]
    #[inline(always)]
    pub fn b680(&self) -> B680_R {
        B680_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B681"]
    #[inline(always)]
    pub fn b681(&self) -> B681_R {
        B681_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B682"]
    #[inline(always)]
    pub fn b682(&self) -> B682_R {
        B682_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B683"]
    #[inline(always)]
    pub fn b683(&self) -> B683_R {
        B683_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B684"]
    #[inline(always)]
    pub fn b684(&self) -> B684_R {
        B684_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B685"]
    #[inline(always)]
    pub fn b685(&self) -> B685_R {
        B685_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B686"]
    #[inline(always)]
    pub fn b686(&self) -> B686_R {
        B686_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B687"]
    #[inline(always)]
    pub fn b687(&self) -> B687_R {
        B687_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B688"]
    #[inline(always)]
    pub fn b688(&self) -> B688_R {
        B688_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B689"]
    #[inline(always)]
    pub fn b689(&self) -> B689_R {
        B689_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B690"]
    #[inline(always)]
    pub fn b690(&self) -> B690_R {
        B690_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B691"]
    #[inline(always)]
    pub fn b691(&self) -> B691_R {
        B691_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B692"]
    #[inline(always)]
    pub fn b692(&self) -> B692_R {
        B692_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B693"]
    #[inline(always)]
    pub fn b693(&self) -> B693_R {
        B693_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B694"]
    #[inline(always)]
    pub fn b694(&self) -> B694_R {
        B694_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B695"]
    #[inline(always)]
    pub fn b695(&self) -> B695_R {
        B695_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B696"]
    #[inline(always)]
    pub fn b696(&self) -> B696_R {
        B696_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B697"]
    #[inline(always)]
    pub fn b697(&self) -> B697_R {
        B697_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B698"]
    #[inline(always)]
    pub fn b698(&self) -> B698_R {
        B698_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B699"]
    #[inline(always)]
    pub fn b699(&self) -> B699_R {
        B699_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B700"]
    #[inline(always)]
    pub fn b700(&self) -> B700_R {
        B700_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B701"]
    #[inline(always)]
    pub fn b701(&self) -> B701_R {
        B701_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B702"]
    #[inline(always)]
    pub fn b702(&self) -> B702_R {
        B702_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B703"]
    #[inline(always)]
    pub fn b703(&self) -> B703_R {
        B703_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B672"]
    #[inline(always)]
    pub fn b672(&mut self) -> B672_W {
        B672_W { w: self }
    }
    #[doc = "Bit 1 - B673"]
    #[inline(always)]
    pub fn b673(&mut self) -> B673_W {
        B673_W { w: self }
    }
    #[doc = "Bit 2 - B674"]
    #[inline(always)]
    pub fn b674(&mut self) -> B674_W {
        B674_W { w: self }
    }
    #[doc = "Bit 3 - B675"]
    #[inline(always)]
    pub fn b675(&mut self) -> B675_W {
        B675_W { w: self }
    }
    #[doc = "Bit 4 - B676"]
    #[inline(always)]
    pub fn b676(&mut self) -> B676_W {
        B676_W { w: self }
    }
    #[doc = "Bit 5 - B677"]
    #[inline(always)]
    pub fn b677(&mut self) -> B677_W {
        B677_W { w: self }
    }
    #[doc = "Bit 6 - B678"]
    #[inline(always)]
    pub fn b678(&mut self) -> B678_W {
        B678_W { w: self }
    }
    #[doc = "Bit 7 - B679"]
    #[inline(always)]
    pub fn b679(&mut self) -> B679_W {
        B679_W { w: self }
    }
    #[doc = "Bit 8 - B680"]
    #[inline(always)]
    pub fn b680(&mut self) -> B680_W {
        B680_W { w: self }
    }
    #[doc = "Bit 9 - B681"]
    #[inline(always)]
    pub fn b681(&mut self) -> B681_W {
        B681_W { w: self }
    }
    #[doc = "Bit 10 - B682"]
    #[inline(always)]
    pub fn b682(&mut self) -> B682_W {
        B682_W { w: self }
    }
    #[doc = "Bit 11 - B683"]
    #[inline(always)]
    pub fn b683(&mut self) -> B683_W {
        B683_W { w: self }
    }
    #[doc = "Bit 12 - B684"]
    #[inline(always)]
    pub fn b684(&mut self) -> B684_W {
        B684_W { w: self }
    }
    #[doc = "Bit 13 - B685"]
    #[inline(always)]
    pub fn b685(&mut self) -> B685_W {
        B685_W { w: self }
    }
    #[doc = "Bit 14 - B686"]
    #[inline(always)]
    pub fn b686(&mut self) -> B686_W {
        B686_W { w: self }
    }
    #[doc = "Bit 15 - B687"]
    #[inline(always)]
    pub fn b687(&mut self) -> B687_W {
        B687_W { w: self }
    }
    #[doc = "Bit 16 - B688"]
    #[inline(always)]
    pub fn b688(&mut self) -> B688_W {
        B688_W { w: self }
    }
    #[doc = "Bit 17 - B689"]
    #[inline(always)]
    pub fn b689(&mut self) -> B689_W {
        B689_W { w: self }
    }
    #[doc = "Bit 18 - B690"]
    #[inline(always)]
    pub fn b690(&mut self) -> B690_W {
        B690_W { w: self }
    }
    #[doc = "Bit 19 - B691"]
    #[inline(always)]
    pub fn b691(&mut self) -> B691_W {
        B691_W { w: self }
    }
    #[doc = "Bit 20 - B692"]
    #[inline(always)]
    pub fn b692(&mut self) -> B692_W {
        B692_W { w: self }
    }
    #[doc = "Bit 21 - B693"]
    #[inline(always)]
    pub fn b693(&mut self) -> B693_W {
        B693_W { w: self }
    }
    #[doc = "Bit 22 - B694"]
    #[inline(always)]
    pub fn b694(&mut self) -> B694_W {
        B694_W { w: self }
    }
    #[doc = "Bit 23 - B695"]
    #[inline(always)]
    pub fn b695(&mut self) -> B695_W {
        B695_W { w: self }
    }
    #[doc = "Bit 24 - B696"]
    #[inline(always)]
    pub fn b696(&mut self) -> B696_W {
        B696_W { w: self }
    }
    #[doc = "Bit 25 - B697"]
    #[inline(always)]
    pub fn b697(&mut self) -> B697_W {
        B697_W { w: self }
    }
    #[doc = "Bit 26 - B698"]
    #[inline(always)]
    pub fn b698(&mut self) -> B698_W {
        B698_W { w: self }
    }
    #[doc = "Bit 27 - B699"]
    #[inline(always)]
    pub fn b699(&mut self) -> B699_W {
        B699_W { w: self }
    }
    #[doc = "Bit 28 - B700"]
    #[inline(always)]
    pub fn b700(&mut self) -> B700_W {
        B700_W { w: self }
    }
    #[doc = "Bit 29 - B701"]
    #[inline(always)]
    pub fn b701(&mut self) -> B701_W {
        B701_W { w: self }
    }
    #[doc = "Bit 30 - B702"]
    #[inline(always)]
    pub fn b702(&mut self) -> B702_W {
        B702_W { w: self }
    }
    #[doc = "Bit 31 - B703"]
    #[inline(always)]
    pub fn b703(&mut self) -> B703_W {
        B703_W { w: self }
    }
}
