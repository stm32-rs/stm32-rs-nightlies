#[doc = "Reader of register MPCBB2_VCTR57"]
pub type R = crate::R<u32, super::MPCBB2_VCTR57>;
#[doc = "Writer for register MPCBB2_VCTR57"]
pub type W = crate::W<u32, super::MPCBB2_VCTR57>;
#[doc = "Register MPCBB2_VCTR57 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB2_VCTR57 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B1824`"]
pub type B1824_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1824`"]
pub struct B1824_W<'a> {
    w: &'a mut W,
}
impl<'a> B1824_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1825`"]
pub type B1825_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1825`"]
pub struct B1825_W<'a> {
    w: &'a mut W,
}
impl<'a> B1825_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1826`"]
pub type B1826_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1826`"]
pub struct B1826_W<'a> {
    w: &'a mut W,
}
impl<'a> B1826_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1827`"]
pub type B1827_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1827`"]
pub struct B1827_W<'a> {
    w: &'a mut W,
}
impl<'a> B1827_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1828`"]
pub type B1828_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1828`"]
pub struct B1828_W<'a> {
    w: &'a mut W,
}
impl<'a> B1828_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1829`"]
pub type B1829_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1829`"]
pub struct B1829_W<'a> {
    w: &'a mut W,
}
impl<'a> B1829_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1830`"]
pub type B1830_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1830`"]
pub struct B1830_W<'a> {
    w: &'a mut W,
}
impl<'a> B1830_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1831`"]
pub type B1831_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1831`"]
pub struct B1831_W<'a> {
    w: &'a mut W,
}
impl<'a> B1831_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1832`"]
pub type B1832_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1832`"]
pub struct B1832_W<'a> {
    w: &'a mut W,
}
impl<'a> B1832_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1833`"]
pub type B1833_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1833`"]
pub struct B1833_W<'a> {
    w: &'a mut W,
}
impl<'a> B1833_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1834`"]
pub type B1834_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1834`"]
pub struct B1834_W<'a> {
    w: &'a mut W,
}
impl<'a> B1834_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1835`"]
pub type B1835_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1835`"]
pub struct B1835_W<'a> {
    w: &'a mut W,
}
impl<'a> B1835_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1836`"]
pub type B1836_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1836`"]
pub struct B1836_W<'a> {
    w: &'a mut W,
}
impl<'a> B1836_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1837`"]
pub type B1837_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1837`"]
pub struct B1837_W<'a> {
    w: &'a mut W,
}
impl<'a> B1837_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1838`"]
pub type B1838_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1838`"]
pub struct B1838_W<'a> {
    w: &'a mut W,
}
impl<'a> B1838_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1839`"]
pub type B1839_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1839`"]
pub struct B1839_W<'a> {
    w: &'a mut W,
}
impl<'a> B1839_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1840`"]
pub type B1840_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1840`"]
pub struct B1840_W<'a> {
    w: &'a mut W,
}
impl<'a> B1840_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1841`"]
pub type B1841_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1841`"]
pub struct B1841_W<'a> {
    w: &'a mut W,
}
impl<'a> B1841_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1842`"]
pub type B1842_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1842`"]
pub struct B1842_W<'a> {
    w: &'a mut W,
}
impl<'a> B1842_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1843`"]
pub type B1843_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1843`"]
pub struct B1843_W<'a> {
    w: &'a mut W,
}
impl<'a> B1843_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1844`"]
pub type B1844_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1844`"]
pub struct B1844_W<'a> {
    w: &'a mut W,
}
impl<'a> B1844_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1845`"]
pub type B1845_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1845`"]
pub struct B1845_W<'a> {
    w: &'a mut W,
}
impl<'a> B1845_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1846`"]
pub type B1846_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1846`"]
pub struct B1846_W<'a> {
    w: &'a mut W,
}
impl<'a> B1846_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1847`"]
pub type B1847_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1847`"]
pub struct B1847_W<'a> {
    w: &'a mut W,
}
impl<'a> B1847_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1848`"]
pub type B1848_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1848`"]
pub struct B1848_W<'a> {
    w: &'a mut W,
}
impl<'a> B1848_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1849`"]
pub type B1849_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1849`"]
pub struct B1849_W<'a> {
    w: &'a mut W,
}
impl<'a> B1849_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1850`"]
pub type B1850_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1850`"]
pub struct B1850_W<'a> {
    w: &'a mut W,
}
impl<'a> B1850_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1851`"]
pub type B1851_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1851`"]
pub struct B1851_W<'a> {
    w: &'a mut W,
}
impl<'a> B1851_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1852`"]
pub type B1852_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1852`"]
pub struct B1852_W<'a> {
    w: &'a mut W,
}
impl<'a> B1852_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1853`"]
pub type B1853_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1853`"]
pub struct B1853_W<'a> {
    w: &'a mut W,
}
impl<'a> B1853_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1854`"]
pub type B1854_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1854`"]
pub struct B1854_W<'a> {
    w: &'a mut W,
}
impl<'a> B1854_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1855`"]
pub type B1855_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1855`"]
pub struct B1855_W<'a> {
    w: &'a mut W,
}
impl<'a> B1855_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1824"]
    #[inline(always)]
    pub fn b1824(&self) -> B1824_R {
        B1824_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1825"]
    #[inline(always)]
    pub fn b1825(&self) -> B1825_R {
        B1825_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1826"]
    #[inline(always)]
    pub fn b1826(&self) -> B1826_R {
        B1826_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1827"]
    #[inline(always)]
    pub fn b1827(&self) -> B1827_R {
        B1827_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1828"]
    #[inline(always)]
    pub fn b1828(&self) -> B1828_R {
        B1828_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1829"]
    #[inline(always)]
    pub fn b1829(&self) -> B1829_R {
        B1829_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1830"]
    #[inline(always)]
    pub fn b1830(&self) -> B1830_R {
        B1830_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1831"]
    #[inline(always)]
    pub fn b1831(&self) -> B1831_R {
        B1831_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1832"]
    #[inline(always)]
    pub fn b1832(&self) -> B1832_R {
        B1832_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1833"]
    #[inline(always)]
    pub fn b1833(&self) -> B1833_R {
        B1833_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1834"]
    #[inline(always)]
    pub fn b1834(&self) -> B1834_R {
        B1834_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1835"]
    #[inline(always)]
    pub fn b1835(&self) -> B1835_R {
        B1835_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1836"]
    #[inline(always)]
    pub fn b1836(&self) -> B1836_R {
        B1836_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1837"]
    #[inline(always)]
    pub fn b1837(&self) -> B1837_R {
        B1837_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1838"]
    #[inline(always)]
    pub fn b1838(&self) -> B1838_R {
        B1838_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1839"]
    #[inline(always)]
    pub fn b1839(&self) -> B1839_R {
        B1839_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1840"]
    #[inline(always)]
    pub fn b1840(&self) -> B1840_R {
        B1840_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1841"]
    #[inline(always)]
    pub fn b1841(&self) -> B1841_R {
        B1841_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1842"]
    #[inline(always)]
    pub fn b1842(&self) -> B1842_R {
        B1842_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1843"]
    #[inline(always)]
    pub fn b1843(&self) -> B1843_R {
        B1843_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1844"]
    #[inline(always)]
    pub fn b1844(&self) -> B1844_R {
        B1844_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1845"]
    #[inline(always)]
    pub fn b1845(&self) -> B1845_R {
        B1845_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1846"]
    #[inline(always)]
    pub fn b1846(&self) -> B1846_R {
        B1846_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1847"]
    #[inline(always)]
    pub fn b1847(&self) -> B1847_R {
        B1847_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1848"]
    #[inline(always)]
    pub fn b1848(&self) -> B1848_R {
        B1848_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1849"]
    #[inline(always)]
    pub fn b1849(&self) -> B1849_R {
        B1849_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1850"]
    #[inline(always)]
    pub fn b1850(&self) -> B1850_R {
        B1850_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1851"]
    #[inline(always)]
    pub fn b1851(&self) -> B1851_R {
        B1851_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1852"]
    #[inline(always)]
    pub fn b1852(&self) -> B1852_R {
        B1852_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1853"]
    #[inline(always)]
    pub fn b1853(&self) -> B1853_R {
        B1853_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1854"]
    #[inline(always)]
    pub fn b1854(&self) -> B1854_R {
        B1854_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1855"]
    #[inline(always)]
    pub fn b1855(&self) -> B1855_R {
        B1855_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1824"]
    #[inline(always)]
    pub fn b1824(&mut self) -> B1824_W {
        B1824_W { w: self }
    }
    #[doc = "Bit 1 - B1825"]
    #[inline(always)]
    pub fn b1825(&mut self) -> B1825_W {
        B1825_W { w: self }
    }
    #[doc = "Bit 2 - B1826"]
    #[inline(always)]
    pub fn b1826(&mut self) -> B1826_W {
        B1826_W { w: self }
    }
    #[doc = "Bit 3 - B1827"]
    #[inline(always)]
    pub fn b1827(&mut self) -> B1827_W {
        B1827_W { w: self }
    }
    #[doc = "Bit 4 - B1828"]
    #[inline(always)]
    pub fn b1828(&mut self) -> B1828_W {
        B1828_W { w: self }
    }
    #[doc = "Bit 5 - B1829"]
    #[inline(always)]
    pub fn b1829(&mut self) -> B1829_W {
        B1829_W { w: self }
    }
    #[doc = "Bit 6 - B1830"]
    #[inline(always)]
    pub fn b1830(&mut self) -> B1830_W {
        B1830_W { w: self }
    }
    #[doc = "Bit 7 - B1831"]
    #[inline(always)]
    pub fn b1831(&mut self) -> B1831_W {
        B1831_W { w: self }
    }
    #[doc = "Bit 8 - B1832"]
    #[inline(always)]
    pub fn b1832(&mut self) -> B1832_W {
        B1832_W { w: self }
    }
    #[doc = "Bit 9 - B1833"]
    #[inline(always)]
    pub fn b1833(&mut self) -> B1833_W {
        B1833_W { w: self }
    }
    #[doc = "Bit 10 - B1834"]
    #[inline(always)]
    pub fn b1834(&mut self) -> B1834_W {
        B1834_W { w: self }
    }
    #[doc = "Bit 11 - B1835"]
    #[inline(always)]
    pub fn b1835(&mut self) -> B1835_W {
        B1835_W { w: self }
    }
    #[doc = "Bit 12 - B1836"]
    #[inline(always)]
    pub fn b1836(&mut self) -> B1836_W {
        B1836_W { w: self }
    }
    #[doc = "Bit 13 - B1837"]
    #[inline(always)]
    pub fn b1837(&mut self) -> B1837_W {
        B1837_W { w: self }
    }
    #[doc = "Bit 14 - B1838"]
    #[inline(always)]
    pub fn b1838(&mut self) -> B1838_W {
        B1838_W { w: self }
    }
    #[doc = "Bit 15 - B1839"]
    #[inline(always)]
    pub fn b1839(&mut self) -> B1839_W {
        B1839_W { w: self }
    }
    #[doc = "Bit 16 - B1840"]
    #[inline(always)]
    pub fn b1840(&mut self) -> B1840_W {
        B1840_W { w: self }
    }
    #[doc = "Bit 17 - B1841"]
    #[inline(always)]
    pub fn b1841(&mut self) -> B1841_W {
        B1841_W { w: self }
    }
    #[doc = "Bit 18 - B1842"]
    #[inline(always)]
    pub fn b1842(&mut self) -> B1842_W {
        B1842_W { w: self }
    }
    #[doc = "Bit 19 - B1843"]
    #[inline(always)]
    pub fn b1843(&mut self) -> B1843_W {
        B1843_W { w: self }
    }
    #[doc = "Bit 20 - B1844"]
    #[inline(always)]
    pub fn b1844(&mut self) -> B1844_W {
        B1844_W { w: self }
    }
    #[doc = "Bit 21 - B1845"]
    #[inline(always)]
    pub fn b1845(&mut self) -> B1845_W {
        B1845_W { w: self }
    }
    #[doc = "Bit 22 - B1846"]
    #[inline(always)]
    pub fn b1846(&mut self) -> B1846_W {
        B1846_W { w: self }
    }
    #[doc = "Bit 23 - B1847"]
    #[inline(always)]
    pub fn b1847(&mut self) -> B1847_W {
        B1847_W { w: self }
    }
    #[doc = "Bit 24 - B1848"]
    #[inline(always)]
    pub fn b1848(&mut self) -> B1848_W {
        B1848_W { w: self }
    }
    #[doc = "Bit 25 - B1849"]
    #[inline(always)]
    pub fn b1849(&mut self) -> B1849_W {
        B1849_W { w: self }
    }
    #[doc = "Bit 26 - B1850"]
    #[inline(always)]
    pub fn b1850(&mut self) -> B1850_W {
        B1850_W { w: self }
    }
    #[doc = "Bit 27 - B1851"]
    #[inline(always)]
    pub fn b1851(&mut self) -> B1851_W {
        B1851_W { w: self }
    }
    #[doc = "Bit 28 - B1852"]
    #[inline(always)]
    pub fn b1852(&mut self) -> B1852_W {
        B1852_W { w: self }
    }
    #[doc = "Bit 29 - B1853"]
    #[inline(always)]
    pub fn b1853(&mut self) -> B1853_W {
        B1853_W { w: self }
    }
    #[doc = "Bit 30 - B1854"]
    #[inline(always)]
    pub fn b1854(&mut self) -> B1854_W {
        B1854_W { w: self }
    }
    #[doc = "Bit 31 - B1855"]
    #[inline(always)]
    pub fn b1855(&mut self) -> B1855_W {
        B1855_W { w: self }
    }
}
