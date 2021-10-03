#[doc = "Register `DDRCTRL_ADDRMAP10` reader"]
pub struct R(crate::R<DDRCTRL_ADDRMAP10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_ADDRMAP10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_ADDRMAP10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_ADDRMAP10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_ADDRMAP10` writer"]
pub struct W(crate::W<DDRCTRL_ADDRMAP10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_ADDRMAP10_SPEC>;
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
impl From<crate::W<DDRCTRL_ADDRMAP10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_ADDRMAP10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRMAP_ROW_B6` reader - ADDRMAP_ROW_B6"]
pub struct ADDRMAP_ROW_B6_R(crate::FieldReader<u8, u8>);
impl ADDRMAP_ROW_B6_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDRMAP_ROW_B6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRMAP_ROW_B6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRMAP_ROW_B6` writer - ADDRMAP_ROW_B6"]
pub struct ADDRMAP_ROW_B6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `ADDRMAP_ROW_B7` reader - ADDRMAP_ROW_B7"]
pub struct ADDRMAP_ROW_B7_R(crate::FieldReader<u8, u8>);
impl ADDRMAP_ROW_B7_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDRMAP_ROW_B7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRMAP_ROW_B7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRMAP_ROW_B7` writer - ADDRMAP_ROW_B7"]
pub struct ADDRMAP_ROW_B7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `ADDRMAP_ROW_B8` reader - ADDRMAP_ROW_B8"]
pub struct ADDRMAP_ROW_B8_R(crate::FieldReader<u8, u8>);
impl ADDRMAP_ROW_B8_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDRMAP_ROW_B8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRMAP_ROW_B8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRMAP_ROW_B8` writer - ADDRMAP_ROW_B8"]
pub struct ADDRMAP_ROW_B8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `ADDRMAP_ROW_B9` reader - ADDRMAP_ROW_B9"]
pub struct ADDRMAP_ROW_B9_R(crate::FieldReader<u8, u8>);
impl ADDRMAP_ROW_B9_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDRMAP_ROW_B9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRMAP_ROW_B9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRMAP_ROW_B9` writer - ADDRMAP_ROW_B9"]
pub struct ADDRMAP_ROW_B9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B6"]
    #[inline(always)]
    pub fn addrmap_row_b6(&self) -> ADDRMAP_ROW_B6_R {
        ADDRMAP_ROW_B6_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B7"]
    #[inline(always)]
    pub fn addrmap_row_b7(&self) -> ADDRMAP_ROW_B7_R {
        ADDRMAP_ROW_B7_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B8"]
    #[inline(always)]
    pub fn addrmap_row_b8(&self) -> ADDRMAP_ROW_B8_R {
        ADDRMAP_ROW_B8_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B9"]
    #[inline(always)]
    pub fn addrmap_row_b9(&self) -> ADDRMAP_ROW_B9_R {
        ADDRMAP_ROW_B9_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B6"]
    #[inline(always)]
    pub fn addrmap_row_b6(&mut self) -> ADDRMAP_ROW_B6_W {
        ADDRMAP_ROW_B6_W { w: self }
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B7"]
    #[inline(always)]
    pub fn addrmap_row_b7(&mut self) -> ADDRMAP_ROW_B7_W {
        ADDRMAP_ROW_B7_W { w: self }
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B8"]
    #[inline(always)]
    pub fn addrmap_row_b8(&mut self) -> ADDRMAP_ROW_B8_W {
        ADDRMAP_ROW_B8_W { w: self }
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B9"]
    #[inline(always)]
    pub fn addrmap_row_b9(&mut self) -> ADDRMAP_ROW_B9_W {
        ADDRMAP_ROW_B9_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL address map register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_addrmap10](index.html) module"]
pub struct DDRCTRL_ADDRMAP10_SPEC;
impl crate::RegisterSpec for DDRCTRL_ADDRMAP10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_addrmap10::R](R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_addrmap10::W](W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_ADDRMAP10 to value 0"]
impl crate::Resettable for DDRCTRL_ADDRMAP10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
