#[doc = "Reader of register MPCBB2_VCTR60"]
pub type R = crate::R<u32, super::MPCBB2_VCTR60>;
#[doc = "Writer for register MPCBB2_VCTR60"]
pub type W = crate::W<u32, super::MPCBB2_VCTR60>;
#[doc = "Register MPCBB2_VCTR60 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB2_VCTR60 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B1920`"]
pub type B1920_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1920`"]
pub struct B1920_W<'a> {
    w: &'a mut W,
}
impl<'a> B1920_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1921`"]
pub type B1921_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1921`"]
pub struct B1921_W<'a> {
    w: &'a mut W,
}
impl<'a> B1921_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1922`"]
pub type B1922_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1922`"]
pub struct B1922_W<'a> {
    w: &'a mut W,
}
impl<'a> B1922_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1923`"]
pub type B1923_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1923`"]
pub struct B1923_W<'a> {
    w: &'a mut W,
}
impl<'a> B1923_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1924`"]
pub type B1924_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1924`"]
pub struct B1924_W<'a> {
    w: &'a mut W,
}
impl<'a> B1924_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1925`"]
pub type B1925_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1925`"]
pub struct B1925_W<'a> {
    w: &'a mut W,
}
impl<'a> B1925_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1926`"]
pub type B1926_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1926`"]
pub struct B1926_W<'a> {
    w: &'a mut W,
}
impl<'a> B1926_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1927`"]
pub type B1927_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1927`"]
pub struct B1927_W<'a> {
    w: &'a mut W,
}
impl<'a> B1927_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1928`"]
pub type B1928_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1928`"]
pub struct B1928_W<'a> {
    w: &'a mut W,
}
impl<'a> B1928_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1929`"]
pub type B1929_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1929`"]
pub struct B1929_W<'a> {
    w: &'a mut W,
}
impl<'a> B1929_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1930`"]
pub type B1930_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1930`"]
pub struct B1930_W<'a> {
    w: &'a mut W,
}
impl<'a> B1930_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1931`"]
pub type B1931_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1931`"]
pub struct B1931_W<'a> {
    w: &'a mut W,
}
impl<'a> B1931_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1932`"]
pub type B1932_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1932`"]
pub struct B1932_W<'a> {
    w: &'a mut W,
}
impl<'a> B1932_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1933`"]
pub type B1933_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1933`"]
pub struct B1933_W<'a> {
    w: &'a mut W,
}
impl<'a> B1933_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1934`"]
pub type B1934_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1934`"]
pub struct B1934_W<'a> {
    w: &'a mut W,
}
impl<'a> B1934_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1935`"]
pub type B1935_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1935`"]
pub struct B1935_W<'a> {
    w: &'a mut W,
}
impl<'a> B1935_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1936`"]
pub type B1936_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1936`"]
pub struct B1936_W<'a> {
    w: &'a mut W,
}
impl<'a> B1936_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1937`"]
pub type B1937_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1937`"]
pub struct B1937_W<'a> {
    w: &'a mut W,
}
impl<'a> B1937_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1938`"]
pub type B1938_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1938`"]
pub struct B1938_W<'a> {
    w: &'a mut W,
}
impl<'a> B1938_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1939`"]
pub type B1939_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1939`"]
pub struct B1939_W<'a> {
    w: &'a mut W,
}
impl<'a> B1939_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1940`"]
pub type B1940_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1940`"]
pub struct B1940_W<'a> {
    w: &'a mut W,
}
impl<'a> B1940_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1941`"]
pub type B1941_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1941`"]
pub struct B1941_W<'a> {
    w: &'a mut W,
}
impl<'a> B1941_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1942`"]
pub type B1942_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1942`"]
pub struct B1942_W<'a> {
    w: &'a mut W,
}
impl<'a> B1942_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1943`"]
pub type B1943_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1943`"]
pub struct B1943_W<'a> {
    w: &'a mut W,
}
impl<'a> B1943_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1944`"]
pub type B1944_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1944`"]
pub struct B1944_W<'a> {
    w: &'a mut W,
}
impl<'a> B1944_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1945`"]
pub type B1945_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1945`"]
pub struct B1945_W<'a> {
    w: &'a mut W,
}
impl<'a> B1945_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1946`"]
pub type B1946_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1946`"]
pub struct B1946_W<'a> {
    w: &'a mut W,
}
impl<'a> B1946_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1947`"]
pub type B1947_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1947`"]
pub struct B1947_W<'a> {
    w: &'a mut W,
}
impl<'a> B1947_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1948`"]
pub type B1948_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1948`"]
pub struct B1948_W<'a> {
    w: &'a mut W,
}
impl<'a> B1948_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1949`"]
pub type B1949_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1949`"]
pub struct B1949_W<'a> {
    w: &'a mut W,
}
impl<'a> B1949_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1950`"]
pub type B1950_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1950`"]
pub struct B1950_W<'a> {
    w: &'a mut W,
}
impl<'a> B1950_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1951`"]
pub type B1951_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1951`"]
pub struct B1951_W<'a> {
    w: &'a mut W,
}
impl<'a> B1951_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1920"]
    #[inline(always)]
    pub fn b1920(&self) -> B1920_R {
        B1920_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1921"]
    #[inline(always)]
    pub fn b1921(&self) -> B1921_R {
        B1921_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1922"]
    #[inline(always)]
    pub fn b1922(&self) -> B1922_R {
        B1922_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1923"]
    #[inline(always)]
    pub fn b1923(&self) -> B1923_R {
        B1923_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1924"]
    #[inline(always)]
    pub fn b1924(&self) -> B1924_R {
        B1924_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1925"]
    #[inline(always)]
    pub fn b1925(&self) -> B1925_R {
        B1925_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1926"]
    #[inline(always)]
    pub fn b1926(&self) -> B1926_R {
        B1926_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1927"]
    #[inline(always)]
    pub fn b1927(&self) -> B1927_R {
        B1927_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1928"]
    #[inline(always)]
    pub fn b1928(&self) -> B1928_R {
        B1928_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1929"]
    #[inline(always)]
    pub fn b1929(&self) -> B1929_R {
        B1929_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1930"]
    #[inline(always)]
    pub fn b1930(&self) -> B1930_R {
        B1930_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1931"]
    #[inline(always)]
    pub fn b1931(&self) -> B1931_R {
        B1931_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1932"]
    #[inline(always)]
    pub fn b1932(&self) -> B1932_R {
        B1932_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1933"]
    #[inline(always)]
    pub fn b1933(&self) -> B1933_R {
        B1933_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1934"]
    #[inline(always)]
    pub fn b1934(&self) -> B1934_R {
        B1934_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1935"]
    #[inline(always)]
    pub fn b1935(&self) -> B1935_R {
        B1935_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1936"]
    #[inline(always)]
    pub fn b1936(&self) -> B1936_R {
        B1936_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1937"]
    #[inline(always)]
    pub fn b1937(&self) -> B1937_R {
        B1937_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1938"]
    #[inline(always)]
    pub fn b1938(&self) -> B1938_R {
        B1938_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1939"]
    #[inline(always)]
    pub fn b1939(&self) -> B1939_R {
        B1939_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1940"]
    #[inline(always)]
    pub fn b1940(&self) -> B1940_R {
        B1940_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1941"]
    #[inline(always)]
    pub fn b1941(&self) -> B1941_R {
        B1941_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1942"]
    #[inline(always)]
    pub fn b1942(&self) -> B1942_R {
        B1942_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1943"]
    #[inline(always)]
    pub fn b1943(&self) -> B1943_R {
        B1943_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1944"]
    #[inline(always)]
    pub fn b1944(&self) -> B1944_R {
        B1944_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1945"]
    #[inline(always)]
    pub fn b1945(&self) -> B1945_R {
        B1945_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1946"]
    #[inline(always)]
    pub fn b1946(&self) -> B1946_R {
        B1946_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1947"]
    #[inline(always)]
    pub fn b1947(&self) -> B1947_R {
        B1947_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1948"]
    #[inline(always)]
    pub fn b1948(&self) -> B1948_R {
        B1948_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1949"]
    #[inline(always)]
    pub fn b1949(&self) -> B1949_R {
        B1949_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1950"]
    #[inline(always)]
    pub fn b1950(&self) -> B1950_R {
        B1950_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1951"]
    #[inline(always)]
    pub fn b1951(&self) -> B1951_R {
        B1951_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1920"]
    #[inline(always)]
    pub fn b1920(&mut self) -> B1920_W {
        B1920_W { w: self }
    }
    #[doc = "Bit 1 - B1921"]
    #[inline(always)]
    pub fn b1921(&mut self) -> B1921_W {
        B1921_W { w: self }
    }
    #[doc = "Bit 2 - B1922"]
    #[inline(always)]
    pub fn b1922(&mut self) -> B1922_W {
        B1922_W { w: self }
    }
    #[doc = "Bit 3 - B1923"]
    #[inline(always)]
    pub fn b1923(&mut self) -> B1923_W {
        B1923_W { w: self }
    }
    #[doc = "Bit 4 - B1924"]
    #[inline(always)]
    pub fn b1924(&mut self) -> B1924_W {
        B1924_W { w: self }
    }
    #[doc = "Bit 5 - B1925"]
    #[inline(always)]
    pub fn b1925(&mut self) -> B1925_W {
        B1925_W { w: self }
    }
    #[doc = "Bit 6 - B1926"]
    #[inline(always)]
    pub fn b1926(&mut self) -> B1926_W {
        B1926_W { w: self }
    }
    #[doc = "Bit 7 - B1927"]
    #[inline(always)]
    pub fn b1927(&mut self) -> B1927_W {
        B1927_W { w: self }
    }
    #[doc = "Bit 8 - B1928"]
    #[inline(always)]
    pub fn b1928(&mut self) -> B1928_W {
        B1928_W { w: self }
    }
    #[doc = "Bit 9 - B1929"]
    #[inline(always)]
    pub fn b1929(&mut self) -> B1929_W {
        B1929_W { w: self }
    }
    #[doc = "Bit 10 - B1930"]
    #[inline(always)]
    pub fn b1930(&mut self) -> B1930_W {
        B1930_W { w: self }
    }
    #[doc = "Bit 11 - B1931"]
    #[inline(always)]
    pub fn b1931(&mut self) -> B1931_W {
        B1931_W { w: self }
    }
    #[doc = "Bit 12 - B1932"]
    #[inline(always)]
    pub fn b1932(&mut self) -> B1932_W {
        B1932_W { w: self }
    }
    #[doc = "Bit 13 - B1933"]
    #[inline(always)]
    pub fn b1933(&mut self) -> B1933_W {
        B1933_W { w: self }
    }
    #[doc = "Bit 14 - B1934"]
    #[inline(always)]
    pub fn b1934(&mut self) -> B1934_W {
        B1934_W { w: self }
    }
    #[doc = "Bit 15 - B1935"]
    #[inline(always)]
    pub fn b1935(&mut self) -> B1935_W {
        B1935_W { w: self }
    }
    #[doc = "Bit 16 - B1936"]
    #[inline(always)]
    pub fn b1936(&mut self) -> B1936_W {
        B1936_W { w: self }
    }
    #[doc = "Bit 17 - B1937"]
    #[inline(always)]
    pub fn b1937(&mut self) -> B1937_W {
        B1937_W { w: self }
    }
    #[doc = "Bit 18 - B1938"]
    #[inline(always)]
    pub fn b1938(&mut self) -> B1938_W {
        B1938_W { w: self }
    }
    #[doc = "Bit 19 - B1939"]
    #[inline(always)]
    pub fn b1939(&mut self) -> B1939_W {
        B1939_W { w: self }
    }
    #[doc = "Bit 20 - B1940"]
    #[inline(always)]
    pub fn b1940(&mut self) -> B1940_W {
        B1940_W { w: self }
    }
    #[doc = "Bit 21 - B1941"]
    #[inline(always)]
    pub fn b1941(&mut self) -> B1941_W {
        B1941_W { w: self }
    }
    #[doc = "Bit 22 - B1942"]
    #[inline(always)]
    pub fn b1942(&mut self) -> B1942_W {
        B1942_W { w: self }
    }
    #[doc = "Bit 23 - B1943"]
    #[inline(always)]
    pub fn b1943(&mut self) -> B1943_W {
        B1943_W { w: self }
    }
    #[doc = "Bit 24 - B1944"]
    #[inline(always)]
    pub fn b1944(&mut self) -> B1944_W {
        B1944_W { w: self }
    }
    #[doc = "Bit 25 - B1945"]
    #[inline(always)]
    pub fn b1945(&mut self) -> B1945_W {
        B1945_W { w: self }
    }
    #[doc = "Bit 26 - B1946"]
    #[inline(always)]
    pub fn b1946(&mut self) -> B1946_W {
        B1946_W { w: self }
    }
    #[doc = "Bit 27 - B1947"]
    #[inline(always)]
    pub fn b1947(&mut self) -> B1947_W {
        B1947_W { w: self }
    }
    #[doc = "Bit 28 - B1948"]
    #[inline(always)]
    pub fn b1948(&mut self) -> B1948_W {
        B1948_W { w: self }
    }
    #[doc = "Bit 29 - B1949"]
    #[inline(always)]
    pub fn b1949(&mut self) -> B1949_W {
        B1949_W { w: self }
    }
    #[doc = "Bit 30 - B1950"]
    #[inline(always)]
    pub fn b1950(&mut self) -> B1950_W {
        B1950_W { w: self }
    }
    #[doc = "Bit 31 - B1951"]
    #[inline(always)]
    pub fn b1951(&mut self) -> B1951_W {
        B1951_W { w: self }
    }
}
