#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Voltage reference buffer ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VRR_A {
    #[doc = "0: The voltage reference buffer output is not ready"]
    NOTREADY = 0,
    #[doc = "1: The voltage reference buffer output reached the requested level"]
    READY = 1,
}
impl From<VRR_A> for bool {
    #[inline(always)]
    fn from(variant: VRR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VRR` reader - Voltage reference buffer ready"]
pub struct VRR_R(crate::FieldReader<bool, VRR_A>);
impl VRR_R {
    pub(crate) fn new(bits: bool) -> Self {
        VRR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VRR_A {
        match self.bits {
            false => VRR_A::NOTREADY,
            true => VRR_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == VRR_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == VRR_A::READY
    }
}
impl core::ops::Deref for VRR_R {
    type Target = crate::FieldReader<bool, VRR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Voltage reference scale\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VRS_A {
    #[doc = "0: Voltage reference set to VREF_OUT1 (around 2.048 V)"]
    V2_048 = 0,
    #[doc = "1: Voltage reference set to VREF_OUT2 (around 2.5 V)"]
    V2_5 = 1,
}
impl From<VRS_A> for bool {
    #[inline(always)]
    fn from(variant: VRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VRS` reader - Voltage reference scale"]
pub struct VRS_R(crate::FieldReader<bool, VRS_A>);
impl VRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VRS_A {
        match self.bits {
            false => VRS_A::V2_048,
            true => VRS_A::V2_5,
        }
    }
    #[doc = "Checks if the value of the field is `V2_048`"]
    #[inline(always)]
    pub fn is_v2_048(&self) -> bool {
        **self == VRS_A::V2_048
    }
    #[doc = "Checks if the value of the field is `V2_5`"]
    #[inline(always)]
    pub fn is_v2_5(&self) -> bool {
        **self == VRS_A::V2_5
    }
}
impl core::ops::Deref for VRS_R {
    type Target = crate::FieldReader<bool, VRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VRS` writer - Voltage reference scale"]
pub struct VRS_W<'a> {
    w: &'a mut W,
}
impl<'a> VRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VRS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Voltage reference set to VREF_OUT1 (around 2.048 V)"]
    #[inline(always)]
    pub fn v2_048(self) -> &'a mut W {
        self.variant(VRS_A::V2_048)
    }
    #[doc = "Voltage reference set to VREF_OUT2 (around 2.5 V)"]
    #[inline(always)]
    pub fn v2_5(self) -> &'a mut W {
        self.variant(VRS_A::V2_5)
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
#[doc = "High impedance mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIZ_A {
    #[doc = "0: VREF+ pin is internally connected to the voltage reference buffer output"]
    CONNECTED = 0,
    #[doc = "1: VREF+ pin is high impedance"]
    HIGHZ = 1,
}
impl From<HIZ_A> for bool {
    #[inline(always)]
    fn from(variant: HIZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIZ` reader - High impedance mode"]
pub struct HIZ_R(crate::FieldReader<bool, HIZ_A>);
impl HIZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        HIZ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIZ_A {
        match self.bits {
            false => HIZ_A::CONNECTED,
            true => HIZ_A::HIGHZ,
        }
    }
    #[doc = "Checks if the value of the field is `CONNECTED`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        **self == HIZ_A::CONNECTED
    }
    #[doc = "Checks if the value of the field is `HIGHZ`"]
    #[inline(always)]
    pub fn is_high_z(&self) -> bool {
        **self == HIZ_A::HIGHZ
    }
}
impl core::ops::Deref for HIZ_R {
    type Target = crate::FieldReader<bool, HIZ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIZ` writer - High impedance mode"]
pub struct HIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> HIZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIZ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VREF+ pin is internally connected to the voltage reference buffer output"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut W {
        self.variant(HIZ_A::CONNECTED)
    }
    #[doc = "VREF+ pin is high impedance"]
    #[inline(always)]
    pub fn high_z(self) -> &'a mut W {
        self.variant(HIZ_A::HIGHZ)
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
#[doc = "Voltage reference buffer mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENVR_A {
    #[doc = "0: Internal voltage reference mode disable (external voltage reference mode)"]
    DISABLED = 0,
    #[doc = "1: Internal voltage reference mode (reference buffer enable or hold mode) enable"]
    ENABLED = 1,
}
impl From<ENVR_A> for bool {
    #[inline(always)]
    fn from(variant: ENVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENVR` reader - Voltage reference buffer mode enable"]
pub struct ENVR_R(crate::FieldReader<bool, ENVR_A>);
impl ENVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENVR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENVR_A {
        match self.bits {
            false => ENVR_A::DISABLED,
            true => ENVR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENVR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENVR_A::ENABLED
    }
}
impl core::ops::Deref for ENVR_R {
    type Target = crate::FieldReader<bool, ENVR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENVR` writer - Voltage reference buffer mode enable"]
pub struct ENVR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENVR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal voltage reference mode disable (external voltage reference mode)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENVR_A::DISABLED)
    }
    #[doc = "Internal voltage reference mode (reference buffer enable or hold mode) enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENVR_A::ENABLED)
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
    #[doc = "Bit 3 - Voltage reference buffer ready"]
    #[inline(always)]
    pub fn vrr(&self) -> VRR_R {
        VRR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Voltage reference scale"]
    #[inline(always)]
    pub fn vrs(&self) -> VRS_R {
        VRS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - High impedance mode"]
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Voltage reference buffer mode enable"]
    #[inline(always)]
    pub fn envr(&self) -> ENVR_R {
        ENVR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Voltage reference scale"]
    #[inline(always)]
    pub fn vrs(&mut self) -> VRS_W {
        VRS_W { w: self }
    }
    #[doc = "Bit 1 - High impedance mode"]
    #[inline(always)]
    pub fn hiz(&mut self) -> HIZ_W {
        HIZ_W { w: self }
    }
    #[doc = "Bit 0 - Voltage reference buffer mode enable"]
    #[inline(always)]
    pub fn envr(&mut self) -> ENVR_W {
        ENVR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR to value 0x02"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
