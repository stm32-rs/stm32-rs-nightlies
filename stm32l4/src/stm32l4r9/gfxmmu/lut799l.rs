#[doc = "Register `LUT799L` reader"]
pub struct R(crate::R<LUT799L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUT799L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUT799L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUT799L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUT799L` writer"]
pub struct W(crate::W<LUT799L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUT799L_SPEC>;
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
impl From<crate::W<LUT799L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUT799L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Field `FVB` reader - First Valid Block"]
pub struct FVB_R(crate::FieldReader<u8, u8>);
impl FVB_R {
    pub(crate) fn new(bits: u8) -> Self {
        FVB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FVB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FVB` writer - First Valid Block"]
pub struct FVB_W<'a> {
    w: &'a mut W,
}
impl<'a> FVB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `LVB` reader - Last Valid Block"]
pub struct LVB_R(crate::FieldReader<u8, u8>);
impl LVB_R {
    pub(crate) fn new(bits: u8) -> Self {
        LVB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVB` writer - Last Valid Block"]
pub struct LVB_W<'a> {
    w: &'a mut W,
}
impl<'a> LVB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - First Valid Block"]
    #[inline(always)]
    pub fn fvb(&self) -> FVB_R {
        FVB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Last Valid Block"]
    #[inline(always)]
    pub fn lvb(&self) -> LVB_R {
        LVB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bits 8:15 - First Valid Block"]
    #[inline(always)]
    pub fn fvb(&mut self) -> FVB_W {
        FVB_W { w: self }
    }
    #[doc = "Bits 16:23 - Last Valid Block"]
    #[inline(always)]
    pub fn lvb(&mut self) -> LVB_W {
        LVB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphic MMU LUT entry 799 low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut799l](index.html) module"]
pub struct LUT799L_SPEC;
impl crate::RegisterSpec for LUT799L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lut799l::R](R) reader structure"]
impl crate::Readable for LUT799L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lut799l::W](W) writer structure"]
impl crate::Writable for LUT799L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LUT799L to value 0"]
impl crate::Resettable for LUT799L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
