#[doc = "Register `TIM1_AF2` reader"]
pub struct R(crate::R<TIM1_AF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM1_AF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM1_AF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM1_AF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM1_AF2` writer"]
pub struct W(crate::W<TIM1_AF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM1_AF2_SPEC>;
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
impl From<crate::W<TIM1_AF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM1_AF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BK2INE` reader - BK2INE"]
pub struct BK2INE_R(crate::FieldReader<bool, bool>);
impl BK2INE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2INE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK2INE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2INE` writer - BK2INE"]
pub struct BK2INE_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2INE_W<'a> {
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
#[doc = "Field `BK2DF1BK1E` reader - BK2DF1BK1E"]
pub struct BK2DF1BK1E_R(crate::FieldReader<bool, bool>);
impl BK2DF1BK1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2DF1BK1E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK2DF1BK1E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2DF1BK1E` writer - BK2DF1BK1E"]
pub struct BK2DF1BK1E_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2DF1BK1E_W<'a> {
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
#[doc = "Field `BK2INP` reader - BK2INP"]
pub struct BK2INP_R(crate::FieldReader<bool, bool>);
impl BK2INP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2INP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BK2INP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2INP` writer - BK2INP"]
pub struct BK2INP_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2INP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - BK2INE"]
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - BK2DF1BK1E"]
    #[inline(always)]
    pub fn bk2df1bk1e(&self) -> BK2DF1BK1E_R {
        BK2DF1BK1E_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BK2INP"]
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BK2INE"]
    #[inline(always)]
    pub fn bk2ine(&mut self) -> BK2INE_W {
        BK2INE_W { w: self }
    }
    #[doc = "Bit 8 - BK2DF1BK1E"]
    #[inline(always)]
    pub fn bk2df1bk1e(&mut self) -> BK2DF1BK1E_W {
        BK2DF1BK1E_W { w: self }
    }
    #[doc = "Bit 9 - BK2INP"]
    #[inline(always)]
    pub fn bk2inp(&mut self) -> BK2INP_W {
        BK2INP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM1 Alternate function register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_af2](index.html) module"]
pub struct TIM1_AF2_SPEC;
impl crate::RegisterSpec for TIM1_AF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim1_af2::R](R) reader structure"]
impl crate::Readable for TIM1_AF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim1_af2::W](W) writer structure"]
impl crate::Writable for TIM1_AF2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM1_AF2 to value 0x01"]
impl crate::Resettable for TIM1_AF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
