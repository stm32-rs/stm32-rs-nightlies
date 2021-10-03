#[doc = "Register `ACR` reader"]
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR` writer"]
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Latency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LATENCY_A {
    #[doc = "0: Zero wait state is used to read a word in the NVM"]
    WS0 = 0,
    #[doc = "1: One wait state is used to read a word in the NVM"]
    WS1 = 1,
}
impl From<LATENCY_A> for bool {
    #[inline(always)]
    fn from(variant: LATENCY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LATENCY` reader - Latency"]
pub struct LATENCY_R(crate::FieldReader<bool, LATENCY_A>);
impl LATENCY_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATENCY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LATENCY_A {
        match self.bits {
            false => LATENCY_A::WS0,
            true => LATENCY_A::WS1,
        }
    }
    #[doc = "Checks if the value of the field is `WS0`"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        **self == LATENCY_A::WS0
    }
    #[doc = "Checks if the value of the field is `WS1`"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        **self == LATENCY_A::WS1
    }
}
impl core::ops::Deref for LATENCY_R {
    type Target = crate::FieldReader<bool, LATENCY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATENCY` writer - Latency"]
pub struct LATENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> LATENCY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LATENCY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Zero wait state is used to read a word in the NVM"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut W {
        self.variant(LATENCY_A::WS0)
    }
    #[doc = "One wait state is used to read a word in the NVM"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut W {
        self.variant(LATENCY_A::WS1)
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
#[doc = "Prefetch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTEN_A {
    #[doc = "0: Prefetch is disabled"]
    DISABLED = 0,
    #[doc = "1: Prefetch is enabled"]
    ENABLED = 1,
}
impl From<PRFTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRFTEN` reader - Prefetch enable"]
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
#[doc = "Field `PRFTEN` writer - Prefetch enable"]
pub struct PRFTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRFTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRFTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Prefetch is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::DISABLED)
    }
    #[doc = "Prefetch is enabled"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Flash mode during Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEP_PD_A {
    #[doc = "0: When the device is in Sleep mode, the NVM is in Idle mode"]
    NVMIDLEMODE = 0,
    #[doc = "1: When the device is in Sleep mode, the NVM is in power-down mode"]
    NVMPWRDOWNMODE = 1,
}
impl From<SLEEP_PD_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEP_PD` reader - Flash mode during Sleep"]
pub struct SLEEP_PD_R(crate::FieldReader<bool, SLEEP_PD_A>);
impl SLEEP_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLEEP_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_PD_A {
        match self.bits {
            false => SLEEP_PD_A::NVMIDLEMODE,
            true => SLEEP_PD_A::NVMPWRDOWNMODE,
        }
    }
    #[doc = "Checks if the value of the field is `NVMIDLEMODE`"]
    #[inline(always)]
    pub fn is_nvmidle_mode(&self) -> bool {
        **self == SLEEP_PD_A::NVMIDLEMODE
    }
    #[doc = "Checks if the value of the field is `NVMPWRDOWNMODE`"]
    #[inline(always)]
    pub fn is_nvmpwr_down_mode(&self) -> bool {
        **self == SLEEP_PD_A::NVMPWRDOWNMODE
    }
}
impl core::ops::Deref for SLEEP_PD_R {
    type Target = crate::FieldReader<bool, SLEEP_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEP_PD` writer - Flash mode during Sleep"]
