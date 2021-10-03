#[doc = "Register `FCCAN_CCU_IR` reader"]
pub struct R(crate::R<FCCAN_CCU_IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCCAN_CCU_IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCCAN_CCU_IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCCAN_CCU_IR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCCAN_CCU_IR` writer"]
pub struct W(crate::W<FCCAN_CCU_IR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCCAN_CCU_IR_SPEC>;
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
impl From<crate::W<FCCAN_CCU_IR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCCAN_CCU_IR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CWE` reader - CWE"]
pub struct CWE_R(crate::FieldReader<bool, bool>);
impl CWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWE` writer - CWE"]
pub struct CWE_W<'a> {
    w: &'a mut W,
}
impl<'a> CWE_W<'a> {
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
#[doc = "Field `CSC` reader - CSC"]
pub struct CSC_R(crate::FieldReader<bool, bool>);
impl CSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSC` writer - CSC"]
pub struct CSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSC_W<'a> {
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
    #[doc = "Bit 0 - CWE"]
    #[inline(always)]
    pub fn cwe(&self) -> CWE_R {
        CWE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CSC"]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CWE"]
    #[inline(always)]
    pub fn cwe(&mut self) -> CWE_W {
        CWE_W { w: self }
    }
    #[doc = "Bit 1 - CSC"]
    #[inline(always)]
    pub fn csc(&mut self) -> CSC_W {
        CSC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of CCU_IE controls whether an interrupt is generated or not.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccan_ccu_ir](index.html) module"]
pub struct FCCAN_CCU_IR_SPEC;
impl crate::RegisterSpec for FCCAN_CCU_IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fccan_ccu_ir::R](R) reader structure"]
impl crate::Readable for FCCAN_CCU_IR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fccan_ccu_ir::W](W) writer structure"]
impl crate::Writable for FCCAN_CCU_IR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCCAN_CCU_IR to value 0"]
impl crate::Resettable for FCCAN_CCU_IR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
