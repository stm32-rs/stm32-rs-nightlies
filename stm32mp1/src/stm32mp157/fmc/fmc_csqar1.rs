#[doc = "Reader of register FMC_CSQAR1"]
pub type R = crate::R<u32, super::FMC_CSQAR1>;
#[doc = "Writer for register FMC_CSQAR1"]
pub type W = crate::W<u32, super::FMC_CSQAR1>;
#[doc = "Register FMC_CSQAR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FMC_CSQAR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDC1`"]
pub type ADDC1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDC1`"]
pub struct ADDC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `ADDC2`"]
pub type ADDC2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDC2`"]
pub struct ADDC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADDC3`"]
pub type ADDC3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDC3`"]
pub struct ADDC3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDC3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADDC4`"]
pub type ADDC4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDC4`"]
pub struct ADDC4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDC4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADDC1"]
    #[inline(always)]
    pub fn addc1(&self) -> ADDC1_R {
        ADDC1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ADDC2"]
    #[inline(always)]
    pub fn addc2(&self) -> ADDC2_R {
        ADDC2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ADDC3"]
    #[inline(always)]
    pub fn addc3(&self) -> ADDC3_R {
        ADDC3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ADDC4"]
    #[inline(always)]
    pub fn addc4(&self) -> ADDC4_R {
        ADDC4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADDC1"]
    #[inline(always)]
    pub fn addc1(&mut self) -> ADDC1_W {
        ADDC1_W { w: self }
    }
    #[doc = "Bits 8:15 - ADDC2"]
    #[inline(always)]
    pub fn addc2(&mut self) -> ADDC2_W {
        ADDC2_W { w: self }
    }
    #[doc = "Bits 16:23 - ADDC3"]
    #[inline(always)]
    pub fn addc3(&mut self) -> ADDC3_W {
        ADDC3_W { w: self }
    }
    #[doc = "Bits 24:31 - ADDC4"]
    #[inline(always)]
    pub fn addc4(&mut self) -> ADDC4_W {
        ADDC4_W { w: self }
    }
}
