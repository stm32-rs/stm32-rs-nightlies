#[doc = "Register `GPIOZ_BSRR` writer"]
pub struct W(crate::W<GPIOZ_BSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOZ_BSRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GPIOZ_BSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOZ_BSRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BS0` writer - BS0"]
pub struct BS0_W<'a> {
    w: &'a mut W,
}
impl<'a> BS0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `BS1` writer - BS1"]
pub struct BS1_W<'a> {
    w: &'a mut W,
}
impl<'a> BS1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `BS2` writer - BS2"]
pub struct BS2_W<'a> {
    w: &'a mut W,
}
impl<'a> BS2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `BS3` writer - BS3"]
pub struct BS3_W<'a> {
    w: &'a mut W,
}
impl<'a> BS3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `BS4` writer - BS4"]
pub struct BS4_W<'a> {
    w: &'a mut W,
}
impl<'a> BS4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `BS5` writer - BS5"]
pub struct BS5_W<'a> {
    w: &'a mut W,
}
impl<'a> BS5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `BS6` writer - BS6"]
pub struct BS6_W<'a> {
    w: &'a mut W,
}
impl<'a> BS6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `BS7` writer - BS7"]
pub struct BS7_W<'a> {
    w: &'a mut W,
}
impl<'a> BS7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `BS8` writer - BS8"]
pub struct BS8_W<'a> {
    w: &'a mut W,
}
impl<'a> BS8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `BS9` writer - BS9"]
pub struct BS9_W<'a> {
    w: &'a mut W,
}
impl<'a> BS9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `BS10` writer - BS10"]
pub struct BS10_W<'a> {
    w: &'a mut W,
}
impl<'a> BS10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `BS11` writer - BS11"]
pub struct BS11_W<'a> {
    w: &'a mut W,
}
impl<'a> BS11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `BS12` writer - BS12"]
pub struct BS12_W<'a> {
    w: &'a mut W,
}
impl<'a> BS12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `BS13` writer - BS13"]
pub struct BS13_W<'a> {
    w: &'a mut W,
}
impl<'a> BS13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `BS14` writer - BS14"]
pub struct BS14_W<'a> {
    w: &'a mut W,
}
impl<'a> BS14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `BS15` writer - BS15"]
pub struct BS15_W<'a> {
    w: &'a mut W,
}
impl<'a> BS15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `BR0` writer - BR0"]
pub struct BR0_W<'a> {
    w: &'a mut W,
}
impl<'a> BR0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `BR1` writer - BR1"]
pub struct BR1_W<'a> {
    w: &'a mut W,
}
impl<'a> BR1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `BR2` writer - BR2"]
pub struct BR2_W<'a> {
    w: &'a mut W,
}
impl<'a> BR2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `BR3` writer - BR3"]
pub struct BR3_W<'a> {
    w: &'a mut W,
}
impl<'a> BR3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `BR4` writer - BR4"]
pub struct BR4_W<'a> {
    w: &'a mut W,
}
impl<'a> BR4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `BR5` writer - BR5"]
pub struct BR5_W<'a> {
    w: &'a mut W,
}
impl<'a> BR5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `BR6` writer - BR6"]
pub struct BR6_W<'a> {
    w: &'a mut W,
}
impl<'a> BR6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `BR7` writer - BR7"]
pub struct BR7_W<'a> {
    w: &'a mut W,
}
impl<'a> BR7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `BR8` writer - BR8"]
pub struct BR8_W<'a> {
    w: &'a mut W,
}
impl<'a> BR8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `BR9` writer - BR9"]
pub struct BR9_W<'a> {
    w: &'a mut W,
}
impl<'a> BR9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `BR10` writer - BR10"]
pub struct BR10_W<'a> {
    w: &'a mut W,
}
impl<'a> BR10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `BR11` writer - BR11"]
pub struct BR11_W<'a> {
    w: &'a mut W,
}
impl<'a> BR11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `BR12` writer - BR12"]
pub struct BR12_W<'a> {
    w: &'a mut W,
}
impl<'a> BR12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `BR13` writer - BR13"]
pub struct BR13_W<'a> {
    w: &'a mut W,
}
impl<'a> BR13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `BR14` writer - BR14"]
pub struct BR14_W<'a> {
    w: &'a mut W,
}
impl<'a> BR14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `BR15` writer - BR15"]
pub struct BR15_W<'a> {
    w: &'a mut W,
}
impl<'a> BR15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - BS0"]
    #[inline(always)]
    pub fn bs0(&mut self) -> BS0_W {
        BS0_W { w: self }
    }
    #[doc = "Bit 1 - BS1"]
    #[inline(always)]
    pub fn bs1(&mut self) -> BS1_W {
        BS1_W { w: self }
    }
    #[doc = "Bit 2 - BS2"]
    #[inline(always)]
    pub fn bs2(&mut self) -> BS2_W {
        BS2_W { w: self }
    }
    #[doc = "Bit 3 - BS3"]
    #[inline(always)]
    pub fn bs3(&mut self) -> BS3_W {
        BS3_W { w: self }
    }
    #[doc = "Bit 4 - BS4"]
    #[inline(always)]
    pub fn bs4(&mut self) -> BS4_W {
        BS4_W { w: self }
    }
    #[doc = "Bit 5 - BS5"]
    #[inline(always)]
    pub fn bs5(&mut self) -> BS5_W {
        BS5_W { w: self }
    }
    #[doc = "Bit 6 - BS6"]
    #[inline(always)]
    pub fn bs6(&mut self) -> BS6_W {
        BS6_W { w: self }
    }
    #[doc = "Bit 7 - BS7"]
    #[inline(always)]
    pub fn bs7(&mut self) -> BS7_W {
        BS7_W { w: self }
    }
    #[doc = "Bit 8 - BS8"]
    #[inline(always)]
    pub fn bs8(&mut self) -> BS8_W {
        BS8_W { w: self }
    }
    #[doc = "Bit 9 - BS9"]
    #[inline(always)]
    pub fn bs9(&mut self) -> BS9_W {
        BS9_W { w: self }
    }
    #[doc = "Bit 10 - BS10"]
    #[inline(always)]
    pub fn bs10(&mut self) -> BS10_W {
        BS10_W { w: self }
    }
    #[doc = "Bit 11 - BS11"]
    #[inline(always)]
    pub fn bs11(&mut self) -> BS11_W {
        BS11_W { w: self }
    }
    #[doc = "Bit 12 - BS12"]
    #[inline(always)]
    pub fn bs12(&mut self) -> BS12_W {
        BS12_W { w: self }
    }
    #[doc = "Bit 13 - BS13"]
    #[inline(always)]
    pub fn bs13(&mut self) -> BS13_W {
        BS13_W { w: self }
    }
    #[doc = "Bit 14 - BS14"]
    #[inline(always)]
    pub fn bs14(&mut self) -> BS14_W {
        BS14_W { w: self }
    }
    #[doc = "Bit 15 - BS15"]
    #[inline(always)]
    pub fn bs15(&mut self) -> BS15_W {
        BS15_W { w: self }
    }
    #[doc = "Bit 16 - BR0"]
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W {
        BR0_W { w: self }
    }
    #[doc = "Bit 17 - BR1"]
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W {
        BR1_W { w: self }
    }
    #[doc = "Bit 18 - BR2"]
    #[inline(always)]
    pub fn br2(&mut self) -> BR2_W {
        BR2_W { w: self }
    }
    #[doc = "Bit 19 - BR3"]
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W {
        BR3_W { w: self }
    }
    #[doc = "Bit 20 - BR4"]
    #[inline(always)]
    pub fn br4(&mut self) -> BR4_W {
        BR4_W { w: self }
    }
    #[doc = "Bit 21 - BR5"]
    #[inline(always)]
    pub fn br5(&mut self) -> BR5_W {
        BR5_W { w: self }
    }
    #[doc = "Bit 22 - BR6"]
    #[inline(always)]
    pub fn br6(&mut self) -> BR6_W {
        BR6_W { w: self }
    }
    #[doc = "Bit 23 - BR7"]
    #[inline(always)]
    pub fn br7(&mut self) -> BR7_W {
        BR7_W { w: self }
    }
    #[doc = "Bit 24 - BR8"]
    #[inline(always)]
    pub fn br8(&mut self) -> BR8_W {
        BR8_W { w: self }
    }
    #[doc = "Bit 25 - BR9"]
    #[inline(always)]
    pub fn br9(&mut self) -> BR9_W {
        BR9_W { w: self }
    }
    #[doc = "Bit 26 - BR10"]
    #[inline(always)]
    pub fn br10(&mut self) -> BR10_W {
        BR10_W { w: self }
    }
    #[doc = "Bit 27 - BR11"]
    #[inline(always)]
    pub fn br11(&mut self) -> BR11_W {
        BR11_W { w: self }
    }
    #[doc = "Bit 28 - BR12"]
    #[inline(always)]
    pub fn br12(&mut self) -> BR12_W {
        BR12_W { w: self }
    }
    #[doc = "Bit 29 - BR13"]
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W {
        BR13_W { w: self }
    }
    #[doc = "Bit 30 - BR14"]
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W {
        BR14_W { w: self }
    }
    #[doc = "Bit 31 - BR15"]
    #[inline(always)]
    pub fn br15(&mut self) -> BR15_W {
        BR15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_bsrr](index.html) module"]
pub struct GPIOZ_BSRR_SPEC;
impl crate::RegisterSpec for GPIOZ_BSRR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpioz_bsrr::W](W) writer structure"]
impl crate::Writable for GPIOZ_BSRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOZ_BSRR to value 0"]
impl crate::Resettable for GPIOZ_BSRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
