#[doc = "Reader of register MPCBB1_VCTR29"]
pub type R = crate::R<u32, super::MPCBB1_VCTR29>;
#[doc = "Writer for register MPCBB1_VCTR29"]
pub type W = crate::W<u32, super::MPCBB1_VCTR29>;
#[doc = "Register MPCBB1_VCTR29 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB1_VCTR29 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B928`"]
pub type B928_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B928`"]
pub struct B928_W<'a> {
    w: &'a mut W,
}
impl<'a> B928_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B929`"]
pub type B929_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B929`"]
pub struct B929_W<'a> {
    w: &'a mut W,
}
impl<'a> B929_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B930`"]
pub type B930_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B930`"]
pub struct B930_W<'a> {
    w: &'a mut W,
}
impl<'a> B930_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B931`"]
pub type B931_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B931`"]
pub struct B931_W<'a> {
    w: &'a mut W,
}
impl<'a> B931_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B932`"]
pub type B932_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B932`"]
pub struct B932_W<'a> {
    w: &'a mut W,
}
impl<'a> B932_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B933`"]
pub type B933_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B933`"]
pub struct B933_W<'a> {
    w: &'a mut W,
}
impl<'a> B933_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B934`"]
pub type B934_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B934`"]
pub struct B934_W<'a> {
    w: &'a mut W,
}
impl<'a> B934_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B935`"]
pub type B935_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B935`"]
pub struct B935_W<'a> {
    w: &'a mut W,
}
impl<'a> B935_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B936`"]
pub type B936_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B936`"]
pub struct B936_W<'a> {
    w: &'a mut W,
}
impl<'a> B936_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B937`"]
pub type B937_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B937`"]
pub struct B937_W<'a> {
    w: &'a mut W,
}
impl<'a> B937_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B938`"]
pub type B938_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B938`"]
pub struct B938_W<'a> {
    w: &'a mut W,
}
impl<'a> B938_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B939`"]
pub type B939_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B939`"]
pub struct B939_W<'a> {
    w: &'a mut W,
}
impl<'a> B939_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B940`"]
pub type B940_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B940`"]
pub struct B940_W<'a> {
    w: &'a mut W,
}
impl<'a> B940_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B941`"]
pub type B941_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B941`"]
pub struct B941_W<'a> {
    w: &'a mut W,
}
impl<'a> B941_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B942`"]
pub type B942_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B942`"]
pub struct B942_W<'a> {
    w: &'a mut W,
}
impl<'a> B942_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B943`"]
pub type B943_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B943`"]
pub struct B943_W<'a> {
    w: &'a mut W,
}
impl<'a> B943_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B944`"]
pub type B944_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B944`"]
pub struct B944_W<'a> {
    w: &'a mut W,
}
impl<'a> B944_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B945`"]
pub type B945_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B945`"]
pub struct B945_W<'a> {
    w: &'a mut W,
}
impl<'a> B945_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B946`"]
pub type B946_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B946`"]
pub struct B946_W<'a> {
    w: &'a mut W,
}
impl<'a> B946_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B947`"]
pub type B947_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B947`"]
pub struct B947_W<'a> {
    w: &'a mut W,
}
impl<'a> B947_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B948`"]
pub type B948_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B948`"]
pub struct B948_W<'a> {
    w: &'a mut W,
}
impl<'a> B948_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B949`"]
pub type B949_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B949`"]
pub struct B949_W<'a> {
    w: &'a mut W,
}
impl<'a> B949_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B950`"]
pub type B950_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B950`"]
pub struct B950_W<'a> {
    w: &'a mut W,
}
impl<'a> B950_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B951`"]
pub type B951_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B951`"]
pub struct B951_W<'a> {
    w: &'a mut W,
}
impl<'a> B951_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B952`"]
pub type B952_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B952`"]
pub struct B952_W<'a> {
    w: &'a mut W,
}
impl<'a> B952_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B953`"]
pub type B953_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B953`"]
pub struct B953_W<'a> {
    w: &'a mut W,
}
impl<'a> B953_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B954`"]
pub type B954_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B954`"]
pub struct B954_W<'a> {
    w: &'a mut W,
}
impl<'a> B954_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B955`"]
pub type B955_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B955`"]
pub struct B955_W<'a> {
    w: &'a mut W,
}
impl<'a> B955_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B956`"]
pub type B956_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B956`"]
pub struct B956_W<'a> {
    w: &'a mut W,
}
impl<'a> B956_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B957`"]
pub type B957_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B957`"]
pub struct B957_W<'a> {
    w: &'a mut W,
}
impl<'a> B957_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B958`"]
pub type B958_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B958`"]
pub struct B958_W<'a> {
    w: &'a mut W,
}
impl<'a> B958_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B959`"]
pub type B959_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B959`"]
pub struct B959_W<'a> {
    w: &'a mut W,
}
impl<'a> B959_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B928"]
    #[inline(always)]
    pub fn b928(&self) -> B928_R {
        B928_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B929"]
    #[inline(always)]
    pub fn b929(&self) -> B929_R {
        B929_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B930"]
    #[inline(always)]
    pub fn b930(&self) -> B930_R {
        B930_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B931"]
    #[inline(always)]
    pub fn b931(&self) -> B931_R {
        B931_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B932"]
    #[inline(always)]
    pub fn b932(&self) -> B932_R {
        B932_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B933"]
    #[inline(always)]
    pub fn b933(&self) -> B933_R {
        B933_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B934"]
    #[inline(always)]
    pub fn b934(&self) -> B934_R {
        B934_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B935"]
    #[inline(always)]
    pub fn b935(&self) -> B935_R {
        B935_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B936"]
    #[inline(always)]
    pub fn b936(&self) -> B936_R {
        B936_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B937"]
    #[inline(always)]
    pub fn b937(&self) -> B937_R {
        B937_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B938"]
    #[inline(always)]
    pub fn b938(&self) -> B938_R {
        B938_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B939"]
    #[inline(always)]
    pub fn b939(&self) -> B939_R {
        B939_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B940"]
    #[inline(always)]
    pub fn b940(&self) -> B940_R {
        B940_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B941"]
    #[inline(always)]
    pub fn b941(&self) -> B941_R {
        B941_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B942"]
    #[inline(always)]
    pub fn b942(&self) -> B942_R {
        B942_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B943"]
    #[inline(always)]
    pub fn b943(&self) -> B943_R {
        B943_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B944"]
    #[inline(always)]
    pub fn b944(&self) -> B944_R {
        B944_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B945"]
    #[inline(always)]
    pub fn b945(&self) -> B945_R {
        B945_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B946"]
    #[inline(always)]
    pub fn b946(&self) -> B946_R {
        B946_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B947"]
    #[inline(always)]
    pub fn b947(&self) -> B947_R {
        B947_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B948"]
    #[inline(always)]
    pub fn b948(&self) -> B948_R {
        B948_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B949"]
    #[inline(always)]
    pub fn b949(&self) -> B949_R {
        B949_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B950"]
    #[inline(always)]
    pub fn b950(&self) -> B950_R {
        B950_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B951"]
    #[inline(always)]
    pub fn b951(&self) -> B951_R {
        B951_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B952"]
    #[inline(always)]
    pub fn b952(&self) -> B952_R {
        B952_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B953"]
    #[inline(always)]
    pub fn b953(&self) -> B953_R {
        B953_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B954"]
    #[inline(always)]
    pub fn b954(&self) -> B954_R {
        B954_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B955"]
    #[inline(always)]
    pub fn b955(&self) -> B955_R {
        B955_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B956"]
    #[inline(always)]
    pub fn b956(&self) -> B956_R {
        B956_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B957"]
    #[inline(always)]
    pub fn b957(&self) -> B957_R {
        B957_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B958"]
    #[inline(always)]
    pub fn b958(&self) -> B958_R {
        B958_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B959"]
    #[inline(always)]
    pub fn b959(&self) -> B959_R {
        B959_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B928"]
    #[inline(always)]
    pub fn b928(&mut self) -> B928_W {
        B928_W { w: self }
    }
    #[doc = "Bit 1 - B929"]
    #[inline(always)]
    pub fn b929(&mut self) -> B929_W {
        B929_W { w: self }
    }
    #[doc = "Bit 2 - B930"]
    #[inline(always)]
    pub fn b930(&mut self) -> B930_W {
        B930_W { w: self }
    }
    #[doc = "Bit 3 - B931"]
    #[inline(always)]
    pub fn b931(&mut self) -> B931_W {
        B931_W { w: self }
    }
    #[doc = "Bit 4 - B932"]
    #[inline(always)]
    pub fn b932(&mut self) -> B932_W {
        B932_W { w: self }
    }
    #[doc = "Bit 5 - B933"]
    #[inline(always)]
    pub fn b933(&mut self) -> B933_W {
        B933_W { w: self }
    }
    #[doc = "Bit 6 - B934"]
    #[inline(always)]
    pub fn b934(&mut self) -> B934_W {
        B934_W { w: self }
    }
    #[doc = "Bit 7 - B935"]
    #[inline(always)]
    pub fn b935(&mut self) -> B935_W {
        B935_W { w: self }
    }
    #[doc = "Bit 8 - B936"]
    #[inline(always)]
    pub fn b936(&mut self) -> B936_W {
        B936_W { w: self }
    }
    #[doc = "Bit 9 - B937"]
    #[inline(always)]
    pub fn b937(&mut self) -> B937_W {
        B937_W { w: self }
    }
    #[doc = "Bit 10 - B938"]
    #[inline(always)]
    pub fn b938(&mut self) -> B938_W {
        B938_W { w: self }
    }
    #[doc = "Bit 11 - B939"]
    #[inline(always)]
    pub fn b939(&mut self) -> B939_W {
        B939_W { w: self }
    }
    #[doc = "Bit 12 - B940"]
    #[inline(always)]
    pub fn b940(&mut self) -> B940_W {
        B940_W { w: self }
    }
    #[doc = "Bit 13 - B941"]
    #[inline(always)]
    pub fn b941(&mut self) -> B941_W {
        B941_W { w: self }
    }
    #[doc = "Bit 14 - B942"]
    #[inline(always)]
    pub fn b942(&mut self) -> B942_W {
        B942_W { w: self }
    }
    #[doc = "Bit 15 - B943"]
    #[inline(always)]
    pub fn b943(&mut self) -> B943_W {
        B943_W { w: self }
    }
    #[doc = "Bit 16 - B944"]
    #[inline(always)]
    pub fn b944(&mut self) -> B944_W {
        B944_W { w: self }
    }
    #[doc = "Bit 17 - B945"]
    #[inline(always)]
    pub fn b945(&mut self) -> B945_W {
        B945_W { w: self }
    }
    #[doc = "Bit 18 - B946"]
    #[inline(always)]
    pub fn b946(&mut self) -> B946_W {
        B946_W { w: self }
    }
    #[doc = "Bit 19 - B947"]
    #[inline(always)]
    pub fn b947(&mut self) -> B947_W {
        B947_W { w: self }
    }
    #[doc = "Bit 20 - B948"]
    #[inline(always)]
    pub fn b948(&mut self) -> B948_W {
        B948_W { w: self }
    }
    #[doc = "Bit 21 - B949"]
    #[inline(always)]
    pub fn b949(&mut self) -> B949_W {
        B949_W { w: self }
    }
    #[doc = "Bit 22 - B950"]
    #[inline(always)]
    pub fn b950(&mut self) -> B950_W {
        B950_W { w: self }
    }
    #[doc = "Bit 23 - B951"]
    #[inline(always)]
    pub fn b951(&mut self) -> B951_W {
        B951_W { w: self }
    }
    #[doc = "Bit 24 - B952"]
    #[inline(always)]
    pub fn b952(&mut self) -> B952_W {
        B952_W { w: self }
    }
    #[doc = "Bit 25 - B953"]
    #[inline(always)]
    pub fn b953(&mut self) -> B953_W {
        B953_W { w: self }
    }
    #[doc = "Bit 26 - B954"]
    #[inline(always)]
    pub fn b954(&mut self) -> B954_W {
        B954_W { w: self }
    }
    #[doc = "Bit 27 - B955"]
    #[inline(always)]
    pub fn b955(&mut self) -> B955_W {
        B955_W { w: self }
    }
    #[doc = "Bit 28 - B956"]
    #[inline(always)]
    pub fn b956(&mut self) -> B956_W {
        B956_W { w: self }
    }
    #[doc = "Bit 29 - B957"]
    #[inline(always)]
    pub fn b957(&mut self) -> B957_W {
        B957_W { w: self }
    }
    #[doc = "Bit 30 - B958"]
    #[inline(always)]
    pub fn b958(&mut self) -> B958_W {
        B958_W { w: self }
    }
    #[doc = "Bit 31 - B959"]
    #[inline(always)]
    pub fn b959(&mut self) -> B959_W {
        B959_W { w: self }
    }
}
