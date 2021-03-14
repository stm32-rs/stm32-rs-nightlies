#[doc = "Reader of register DDRPHYC_DTDR0"]
pub type R = crate::R<u32, super::DDRPHYC_DTDR0>;
#[doc = "Writer for register DDRPHYC_DTDR0"]
pub type W = crate::W<u32, super::DDRPHYC_DTDR0>;
#[doc = "Register DDRPHYC_DTDR0 `reset()`'s with value 0xdd22_ee11"]
impl crate::ResetValue for super::DDRPHYC_DTDR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xdd22_ee11
    }
}
#[doc = "Reader of field `DTBYTE0`"]
pub type DTBYTE0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTBYTE0`"]
pub struct DTBYTE0_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBYTE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DTBYTE1`"]
pub type DTBYTE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTBYTE1`"]
pub struct DTBYTE1_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBYTE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DTBYTE2`"]
pub type DTBYTE2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTBYTE2`"]
pub struct DTBYTE2_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBYTE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DTBYTE3`"]
pub type DTBYTE3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTBYTE3`"]
pub struct DTBYTE3_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBYTE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DTBYTE0"]
    #[inline(always)]
    pub fn dtbyte0(&self) -> DTBYTE0_R {
        DTBYTE0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DTBYTE1"]
    #[inline(always)]
    pub fn dtbyte1(&self) -> DTBYTE1_R {
        DTBYTE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DTBYTE2"]
    #[inline(always)]
    pub fn dtbyte2(&self) -> DTBYTE2_R {
        DTBYTE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DTBYTE3"]
    #[inline(always)]
    pub fn dtbyte3(&self) -> DTBYTE3_R {
        DTBYTE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DTBYTE0"]
    #[inline(always)]
    pub fn dtbyte0(&mut self) -> DTBYTE0_W {
        DTBYTE0_W { w: self }
    }
    #[doc = "Bits 8:15 - DTBYTE1"]
    #[inline(always)]
    pub fn dtbyte1(&mut self) -> DTBYTE1_W {
        DTBYTE1_W { w: self }
    }
    #[doc = "Bits 16:23 - DTBYTE2"]
    #[inline(always)]
    pub fn dtbyte2(&mut self) -> DTBYTE2_W {
        DTBYTE2_W { w: self }
    }
    #[doc = "Bits 24:31 - DTBYTE3"]
    #[inline(always)]
    pub fn dtbyte3(&mut self) -> DTBYTE3_W {
        DTBYTE3_W { w: self }
    }
}
