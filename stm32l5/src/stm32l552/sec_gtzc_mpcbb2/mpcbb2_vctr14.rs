#[doc = "Reader of register MPCBB2_VCTR14"]
pub type R = crate::R<u32, super::MPCBB2_VCTR14>;
#[doc = "Writer for register MPCBB2_VCTR14"]
pub type W = crate::W<u32, super::MPCBB2_VCTR14>;
#[doc = "Register MPCBB2_VCTR14 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB2_VCTR14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B448`"]
pub type B448_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B448`"]
pub struct B448_W<'a> {
    w: &'a mut W,
}
impl<'a> B448_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B449`"]
pub type B449_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B449`"]
pub struct B449_W<'a> {
    w: &'a mut W,
}
impl<'a> B449_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B450`"]
pub type B450_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B450`"]
pub struct B450_W<'a> {
    w: &'a mut W,
}
impl<'a> B450_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B451`"]
pub type B451_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B451`"]
pub struct B451_W<'a> {
    w: &'a mut W,
}
impl<'a> B451_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B452`"]
pub type B452_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B452`"]
pub struct B452_W<'a> {
    w: &'a mut W,
}
impl<'a> B452_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B453`"]
pub type B453_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B453`"]
pub struct B453_W<'a> {
    w: &'a mut W,
}
impl<'a> B453_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B454`"]
pub type B454_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B454`"]
pub struct B454_W<'a> {
    w: &'a mut W,
}
impl<'a> B454_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B455`"]
pub type B455_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B455`"]
pub struct B455_W<'a> {
    w: &'a mut W,
}
impl<'a> B455_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B456`"]
pub type B456_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B456`"]
pub struct B456_W<'a> {
    w: &'a mut W,
}
impl<'a> B456_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B457`"]
pub type B457_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B457`"]
pub struct B457_W<'a> {
    w: &'a mut W,
}
impl<'a> B457_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B458`"]
pub type B458_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B458`"]
pub struct B458_W<'a> {
    w: &'a mut W,
}
impl<'a> B458_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B459`"]
pub type B459_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B459`"]
pub struct B459_W<'a> {
    w: &'a mut W,
}
impl<'a> B459_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B460`"]
pub type B460_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B460`"]
pub struct B460_W<'a> {
    w: &'a mut W,
}
impl<'a> B460_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B461`"]
pub type B461_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B461`"]
pub struct B461_W<'a> {
    w: &'a mut W,
}
impl<'a> B461_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B462`"]
pub type B462_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B462`"]
pub struct B462_W<'a> {
    w: &'a mut W,
}
impl<'a> B462_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B463`"]
pub type B463_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B463`"]
pub struct B463_W<'a> {
    w: &'a mut W,
}
impl<'a> B463_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B464`"]
pub type B464_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B464`"]
pub struct B464_W<'a> {
    w: &'a mut W,
}
impl<'a> B464_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B465`"]
pub type B465_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B465`"]
pub struct B465_W<'a> {
    w: &'a mut W,
}
impl<'a> B465_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B466`"]
pub type B466_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B466`"]
pub struct B466_W<'a> {
    w: &'a mut W,
}
impl<'a> B466_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B467`"]
pub type B467_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B467`"]
pub struct B467_W<'a> {
    w: &'a mut W,
}
impl<'a> B467_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B468`"]
pub type B468_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B468`"]
pub struct B468_W<'a> {
    w: &'a mut W,
}
impl<'a> B468_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B469`"]
pub type B469_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B469`"]
pub struct B469_W<'a> {
    w: &'a mut W,
}
impl<'a> B469_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B470`"]
pub type B470_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B470`"]
pub struct B470_W<'a> {
    w: &'a mut W,
}
impl<'a> B470_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B471`"]
pub type B471_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B471`"]
pub struct B471_W<'a> {
    w: &'a mut W,
}
impl<'a> B471_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B472`"]
pub type B472_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B472`"]
pub struct B472_W<'a> {
    w: &'a mut W,
}
impl<'a> B472_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B473`"]
pub type B473_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B473`"]
pub struct B473_W<'a> {
    w: &'a mut W,
}
impl<'a> B473_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B474`"]
pub type B474_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B474`"]
pub struct B474_W<'a> {
    w: &'a mut W,
}
impl<'a> B474_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B475`"]
pub type B475_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B475`"]
pub struct B475_W<'a> {
    w: &'a mut W,
}
impl<'a> B475_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B476`"]
pub type B476_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B476`"]
pub struct B476_W<'a> {
    w: &'a mut W,
}
impl<'a> B476_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B477`"]
pub type B477_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B477`"]
pub struct B477_W<'a> {
    w: &'a mut W,
}
impl<'a> B477_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B478`"]
pub type B478_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B478`"]
pub struct B478_W<'a> {
    w: &'a mut W,
}
impl<'a> B478_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B479`"]
pub type B479_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B479`"]
pub struct B479_W<'a> {
    w: &'a mut W,
}
impl<'a> B479_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B448"]
    #[inline(always)]
    pub fn b448(&self) -> B448_R {
        B448_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B449"]
    #[inline(always)]
    pub fn b449(&self) -> B449_R {
        B449_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B450"]
    #[inline(always)]
    pub fn b450(&self) -> B450_R {
        B450_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B451"]
    #[inline(always)]
    pub fn b451(&self) -> B451_R {
        B451_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B452"]
    #[inline(always)]
    pub fn b452(&self) -> B452_R {
        B452_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B453"]
    #[inline(always)]
    pub fn b453(&self) -> B453_R {
        B453_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B454"]
    #[inline(always)]
    pub fn b454(&self) -> B454_R {
        B454_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B455"]
    #[inline(always)]
    pub fn b455(&self) -> B455_R {
        B455_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B456"]
    #[inline(always)]
    pub fn b456(&self) -> B456_R {
        B456_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B457"]
    #[inline(always)]
    pub fn b457(&self) -> B457_R {
        B457_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B458"]
    #[inline(always)]
    pub fn b458(&self) -> B458_R {
        B458_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B459"]
    #[inline(always)]
    pub fn b459(&self) -> B459_R {
        B459_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B460"]
    #[inline(always)]
    pub fn b460(&self) -> B460_R {
        B460_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B461"]
    #[inline(always)]
    pub fn b461(&self) -> B461_R {
        B461_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B462"]
    #[inline(always)]
    pub fn b462(&self) -> B462_R {
        B462_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B463"]
    #[inline(always)]
    pub fn b463(&self) -> B463_R {
        B463_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B464"]
    #[inline(always)]
    pub fn b464(&self) -> B464_R {
        B464_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B465"]
    #[inline(always)]
    pub fn b465(&self) -> B465_R {
        B465_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B466"]
    #[inline(always)]
    pub fn b466(&self) -> B466_R {
        B466_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B467"]
    #[inline(always)]
    pub fn b467(&self) -> B467_R {
        B467_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B468"]
    #[inline(always)]
    pub fn b468(&self) -> B468_R {
        B468_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B469"]
    #[inline(always)]
    pub fn b469(&self) -> B469_R {
        B469_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B470"]
    #[inline(always)]
    pub fn b470(&self) -> B470_R {
        B470_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B471"]
    #[inline(always)]
    pub fn b471(&self) -> B471_R {
        B471_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B472"]
    #[inline(always)]
    pub fn b472(&self) -> B472_R {
        B472_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B473"]
    #[inline(always)]
    pub fn b473(&self) -> B473_R {
        B473_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B474"]
    #[inline(always)]
    pub fn b474(&self) -> B474_R {
        B474_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B475"]
    #[inline(always)]
    pub fn b475(&self) -> B475_R {
        B475_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B476"]
    #[inline(always)]
    pub fn b476(&self) -> B476_R {
        B476_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B477"]
    #[inline(always)]
    pub fn b477(&self) -> B477_R {
        B477_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B478"]
    #[inline(always)]
    pub fn b478(&self) -> B478_R {
        B478_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B479"]
    #[inline(always)]
    pub fn b479(&self) -> B479_R {
        B479_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B448"]
    #[inline(always)]
    pub fn b448(&mut self) -> B448_W {
        B448_W { w: self }
    }
    #[doc = "Bit 1 - B449"]
    #[inline(always)]
    pub fn b449(&mut self) -> B449_W {
        B449_W { w: self }
    }
    #[doc = "Bit 2 - B450"]
    #[inline(always)]
    pub fn b450(&mut self) -> B450_W {
        B450_W { w: self }
    }
    #[doc = "Bit 3 - B451"]
    #[inline(always)]
    pub fn b451(&mut self) -> B451_W {
        B451_W { w: self }
    }
    #[doc = "Bit 4 - B452"]
    #[inline(always)]
    pub fn b452(&mut self) -> B452_W {
        B452_W { w: self }
    }
    #[doc = "Bit 5 - B453"]
    #[inline(always)]
    pub fn b453(&mut self) -> B453_W {
        B453_W { w: self }
    }
    #[doc = "Bit 6 - B454"]
    #[inline(always)]
    pub fn b454(&mut self) -> B454_W {
        B454_W { w: self }
    }
    #[doc = "Bit 7 - B455"]
    #[inline(always)]
    pub fn b455(&mut self) -> B455_W {
        B455_W { w: self }
    }
    #[doc = "Bit 8 - B456"]
    #[inline(always)]
    pub fn b456(&mut self) -> B456_W {
        B456_W { w: self }
    }
    #[doc = "Bit 9 - B457"]
    #[inline(always)]
    pub fn b457(&mut self) -> B457_W {
        B457_W { w: self }
    }
    #[doc = "Bit 10 - B458"]
    #[inline(always)]
    pub fn b458(&mut self) -> B458_W {
        B458_W { w: self }
    }
    #[doc = "Bit 11 - B459"]
    #[inline(always)]
    pub fn b459(&mut self) -> B459_W {
        B459_W { w: self }
    }
    #[doc = "Bit 12 - B460"]
    #[inline(always)]
    pub fn b460(&mut self) -> B460_W {
        B460_W { w: self }
    }
    #[doc = "Bit 13 - B461"]
    #[inline(always)]
    pub fn b461(&mut self) -> B461_W {
        B461_W { w: self }
    }
    #[doc = "Bit 14 - B462"]
    #[inline(always)]
    pub fn b462(&mut self) -> B462_W {
        B462_W { w: self }
    }
    #[doc = "Bit 15 - B463"]
    #[inline(always)]
    pub fn b463(&mut self) -> B463_W {
        B463_W { w: self }
    }
    #[doc = "Bit 16 - B464"]
    #[inline(always)]
    pub fn b464(&mut self) -> B464_W {
        B464_W { w: self }
    }
    #[doc = "Bit 17 - B465"]
    #[inline(always)]
    pub fn b465(&mut self) -> B465_W {
        B465_W { w: self }
    }
    #[doc = "Bit 18 - B466"]
    #[inline(always)]
    pub fn b466(&mut self) -> B466_W {
        B466_W { w: self }
    }
    #[doc = "Bit 19 - B467"]
    #[inline(always)]
    pub fn b467(&mut self) -> B467_W {
        B467_W { w: self }
    }
    #[doc = "Bit 20 - B468"]
    #[inline(always)]
    pub fn b468(&mut self) -> B468_W {
        B468_W { w: self }
    }
    #[doc = "Bit 21 - B469"]
    #[inline(always)]
    pub fn b469(&mut self) -> B469_W {
        B469_W { w: self }
    }
    #[doc = "Bit 22 - B470"]
    #[inline(always)]
    pub fn b470(&mut self) -> B470_W {
        B470_W { w: self }
    }
    #[doc = "Bit 23 - B471"]
    #[inline(always)]
    pub fn b471(&mut self) -> B471_W {
        B471_W { w: self }
    }
    #[doc = "Bit 24 - B472"]
    #[inline(always)]
    pub fn b472(&mut self) -> B472_W {
        B472_W { w: self }
    }
    #[doc = "Bit 25 - B473"]
    #[inline(always)]
    pub fn b473(&mut self) -> B473_W {
        B473_W { w: self }
    }
    #[doc = "Bit 26 - B474"]
    #[inline(always)]
    pub fn b474(&mut self) -> B474_W {
        B474_W { w: self }
    }
    #[doc = "Bit 27 - B475"]
    #[inline(always)]
    pub fn b475(&mut self) -> B475_W {
        B475_W { w: self }
    }
    #[doc = "Bit 28 - B476"]
    #[inline(always)]
    pub fn b476(&mut self) -> B476_W {
        B476_W { w: self }
    }
    #[doc = "Bit 29 - B477"]
    #[inline(always)]
    pub fn b477(&mut self) -> B477_W {
        B477_W { w: self }
    }
    #[doc = "Bit 30 - B478"]
    #[inline(always)]
    pub fn b478(&mut self) -> B478_W {
        B478_W { w: self }
    }
    #[doc = "Bit 31 - B479"]
    #[inline(always)]
    pub fn b479(&mut self) -> B479_W {
        B479_W { w: self }
    }
}
