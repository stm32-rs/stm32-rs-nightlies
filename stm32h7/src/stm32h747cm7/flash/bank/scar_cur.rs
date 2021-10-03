#[doc = "Register `SCAR_CUR` reader"]
pub struct R(crate::R<SCAR_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCAR_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCAR_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCAR_CUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCAR_CUR` writer"]
pub struct W(crate::W<SCAR_CUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCAR_CUR_SPEC>;
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
impl From<crate::W<SCAR_CUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCAR_CUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMES` reader - Bank 1 secure access protected erase enable option status bit"]
pub struct DMES_R(crate::FieldReader<bool, bool>);
impl DMES_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMES` writer - Bank 1 secure access protected erase enable option status bit"]
pub struct DMES_W<'a> {
    w: &'a mut W,
}
impl<'a> DMES_W<'a> {
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
#[doc = "Field `SEC_AREA_END` reader - Bank 1 secure-only area end status bits"]
pub struct SEC_AREA_END_R(crate::FieldReader<u16, u16>);
impl SEC_AREA_END_R {
    pub(crate) fn new(bits: u16) -> Self {
        SEC_AREA_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_AREA_END_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC_AREA_END` writer - Bank 1 secure-only area end status bits"]
pub struct SEC_AREA_END_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_AREA_END_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `SEC_AREA_START` reader - Bank 1 secure-only area start status bits"]
pub struct SEC_AREA_START_R(crate::FieldReader<u16, u16>);
impl SEC_AREA_START_R {
    pub(crate) fn new(bits: u16) -> Self {
        SEC_AREA_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_AREA_START_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC_AREA_START` writer - Bank 1 secure-only area start status bits"]
pub struct SEC_AREA_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_AREA_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Bank 1 secure access protected erase enable option status bit"]
    #[inline(always)]
    pub fn dmes(&self) -> DMES_R {
        DMES_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 16:27 - Bank 1 secure-only area end status bits"]
    #[inline(always)]
    pub fn sec_area_end(&self) -> SEC_AREA_END_R {
        SEC_AREA_END_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11 - Bank 1 secure-only area start status bits"]
    #[inline(always)]
    pub fn sec_area_start(&self) -> SEC_AREA_START_R {
        SEC_AREA_START_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Bank 1 secure access protected erase enable option status bit"]
    #[inline(always)]
    pub fn dmes(&mut self) -> DMES_W {
        DMES_W { w: self }
    }
    #[doc = "Bits 16:27 - Bank 1 secure-only area end status bits"]
    #[inline(always)]
    pub fn sec_area_end(&mut self) -> SEC_AREA_END_W {
        SEC_AREA_END_W { w: self }
    }
    #[doc = "Bits 0:11 - Bank 1 secure-only area start status bits"]
    #[inline(always)]
    pub fn sec_area_start(&mut self) -> SEC_AREA_START_W {
        SEC_AREA_START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH secure address for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scar_cur](index.html) module"]
pub struct SCAR_CUR_SPEC;
impl crate::RegisterSpec for SCAR_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scar_cur::R](R) reader structure"]
impl crate::Readable for SCAR_CUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scar_cur::W](W) writer structure"]
impl crate::Writable for SCAR_CUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCAR_CUR to value 0"]
impl crate::Resettable for SCAR_CUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
