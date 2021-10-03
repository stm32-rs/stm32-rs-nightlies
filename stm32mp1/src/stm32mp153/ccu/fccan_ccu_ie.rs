#[doc = "Register `FCCAN_CCU_IE` reader"]
pub struct R(crate::R<FCCAN_CCU_IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCCAN_CCU_IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCCAN_CCU_IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCCAN_CCU_IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCCAN_CCU_IE` writer"]
pub struct W(crate::W<FCCAN_CCU_IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCCAN_CCU_IE_SPEC>;
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
impl From<crate::W<FCCAN_CCU_IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCCAN_CCU_IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CWEE` reader - CWEE"]
pub struct CWEE_R(crate::FieldReader<bool, bool>);
impl CWEE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CWEE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWEE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWEE` writer - CWEE"]
pub struct CWEE_W<'a> {
    w: &'a mut W,
}
impl<'a> CWEE_W<'a> {
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
#[doc = "Field `CSCE` reader - CSCE"]
pub struct CSCE_R(crate::FieldReader<bool, bool>);
impl CSCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSCE` writer - CSCE"]
pub struct CSCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCE_W<'a> {
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
    #[doc = "Bit 0 - CWEE"]
    #[inline(always)]
    pub fn cwee(&self) -> CWEE_R {
        CWEE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CSCE"]
    #[inline(always)]
    pub fn csce(&self) -> CSCE_R {
        CSCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CWEE"]
    #[inline(always)]
    pub fn cwee(&mut self) -> CWEE_W {
        CWEE_W { w: self }
    }
    #[doc = "Bit 1 - CSCE"]
    #[inline(always)]
    pub fn csce(&mut self) -> CSCE_W {
        CSCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The settings in the CU interrupt enable register determine whether a status change in the CU interrupt register will be signaled on an interrupt line.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccan_ccu_ie](index.html) module"]
pub struct FCCAN_CCU_IE_SPEC;
impl crate::RegisterSpec for FCCAN_CCU_IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fccan_ccu_ie::R](R) reader structure"]
impl crate::Readable for FCCAN_CCU_IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fccan_ccu_ie::W](W) writer structure"]
impl crate::Writable for FCCAN_CCU_IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCCAN_CCU_IE to value 0"]
impl crate::Resettable for FCCAN_CCU_IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
