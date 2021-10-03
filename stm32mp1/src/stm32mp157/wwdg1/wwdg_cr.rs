#[doc = "Register `WWDG_CR` reader"]
pub struct R(crate::R<WWDG_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WWDG_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WWDG_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WWDG_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WWDG_CR` writer"]
pub struct W(crate::W<WWDG_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WWDG_CR_SPEC>;
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
impl From<crate::W<WWDG_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WWDG_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T` reader - T"]
pub struct T_R(crate::FieldReader<u8, u8>);
impl T_R {
    pub(crate) fn new(bits: u8) -> Self {
        T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T` writer - T"]
pub struct T_W<'a> {
    w: &'a mut W,
}
impl<'a> T_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `WDGA` reader - WDGA"]
pub struct WDGA_R(crate::FieldReader<bool, bool>);
impl WDGA_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDGA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDGA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDGA` writer - WDGA"]
pub struct WDGA_W<'a> {
    w: &'a mut W,
}
impl<'a> WDGA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - T"]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - WDGA"]
    #[inline(always)]
    pub fn wdga(&self) -> WDGA_R {
        WDGA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - T"]
    #[inline(always)]
    pub fn t(&mut self) -> T_W {
        T_W { w: self }
    }
    #[doc = "Bit 7 - WDGA"]
    #[inline(always)]
    pub fn wdga(&mut self) -> WDGA_W {
        WDGA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdg_cr](index.html) module"]
pub struct WWDG_CR_SPEC;
impl crate::RegisterSpec for WWDG_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wwdg_cr::R](R) reader structure"]
impl crate::Readable for WWDG_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wwdg_cr::W](W) writer structure"]
impl crate::Writable for WWDG_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WWDG_CR to value 0x7f"]
impl crate::Resettable for WWDG_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f
    }
}
