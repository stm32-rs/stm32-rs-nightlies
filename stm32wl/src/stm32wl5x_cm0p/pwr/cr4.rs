#[doc = "Register `CR4` reader"]
pub struct R(crate::R<CR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR4` writer"]
pub struct W(crate::W<CR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR4_SPEC>;
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
impl From<crate::W<CR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C2BOOT` reader - oot CPU2 after reset or wakeup from Stop or Standby modes."]
pub struct C2BOOT_R(crate::FieldReader<bool, bool>);
impl C2BOOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        C2BOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2BOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C2BOOT` writer - oot CPU2 after reset or wakeup from Stop or Standby modes."]
pub struct C2BOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> C2BOOT_W<'a> {
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
#[doc = "Wakeup Radio BUSY polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRFBUSYP_A {
    #[doc = "0: Detection on high level (rising edge)"]
    RISINGEDGE = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    FALLINGEDGE = 1,
}
impl From<WRFBUSYP_A> for bool {
    #[inline(always)]
    fn from(variant: WRFBUSYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRFBUSYP` reader - Wakeup Radio BUSY polarity"]
pub struct WRFBUSYP_R(crate::FieldReader<bool, WRFBUSYP_A>);
impl WRFBUSYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRFBUSYP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRFBUSYP_A {
        match self.bits {
            false => WRFBUSYP_A::RISINGEDGE,
            true => WRFBUSYP_A::FALLINGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == WRFBUSYP_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == WRFBUSYP_A::FALLINGEDGE
    }
}
impl core::ops::Deref for WRFBUSYP_R {
    type Target = crate::FieldReader<bool, WRFBUSYP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRFBUSYP` writer - Wakeup Radio BUSY polarity"]
pub struct WRFBUSYP_W<'a> {
    w: &'a mut W,
}
impl<'a> WRFBUSYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRFBUSYP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(WRFBUSYP_A::RISINGEDGE)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(WRFBUSYP_A::FALLINGEDGE)
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
#[doc = "VBAT battery charging resistor selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBRS_A {
    #[doc = "0: VBAT charging through a 5 kΩ resistor"]
    R5K = 0,
    #[doc = "1: VBAT charging through a 1.5 kΩ resistor"]
    R1_5K = 1,
}
impl From<VBRS_A> for bool {
    #[inline(always)]
    fn from(variant: VBRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBRS` reader - VBAT battery charging resistor selection"]
pub struct VBRS_R(crate::FieldReader<bool, VBRS_A>);
impl VBRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBRS_A {
        match self.bits {
            false => VBRS_A::R5K,
            true => VBRS_A::R1_5K,
        }
    }
    #[doc = "Checks if the value of the field is `R5K`"]
    #[inline(always)]
    pub fn is_r5k(&self) -> bool {
        **self == VBRS_A::R5K
    }
    #[doc = "Checks if the value of the field is `R1_5K`"]
    #[inline(always)]
    pub fn is_r1_5k(&self) -> bool {
        **self == VBRS_A::R1_5K
    }
}
impl core::ops::Deref for VBRS_R {
    type Target = crate::FieldReader<bool, VBRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBRS` writer - VBAT battery charging resistor selection"]
pub struct VBRS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBRS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VBAT charging through a 5 kΩ resistor"]
    #[inline(always)]
    pub fn r5k(self) -> &'a mut W {
        self.variant(VBRS_A::R5K)
    }
    #[doc = "VBAT charging through a 1.5 kΩ resistor"]
    #[inline(always)]
    pub fn r1_5k(self) -> &'a mut W {
        self.variant(VBRS_A::R1_5K)
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
#[doc = "VBAT battery charging enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBE_A {
    #[doc = "0: VBAT battery charging disabled"]
    DISABLED = 0,
    #[doc = "1: VBAT battery charging enabled"]
    ENABLED = 1,
}
impl From<VBE_A> for bool {
    #[inline(always)]
    fn from(variant: VBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBE` reader - VBAT battery charging enable"]
pub struct VBE_R(crate::FieldReader<bool, VBE_A>);
impl VBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBE_A {
        match self.bits {
            false => VBE_A::DISABLED,
            true => VBE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == VBE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == VBE_A::ENABLED
    }
}
impl core::ops::Deref for VBE_R {
    type Target = crate::FieldReader<bool, VBE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBE` writer - VBAT battery charging enable"]
pub struct VBE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VBAT battery charging disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VBE_A::DISABLED)
    }
    #[doc = "VBAT battery charging enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VBE_A::ENABLED)
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
#[doc = "Wakeup pin WKUP3 polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP3_A {
    #[doc = "0: Detection on high level (rising edge)"]
    RISINGEDGE = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    FALLINGEDGE = 1,
}
impl From<WP3_A> for bool {
    #[inline(always)]
    fn from(variant: WP3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP3` reader - Wakeup pin WKUP3 polarity"]
pub struct WP3_R(crate::FieldReader<bool, WP3_A>);
impl WP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        WP3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP3_A {
        match self.bits {
            false => WP3_A::RISINGEDGE,
            true => WP3_A::FALLINGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == WP3_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == WP3_A::FALLINGEDGE
    }
}
impl core::ops::Deref for WP3_R {
    type Target = crate::FieldReader<bool, WP3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP3` writer - Wakeup pin WKUP3 polarity"]
pub struct WP3_W<'a> {
    w: &'a mut W,
}
impl<'a> WP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(WP3_A::RISINGEDGE)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(WP3_A::FALLINGEDGE)
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
#[doc = "Wakeup pin WKUP2 polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP2_A {
    #[doc = "0: Detection on high level (rising edge)"]
    RISINGEDGE = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    FALLINGEDGE = 1,
}
impl From<WP2_A> for bool {
    #[inline(always)]
    fn from(variant: WP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP2` reader - Wakeup pin WKUP2 polarity"]
pub struct WP2_R(crate::FieldReader<bool, WP2_A>);
impl WP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WP2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP2_A {
        match self.bits {
            false => WP2_A::RISINGEDGE,
            true => WP2_A::FALLINGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == WP2_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == WP2_A::FALLINGEDGE
    }
}
impl core::ops::Deref for WP2_R {
    type Target = crate::FieldReader<bool, WP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP2` writer - Wakeup pin WKUP2 polarity"]
pub struct WP2_W<'a> {
    w: &'a mut W,
}
impl<'a> WP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(WP2_A::RISINGEDGE)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(WP2_A::FALLINGEDGE)
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
#[doc = "Wakeup pin WKUP1 polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP1_A {
    #[doc = "0: Detection on high level (rising edge)"]
    RISINGEDGE = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    FALLINGEDGE = 1,
}
impl From<WP1_A> for bool {
    #[inline(always)]
    fn from(variant: WP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP1` reader - Wakeup pin WKUP1 polarity"]
pub struct WP1_R(crate::FieldReader<bool, WP1_A>);
impl WP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WP1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP1_A {
        match self.bits {
            false => WP1_A::RISINGEDGE,
            true => WP1_A::FALLINGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == WP1_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == WP1_A::FALLINGEDGE
    }
}
impl core::ops::Deref for WP1_R {
    type Target = crate::FieldReader<bool, WP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP1` writer - Wakeup pin WKUP1 polarity"]
pub struct WP1_W<'a> {
    w: &'a mut W,
}
impl<'a> WP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(WP1_A::RISINGEDGE)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(WP1_A::FALLINGEDGE)
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
    #[doc = "Bit 15 - oot CPU2 after reset or wakeup from Stop or Standby modes."]
    #[inline(always)]
    pub fn c2boot(&self) -> C2BOOT_R {
        C2BOOT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Wakeup Radio BUSY polarity"]
    #[inline(always)]
    pub fn wrfbusyp(&self) -> WRFBUSYP_R {
        WRFBUSYP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VBAT battery charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - VBAT battery charging enable"]
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 polarity"]
    #[inline(always)]
    pub fn wp3(&self) -> WP3_R {
        WP3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 polarity"]
    #[inline(always)]
    pub fn wp2(&self) -> WP2_R {
        WP2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Wakeup pin WKUP1 polarity"]
    #[inline(always)]
    pub fn wp1(&self) -> WP1_R {
        WP1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - oot CPU2 after reset or wakeup from Stop or Standby modes."]
    #[inline(always)]
    pub fn c2boot(&mut self) -> C2BOOT_W {
        C2BOOT_W { w: self }
    }
    #[doc = "Bit 11 - Wakeup Radio BUSY polarity"]
    #[inline(always)]
    pub fn wrfbusyp(&mut self) -> WRFBUSYP_W {
        WRFBUSYP_W { w: self }
    }
    #[doc = "Bit 9 - VBAT battery charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&mut self) -> VBRS_W {
        VBRS_W { w: self }
    }
    #[doc = "Bit 8 - VBAT battery charging enable"]
    #[inline(always)]
    pub fn vbe(&mut self) -> VBE_W {
        VBE_W { w: self }
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 polarity"]
    #[inline(always)]
    pub fn wp3(&mut self) -> WP3_W {
        WP3_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 polarity"]
    #[inline(always)]
    pub fn wp2(&mut self) -> WP2_W {
        WP2_W { w: self }
    }
    #[doc = "Bit 0 - Wakeup pin WKUP1 polarity"]
    #[inline(always)]
    pub fn wp1(&mut self) -> WP1_W {
        WP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr4](index.html) module"]
pub struct CR4_SPEC;
impl crate::RegisterSpec for CR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr4::R](R) reader structure"]
impl crate::Readable for CR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr4::W](W) writer structure"]
impl crate::Writable for CR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR4 to value 0"]
impl crate::Resettable for CR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
