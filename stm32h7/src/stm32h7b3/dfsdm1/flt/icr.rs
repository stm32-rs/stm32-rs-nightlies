#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear the injected conversion overrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRJOVRF_A {
    #[doc = "0: Writing '0â\u{80}\u{99} has no effect"]
    B_0X0 = 0,
    #[doc = "1: Writing '1â\u{80}\u{99} clears the JOVRF bit in the DFSDM_FLTxISR register"]
    B_0X1 = 1,
}
impl From<CLRJOVRF_A> for bool {
    #[inline(always)]
    fn from(variant: CLRJOVRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRJOVRF` reader - Clear the injected conversion overrun flag"]
pub struct CLRJOVRF_R(crate::FieldReader<bool, CLRJOVRF_A>);
impl CLRJOVRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRJOVRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRJOVRF_A {
        match self.bits {
            false => CLRJOVRF_A::B_0X0,
            true => CLRJOVRF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CLRJOVRF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CLRJOVRF_A::B_0X1
    }
}
impl core::ops::Deref for CLRJOVRF_R {
    type Target = crate::FieldReader<bool, CLRJOVRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRJOVRF` writer - Clear the injected conversion overrun flag"]
pub struct CLRJOVRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRJOVRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRJOVRF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Writing '0â\u{80}\u{99} has no effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CLRJOVRF_A::B_0X0)
    }
    #[doc = "Writing '1â\u{80}\u{99} clears the JOVRF bit in the DFSDM_FLTxISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CLRJOVRF_A::B_0X1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Clear the regular conversion overrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRROVRF_A {
    #[doc = "0: Writing '0â\u{80}\u{99} has no effect"]
    B_0X0 = 0,
    #[doc = "1: Writing '1â\u{80}\u{99} clears the ROVRF bit in the DFSDM_FLTxISR register"]
    B_0X1 = 1,
}
impl From<CLRROVRF_A> for bool {
    #[inline(always)]
    fn from(variant: CLRROVRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRROVRF` reader - Clear the regular conversion overrun flag"]
pub struct CLRROVRF_R(crate::FieldReader<bool, CLRROVRF_A>);
impl CLRROVRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRROVRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRROVRF_A {
        match self.bits {
            false => CLRROVRF_A::B_0X0,
            true => CLRROVRF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CLRROVRF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CLRROVRF_A::B_0X1
    }
}
impl core::ops::Deref for CLRROVRF_R {
    type Target = crate::FieldReader<bool, CLRROVRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRROVRF` writer - Clear the regular conversion overrun flag"]
pub struct CLRROVRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRROVRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRROVRF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Writing '0â\u{80}\u{99} has no effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CLRROVRF_A::B_0X0)
    }
    #[doc = "Writing '1â\u{80}\u{99} clears the ROVRF bit in the DFSDM_FLTxISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CLRROVRF_A::B_0X1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CLRCKABF` reader - Clear the clock absence flag CLRCKABF\\[y\\]=0: Writing '0â\u{80}\u{99} has no effect CLRCKABF\\[y\\]=1: Writing '1â\u{80}\u{99} to position y clears the corresponding CKABF\\[y\\]
bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\\[y\\]. Note: CLRCKABF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)"]
pub struct CLRCKABF_R(crate::FieldReader<u8, u8>);
impl CLRCKABF_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLRCKABF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRCKABF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRCKABF` writer - Clear the clock absence flag CLRCKABF\\[y\\]=0: Writing '0â\u{80}\u{99} has no effect CLRCKABF\\[y\\]=1: Writing '1â\u{80}\u{99} to position y clears the corresponding CKABF\\[y\\]
bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\\[y\\]. Note: CLRCKABF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)"]
pub struct CLRCKABF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRCKABF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `CLRSCDF` reader - Clear the short-circuit detector flag CLRSCDF\\[y\\]=0: Writing '0â\u{80}\u{99} has no effect CLRSCDF\\[y\\]=1: Writing '1â\u{80}\u{99} to position y clears the corresponding SCDF\\[y\\]
bit in the DFSDM_FLTxISR register Note: CLRSCDF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)"]
pub struct CLRSCDF_R(crate::FieldReader<u8, u8>);
impl CLRSCDF_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLRSCDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRSCDF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRSCDF` writer - Clear the short-circuit detector flag CLRSCDF\\[y\\]=0: Writing '0â\u{80}\u{99} has no effect CLRSCDF\\[y\\]=1: Writing '1â\u{80}\u{99} to position y clears the corresponding SCDF\\[y\\]
bit in the DFSDM_FLTxISR register Note: CLRSCDF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)"]
pub struct CLRSCDF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRSCDF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Clear the injected conversion overrun flag"]
    #[inline(always)]
    pub fn clrjovrf(&self) -> CLRJOVRF_R {
        CLRJOVRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    pub fn clrrovrf(&self) -> CLRROVRF_R {
        CLRROVRF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Clear the clock absence flag CLRCKABF\\[y\\]=0: Writing '0â\u{80}\u{99} has no effect CLRCKABF\\[y\\]=1: Writing '1â\u{80}\u{99} to position y clears the corresponding CKABF\\[y\\]
bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\\[y\\]. Note: CLRCKABF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)"]
    #[inline(always)]
    pub fn clrckabf(&self) -> CLRCKABF_R {
        CLRCKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clear the short-circuit detector flag CLRSCDF\\[y\\]=0: Writing '0â\u{80}\u{99} has no effect CLRSCDF\\[y\\]=1: Writing '1â\u{80}\u{99} to position y clears the corresponding SCDF\\[y\\]
bit in the DFSDM_FLTxISR register Note: CLRSCDF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)"]
    #[inline(always)]
    pub fn clrscdf(&self) -> CLRSCDF_R {
        CLRSCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Clear the injected conversion overrun flag"]
    #[inline(always)]
    pub fn clrjovrf(&mut self) -> CLRJOVRF_W {
        CLRJOVRF_W { w: self }
    }
    #[doc = "Bit 3 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    pub fn clrrovrf(&mut self) -> CLRROVRF_W {
        CLRROVRF_W { w: self }
    }
    #[doc = "Bits 16:23 - Clear the clock absence flag CLRCKABF\\[y\\]=0: Writing '0â\u{80}\u{99} has no effect CLRCKABF\\[y\\]=1: Writing '1â\u{80}\u{99} to position y clears the corresponding CKABF\\[y\\]
bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\\[y\\]. Note: CLRCKABF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)"]
    #[inline(always)]
    pub fn clrckabf(&mut self) -> CLRCKABF_W {
        CLRCKABF_W { w: self }
    }
    #[doc = "Bits 24:31 - Clear the short-circuit detector flag CLRSCDF\\[y\\]=0: Writing '0â\u{80}\u{99} has no effect CLRSCDF\\[y\\]=1: Writing '1â\u{80}\u{99} to position y clears the corresponding SCDF\\[y\\]
bit in the DFSDM_FLTxISR register Note: CLRSCDF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)"]
    #[inline(always)]
    pub fn clrscdf(&mut self) -> CLRSCDF_W {
        CLRSCDF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
