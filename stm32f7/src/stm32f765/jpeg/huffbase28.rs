#[doc = "Register `HUFFBASE28` reader"]
pub struct R(crate::R<HUFFBASE28_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HUFFBASE28_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HUFFBASE28_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HUFFBASE28_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HUFFBASE28` writer"]
pub struct W(crate::W<HUFFBASE28_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HUFFBASE28_SPEC>;
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
impl From<crate::W<HUFFBASE28_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HUFFBASE28_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HuffBase_RAM_0` reader - HuffBase RAM"]
pub struct HUFFBASE_RAM_0_R(crate::FieldReader<u16, u16>);
impl HUFFBASE_RAM_0_R {
    pub(crate) fn new(bits: u16) -> Self {
        HUFFBASE_RAM_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HUFFBASE_RAM_0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HuffBase_RAM_0` writer - HuffBase RAM"]
pub struct HUFFBASE_RAM_0_W<'a> {
    w: &'a mut W,
}
impl<'a> HUFFBASE_RAM_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `HuffBase_RAM_1` reader - HuffBase RAM"]
pub struct HUFFBASE_RAM_1_R(crate::FieldReader<u16, u16>);
impl HUFFBASE_RAM_1_R {
    pub(crate) fn new(bits: u16) -> Self {
        HUFFBASE_RAM_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HUFFBASE_RAM_1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HuffBase_RAM_1` writer - HuffBase RAM"]
pub struct HUFFBASE_RAM_1_W<'a> {
    w: &'a mut W,
}
impl<'a> HUFFBASE_RAM_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - HuffBase RAM"]
    #[inline(always)]
    pub fn huff_base_ram_0(&self) -> HUFFBASE_RAM_0_R {
        HUFFBASE_RAM_0_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - HuffBase RAM"]
    #[inline(always)]
    pub fn huff_base_ram_1(&self) -> HUFFBASE_RAM_1_R {
        HUFFBASE_RAM_1_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - HuffBase RAM"]
    #[inline(always)]
    pub fn huff_base_ram_0(&mut self) -> HUFFBASE_RAM_0_W {
        HUFFBASE_RAM_0_W { w: self }
    }
    #[doc = "Bits 16:24 - HuffBase RAM"]
    #[inline(always)]
    pub fn huff_base_ram_1(&mut self) -> HUFFBASE_RAM_1_W {
        HUFFBASE_RAM_1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase28](index.html) module"]
pub struct HUFFBASE28_SPEC;
impl crate::RegisterSpec for HUFFBASE28_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [huffbase28::R](R) reader structure"]
impl crate::Readable for HUFFBASE28_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [huffbase28::W](W) writer structure"]
impl crate::Writable for HUFFBASE28_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HUFFBASE28 to value 0"]
impl crate::Resettable for HUFFBASE28_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