pub struct SLEEP_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEP_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "When the device is in Sleep mode, the NVM is in Idle mode"]
    #[inline(always)]
    pub fn nvmidle_mode(self) -> &'a mut W {
        self.variant(SLEEP_PD_A::NVMIDLEMODE)
    }
    #[doc = "When the device is in Sleep mode, the NVM is in power-down mode"]
    #[inline(always)]
    pub fn nvmpwr_down_mode(self) -> &'a mut W {
        self.variant(SLEEP_PD_A::NVMPWRDOWNMODE)
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
#[doc = "Flash mode during Run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN_PD_A {
    #[doc = "0: When the device is in Run mode, the NVM is in Idle mode"]
    NVMIDLEMODE = 0,
    #[doc = "1: When the device is in Run mode, the NVM is in power-down mode"]
    NVMPWRDOWNMODE = 1,
}
impl From<RUN_PD_A> for bool {
    #[inline(always)]
    fn from(variant: RUN_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN_PD` reader - Flash mode during Run"]
pub struct RUN_PD_R(crate::FieldReader<bool, RUN_PD_A>);
impl RUN_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RUN_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUN_PD_A {
        match self.bits {
            false => RUN_PD_A::NVMIDLEMODE,
            true => RUN_PD_A::NVMPWRDOWNMODE,
        }
    }
    #[doc = "Checks if the value of the field is `NVMIDLEMODE`"]
    #[inline(always)]
    pub fn is_nvmidle_mode(&self) -> bool {
        **self == RUN_PD_A::NVMIDLEMODE
    }
    #[doc = "Checks if the value of the field is `NVMPWRDOWNMODE`"]
    #[inline(always)]
    pub fn is_nvmpwr_down_mode(&self) -> bool {
        **self == RUN_PD_A::NVMPWRDOWNMODE
    }
}
impl core::ops::Deref for RUN_PD_R {
    type Target = crate::FieldReader<bool, RUN_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUN_PD` writer - Flash mode during Run"]
pub struct RUN_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RUN_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "When the device is in Run mode, the NVM is in Idle mode"]
    #[inline(always)]
    pub fn nvmidle_mode(self) -> &'a mut W {
        self.variant(RUN_PD_A::NVMIDLEMODE)
    }
    #[doc = "When the device is in Run mode, the NVM is in power-down mode"]
    #[inline(always)]
    pub fn nvmpwr_down_mode(self) -> &'a mut W {
        self.variant(RUN_PD_A::NVMPWRDOWNMODE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Disable Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISAB_BUF_A {
    #[doc = "0: The buffers are enabled"]
    ENABLED = 0,
    #[doc = "1: The buffers are disabled"]
    DISABLED = 1,
}
impl From<DISAB_BUF_A> for bool {
    #[inline(always)]
    fn from(variant: DISAB_BUF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISAB_BUF` reader - Disable Buffer"]
pub struct DISAB_BUF_R(crate::FieldReader<bool, DISAB_BUF_A>);
impl DISAB_BUF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISAB_BUF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISAB_BUF_A {
        match self.bits {
            false => DISAB_BUF_A::ENABLED,
            true => DISAB_BUF_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DISAB_BUF_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DISAB_BUF_A::DISABLED
    }
}
impl core::ops::Deref for DISAB_BUF_R {
    type Target = crate::FieldReader<bool, DISAB_BUF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISAB_BUF` writer - Disable Buffer"]
pub struct DISAB_BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> DISAB_BUF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISAB_BUF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The buffers are enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISAB_BUF_A::ENABLED)
    }
    #[doc = "The buffers are disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISAB_BUF_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Pre-read data address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRE_READ_A {
    #[doc = "0: The pre-read is disabled"]
    DISABLED = 0,
    #[doc = "1: The pre-read is enabled"]
    ENABLED = 1,
}
impl From<PRE_READ_A> for bool {
    #[inline(always)]
    fn from(variant: PRE_READ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRE_READ` reader - Pre-read data address"]
pub struct PRE_READ_R(crate::FieldReader<bool, PRE_READ_A>);
impl PRE_READ_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRE_READ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRE_READ_A {
        match self.bits {
            false => PRE_READ_A::DISABLED,
            true => PRE_READ_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PRE_READ_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PRE_READ_A::ENABLED
    }
}
impl core::ops::Deref for PRE_READ_R {
    type Target = crate::FieldReader<bool, PRE_READ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRE_READ` writer - Pre-read data address"]
pub struct PRE_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_READ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRE_READ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The pre-read is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRE_READ_A::DISABLED)
    }
    #[doc = "The pre-read is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRE_READ_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Flash mode during Sleep"]
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flash mode during Run"]
    #[inline(always)]
    pub fn run_pd(&self) -> RUN_PD_R {
        RUN_PD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Disable Buffer"]
    #[inline(always)]
    pub fn disab_buf(&self) -> DISAB_BUF_R {
        DISAB_BUF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pre-read data address"]
    #[inline(always)]
    pub fn pre_read(&self) -> PRE_READ_R {
        PRE_READ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Latency"]
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W {
        LATENCY_W { w: self }
    }
    #[doc = "Bit 1 - Prefetch enable"]
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W {
        PRFTEN_W { w: self }
    }
    #[doc = "Bit 3 - Flash mode during Sleep"]
    #[inline(always)]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W {
        SLEEP_PD_W { w: self }
    }
    #[doc = "Bit 4 - Flash mode during Run"]
    #[inline(always)]
    pub fn run_pd(&mut self) -> RUN_PD_W {
        RUN_PD_W { w: self }
    }
    #[doc = "Bit 5 - Disable Buffer"]
    #[inline(always)]
    pub fn disab_buf(&mut self) -> DISAB_BUF_W {
        DISAB_BUF_W { w: self }
    }
    #[doc = "Bit 6 - Pre-read data address"]
    #[inline(always)]
    pub fn pre_read(&mut self) -> PRE_READ_W {
        PRE_READ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](index.html) module"]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr::R](R) reader structure"]
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr::W](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
