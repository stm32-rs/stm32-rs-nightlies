#[doc = "Register `UR14` reader"]
pub struct R(crate::R<UR14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UR14` writer"]
pub struct W(crate::W<UR14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UR14_SPEC>;
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
impl From<crate::W<UR14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UR14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D1STPRST` reader - D1 Stop Reset"]
pub struct D1STPRST_R(crate::FieldReader<bool, bool>);
impl D1STPRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        D1STPRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D1STPRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D1STPRST` writer - D1 Stop Reset"]
pub struct D1STPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> D1STPRST_W<'a> {
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
    #[doc = "Bit 0 - D1 Stop Reset"]
    #[inline(always)]
    pub fn d1stprst(&self) -> D1STPRST_R {
        D1STPRST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D1 Stop Reset"]
    #[inline(always)]
    pub fn d1stprst(&mut self) -> D1STPRST_W {
        D1STPRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG user register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur14](index.html) module"]
pub struct UR14_SPEC;
impl crate::RegisterSpec for UR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur14::R](R) reader structure"]
impl crate::Readable for UR14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ur14::W](W) writer structure"]
impl crate::Writable for UR14_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UR14 to value 0"]
impl crate::Resettable for UR14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
