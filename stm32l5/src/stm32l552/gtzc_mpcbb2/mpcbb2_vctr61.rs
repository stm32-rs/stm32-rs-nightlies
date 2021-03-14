#[doc = "Reader of register MPCBB2_VCTR61"]
pub type R = crate::R<u32, super::MPCBB2_VCTR61>;
#[doc = "Writer for register MPCBB2_VCTR61"]
pub type W = crate::W<u32, super::MPCBB2_VCTR61>;
#[doc = "Register MPCBB2_VCTR61 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB2_VCTR61 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B1952`"]
pub type B1952_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1952`"]
pub struct B1952_W<'a> {
    w: &'a mut W,
}
impl<'a> B1952_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1953`"]
pub type B1953_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1953`"]
pub struct B1953_W<'a> {
    w: &'a mut W,
}
impl<'a> B1953_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1954`"]
pub type B1954_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1954`"]
pub struct B1954_W<'a> {
    w: &'a mut W,
}
impl<'a> B1954_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1955`"]
pub type B1955_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1955`"]
pub struct B1955_W<'a> {
    w: &'a mut W,
}
impl<'a> B1955_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1956`"]
pub type B1956_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1956`"]
pub struct B1956_W<'a> {
    w: &'a mut W,
}
impl<'a> B1956_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1957`"]
pub type B1957_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1957`"]
pub struct B1957_W<'a> {
    w: &'a mut W,
}
impl<'a> B1957_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1958`"]
pub type B1958_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1958`"]
pub struct B1958_W<'a> {
    w: &'a mut W,
}
impl<'a> B1958_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1959`"]
pub type B1959_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1959`"]
pub struct B1959_W<'a> {
    w: &'a mut W,
}
impl<'a> B1959_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1960`"]
pub type B1960_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1960`"]
pub struct B1960_W<'a> {
    w: &'a mut W,
}
impl<'a> B1960_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1961`"]
pub type B1961_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1961`"]
pub struct B1961_W<'a> {
    w: &'a mut W,
}
impl<'a> B1961_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1962`"]
pub type B1962_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1962`"]
pub struct B1962_W<'a> {
    w: &'a mut W,
}
impl<'a> B1962_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1963`"]
pub type B1963_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1963`"]
pub struct B1963_W<'a> {
    w: &'a mut W,
}
impl<'a> B1963_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1964`"]
pub type B1964_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1964`"]
pub struct B1964_W<'a> {
    w: &'a mut W,
}
impl<'a> B1964_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1965`"]
pub type B1965_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1965`"]
pub struct B1965_W<'a> {
    w: &'a mut W,
}
impl<'a> B1965_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1966`"]
pub type B1966_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1966`"]
pub struct B1966_W<'a> {
    w: &'a mut W,
}
impl<'a> B1966_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1967`"]
pub type B1967_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1967`"]
pub struct B1967_W<'a> {
    w: &'a mut W,
}
impl<'a> B1967_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1968`"]
pub type B1968_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1968`"]
pub struct B1968_W<'a> {
    w: &'a mut W,
}
impl<'a> B1968_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1969`"]
pub type B1969_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1969`"]
pub struct B1969_W<'a> {
    w: &'a mut W,
}
impl<'a> B1969_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1970`"]
pub type B1970_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1970`"]
pub struct B1970_W<'a> {
    w: &'a mut W,
}
impl<'a> B1970_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1971`"]
pub type B1971_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1971`"]
pub struct B1971_W<'a> {
    w: &'a mut W,
}
impl<'a> B1971_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1972`"]
pub type B1972_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1972`"]
pub struct B1972_W<'a> {
    w: &'a mut W,
}
impl<'a> B1972_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1973`"]
pub type B1973_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1973`"]
pub struct B1973_W<'a> {
    w: &'a mut W,
}
impl<'a> B1973_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1974`"]
pub type B1974_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1974`"]
pub struct B1974_W<'a> {
    w: &'a mut W,
}
impl<'a> B1974_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1975`"]
pub type B1975_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1975`"]
pub struct B1975_W<'a> {
    w: &'a mut W,
}
impl<'a> B1975_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1976`"]
pub type B1976_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1976`"]
pub struct B1976_W<'a> {
    w: &'a mut W,
}
impl<'a> B1976_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1977`"]
pub type B1977_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1977`"]
pub struct B1977_W<'a> {
    w: &'a mut W,
}
impl<'a> B1977_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1978`"]
pub type B1978_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1978`"]
pub struct B1978_W<'a> {
    w: &'a mut W,
}
impl<'a> B1978_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1979`"]
pub type B1979_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1979`"]
pub struct B1979_W<'a> {
    w: &'a mut W,
}
impl<'a> B1979_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1980`"]
pub type B1980_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1980`"]
pub struct B1980_W<'a> {
    w: &'a mut W,
}
impl<'a> B1980_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1981`"]
pub type B1981_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1981`"]
pub struct B1981_W<'a> {
    w: &'a mut W,
}
impl<'a> B1981_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1982`"]
pub type B1982_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1982`"]
pub struct B1982_W<'a> {
    w: &'a mut W,
}
impl<'a> B1982_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1983`"]
pub type B1983_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1983`"]
pub struct B1983_W<'a> {
    w: &'a mut W,
}
impl<'a> B1983_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1952"]
    #[inline(always)]
    pub fn b1952(&self) -> B1952_R {
        B1952_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1953"]
    #[inline(always)]
    pub fn b1953(&self) -> B1953_R {
        B1953_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1954"]
    #[inline(always)]
    pub fn b1954(&self) -> B1954_R {
        B1954_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1955"]
    #[inline(always)]
    pub fn b1955(&self) -> B1955_R {
        B1955_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1956"]
    #[inline(always)]
    pub fn b1956(&self) -> B1956_R {
        B1956_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1957"]
    #[inline(always)]
    pub fn b1957(&self) -> B1957_R {
        B1957_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1958"]
    #[inline(always)]
    pub fn b1958(&self) -> B1958_R {
        B1958_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1959"]
    #[inline(always)]
    pub fn b1959(&self) -> B1959_R {
        B1959_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1960"]
    #[inline(always)]
    pub fn b1960(&self) -> B1960_R {
        B1960_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1961"]
    #[inline(always)]
    pub fn b1961(&self) -> B1961_R {
        B1961_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1962"]
    #[inline(always)]
    pub fn b1962(&self) -> B1962_R {
        B1962_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1963"]
    #[inline(always)]
    pub fn b1963(&self) -> B1963_R {
        B1963_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1964"]
    #[inline(always)]
    pub fn b1964(&self) -> B1964_R {
        B1964_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1965"]
    #[inline(always)]
    pub fn b1965(&self) -> B1965_R {
        B1965_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1966"]
    #[inline(always)]
    pub fn b1966(&self) -> B1966_R {
        B1966_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1967"]
    #[inline(always)]
    pub fn b1967(&self) -> B1967_R {
        B1967_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1968"]
    #[inline(always)]
    pub fn b1968(&self) -> B1968_R {
        B1968_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1969"]
    #[inline(always)]
    pub fn b1969(&self) -> B1969_R {
        B1969_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1970"]
    #[inline(always)]
    pub fn b1970(&self) -> B1970_R {
        B1970_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1971"]
    #[inline(always)]
    pub fn b1971(&self) -> B1971_R {
        B1971_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1972"]
    #[inline(always)]
    pub fn b1972(&self) -> B1972_R {
        B1972_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1973"]
    #[inline(always)]
    pub fn b1973(&self) -> B1973_R {
        B1973_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1974"]
    #[inline(always)]
    pub fn b1974(&self) -> B1974_R {
        B1974_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1975"]
    #[inline(always)]
    pub fn b1975(&self) -> B1975_R {
        B1975_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1976"]
    #[inline(always)]
    pub fn b1976(&self) -> B1976_R {
        B1976_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1977"]
    #[inline(always)]
    pub fn b1977(&self) -> B1977_R {
        B1977_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1978"]
    #[inline(always)]
    pub fn b1978(&self) -> B1978_R {
        B1978_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1979"]
    #[inline(always)]
    pub fn b1979(&self) -> B1979_R {
        B1979_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1980"]
    #[inline(always)]
    pub fn b1980(&self) -> B1980_R {
        B1980_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1981"]
    #[inline(always)]
    pub fn b1981(&self) -> B1981_R {
        B1981_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1982"]
    #[inline(always)]
    pub fn b1982(&self) -> B1982_R {
        B1982_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1983"]
    #[inline(always)]
    pub fn b1983(&self) -> B1983_R {
        B1983_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1952"]
    #[inline(always)]
    pub fn b1952(&mut self) -> B1952_W {
        B1952_W { w: self }
    }
    #[doc = "Bit 1 - B1953"]
    #[inline(always)]
    pub fn b1953(&mut self) -> B1953_W {
        B1953_W { w: self }
    }
    #[doc = "Bit 2 - B1954"]
    #[inline(always)]
    pub fn b1954(&mut self) -> B1954_W {
        B1954_W { w: self }
    }
    #[doc = "Bit 3 - B1955"]
    #[inline(always)]
    pub fn b1955(&mut self) -> B1955_W {
        B1955_W { w: self }
    }
    #[doc = "Bit 4 - B1956"]
    #[inline(always)]
    pub fn b1956(&mut self) -> B1956_W {
        B1956_W { w: self }
    }
    #[doc = "Bit 5 - B1957"]
    #[inline(always)]
    pub fn b1957(&mut self) -> B1957_W {
        B1957_W { w: self }
    }
    #[doc = "Bit 6 - B1958"]
    #[inline(always)]
    pub fn b1958(&mut self) -> B1958_W {
        B1958_W { w: self }
    }
    #[doc = "Bit 7 - B1959"]
    #[inline(always)]
    pub fn b1959(&mut self) -> B1959_W {
        B1959_W { w: self }
    }
    #[doc = "Bit 8 - B1960"]
    #[inline(always)]
    pub fn b1960(&mut self) -> B1960_W {
        B1960_W { w: self }
    }
    #[doc = "Bit 9 - B1961"]
    #[inline(always)]
    pub fn b1961(&mut self) -> B1961_W {
        B1961_W { w: self }
    }
    #[doc = "Bit 10 - B1962"]
    #[inline(always)]
    pub fn b1962(&mut self) -> B1962_W {
        B1962_W { w: self }
    }
    #[doc = "Bit 11 - B1963"]
    #[inline(always)]
    pub fn b1963(&mut self) -> B1963_W {
        B1963_W { w: self }
    }
    #[doc = "Bit 12 - B1964"]
    #[inline(always)]
    pub fn b1964(&mut self) -> B1964_W {
        B1964_W { w: self }
    }
    #[doc = "Bit 13 - B1965"]
    #[inline(always)]
    pub fn b1965(&mut self) -> B1965_W {
        B1965_W { w: self }
    }
    #[doc = "Bit 14 - B1966"]
    #[inline(always)]
    pub fn b1966(&mut self) -> B1966_W {
        B1966_W { w: self }
    }
    #[doc = "Bit 15 - B1967"]
    #[inline(always)]
    pub fn b1967(&mut self) -> B1967_W {
        B1967_W { w: self }
    }
    #[doc = "Bit 16 - B1968"]
    #[inline(always)]
    pub fn b1968(&mut self) -> B1968_W {
        B1968_W { w: self }
    }
    #[doc = "Bit 17 - B1969"]
    #[inline(always)]
    pub fn b1969(&mut self) -> B1969_W {
        B1969_W { w: self }
    }
    #[doc = "Bit 18 - B1970"]
    #[inline(always)]
    pub fn b1970(&mut self) -> B1970_W {
        B1970_W { w: self }
    }
    #[doc = "Bit 19 - B1971"]
    #[inline(always)]
    pub fn b1971(&mut self) -> B1971_W {
        B1971_W { w: self }
    }
    #[doc = "Bit 20 - B1972"]
    #[inline(always)]
    pub fn b1972(&mut self) -> B1972_W {
        B1972_W { w: self }
    }
    #[doc = "Bit 21 - B1973"]
    #[inline(always)]
    pub fn b1973(&mut self) -> B1973_W {
        B1973_W { w: self }
    }
    #[doc = "Bit 22 - B1974"]
    #[inline(always)]
    pub fn b1974(&mut self) -> B1974_W {
        B1974_W { w: self }
    }
    #[doc = "Bit 23 - B1975"]
    #[inline(always)]
    pub fn b1975(&mut self) -> B1975_W {
        B1975_W { w: self }
    }
    #[doc = "Bit 24 - B1976"]
    #[inline(always)]
    pub fn b1976(&mut self) -> B1976_W {
        B1976_W { w: self }
    }
    #[doc = "Bit 25 - B1977"]
    #[inline(always)]
    pub fn b1977(&mut self) -> B1977_W {
        B1977_W { w: self }
    }
    #[doc = "Bit 26 - B1978"]
    #[inline(always)]
    pub fn b1978(&mut self) -> B1978_W {
        B1978_W { w: self }
    }
    #[doc = "Bit 27 - B1979"]
    #[inline(always)]
    pub fn b1979(&mut self) -> B1979_W {
        B1979_W { w: self }
    }
    #[doc = "Bit 28 - B1980"]
    #[inline(always)]
    pub fn b1980(&mut self) -> B1980_W {
        B1980_W { w: self }
    }
    #[doc = "Bit 29 - B1981"]
    #[inline(always)]
    pub fn b1981(&mut self) -> B1981_W {
        B1981_W { w: self }
    }
    #[doc = "Bit 30 - B1982"]
    #[inline(always)]
    pub fn b1982(&mut self) -> B1982_W {
        B1982_W { w: self }
    }
    #[doc = "Bit 31 - B1983"]
    #[inline(always)]
    pub fn b1983(&mut self) -> B1983_W {
        B1983_W { w: self }
    }
}
