#[doc = "Register `RCC_MP_IWDGFZCLRR` reader"]
pub struct R(crate::R<RCC_MP_IWDGFZCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_IWDGFZCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_IWDGFZCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_IWDGFZCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_IWDGFZCLRR` writer"]
pub struct W(crate::W<RCC_MP_IWDGFZCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_IWDGFZCLRR_SPEC>;
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
impl From<crate::W<RCC_MP_IWDGFZCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_IWDGFZCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FZ_IWDG1` reader - FZ_IWDG1"]
pub struct FZ_IWDG1_R(crate::FieldReader<bool, bool>);
impl FZ_IWDG1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FZ_IWDG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FZ_IWDG1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FZ_IWDG1` writer - FZ_IWDG1"]
pub struct FZ_IWDG1_W<'a> {
    w: &'a mut W,
}
impl<'a> FZ_IWDG1_W<'a> {
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
#[doc = "Field `FZ_IWDG2` reader - FZ_IWDG2"]
pub struct FZ_IWDG2_R(crate::FieldReader<bool, bool>);
impl FZ_IWDG2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FZ_IWDG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FZ_IWDG2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FZ_IWDG2` writer - FZ_IWDG2"]
pub struct FZ_IWDG2_W<'a> {
    w: &'a mut W,
}
impl<'a> FZ_IWDG2_W<'a> {
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
    #[doc = "Bit 0 - FZ_IWDG1"]
    #[inline(always)]
    pub fn fz_iwdg1(&self) -> FZ_IWDG1_R {
        FZ_IWDG1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FZ_IWDG2"]
    #[inline(always)]
    pub fn fz_iwdg2(&self) -> FZ_IWDG2_R {
        FZ_IWDG2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FZ_IWDG1"]
    #[inline(always)]
    pub fn fz_iwdg1(&mut self) -> FZ_IWDG1_W {
        FZ_IWDG1_W { w: self }
    }
    #[doc = "Bit 1 - FZ_IWDG2"]
    #[inline(always)]
    pub fn fz_iwdg2(&mut self) -> FZ_IWDG2_W {
        FZ_IWDG2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the BOOTROM in order to unfreeze the IWDGs clocks. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_iwdgfzclrr](index.html) module"]
pub struct RCC_MP_IWDGFZCLRR_SPEC;
impl crate::RegisterSpec for RCC_MP_IWDGFZCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_iwdgfzclrr::R](R) reader structure"]
impl crate::Readable for RCC_MP_IWDGFZCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_iwdgfzclrr::W](W) writer structure"]
impl crate::Writable for RCC_MP_IWDGFZCLRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_IWDGFZCLRR to value 0"]
impl crate::Resettable for RCC_MP_IWDGFZCLRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
