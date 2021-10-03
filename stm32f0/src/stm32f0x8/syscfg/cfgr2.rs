#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SRAM parity flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_PEF_A {
    #[doc = "0: No SRAM parity error detected"]
    NOPARITYERROR = 0,
    #[doc = "1: SRAM parity error detected"]
    PARITYERRORDETECTED = 1,
}
impl From<SRAM_PEF_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PEF` reader - SRAM parity flag"]
pub struct SRAM_PEF_R(crate::FieldReader<bool, SRAM_PEF_A>);
impl SRAM_PEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_PEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_PEF_A {
        match self.bits {
            false => SRAM_PEF_A::NOPARITYERROR,
            true => SRAM_PEF_A::PARITYERRORDETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOPARITYERROR`"]
    #[inline(always)]
    pub fn is_no_parity_error(&self) -> bool {
        **self == SRAM_PEF_A::NOPARITYERROR
    }
    #[doc = "Checks if the value of the field is `PARITYERRORDETECTED`"]
    #[inline(always)]
    pub fn is_parity_error_detected(&self) -> bool {
        **self == SRAM_PEF_A::PARITYERRORDETECTED
    }
}
impl core::ops::Deref for SRAM_PEF_R {
    type Target = crate::FieldReader<bool, SRAM_PEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SRAM parity flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_PEF_AW {
    #[doc = "1: Clear SRAM parity error flag"]
    CLEAR = 1,
}
impl From<SRAM_PEF_AW> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PEF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PEF` writer - SRAM parity flag"]
pub struct SRAM_PEF_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_PEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_PEF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear SRAM parity error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SRAM_PEF_AW::CLEAR)
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
#[doc = "PVD lock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PVD_LOCK_A {
    #[doc = "0: PVD interrupt disconnected from TIM1/15/16/17 Break input"]
    DISCONNECTED = 0,
    #[doc = "1: PVD interrupt connected to TIM1/15/16/17 Break input"]
    CONNECTED = 1,
}
impl From<PVD_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: PVD_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVD_LOCK` reader - PVD lock enable bit"]
pub struct PVD_LOCK_R(crate::FieldReader<bool, PVD_LOCK_A>);
impl PVD_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVD_LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVD_LOCK_A {
        match self.bits {
            false => PVD_LOCK_A::DISCONNECTED,
            true => PVD_LOCK_A::CONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        **self == PVD_LOCK_A::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `CONNECTED`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        **self == PVD_LOCK_A::CONNECTED
    }
}
impl core::ops::Deref for PVD_LOCK_R {
    type Target = crate::FieldReader<bool, PVD_LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVD_LOCK` writer - PVD lock enable bit"]
pub struct PVD_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PVD_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PVD_LOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PVD interrupt disconnected from TIM1/15/16/17 Break input"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PVD_LOCK_A::DISCONNECTED)
    }
    #[doc = "PVD interrupt connected to TIM1/15/16/17 Break input"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut W {
        self.variant(PVD_LOCK_A::CONNECTED)
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
#[doc = "SRAM parity lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_PARITY_LOCK_A {
    #[doc = "0: SRAM parity error disconnected from TIM1/15/16/17 Break input"]
    DISCONNECTED = 0,
    #[doc = "1: SRAM parity error connected to TIM1/15/16/17 Break input"]
    CONNECTED = 1,
}
impl From<SRAM_PARITY_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PARITY_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PARITY_LOCK` reader - SRAM parity lock bit"]
pub struct SRAM_PARITY_LOCK_R(crate::FieldReader<bool, SRAM_PARITY_LOCK_A>);
impl SRAM_PARITY_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_PARITY_LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_PARITY_LOCK_A {
        match self.bits {
            false => SRAM_PARITY_LOCK_A::DISCONNECTED,
            true => SRAM_PARITY_LOCK_A::CONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        **self == SRAM_PARITY_LOCK_A::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `CONNECTED`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        **self == SRAM_PARITY_LOCK_A::CONNECTED
    }
}
impl core::ops::Deref for SRAM_PARITY_LOCK_R {
    type Target = crate::FieldReader<bool, SRAM_PARITY_LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_PARITY_LOCK` writer - SRAM parity lock bit"]
pub struct SRAM_PARITY_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_PARITY_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_PARITY_LOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SRAM parity error disconnected from TIM1/15/16/17 Break input"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(SRAM_PARITY_LOCK_A::DISCONNECTED)
    }
    #[doc = "SRAM parity error connected to TIM1/15/16/17 Break input"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut W {
        self.variant(SRAM_PARITY_LOCK_A::CONNECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Cortex-M0 LOCKUP bit enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUP_LOCK_A {
    #[doc = "0: Cortex-M0 LOCKUP output disconnected from TIM1/15/16/17 Break input"]
    DISCONNECTED = 0,
    #[doc = "1: Cortex-M0 LOCKUP output connected to TIM1/15/16/17 Break input"]
    CONNECTED = 1,
}
impl From<LOCKUP_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKUP_LOCK` reader - Cortex-M0 LOCKUP bit enable bit"]
pub struct LOCKUP_LOCK_R(crate::FieldReader<bool, LOCKUP_LOCK_A>);
impl LOCKUP_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKUP_LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUP_LOCK_A {
        match self.bits {
            false => LOCKUP_LOCK_A::DISCONNECTED,
            true => LOCKUP_LOCK_A::CONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        **self == LOCKUP_LOCK_A::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `CONNECTED`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        **self == LOCKUP_LOCK_A::CONNECTED
    }
}
impl core::ops::Deref for LOCKUP_LOCK_R {
    type Target = crate::FieldReader<bool, LOCKUP_LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKUP_LOCK` writer - Cortex-M0 LOCKUP bit enable bit"]
pub struct LOCKUP_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKUP_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKUP_LOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Cortex-M0 LOCKUP output disconnected from TIM1/15/16/17 Break input"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(LOCKUP_LOCK_A::DISCONNECTED)
    }
    #[doc = "Cortex-M0 LOCKUP output connected to TIM1/15/16/17 Break input"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut W {
        self.variant(LOCKUP_LOCK_A::CONNECTED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - SRAM parity flag"]
    #[inline(always)]
    pub fn sram_pef(&self) -> SRAM_PEF_R {
        SRAM_PEF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&self) -> PVD_LOCK_R {
        PVD_LOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    pub fn sram_parity_lock(&self) -> SRAM_PARITY_LOCK_R {
        SRAM_PARITY_LOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Cortex-M0 LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn lockup_lock(&self) -> LOCKUP_LOCK_R {
        LOCKUP_LOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - SRAM parity flag"]
    #[inline(always)]
    pub fn sram_pef(&mut self) -> SRAM_PEF_W {
        SRAM_PEF_W { w: self }
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&mut self) -> PVD_LOCK_W {
        PVD_LOCK_W { w: self }
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    pub fn sram_parity_lock(&mut self) -> SRAM_PARITY_LOCK_W {
        SRAM_PARITY_LOCK_W { w: self }
    }
    #[doc = "Bit 0 - Cortex-M0 LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn lockup_lock(&mut self) -> LOCKUP_LOCK_W {
        LOCKUP_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
