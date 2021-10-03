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
#[doc = "SRAM2 parity error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPF_A {
    #[doc = "0: No SRAM2 parity error detected"]
    NOMINAL = 0,
    #[doc = "1: SRAM2 parity error detected"]
    ERROR = 1,
}
impl From<SPF_A> for bool {
    #[inline(always)]
    fn from(variant: SPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPF` reader - SRAM2 parity error flag"]
pub struct SPF_R(crate::FieldReader<bool, SPF_A>);
impl SPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPF_A {
        match self.bits {
            false => SPF_A::NOMINAL,
            true => SPF_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOMINAL`"]
    #[inline(always)]
    pub fn is_nominal(&self) -> bool {
        **self == SPF_A::NOMINAL
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == SPF_A::ERROR
    }
}
impl core::ops::Deref for SPF_R {
    type Target = crate::FieldReader<bool, SPF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SRAM2 parity error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPF_AW {
    #[doc = "1: Clear SRAM2 parity error flag"]
    CLEAR = 1,
}
impl From<SPF_AW> for bool {
    #[inline(always)]
    fn from(variant: SPF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPF` writer - SRAM2 parity error flag"]
pub struct SPF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear SRAM2 parity error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SPF_AW::CLEAR)
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
#[doc = "ECC Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCL_A {
    #[doc = "0: ECC error disconnected from TIM1/16/17 break input"]
    DISCONNECTED = 0,
    #[doc = "1: ECC error connected to TIM1/16/17 break input"]
    CONNECTED = 1,
}
impl From<ECCL_A> for bool {
    #[inline(always)]
    fn from(variant: ECCL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCL` reader - ECC Lock"]
pub struct ECCL_R(crate::FieldReader<bool, ECCL_A>);
impl ECCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCL_A {
        match self.bits {
            false => ECCL_A::DISCONNECTED,
            true => ECCL_A::CONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        **self == ECCL_A::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `CONNECTED`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        **self == ECCL_A::CONNECTED
    }
}
impl core::ops::Deref for ECCL_R {
    type Target = crate::FieldReader<bool, ECCL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ECC Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCL_AW {
    #[doc = "1: Connect ECC error to TIM1/16/17 break input"]
    CONNECT = 1,
}
impl From<ECCL_AW> for bool {
    #[inline(always)]
    fn from(variant: ECCL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCL` writer - ECC Lock"]
pub struct ECCL_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECCL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Connect ECC error to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut W {
        self.variant(ECCL_AW::CONNECT)
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
#[doc = "PVD lock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PVDL_A {
    #[doc = "0: PVD interrupt disconnected from TIM1/16/17 break input. PVDE and PLS\\[2:0\\]
bits can be programmed by the application"]
    DISCONNECTED = 0,
    #[doc = "1: PVD interrupt connected to TIM1/16/17 break input. PVDE and PLS\\[2:0\\]
bits are read only"]
    CONNECTED = 1,
}
impl From<PVDL_A> for bool {
    #[inline(always)]
    fn from(variant: PVDL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDL` reader - PVD lock enable bit"]
pub struct PVDL_R(crate::FieldReader<bool, PVDL_A>);
impl PVDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVDL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVDL_A {
        match self.bits {
            false => PVDL_A::DISCONNECTED,
            true => PVDL_A::CONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        **self == PVDL_A::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `CONNECTED`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        **self == PVDL_A::CONNECTED
    }
}
impl core::ops::Deref for PVDL_R {
    type Target = crate::FieldReader<bool, PVDL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PVD lock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PVDL_AW {
    #[doc = "1: Connect PVD interretup to TIM1/16/17 break input"]
    CONNECT = 1,
}
impl From<PVDL_AW> for bool {
    #[inline(always)]
    fn from(variant: PVDL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDL` writer - PVD lock enable bit"]
pub struct PVDL_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PVDL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Connect PVD interretup to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut W {
        self.variant(PVDL_AW::CONNECT)
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
#[doc = "SRAM2 parity lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPL_A {
    #[doc = "0: SRAM2 parity error signal disconnected from TIM1/16/17 break input"]
    DISCONNECTED = 0,
    #[doc = "1: SRAM2 parity error signal connected to TIM1/16/17 break input"]
    CONNECTED = 1,
}
impl From<SPL_A> for bool {
    #[inline(always)]
    fn from(variant: SPL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPL` reader - SRAM2 parity lock bit"]
pub struct SPL_R(crate::FieldReader<bool, SPL_A>);
impl SPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPL_A {
        match self.bits {
            false => SPL_A::DISCONNECTED,
            true => SPL_A::CONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        **self == SPL_A::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `CONNECTED`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        **self == SPL_A::CONNECTED
    }
}
impl core::ops::Deref for SPL_R {
    type Target = crate::FieldReader<bool, SPL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SRAM2 parity lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPL_AW {
    #[doc = "1: Connect SRAM2 parity error signal to TIM1/16/17 break input"]
    CONNECT = 1,
}
impl From<SPL_AW> for bool {
    #[inline(always)]
    fn from(variant: SPL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPL` writer - SRAM2 parity lock bit"]
pub struct SPL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Connect SRAM2 parity error signal to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut W {
        self.variant(SPL_AW::CONNECT)
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
#[doc = "CPU1 LOCKUP (Hardfault) output enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLL_A {
    #[doc = "0: CPU LOCKUP output disconnected from TIM1/16/17 break input"]
    DISCONNECTED = 0,
    #[doc = "1: CPU LOCKUP output connected to TIM1/16/17 break input"]
    CONNECTED = 1,
}
impl From<CLL_A> for bool {
    #[inline(always)]
    fn from(variant: CLL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLL` reader - CPU1 LOCKUP (Hardfault) output enable bit"]
pub struct CLL_R(crate::FieldReader<bool, CLL_A>);
impl CLL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLL_A {
        match self.bits {
            false => CLL_A::DISCONNECTED,
            true => CLL_A::CONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        **self == CLL_A::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `CONNECTED`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        **self == CLL_A::CONNECTED
    }
}
impl core::ops::Deref for CLL_R {
    type Target = crate::FieldReader<bool, CLL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CPU1 LOCKUP (Hardfault) output enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLL_AW {
    #[doc = "1: Connect CPU LOCKUP output to TIM1/16/17 break input"]
    CONNECT = 1,
}
impl From<CLL_AW> for bool {
    #[inline(always)]
    fn from(variant: CLL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLL` writer - CPU1 LOCKUP (Hardfault) output enable bit"]
pub struct CLL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Connect CPU LOCKUP output to TIM1/16/17 break input"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut W {
        self.variant(CLL_AW::CONNECT)
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
    #[doc = "Bit 8 - SRAM2 parity error flag"]
    #[inline(always)]
    pub fn spf(&self) -> SPF_R {
        SPF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ECC Lock"]
    #[inline(always)]
    pub fn eccl(&self) -> ECCL_R {
        ECCL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRAM2 parity lock bit"]
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CPU1 LOCKUP (Hardfault) output enable bit"]
    #[inline(always)]
    pub fn cll(&self) -> CLL_R {
        CLL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - SRAM2 parity error flag"]
    #[inline(always)]
    pub fn spf(&mut self) -> SPF_W {
        SPF_W { w: self }
    }
    #[doc = "Bit 3 - ECC Lock"]
    #[inline(always)]
    pub fn eccl(&mut self) -> ECCL_W {
        ECCL_W { w: self }
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvdl(&mut self) -> PVDL_W {
        PVDL_W { w: self }
    }
    #[doc = "Bit 1 - SRAM2 parity lock bit"]
    #[inline(always)]
    pub fn spl(&mut self) -> SPL_W {
        SPL_W { w: self }
    }
    #[doc = "Bit 0 - CPU1 LOCKUP (Hardfault) output enable bit"]
    #[inline(always)]
    pub fn cll(&mut self) -> CLL_W {
        CLL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CFGR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
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
