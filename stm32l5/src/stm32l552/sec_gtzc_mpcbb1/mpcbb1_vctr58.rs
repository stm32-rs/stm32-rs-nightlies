#[doc = "Reader of register MPCBB1_VCTR58"]
pub type R = crate::R<u32, super::MPCBB1_VCTR58>;
#[doc = "Writer for register MPCBB1_VCTR58"]
pub type W = crate::W<u32, super::MPCBB1_VCTR58>;
#[doc = "Register MPCBB1_VCTR58 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB1_VCTR58 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B1856`"]
pub type B1856_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1856`"]
pub struct B1856_W<'a> {
    w: &'a mut W,
}
impl<'a> B1856_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1857`"]
pub type B1857_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1857`"]
pub struct B1857_W<'a> {
    w: &'a mut W,
}
impl<'a> B1857_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1858`"]
pub type B1858_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1858`"]
pub struct B1858_W<'a> {
    w: &'a mut W,
}
impl<'a> B1858_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1859`"]
pub type B1859_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1859`"]
pub struct B1859_W<'a> {
    w: &'a mut W,
}
impl<'a> B1859_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1860`"]
pub type B1860_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1860`"]
pub struct B1860_W<'a> {
    w: &'a mut W,
}
impl<'a> B1860_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1861`"]
pub type B1861_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1861`"]
pub struct B1861_W<'a> {
    w: &'a mut W,
}
impl<'a> B1861_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1862`"]
pub type B1862_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1862`"]
pub struct B1862_W<'a> {
    w: &'a mut W,
}
impl<'a> B1862_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1863`"]
pub type B1863_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1863`"]
pub struct B1863_W<'a> {
    w: &'a mut W,
}
impl<'a> B1863_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1864`"]
pub type B1864_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1864`"]
pub struct B1864_W<'a> {
    w: &'a mut W,
}
impl<'a> B1864_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1865`"]
pub type B1865_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1865`"]
pub struct B1865_W<'a> {
    w: &'a mut W,
}
impl<'a> B1865_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1866`"]
pub type B1866_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1866`"]
pub struct B1866_W<'a> {
    w: &'a mut W,
}
impl<'a> B1866_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1867`"]
pub type B1867_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1867`"]
pub struct B1867_W<'a> {
    w: &'a mut W,
}
impl<'a> B1867_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1868`"]
pub type B1868_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1868`"]
pub struct B1868_W<'a> {
    w: &'a mut W,
}
impl<'a> B1868_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1869`"]
pub type B1869_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1869`"]
pub struct B1869_W<'a> {
    w: &'a mut W,
}
impl<'a> B1869_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1870`"]
pub type B1870_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1870`"]
pub struct B1870_W<'a> {
    w: &'a mut W,
}
impl<'a> B1870_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1871`"]
pub type B1871_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1871`"]
pub struct B1871_W<'a> {
    w: &'a mut W,
}
impl<'a> B1871_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1872`"]
pub type B1872_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1872`"]
pub struct B1872_W<'a> {
    w: &'a mut W,
}
impl<'a> B1872_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1873`"]
pub type B1873_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1873`"]
pub struct B1873_W<'a> {
    w: &'a mut W,
}
impl<'a> B1873_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1874`"]
pub type B1874_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1874`"]
pub struct B1874_W<'a> {
    w: &'a mut W,
}
impl<'a> B1874_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1875`"]
pub type B1875_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1875`"]
pub struct B1875_W<'a> {
    w: &'a mut W,
}
impl<'a> B1875_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1876`"]
pub type B1876_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1876`"]
pub struct B1876_W<'a> {
    w: &'a mut W,
}
impl<'a> B1876_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1877`"]
pub type B1877_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1877`"]
pub struct B1877_W<'a> {
    w: &'a mut W,
}
impl<'a> B1877_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1878`"]
pub type B1878_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1878`"]
pub struct B1878_W<'a> {
    w: &'a mut W,
}
impl<'a> B1878_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1879`"]
pub type B1879_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1879`"]
pub struct B1879_W<'a> {
    w: &'a mut W,
}
impl<'a> B1879_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1880`"]
pub type B1880_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1880`"]
pub struct B1880_W<'a> {
    w: &'a mut W,
}
impl<'a> B1880_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1881`"]
pub type B1881_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1881`"]
pub struct B1881_W<'a> {
    w: &'a mut W,
}
impl<'a> B1881_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1882`"]
pub type B1882_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1882`"]
pub struct B1882_W<'a> {
    w: &'a mut W,
}
impl<'a> B1882_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1883`"]
pub type B1883_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1883`"]
pub struct B1883_W<'a> {
    w: &'a mut W,
}
impl<'a> B1883_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1884`"]
pub type B1884_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1884`"]
pub struct B1884_W<'a> {
    w: &'a mut W,
}
impl<'a> B1884_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1885`"]
pub type B1885_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1885`"]
pub struct B1885_W<'a> {
    w: &'a mut W,
}
impl<'a> B1885_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1886`"]
pub type B1886_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1886`"]
pub struct B1886_W<'a> {
    w: &'a mut W,
}
impl<'a> B1886_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1887`"]
pub type B1887_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1887`"]
pub struct B1887_W<'a> {
    w: &'a mut W,
}
impl<'a> B1887_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1856"]
    #[inline(always)]
    pub fn b1856(&self) -> B1856_R {
        B1856_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1857"]
    #[inline(always)]
    pub fn b1857(&self) -> B1857_R {
        B1857_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1858"]
    #[inline(always)]
    pub fn b1858(&self) -> B1858_R {
        B1858_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1859"]
    #[inline(always)]
    pub fn b1859(&self) -> B1859_R {
        B1859_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1860"]
    #[inline(always)]
    pub fn b1860(&self) -> B1860_R {
        B1860_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1861"]
    #[inline(always)]
    pub fn b1861(&self) -> B1861_R {
        B1861_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1862"]
    #[inline(always)]
    pub fn b1862(&self) -> B1862_R {
        B1862_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1863"]
    #[inline(always)]
    pub fn b1863(&self) -> B1863_R {
        B1863_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1864"]
    #[inline(always)]
    pub fn b1864(&self) -> B1864_R {
        B1864_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1865"]
    #[inline(always)]
    pub fn b1865(&self) -> B1865_R {
        B1865_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1866"]
    #[inline(always)]
    pub fn b1866(&self) -> B1866_R {
        B1866_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1867"]
    #[inline(always)]
    pub fn b1867(&self) -> B1867_R {
        B1867_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1868"]
    #[inline(always)]
    pub fn b1868(&self) -> B1868_R {
        B1868_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1869"]
    #[inline(always)]
    pub fn b1869(&self) -> B1869_R {
        B1869_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1870"]
    #[inline(always)]
    pub fn b1870(&self) -> B1870_R {
        B1870_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1871"]
    #[inline(always)]
    pub fn b1871(&self) -> B1871_R {
        B1871_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1872"]
    #[inline(always)]
    pub fn b1872(&self) -> B1872_R {
        B1872_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1873"]
    #[inline(always)]
    pub fn b1873(&self) -> B1873_R {
        B1873_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1874"]
    #[inline(always)]
    pub fn b1874(&self) -> B1874_R {
        B1874_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1875"]
    #[inline(always)]
    pub fn b1875(&self) -> B1875_R {
        B1875_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1876"]
    #[inline(always)]
    pub fn b1876(&self) -> B1876_R {
        B1876_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1877"]
    #[inline(always)]
    pub fn b1877(&self) -> B1877_R {
        B1877_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1878"]
    #[inline(always)]
    pub fn b1878(&self) -> B1878_R {
        B1878_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1879"]
    #[inline(always)]
    pub fn b1879(&self) -> B1879_R {
        B1879_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1880"]
    #[inline(always)]
    pub fn b1880(&self) -> B1880_R {
        B1880_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1881"]
    #[inline(always)]
    pub fn b1881(&self) -> B1881_R {
        B1881_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1882"]
    #[inline(always)]
    pub fn b1882(&self) -> B1882_R {
        B1882_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1883"]
    #[inline(always)]
    pub fn b1883(&self) -> B1883_R {
        B1883_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1884"]
    #[inline(always)]
    pub fn b1884(&self) -> B1884_R {
        B1884_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1885"]
    #[inline(always)]
    pub fn b1885(&self) -> B1885_R {
        B1885_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1886"]
    #[inline(always)]
    pub fn b1886(&self) -> B1886_R {
        B1886_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1887"]
    #[inline(always)]
    pub fn b1887(&self) -> B1887_R {
        B1887_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1856"]
    #[inline(always)]
    pub fn b1856(&mut self) -> B1856_W {
        B1856_W { w: self }
    }
    #[doc = "Bit 1 - B1857"]
    #[inline(always)]
    pub fn b1857(&mut self) -> B1857_W {
        B1857_W { w: self }
    }
    #[doc = "Bit 2 - B1858"]
    #[inline(always)]
    pub fn b1858(&mut self) -> B1858_W {
        B1858_W { w: self }
    }
    #[doc = "Bit 3 - B1859"]
    #[inline(always)]
    pub fn b1859(&mut self) -> B1859_W {
        B1859_W { w: self }
    }
    #[doc = "Bit 4 - B1860"]
    #[inline(always)]
    pub fn b1860(&mut self) -> B1860_W {
        B1860_W { w: self }
    }
    #[doc = "Bit 5 - B1861"]
    #[inline(always)]
    pub fn b1861(&mut self) -> B1861_W {
        B1861_W { w: self }
    }
    #[doc = "Bit 6 - B1862"]
    #[inline(always)]
    pub fn b1862(&mut self) -> B1862_W {
        B1862_W { w: self }
    }
    #[doc = "Bit 7 - B1863"]
    #[inline(always)]
    pub fn b1863(&mut self) -> B1863_W {
        B1863_W { w: self }
    }
    #[doc = "Bit 8 - B1864"]
    #[inline(always)]
    pub fn b1864(&mut self) -> B1864_W {
        B1864_W { w: self }
    }
    #[doc = "Bit 9 - B1865"]
    #[inline(always)]
    pub fn b1865(&mut self) -> B1865_W {
        B1865_W { w: self }
    }
    #[doc = "Bit 10 - B1866"]
    #[inline(always)]
    pub fn b1866(&mut self) -> B1866_W {
        B1866_W { w: self }
    }
    #[doc = "Bit 11 - B1867"]
    #[inline(always)]
    pub fn b1867(&mut self) -> B1867_W {
        B1867_W { w: self }
    }
    #[doc = "Bit 12 - B1868"]
    #[inline(always)]
    pub fn b1868(&mut self) -> B1868_W {
        B1868_W { w: self }
    }
    #[doc = "Bit 13 - B1869"]
    #[inline(always)]
    pub fn b1869(&mut self) -> B1869_W {
        B1869_W { w: self }
    }
    #[doc = "Bit 14 - B1870"]
    #[inline(always)]
    pub fn b1870(&mut self) -> B1870_W {
        B1870_W { w: self }
    }
    #[doc = "Bit 15 - B1871"]
    #[inline(always)]
    pub fn b1871(&mut self) -> B1871_W {
        B1871_W { w: self }
    }
    #[doc = "Bit 16 - B1872"]
    #[inline(always)]
    pub fn b1872(&mut self) -> B1872_W {
        B1872_W { w: self }
    }
    #[doc = "Bit 17 - B1873"]
    #[inline(always)]
    pub fn b1873(&mut self) -> B1873_W {
        B1873_W { w: self }
    }
    #[doc = "Bit 18 - B1874"]
    #[inline(always)]
    pub fn b1874(&mut self) -> B1874_W {
        B1874_W { w: self }
    }
    #[doc = "Bit 19 - B1875"]
    #[inline(always)]
    pub fn b1875(&mut self) -> B1875_W {
        B1875_W { w: self }
    }
    #[doc = "Bit 20 - B1876"]
    #[inline(always)]
    pub fn b1876(&mut self) -> B1876_W {
        B1876_W { w: self }
    }
    #[doc = "Bit 21 - B1877"]
    #[inline(always)]
    pub fn b1877(&mut self) -> B1877_W {
        B1877_W { w: self }
    }
    #[doc = "Bit 22 - B1878"]
    #[inline(always)]
    pub fn b1878(&mut self) -> B1878_W {
        B1878_W { w: self }
    }
    #[doc = "Bit 23 - B1879"]
    #[inline(always)]
    pub fn b1879(&mut self) -> B1879_W {
        B1879_W { w: self }
    }
    #[doc = "Bit 24 - B1880"]
    #[inline(always)]
    pub fn b1880(&mut self) -> B1880_W {
        B1880_W { w: self }
    }
    #[doc = "Bit 25 - B1881"]
    #[inline(always)]
    pub fn b1881(&mut self) -> B1881_W {
        B1881_W { w: self }
    }
    #[doc = "Bit 26 - B1882"]
    #[inline(always)]
    pub fn b1882(&mut self) -> B1882_W {
        B1882_W { w: self }
    }
    #[doc = "Bit 27 - B1883"]
    #[inline(always)]
    pub fn b1883(&mut self) -> B1883_W {
        B1883_W { w: self }
    }
    #[doc = "Bit 28 - B1884"]
    #[inline(always)]
    pub fn b1884(&mut self) -> B1884_W {
        B1884_W { w: self }
    }
    #[doc = "Bit 29 - B1885"]
    #[inline(always)]
    pub fn b1885(&mut self) -> B1885_W {
        B1885_W { w: self }
    }
    #[doc = "Bit 30 - B1886"]
    #[inline(always)]
    pub fn b1886(&mut self) -> B1886_W {
        B1886_W { w: self }
    }
    #[doc = "Bit 31 - B1887"]
    #[inline(always)]
    pub fn b1887(&mut self) -> B1887_W {
        B1887_W { w: self }
    }
}
