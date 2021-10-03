#[doc = "Register `APB3SMENR` reader"]
pub struct R(crate::R<APB3SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB3SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB3SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB3SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB3SMENR` writer"]
pub struct W(crate::W<APB3SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB3SMENR_SPEC>;
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
impl From<crate::W<APB3SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB3SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBGHZSPISMEN` reader - Sub-GHz radio SPI clock enable during Sleep and Stop modes"]
pub struct SUBGHZSPISMEN_R(crate::FieldReader<bool, bool>);
impl SUBGHZSPISMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUBGHZSPISMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUBGHZSPISMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUBGHZSPISMEN` writer - Sub-GHz radio SPI clock enable during Sleep and Stop modes"]
pub struct SUBGHZSPISMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBGHZSPISMEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Sub-GHz radio SPI clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn subghzspismen(&self) -> SUBGHZSPISMEN_R {
        SUBGHZSPISMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sub-GHz radio SPI clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn subghzspismen(&mut self) -> SUBGHZSPISMEN_W {
        SUBGHZSPISMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB3 peripheral clock enable in Sleep mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3smenr](index.html) module"]
pub struct APB3SMENR_SPEC;
impl crate::RegisterSpec for APB3SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb3smenr::R](R) reader structure"]
impl crate::Readable for APB3SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb3smenr::W](W) writer structure"]
impl crate::Writable for APB3SMENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB3SMENR to value 0x01"]
impl crate::Resettable for APB3SMENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
