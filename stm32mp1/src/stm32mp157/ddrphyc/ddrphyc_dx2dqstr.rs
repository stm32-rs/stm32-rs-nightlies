#[doc = "Reader of register DDRPHYC_DX2DQSTR"]
pub type R = crate::R<u32, super::DDRPHYC_DX2DQSTR>;
#[doc = "Writer for register DDRPHYC_DX2DQSTR"]
pub type W = crate::W<u32, super::DDRPHYC_DX2DQSTR>;
#[doc = "Register DDRPHYC_DX2DQSTR `reset()`'s with value 0x3db0_2000"]
impl crate::ResetValue for super::DDRPHYC_DX2DQSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3db0_2000
    }
}
#[doc = "Reader of field `R0DGSL`"]
pub type R0DGSL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `R0DGSL`"]
pub struct R0DGSL_W<'a> {
    w: &'a mut W,
}
impl<'a> R0DGSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `R0DGPS`"]
pub type R0DGPS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `R0DGPS`"]
pub struct R0DGPS_W<'a> {
    w: &'a mut W,
}
impl<'a> R0DGPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `DQSDLY`"]
pub type DQSDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DQSDLY`"]
pub struct DQSDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `DQSNDLY`"]
pub type DQSNDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DQSNDLY`"]
pub struct DQSNDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSNDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Reader of field `DMDLY`"]
pub type DMDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMDLY`"]
pub struct DMDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> DMDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 26)) | (((value as u32) & 0x0f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - R0DGSL"]
    #[inline(always)]
    pub fn r0dgsl(&self) -> R0DGSL_R {
        R0DGSL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 12:13 - R0DGPS"]
    #[inline(always)]
    pub fn r0dgps(&self) -> R0DGPS_R {
        R0DGPS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 20:22 - DQSDLY"]
    #[inline(always)]
    pub fn dqsdly(&self) -> DQSDLY_R {
        DQSDLY_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - DQSNDLY"]
    #[inline(always)]
    pub fn dqsndly(&self) -> DQSNDLY_R {
        DQSNDLY_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 26:29 - DMDLY"]
    #[inline(always)]
    pub fn dmdly(&self) -> DMDLY_R {
        DMDLY_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - R0DGSL"]
    #[inline(always)]
    pub fn r0dgsl(&mut self) -> R0DGSL_W {
        R0DGSL_W { w: self }
    }
    #[doc = "Bits 12:13 - R0DGPS"]
    #[inline(always)]
    pub fn r0dgps(&mut self) -> R0DGPS_W {
        R0DGPS_W { w: self }
    }
    #[doc = "Bits 20:22 - DQSDLY"]
    #[inline(always)]
    pub fn dqsdly(&mut self) -> DQSDLY_W {
        DQSDLY_W { w: self }
    }
    #[doc = "Bits 23:25 - DQSNDLY"]
    #[inline(always)]
    pub fn dqsndly(&mut self) -> DQSNDLY_W {
        DQSNDLY_W { w: self }
    }
    #[doc = "Bits 26:29 - DMDLY"]
    #[inline(always)]
    pub fn dmdly(&mut self) -> DMDLY_W {
        DMDLY_W { w: self }
    }
}
