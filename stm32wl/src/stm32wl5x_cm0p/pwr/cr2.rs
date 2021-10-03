#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PVME3_A {
    #[doc = "0: PVM3 (VDDA monitoring versus 1.62 V threshold) disable"]
    DISABLED = 0,
    #[doc = "1: PVM3 (VDDA monitoring versus 1.62 V threshold) enable"]
    ENABLED = 1,
}
impl From<PVME3_A> for bool {
    #[inline(always)]
    fn from(variant: PVME3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVME3` reader - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
pub struct PVME3_R(crate::FieldReader<bool, PVME3_A>);
impl PVME3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVME3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVME3_A {
        match self.bits {
            false => PVME3_A::DISABLED,
            true => PVME3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PVME3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PVME3_A::ENABLED
    }
}
impl core::ops::Deref for PVME3_R {
    type Target = crate::FieldReader<bool, PVME3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVME3` writer - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
pub struct PVME3_W<'a> {
    w: &'a mut W,
}
impl<'a> PVME3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PVME3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PVM3 (VDDA monitoring versus 1.62 V threshold) disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PVME3_A::DISABLED)
    }
    #[doc = "PVM3 (VDDA monitoring versus 1.62 V threshold) enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PVME3_A::ENABLED)
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
#[doc = "Power voltage detector level selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLS_A {
    #[doc = "0: 2.0V"]
    V2_0 = 0,
    #[doc = "1: 2.2V"]
    V2_2 = 1,
    #[doc = "2: 2.4V"]
    V2_4 = 2,
    #[doc = "3: 2.5V"]
    V2_5 = 3,
    #[doc = "4: 2.6V"]
    V2_6 = 4,
    #[doc = "5: 2.8V"]
    V2_8 = 5,
    #[doc = "6: 2.9V"]
    V2_9 = 6,
    #[doc = "7: External input analog voltage PVD_IN (compared internally to VREFINT)"]
    EXTERNAL = 7,
}
impl From<PLS_A> for u8 {
    #[inline(always)]
    fn from(variant: PLS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLS` reader - Power voltage detector level selection."]
pub struct PLS_R(crate::FieldReader<u8, PLS_A>);
impl PLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLS_A {
        match self.bits {
            0 => PLS_A::V2_0,
            1 => PLS_A::V2_2,
            2 => PLS_A::V2_4,
            3 => PLS_A::V2_5,
            4 => PLS_A::V2_6,
            5 => PLS_A::V2_8,
            6 => PLS_A::V2_9,
            7 => PLS_A::EXTERNAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V2_0`"]
    #[inline(always)]
    pub fn is_v2_0(&self) -> bool {
        **self == PLS_A::V2_0
    }
    #[doc = "Checks if the value of the field is `V2_2`"]
    #[inline(always)]
    pub fn is_v2_2(&self) -> bool {
        **self == PLS_A::V2_2
    }
    #[doc = "Checks if the value of the field is `V2_4`"]
    #[inline(always)]
    pub fn is_v2_4(&self) -> bool {
        **self == PLS_A::V2_4
    }
    #[doc = "Checks if the value of the field is `V2_5`"]
    #[inline(always)]
    pub fn is_v2_5(&self) -> bool {
        **self == PLS_A::V2_5
    }
    #[doc = "Checks if the value of the field is `V2_6`"]
    #[inline(always)]
    pub fn is_v2_6(&self) -> bool {
        **self == PLS_A::V2_6
    }
    #[doc = "Checks if the value of the field is `V2_8`"]
    #[inline(always)]
    pub fn is_v2_8(&self) -> bool {
        **self == PLS_A::V2_8
    }
    #[doc = "Checks if the value of the field is `V2_9`"]
    #[inline(always)]
    pub fn is_v2_9(&self) -> bool {
        **self == PLS_A::V2_9
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        **self == PLS_A::EXTERNAL
    }
}
impl core::ops::Deref for PLS_R {
    type Target = crate::FieldReader<u8, PLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLS` writer - Power voltage detector level selection."]
pub struct PLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2.0V"]
    #[inline(always)]
    pub fn v2_0(self) -> &'a mut W {
        self.variant(PLS_A::V2_0)
    }
    #[doc = "2.2V"]
    #[inline(always)]
    pub fn v2_2(self) -> &'a mut W {
        self.variant(PLS_A::V2_2)
    }
    #[doc = "2.4V"]
    #[inline(always)]
    pub fn v2_4(self) -> &'a mut W {
        self.variant(PLS_A::V2_4)
    }
    #[doc = "2.5V"]
    #[inline(always)]
    pub fn v2_5(self) -> &'a mut W {
        self.variant(PLS_A::V2_5)
    }
    #[doc = "2.6V"]
    #[inline(always)]
    pub fn v2_6(self) -> &'a mut W {
        self.variant(PLS_A::V2_6)
    }
    #[doc = "2.8V"]
    #[inline(always)]
    pub fn v2_8(self) -> &'a mut W {
        self.variant(PLS_A::V2_8)
    }
    #[doc = "2.9V"]
    #[inline(always)]
    pub fn v2_9(self) -> &'a mut W {
        self.variant(PLS_A::V2_9)
    }
    #[doc = "External input analog voltage PVD_IN (compared internally to VREFINT)"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(PLS_A::EXTERNAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Power voltage detector enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PVDE_A {
    #[doc = "0: PVD Disabled"]
    DISABLED = 0,
    #[doc = "1: PVD Enabled"]
    ENABLED = 1,
}
impl From<PVDE_A> for bool {
    #[inline(always)]
    fn from(variant: PVDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub struct PVDE_R(crate::FieldReader<bool, PVDE_A>);
impl PVDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVDE_A {
        match self.bits {
            false => PVDE_A::DISABLED,
            true => PVDE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PVDE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PVDE_A::ENABLED
    }
}
impl core::ops::Deref for PVDE_R {
    type Target = crate::FieldReader<bool, PVDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub struct PVDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PVDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PVD Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PVDE_A::DISABLED)
    }
    #[doc = "PVD Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PVDE_A::ENABLED)
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
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
    #[inline(always)]
    pub fn pvme3(&self) -> PVME3_R {
        PVME3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Power voltage detector level selection."]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
    #[inline(always)]
    pub fn pvme3(&mut self) -> PVME3_W {
        PVME3_W { w: self }
    }
    #[doc = "Bits 1:3 - Power voltage detector level selection."]
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W {
        PLS_W { w: self }
    }
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W {
        PVDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
