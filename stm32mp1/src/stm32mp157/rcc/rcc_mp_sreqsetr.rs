#[doc = "Register `RCC_MP_SREQSETR` reader"]
pub struct R(crate::R<RCC_MP_SREQSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_SREQSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_SREQSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_SREQSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_SREQSETR` writer"]
pub struct W(crate::W<RCC_MP_SREQSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_SREQSETR_SPEC>;
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
impl From<crate::W<RCC_MP_SREQSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_SREQSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STPREQ_P0` reader - STPREQ_P0"]
pub struct STPREQ_P0_R(crate::FieldReader<bool, bool>);
impl STPREQ_P0_R {
    pub(crate) fn new(bits: bool) -> Self {
        STPREQ_P0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STPREQ_P0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STPREQ_P0` writer - STPREQ_P0"]
pub struct STPREQ_P0_W<'a> {
    w: &'a mut W,
}
impl<'a> STPREQ_P0_W<'a> {
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
#[doc = "Field `STPREQ_P1` reader - STPREQ_P1"]
pub struct STPREQ_P1_R(crate::FieldReader<bool, bool>);
impl STPREQ_P1_R {
    pub(crate) fn new(bits: bool) -> Self {
        STPREQ_P1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STPREQ_P1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STPREQ_P1` writer - STPREQ_P1"]
pub struct STPREQ_P1_W<'a> {
    w: &'a mut W,
}
impl<'a> STPREQ_P1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - STPREQ_P0"]
    #[inline(always)]
    pub fn stpreq_p0(&self) -> STPREQ_P0_R {
        STPREQ_P0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - STPREQ_P1"]
    #[inline(always)]
    pub fn stpreq_p1(&self) -> STPREQ_P1_R {
        STPREQ_P1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STPREQ_P0"]
    #[inline(always)]
    pub fn stpreq_p0(&mut self) -> STPREQ_P0_W {
        STPREQ_P0_W { w: self }
    }
    #[doc = "Bit 1 - STPREQ_P1"]
    #[inline(always)]
    pub fn stpreq_p1(&mut self) -> STPREQ_P1_W {
        STPREQ_P1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Writing has no effect, reading will return the values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_sreqsetr](index.html) module"]
pub struct RCC_MP_SREQSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_SREQSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_sreqsetr::R](R) reader structure"]
impl crate::Readable for RCC_MP_SREQSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_sreqsetr::W](W) writer structure"]
impl crate::Writable for RCC_MP_SREQSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_SREQSETR to value 0"]
impl crate::Resettable for RCC_MP_SREQSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
