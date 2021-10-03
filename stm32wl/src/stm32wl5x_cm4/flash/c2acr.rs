#[doc = "Register `C2ACR` reader"]
pub struct R(crate::R<C2ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2ACR` writer"]
pub struct W(crate::W<C2ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2ACR_SPEC>;
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
impl From<crate::W<C2ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CPU2 Prefetch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTEN_A {
    #[doc = "0: CPU2 prefetch is disabled"]
    DISABLED = 0,
    #[doc = "1: CPU2 prefetch is enabled"]
    ENABLED = 1,
}
impl From<PRFTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRFTEN` reader - CPU2 Prefetch enable"]
pub struct PRFTEN_R(crate::FieldReader<bool, PRFTEN_A>);
impl PRFTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRFTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRFTEN_A {
        match self.bits {
            false => PRFTEN_A::DISABLED,
            true => PRFTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PRFTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PRFTEN_A::ENABLED
    }
}
impl core::ops::Deref for PRFTEN_R {
    type Target = crate::FieldReader<bool, PRFTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRFTEN` writer - CPU2 Prefetch enable"]
pub struct PRFTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRFTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRFTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CPU2 prefetch is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::DISABLED)
    }
    #[doc = "CPU2 prefetch is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "CPU2 Instruction cache enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICEN_A {
    #[doc = "0: CPU2 instruction cache is disabled"]
    DISABLED = 0,
    #[doc = "1: CPU2 instruction cache is enabled"]
    ENABLED = 1,
}
impl From<ICEN_A> for bool {
    #[inline(always)]
    fn from(variant: ICEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICEN` reader - CPU2 Instruction cache enable"]
pub struct ICEN_R(crate::FieldReader<bool, ICEN_A>);
impl ICEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICEN_A {
        match self.bits {
            false => ICEN_A::DISABLED,
            true => ICEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ICEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ICEN_A::ENABLED
    }
}
impl core::ops::Deref for ICEN_R {
    type Target = crate::FieldReader<bool, ICEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICEN` writer - CPU2 Instruction cache enable"]
pub struct ICEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CPU2 instruction cache is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ICEN_A::DISABLED)
    }
    #[doc = "CPU2 instruction cache is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ICEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "CPU2 Instruction cache reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICRST_A {
    #[doc = "0: CPU2 instruction cache is not reset"]
    NOTRESET = 0,
    #[doc = "1: CPU2 instruction cache is reset"]
    RESET = 1,
}
impl From<ICRST_A> for bool {
    #[inline(always)]
    fn from(variant: ICRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICRST` reader - CPU2 Instruction cache reset"]
pub struct ICRST_R(crate::FieldReader<bool, ICRST_A>);
impl ICRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICRST_A {
        match self.bits {
            false => ICRST_A::NOTRESET,
            true => ICRST_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRESET`"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        **self == ICRST_A::NOTRESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == ICRST_A::RESET
    }
}
impl core::ops::Deref for ICRST_R {
    type Target = crate::FieldReader<bool, ICRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICRST` writer - CPU2 Instruction cache reset"]
pub struct ICRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ICRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CPU2 instruction cache is not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(ICRST_A::NOTRESET)
    }
    #[doc = "CPU2 instruction cache is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ICRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "CPU2 program / erase suspend request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PES_A {
    #[doc = "0: Flash program and erase operations granted"]
    GRANTED = 0,
    #[doc = "1: Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_C2SR is set when PES bit in FLASH_C2ACR is set"]
    SUSPENDED = 1,
}
impl From<PES_A> for bool {
    #[inline(always)]
    fn from(variant: PES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PES` reader - CPU2 program / erase suspend request"]
pub struct PES_R(crate::FieldReader<bool, PES_A>);
impl PES_R {
    pub(crate) fn new(bits: bool) -> Self {
        PES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PES_A {
        match self.bits {
            false => PES_A::GRANTED,
            true => PES_A::SUSPENDED,
        }
    }
    #[doc = "Checks if the value of the field is `GRANTED`"]
    #[inline(always)]
    pub fn is_granted(&self) -> bool {
        **self == PES_A::GRANTED
    }
    #[doc = "Checks if the value of the field is `SUSPENDED`"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        **self == PES_A::SUSPENDED
    }
}
impl core::ops::Deref for PES_R {
    type Target = crate::FieldReader<bool, PES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PES` writer - CPU2 program / erase suspend request"]
pub struct PES_W<'a> {
    w: &'a mut W,
}
impl<'a> PES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PES_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flash program and erase operations granted"]
    #[inline(always)]
    pub fn granted(self) -> &'a mut W {
        self.variant(PES_A::GRANTED)
    }
    #[doc = "Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_C2SR is set when PES bit in FLASH_C2ACR is set"]
    #[inline(always)]
    pub fn suspended(self) -> &'a mut W {
        self.variant(PES_A::SUSPENDED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - CPU2 Prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CPU2 Instruction cache enable"]
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CPU2 Instruction cache reset"]
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CPU2 program / erase suspend request"]
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - CPU2 Prefetch enable"]
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W {
        PRFTEN_W { w: self }
    }
    #[doc = "Bit 9 - CPU2 Instruction cache enable"]
    #[inline(always)]
    pub fn icen(&mut self) -> ICEN_W {
        ICEN_W { w: self }
    }
    #[doc = "Bit 11 - CPU2 Instruction cache reset"]
    #[inline(always)]
    pub fn icrst(&mut self) -> ICRST_W {
        ICRST_W { w: self }
    }
    #[doc = "Bit 15 - CPU2 program / erase suspend request"]
    #[inline(always)]
    pub fn pes(&mut self) -> PES_W {
        PES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash CPU2 access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2acr](index.html) module"]
pub struct C2ACR_SPEC;
impl crate::RegisterSpec for C2ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2acr::R](R) reader structure"]
impl crate::Readable for C2ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2acr::W](W) writer structure"]
impl crate::Writable for C2ACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2ACR to value 0x0600"]
impl crate::Resettable for C2ACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0600
    }
}
