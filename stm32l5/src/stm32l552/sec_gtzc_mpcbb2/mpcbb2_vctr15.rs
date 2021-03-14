#[doc = "Reader of register MPCBB2_VCTR15"]
pub type R = crate::R<u32, super::MPCBB2_VCTR15>;
#[doc = "Writer for register MPCBB2_VCTR15"]
pub type W = crate::W<u32, super::MPCBB2_VCTR15>;
#[doc = "Register MPCBB2_VCTR15 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB2_VCTR15 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B480`"]
pub type B480_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B480`"]
pub struct B480_W<'a> {
    w: &'a mut W,
}
impl<'a> B480_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B481`"]
pub type B481_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B481`"]
pub struct B481_W<'a> {
    w: &'a mut W,
}
impl<'a> B481_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B482`"]
pub type B482_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B482`"]
pub struct B482_W<'a> {
    w: &'a mut W,
}
impl<'a> B482_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B483`"]
pub type B483_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B483`"]
pub struct B483_W<'a> {
    w: &'a mut W,
}
impl<'a> B483_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B484`"]
pub type B484_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B484`"]
pub struct B484_W<'a> {
    w: &'a mut W,
}
impl<'a> B484_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B485`"]
pub type B485_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B485`"]
pub struct B485_W<'a> {
    w: &'a mut W,
}
impl<'a> B485_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B486`"]
pub type B486_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B486`"]
pub struct B486_W<'a> {
    w: &'a mut W,
}
impl<'a> B486_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B487`"]
pub type B487_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B487`"]
pub struct B487_W<'a> {
    w: &'a mut W,
}
impl<'a> B487_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B488`"]
pub type B488_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B488`"]
pub struct B488_W<'a> {
    w: &'a mut W,
}
impl<'a> B488_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B489`"]
pub type B489_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B489`"]
pub struct B489_W<'a> {
    w: &'a mut W,
}
impl<'a> B489_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B490`"]
pub type B490_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B490`"]
pub struct B490_W<'a> {
    w: &'a mut W,
}
impl<'a> B490_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B491`"]
pub type B491_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B491`"]
pub struct B491_W<'a> {
    w: &'a mut W,
}
impl<'a> B491_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B492`"]
pub type B492_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B492`"]
pub struct B492_W<'a> {
    w: &'a mut W,
}
impl<'a> B492_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B493`"]
pub type B493_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B493`"]
pub struct B493_W<'a> {
    w: &'a mut W,
}
impl<'a> B493_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B494`"]
pub type B494_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B494`"]
pub struct B494_W<'a> {
    w: &'a mut W,
}
impl<'a> B494_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B495`"]
pub type B495_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B495`"]
pub struct B495_W<'a> {
    w: &'a mut W,
}
impl<'a> B495_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B496`"]
pub type B496_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B496`"]
pub struct B496_W<'a> {
    w: &'a mut W,
}
impl<'a> B496_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B497`"]
pub type B497_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B497`"]
pub struct B497_W<'a> {
    w: &'a mut W,
}
impl<'a> B497_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B498`"]
pub type B498_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B498`"]
pub struct B498_W<'a> {
    w: &'a mut W,
}
impl<'a> B498_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B499`"]
pub type B499_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B499`"]
pub struct B499_W<'a> {
    w: &'a mut W,
}
impl<'a> B499_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B500`"]
pub type B500_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B500`"]
pub struct B500_W<'a> {
    w: &'a mut W,
}
impl<'a> B500_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B501`"]
pub type B501_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B501`"]
pub struct B501_W<'a> {
    w: &'a mut W,
}
impl<'a> B501_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B502`"]
pub type B502_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B502`"]
pub struct B502_W<'a> {
    w: &'a mut W,
}
impl<'a> B502_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B503`"]
pub type B503_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B503`"]
pub struct B503_W<'a> {
    w: &'a mut W,
}
impl<'a> B503_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B504`"]
pub type B504_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B504`"]
pub struct B504_W<'a> {
    w: &'a mut W,
}
impl<'a> B504_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B505`"]
pub type B505_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B505`"]
pub struct B505_W<'a> {
    w: &'a mut W,
}
impl<'a> B505_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B506`"]
pub type B506_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B506`"]
pub struct B506_W<'a> {
    w: &'a mut W,
}
impl<'a> B506_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B507`"]
pub type B507_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B507`"]
pub struct B507_W<'a> {
    w: &'a mut W,
}
impl<'a> B507_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B508`"]
pub type B508_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B508`"]
pub struct B508_W<'a> {
    w: &'a mut W,
}
impl<'a> B508_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B509`"]
pub type B509_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B509`"]
pub struct B509_W<'a> {
    w: &'a mut W,
}
impl<'a> B509_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B510`"]
pub type B510_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B510`"]
pub struct B510_W<'a> {
    w: &'a mut W,
}
impl<'a> B510_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B511`"]
pub type B511_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B511`"]
pub struct B511_W<'a> {
    w: &'a mut W,
}
impl<'a> B511_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B480"]
    #[inline(always)]
    pub fn b480(&self) -> B480_R {
        B480_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B481"]
    #[inline(always)]
    pub fn b481(&self) -> B481_R {
        B481_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B482"]
    #[inline(always)]
    pub fn b482(&self) -> B482_R {
        B482_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B483"]
    #[inline(always)]
    pub fn b483(&self) -> B483_R {
        B483_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B484"]
    #[inline(always)]
    pub fn b484(&self) -> B484_R {
        B484_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B485"]
    #[inline(always)]
    pub fn b485(&self) -> B485_R {
        B485_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B486"]
    #[inline(always)]
    pub fn b486(&self) -> B486_R {
        B486_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B487"]
    #[inline(always)]
    pub fn b487(&self) -> B487_R {
        B487_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B488"]
    #[inline(always)]
    pub fn b488(&self) -> B488_R {
        B488_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B489"]
    #[inline(always)]
    pub fn b489(&self) -> B489_R {
        B489_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B490"]
    #[inline(always)]
    pub fn b490(&self) -> B490_R {
        B490_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B491"]
    #[inline(always)]
    pub fn b491(&self) -> B491_R {
        B491_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B492"]
    #[inline(always)]
    pub fn b492(&self) -> B492_R {
        B492_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B493"]
    #[inline(always)]
    pub fn b493(&self) -> B493_R {
        B493_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B494"]
    #[inline(always)]
    pub fn b494(&self) -> B494_R {
        B494_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B495"]
    #[inline(always)]
    pub fn b495(&self) -> B495_R {
        B495_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B496"]
    #[inline(always)]
    pub fn b496(&self) -> B496_R {
        B496_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B497"]
    #[inline(always)]
    pub fn b497(&self) -> B497_R {
        B497_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B498"]
    #[inline(always)]
    pub fn b498(&self) -> B498_R {
        B498_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B499"]
    #[inline(always)]
    pub fn b499(&self) -> B499_R {
        B499_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B500"]
    #[inline(always)]
    pub fn b500(&self) -> B500_R {
        B500_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B501"]
    #[inline(always)]
    pub fn b501(&self) -> B501_R {
        B501_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B502"]
    #[inline(always)]
    pub fn b502(&self) -> B502_R {
        B502_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B503"]
    #[inline(always)]
    pub fn b503(&self) -> B503_R {
        B503_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B504"]
    #[inline(always)]
    pub fn b504(&self) -> B504_R {
        B504_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B505"]
    #[inline(always)]
    pub fn b505(&self) -> B505_R {
        B505_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B506"]
    #[inline(always)]
    pub fn b506(&self) -> B506_R {
        B506_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B507"]
    #[inline(always)]
    pub fn b507(&self) -> B507_R {
        B507_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B508"]
    #[inline(always)]
    pub fn b508(&self) -> B508_R {
        B508_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B509"]
    #[inline(always)]
    pub fn b509(&self) -> B509_R {
        B509_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B510"]
    #[inline(always)]
    pub fn b510(&self) -> B510_R {
        B510_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B511"]
    #[inline(always)]
    pub fn b511(&self) -> B511_R {
        B511_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B480"]
    #[inline(always)]
    pub fn b480(&mut self) -> B480_W {
        B480_W { w: self }
    }
    #[doc = "Bit 1 - B481"]
    #[inline(always)]
    pub fn b481(&mut self) -> B481_W {
        B481_W { w: self }
    }
    #[doc = "Bit 2 - B482"]
    #[inline(always)]
    pub fn b482(&mut self) -> B482_W {
        B482_W { w: self }
    }
    #[doc = "Bit 3 - B483"]
    #[inline(always)]
    pub fn b483(&mut self) -> B483_W {
        B483_W { w: self }
    }
    #[doc = "Bit 4 - B484"]
    #[inline(always)]
    pub fn b484(&mut self) -> B484_W {
        B484_W { w: self }
    }
    #[doc = "Bit 5 - B485"]
    #[inline(always)]
    pub fn b485(&mut self) -> B485_W {
        B485_W { w: self }
    }
    #[doc = "Bit 6 - B486"]
    #[inline(always)]
    pub fn b486(&mut self) -> B486_W {
        B486_W { w: self }
    }
    #[doc = "Bit 7 - B487"]
    #[inline(always)]
    pub fn b487(&mut self) -> B487_W {
        B487_W { w: self }
    }
    #[doc = "Bit 8 - B488"]
    #[inline(always)]
    pub fn b488(&mut self) -> B488_W {
        B488_W { w: self }
    }
    #[doc = "Bit 9 - B489"]
    #[inline(always)]
    pub fn b489(&mut self) -> B489_W {
        B489_W { w: self }
    }
    #[doc = "Bit 10 - B490"]
    #[inline(always)]
    pub fn b490(&mut self) -> B490_W {
        B490_W { w: self }
    }
    #[doc = "Bit 11 - B491"]
    #[inline(always)]
    pub fn b491(&mut self) -> B491_W {
        B491_W { w: self }
    }
    #[doc = "Bit 12 - B492"]
    #[inline(always)]
    pub fn b492(&mut self) -> B492_W {
        B492_W { w: self }
    }
    #[doc = "Bit 13 - B493"]
    #[inline(always)]
    pub fn b493(&mut self) -> B493_W {
        B493_W { w: self }
    }
    #[doc = "Bit 14 - B494"]
    #[inline(always)]
    pub fn b494(&mut self) -> B494_W {
        B494_W { w: self }
    }
    #[doc = "Bit 15 - B495"]
    #[inline(always)]
    pub fn b495(&mut self) -> B495_W {
        B495_W { w: self }
    }
    #[doc = "Bit 16 - B496"]
    #[inline(always)]
    pub fn b496(&mut self) -> B496_W {
        B496_W { w: self }
    }
    #[doc = "Bit 17 - B497"]
    #[inline(always)]
    pub fn b497(&mut self) -> B497_W {
        B497_W { w: self }
    }
    #[doc = "Bit 18 - B498"]
    #[inline(always)]
    pub fn b498(&mut self) -> B498_W {
        B498_W { w: self }
    }
    #[doc = "Bit 19 - B499"]
    #[inline(always)]
    pub fn b499(&mut self) -> B499_W {
        B499_W { w: self }
    }
    #[doc = "Bit 20 - B500"]
    #[inline(always)]
    pub fn b500(&mut self) -> B500_W {
        B500_W { w: self }
    }
    #[doc = "Bit 21 - B501"]
    #[inline(always)]
    pub fn b501(&mut self) -> B501_W {
        B501_W { w: self }
    }
    #[doc = "Bit 22 - B502"]
    #[inline(always)]
    pub fn b502(&mut self) -> B502_W {
        B502_W { w: self }
    }
    #[doc = "Bit 23 - B503"]
    #[inline(always)]
    pub fn b503(&mut self) -> B503_W {
        B503_W { w: self }
    }
    #[doc = "Bit 24 - B504"]
    #[inline(always)]
    pub fn b504(&mut self) -> B504_W {
        B504_W { w: self }
    }
    #[doc = "Bit 25 - B505"]
    #[inline(always)]
    pub fn b505(&mut self) -> B505_W {
        B505_W { w: self }
    }
    #[doc = "Bit 26 - B506"]
    #[inline(always)]
    pub fn b506(&mut self) -> B506_W {
        B506_W { w: self }
    }
    #[doc = "Bit 27 - B507"]
    #[inline(always)]
    pub fn b507(&mut self) -> B507_W {
        B507_W { w: self }
    }
    #[doc = "Bit 28 - B508"]
    #[inline(always)]
    pub fn b508(&mut self) -> B508_W {
        B508_W { w: self }
    }
    #[doc = "Bit 29 - B509"]
    #[inline(always)]
    pub fn b509(&mut self) -> B509_W {
        B509_W { w: self }
    }
    #[doc = "Bit 30 - B510"]
    #[inline(always)]
    pub fn b510(&mut self) -> B510_W {
        B510_W { w: self }
    }
    #[doc = "Bit 31 - B511"]
    #[inline(always)]
    pub fn b511(&mut self) -> B511_W {
        B511_W { w: self }
    }
}
