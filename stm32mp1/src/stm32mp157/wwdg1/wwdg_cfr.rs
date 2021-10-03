#[doc = "Register `WWDG_CFR` reader"]
pub struct R(crate::R<WWDG_CFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WWDG_CFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WWDG_CFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WWDG_CFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WWDG_CFR` writer"]
pub struct W(crate::W<WWDG_CFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WWDG_CFR_SPEC>;
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
impl From<crate::W<WWDG_CFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WWDG_CFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `W` reader - W"]
pub struct W_R(crate::FieldReader<u8, u8>);
impl W_R {
    pub(crate) fn new(bits: u8) -> Self {
        W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for W_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `W` writer - W"]
pub struct W_W<'a> {
    w: &'a mut W,
}
impl<'a> W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `EWI` reader - EWI"]
pub struct EWI_R(crate::FieldReader<bool, bool>);
impl EWI_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EWI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWI` writer - EWI"]
pub struct EWI_W<'a> {
    w: &'a mut W,
}
impl<'a> EWI_W<'a> {
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
#[doc = "Field `WDGTB` reader - WDGTB"]
pub struct WDGTB_R(crate::FieldReader<u8, u8>);
impl WDGTB_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDGTB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDGTB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDGTB` writer - WDGTB"]
pub struct WDGTB_W<'a> {
    w: &'a mut W,
}
impl<'a> WDGTB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - W"]
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 9 - EWI"]
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - WDGTB"]
    #[inline(always)]
    pub fn wdgtb(&self) -> WDGTB_R {
        WDGTB_R::new(((self.bits >> 11) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - W"]
    #[inline(always)]
    pub fn w(&mut self) -> W_W {
        W_W { w: self }
    }
    #[doc = "Bit 9 - EWI"]
    #[inline(always)]
    pub fn ewi(&mut self) -> EWI_W {
        EWI_W { w: self }
    }
    #[doc = "Bits 11:13 - WDGTB"]
    #[inline(always)]
    pub fn wdgtb(&mut self) -> WDGTB_W {
        WDGTB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdg_cfr](index.html) module"]
pub struct WWDG_CFR_SPEC;
impl crate::RegisterSpec for WWDG_CFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wwdg_cfr::R](R) reader structure"]
impl crate::Readable for WWDG_CFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wwdg_cfr::W](W) writer structure"]
impl crate::Writable for WWDG_CFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WWDG_CFR to value 0x7f"]
impl crate::Resettable for WWDG_CFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f
    }
}
