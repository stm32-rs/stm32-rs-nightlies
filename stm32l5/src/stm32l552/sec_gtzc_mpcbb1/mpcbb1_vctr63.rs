#[doc = "Reader of register MPCBB1_VCTR63"]
pub type R = crate::R<u32, super::MPCBB1_VCTR63>;
#[doc = "Writer for register MPCBB1_VCTR63"]
pub type W = crate::W<u32, super::MPCBB1_VCTR63>;
#[doc = "Register MPCBB1_VCTR63 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB1_VCTR63 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B2016`"]
pub type B2016_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2016`"]
pub struct B2016_W<'a> {
    w: &'a mut W,
}
impl<'a> B2016_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2017`"]
pub type B2017_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2017`"]
pub struct B2017_W<'a> {
    w: &'a mut W,
}
impl<'a> B2017_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2018`"]
pub type B2018_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2018`"]
pub struct B2018_W<'a> {
    w: &'a mut W,
}
impl<'a> B2018_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2019`"]
pub type B2019_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2019`"]
pub struct B2019_W<'a> {
    w: &'a mut W,
}
impl<'a> B2019_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2020`"]
pub type B2020_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2020`"]
pub struct B2020_W<'a> {
    w: &'a mut W,
}
impl<'a> B2020_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2021`"]
pub type B2021_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2021`"]
pub struct B2021_W<'a> {
    w: &'a mut W,
}
impl<'a> B2021_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2022`"]
pub type B2022_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2022`"]
pub struct B2022_W<'a> {
    w: &'a mut W,
}
impl<'a> B2022_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2023`"]
pub type B2023_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2023`"]
pub struct B2023_W<'a> {
    w: &'a mut W,
}
impl<'a> B2023_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2024`"]
pub type B2024_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2024`"]
pub struct B2024_W<'a> {
    w: &'a mut W,
}
impl<'a> B2024_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2025`"]
pub type B2025_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2025`"]
pub struct B2025_W<'a> {
    w: &'a mut W,
}
impl<'a> B2025_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2026`"]
pub type B2026_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2026`"]
pub struct B2026_W<'a> {
    w: &'a mut W,
}
impl<'a> B2026_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2027`"]
pub type B2027_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2027`"]
pub struct B2027_W<'a> {
    w: &'a mut W,
}
impl<'a> B2027_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2028`"]
pub type B2028_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2028`"]
pub struct B2028_W<'a> {
    w: &'a mut W,
}
impl<'a> B2028_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2029`"]
pub type B2029_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2029`"]
pub struct B2029_W<'a> {
    w: &'a mut W,
}
impl<'a> B2029_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2030`"]
pub type B2030_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2030`"]
pub struct B2030_W<'a> {
    w: &'a mut W,
}
impl<'a> B2030_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2031`"]
pub type B2031_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2031`"]
pub struct B2031_W<'a> {
    w: &'a mut W,
}
impl<'a> B2031_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2032`"]
pub type B2032_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2032`"]
pub struct B2032_W<'a> {
    w: &'a mut W,
}
impl<'a> B2032_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2033`"]
pub type B2033_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2033`"]
pub struct B2033_W<'a> {
    w: &'a mut W,
}
impl<'a> B2033_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2034`"]
pub type B2034_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2034`"]
pub struct B2034_W<'a> {
    w: &'a mut W,
}
impl<'a> B2034_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2035`"]
pub type B2035_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2035`"]
pub struct B2035_W<'a> {
    w: &'a mut W,
}
impl<'a> B2035_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2036`"]
pub type B2036_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2036`"]
pub struct B2036_W<'a> {
    w: &'a mut W,
}
impl<'a> B2036_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2037`"]
pub type B2037_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2037`"]
pub struct B2037_W<'a> {
    w: &'a mut W,
}
impl<'a> B2037_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2038`"]
pub type B2038_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2038`"]
pub struct B2038_W<'a> {
    w: &'a mut W,
}
impl<'a> B2038_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2039`"]
pub type B2039_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2039`"]
pub struct B2039_W<'a> {
    w: &'a mut W,
}
impl<'a> B2039_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2040`"]
pub type B2040_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2040`"]
pub struct B2040_W<'a> {
    w: &'a mut W,
}
impl<'a> B2040_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2041`"]
pub type B2041_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2041`"]
pub struct B2041_W<'a> {
    w: &'a mut W,
}
impl<'a> B2041_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2042`"]
pub type B2042_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2042`"]
pub struct B2042_W<'a> {
    w: &'a mut W,
}
impl<'a> B2042_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2043`"]
pub type B2043_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2043`"]
pub struct B2043_W<'a> {
    w: &'a mut W,
}
impl<'a> B2043_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2044`"]
pub type B2044_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2044`"]
pub struct B2044_W<'a> {
    w: &'a mut W,
}
impl<'a> B2044_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2045`"]
pub type B2045_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2045`"]
pub struct B2045_W<'a> {
    w: &'a mut W,
}
impl<'a> B2045_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2046`"]
pub type B2046_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2046`"]
pub struct B2046_W<'a> {
    w: &'a mut W,
}
impl<'a> B2046_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B2047`"]
pub type B2047_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B2047`"]
pub struct B2047_W<'a> {
    w: &'a mut W,
}
impl<'a> B2047_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B2016"]
    #[inline(always)]
    pub fn b2016(&self) -> B2016_R {
        B2016_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B2017"]
    #[inline(always)]
    pub fn b2017(&self) -> B2017_R {
        B2017_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B2018"]
    #[inline(always)]
    pub fn b2018(&self) -> B2018_R {
        B2018_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B2019"]
    #[inline(always)]
    pub fn b2019(&self) -> B2019_R {
        B2019_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B2020"]
    #[inline(always)]
    pub fn b2020(&self) -> B2020_R {
        B2020_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B2021"]
    #[inline(always)]
    pub fn b2021(&self) -> B2021_R {
        B2021_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B2022"]
    #[inline(always)]
    pub fn b2022(&self) -> B2022_R {
        B2022_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B2023"]
    #[inline(always)]
    pub fn b2023(&self) -> B2023_R {
        B2023_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B2024"]
    #[inline(always)]
    pub fn b2024(&self) -> B2024_R {
        B2024_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B2025"]
    #[inline(always)]
    pub fn b2025(&self) -> B2025_R {
        B2025_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B2026"]
    #[inline(always)]
    pub fn b2026(&self) -> B2026_R {
        B2026_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B2027"]
    #[inline(always)]
    pub fn b2027(&self) -> B2027_R {
        B2027_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B2028"]
    #[inline(always)]
    pub fn b2028(&self) -> B2028_R {
        B2028_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B2029"]
    #[inline(always)]
    pub fn b2029(&self) -> B2029_R {
        B2029_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B2030"]
    #[inline(always)]
    pub fn b2030(&self) -> B2030_R {
        B2030_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B2031"]
    #[inline(always)]
    pub fn b2031(&self) -> B2031_R {
        B2031_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B2032"]
    #[inline(always)]
    pub fn b2032(&self) -> B2032_R {
        B2032_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B2033"]
    #[inline(always)]
    pub fn b2033(&self) -> B2033_R {
        B2033_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B2034"]
    #[inline(always)]
    pub fn b2034(&self) -> B2034_R {
        B2034_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B2035"]
    #[inline(always)]
    pub fn b2035(&self) -> B2035_R {
        B2035_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B2036"]
    #[inline(always)]
    pub fn b2036(&self) -> B2036_R {
        B2036_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B2037"]
    #[inline(always)]
    pub fn b2037(&self) -> B2037_R {
        B2037_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B2038"]
    #[inline(always)]
    pub fn b2038(&self) -> B2038_R {
        B2038_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B2039"]
    #[inline(always)]
    pub fn b2039(&self) -> B2039_R {
        B2039_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B2040"]
    #[inline(always)]
    pub fn b2040(&self) -> B2040_R {
        B2040_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B2041"]
    #[inline(always)]
    pub fn b2041(&self) -> B2041_R {
        B2041_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B2042"]
    #[inline(always)]
    pub fn b2042(&self) -> B2042_R {
        B2042_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B2043"]
    #[inline(always)]
    pub fn b2043(&self) -> B2043_R {
        B2043_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B2044"]
    #[inline(always)]
    pub fn b2044(&self) -> B2044_R {
        B2044_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B2045"]
    #[inline(always)]
    pub fn b2045(&self) -> B2045_R {
        B2045_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B2046"]
    #[inline(always)]
    pub fn b2046(&self) -> B2046_R {
        B2046_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B2047"]
    #[inline(always)]
    pub fn b2047(&self) -> B2047_R {
        B2047_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B2016"]
    #[inline(always)]
    pub fn b2016(&mut self) -> B2016_W {
        B2016_W { w: self }
    }
    #[doc = "Bit 1 - B2017"]
    #[inline(always)]
    pub fn b2017(&mut self) -> B2017_W {
        B2017_W { w: self }
    }
    #[doc = "Bit 2 - B2018"]
    #[inline(always)]
    pub fn b2018(&mut self) -> B2018_W {
        B2018_W { w: self }
    }
    #[doc = "Bit 3 - B2019"]
    #[inline(always)]
    pub fn b2019(&mut self) -> B2019_W {
        B2019_W { w: self }
    }
    #[doc = "Bit 4 - B2020"]
    #[inline(always)]
    pub fn b2020(&mut self) -> B2020_W {
        B2020_W { w: self }
    }
    #[doc = "Bit 5 - B2021"]
    #[inline(always)]
    pub fn b2021(&mut self) -> B2021_W {
        B2021_W { w: self }
    }
    #[doc = "Bit 6 - B2022"]
    #[inline(always)]
    pub fn b2022(&mut self) -> B2022_W {
        B2022_W { w: self }
    }
    #[doc = "Bit 7 - B2023"]
    #[inline(always)]
    pub fn b2023(&mut self) -> B2023_W {
        B2023_W { w: self }
    }
    #[doc = "Bit 8 - B2024"]
    #[inline(always)]
    pub fn b2024(&mut self) -> B2024_W {
        B2024_W { w: self }
    }
    #[doc = "Bit 9 - B2025"]
    #[inline(always)]
    pub fn b2025(&mut self) -> B2025_W {
        B2025_W { w: self }
    }
    #[doc = "Bit 10 - B2026"]
    #[inline(always)]
    pub fn b2026(&mut self) -> B2026_W {
        B2026_W { w: self }
    }
    #[doc = "Bit 11 - B2027"]
    #[inline(always)]
    pub fn b2027(&mut self) -> B2027_W {
        B2027_W { w: self }
    }
    #[doc = "Bit 12 - B2028"]
    #[inline(always)]
    pub fn b2028(&mut self) -> B2028_W {
        B2028_W { w: self }
    }
    #[doc = "Bit 13 - B2029"]
    #[inline(always)]
    pub fn b2029(&mut self) -> B2029_W {
        B2029_W { w: self }
    }
    #[doc = "Bit 14 - B2030"]
    #[inline(always)]
    pub fn b2030(&mut self) -> B2030_W {
        B2030_W { w: self }
    }
    #[doc = "Bit 15 - B2031"]
    #[inline(always)]
    pub fn b2031(&mut self) -> B2031_W {
        B2031_W { w: self }
    }
    #[doc = "Bit 16 - B2032"]
    #[inline(always)]
    pub fn b2032(&mut self) -> B2032_W {
        B2032_W { w: self }
    }
    #[doc = "Bit 17 - B2033"]
    #[inline(always)]
    pub fn b2033(&mut self) -> B2033_W {
        B2033_W { w: self }
    }
    #[doc = "Bit 18 - B2034"]
    #[inline(always)]
    pub fn b2034(&mut self) -> B2034_W {
        B2034_W { w: self }
    }
    #[doc = "Bit 19 - B2035"]
    #[inline(always)]
    pub fn b2035(&mut self) -> B2035_W {
        B2035_W { w: self }
    }
    #[doc = "Bit 20 - B2036"]
    #[inline(always)]
    pub fn b2036(&mut self) -> B2036_W {
        B2036_W { w: self }
    }
    #[doc = "Bit 21 - B2037"]
    #[inline(always)]
    pub fn b2037(&mut self) -> B2037_W {
        B2037_W { w: self }
    }
    #[doc = "Bit 22 - B2038"]
    #[inline(always)]
    pub fn b2038(&mut self) -> B2038_W {
        B2038_W { w: self }
    }
    #[doc = "Bit 23 - B2039"]
    #[inline(always)]
    pub fn b2039(&mut self) -> B2039_W {
        B2039_W { w: self }
    }
    #[doc = "Bit 24 - B2040"]
    #[inline(always)]
    pub fn b2040(&mut self) -> B2040_W {
        B2040_W { w: self }
    }
    #[doc = "Bit 25 - B2041"]
    #[inline(always)]
    pub fn b2041(&mut self) -> B2041_W {
        B2041_W { w: self }
    }
    #[doc = "Bit 26 - B2042"]
    #[inline(always)]
    pub fn b2042(&mut self) -> B2042_W {
        B2042_W { w: self }
    }
    #[doc = "Bit 27 - B2043"]
    #[inline(always)]
    pub fn b2043(&mut self) -> B2043_W {
        B2043_W { w: self }
    }
    #[doc = "Bit 28 - B2044"]
    #[inline(always)]
    pub fn b2044(&mut self) -> B2044_W {
        B2044_W { w: self }
    }
    #[doc = "Bit 29 - B2045"]
    #[inline(always)]
    pub fn b2045(&mut self) -> B2045_W {
        B2045_W { w: self }
    }
    #[doc = "Bit 30 - B2046"]
    #[inline(always)]
    pub fn b2046(&mut self) -> B2046_W {
        B2046_W { w: self }
    }
    #[doc = "Bit 31 - B2047"]
    #[inline(always)]
    pub fn b2047(&mut self) -> B2047_W {
        B2047_W { w: self }
    }
}
