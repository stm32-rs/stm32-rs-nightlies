#[doc = "Register `C2APB3SMENR` reader"]
pub struct R(crate::R<C2APB3SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2APB3SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2APB3SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2APB3SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2APB3SMENR` writer"]
pub struct W(crate::W<C2APB3SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2APB3SMENR_SPEC>;
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
impl From<crate::W<C2APB3SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2APB3SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBGHZSPISMEN` reader - sub-GHz radio SPI clock enable during CPU2 CSleep and CStop modes"]
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
#[doc = "Field `SUBGHZSPISMEN` writer - sub-GHz radio SPI clock enable during CPU2 CSleep and CStop modes"]
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
    #[doc = "Bit 0 - sub-GHz radio SPI clock enable during CPU2 CSleep and CStop modes"]
    #[inline(always)]
    pub fn subghzspismen(&self) -> SUBGHZSPISMEN_R {
        SUBGHZSPISMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - sub-GHz radio SPI clock enable during CPU2 CSleep and CStop modes"]
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
#[doc = "CPU2 APB3 peripheral clock enable in Sleep mode register \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb3smenr](index.html) module"]
pub struct C2APB3SMENR_SPEC;
impl crate::RegisterSpec for C2APB3SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2apb3smenr::R](R) reader structure"]
impl crate::Readable for C2APB3SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2apb3smenr::W](W) writer structure"]
impl crate::Writable for C2APB3SMENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2APB3SMENR to value 0x01"]
impl crate::Resettable for C2APB3SMENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
