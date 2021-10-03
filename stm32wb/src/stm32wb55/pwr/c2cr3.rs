#[doc = "Register `C2CR3` reader"]
pub struct R(crate::R<C2CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2CR3` writer"]
pub struct W(crate::W<C2CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2CR3_SPEC>;
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
impl From<crate::W<C2CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EIWUL` reader - Enable internal wakeup line for CPU2"]
pub struct EIWUL_R(crate::FieldReader<bool, bool>);
impl EIWUL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EIWUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EIWUL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIWUL` writer - Enable internal wakeup line for CPU2"]
pub struct EIWUL_W<'a> {
    w: &'a mut W,
}
impl<'a> EIWUL_W<'a> {
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
#[doc = "Field `APC` reader - Apply pull-up and pull-down configuration for CPU2"]
pub struct APC_R(crate::FieldReader<bool, bool>);
impl APC_R {
    pub(crate) fn new(bits: bool) -> Self {
        APC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APC` writer - Apply pull-up and pull-down configuration for CPU2"]
pub struct APC_W<'a> {
    w: &'a mut W,
}
impl<'a> APC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `E802WUP` reader - Enable 802.15.4 host wakeup interrupt for CPU2"]
pub struct E802WUP_R(crate::FieldReader<bool, bool>);
impl E802WUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        E802WUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for E802WUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E802WUP` writer - Enable 802.15.4 host wakeup interrupt for CPU2"]
pub struct E802WUP_W<'a> {
    w: &'a mut W,
}
impl<'a> E802WUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `EBLEWUP` reader - Enable BLE host wakeup interrupt for CPU2"]
pub struct EBLEWUP_R(crate::FieldReader<bool, bool>);
impl EBLEWUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EBLEWUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EBLEWUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EBLEWUP` writer - Enable BLE host wakeup interrupt for CPU2"]
pub struct EBLEWUP_W<'a> {
    w: &'a mut W,
}
impl<'a> EBLEWUP_W<'a> {
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
#[doc = "Field `EWUP5` reader - Enable Wakeup pin WKUP5 for CPU2"]
pub struct EWUP5_R(crate::FieldReader<bool, bool>);
impl EWUP5_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWUP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EWUP5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWUP5` writer - Enable Wakeup pin WKUP5 for CPU2"]
pub struct EWUP5_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP5_W<'a> {
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
#[doc = "Field `EWUP4` reader - Enable Wakeup pin WKUP4 for CPU2"]
pub struct EWUP4_R(crate::FieldReader<bool, bool>);
impl EWUP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWUP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EWUP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWUP4` writer - Enable Wakeup pin WKUP4 for CPU2"]
pub struct EWUP4_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP4_W<'a> {
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
#[doc = "Field `EWUP3` reader - Enable Wakeup pin WKUP3 for CPU2"]
pub struct EWUP3_R(crate::FieldReader<bool, bool>);
impl EWUP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWUP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EWUP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWUP3` writer - Enable Wakeup pin WKUP3 for CPU2"]
pub struct EWUP3_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP3_W<'a> {
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
#[doc = "Field `EWUP2` reader - Enable Wakeup pin WKUP2 for CPU2"]
pub struct EWUP2_R(crate::FieldReader<bool, bool>);
impl EWUP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWUP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EWUP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWUP2` writer - Enable Wakeup pin WKUP2 for CPU2"]
pub struct EWUP2_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP2_W<'a> {
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
#[doc = "Field `EWUP1` reader - Enable Wakeup pin WKUP1 for CPU2"]
pub struct EWUP1_R(crate::FieldReader<bool, bool>);
impl EWUP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWUP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EWUP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWUP1` writer - Enable Wakeup pin WKUP1 for CPU2"]
pub struct EWUP1_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP1_W<'a> {
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
    #[doc = "Bit 15 - Enable internal wakeup line for CPU2"]
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Apply pull-up and pull-down configuration for CPU2"]
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable 802.15.4 host wakeup interrupt for CPU2"]
    #[inline(always)]
    pub fn e802wup(&self) -> E802WUP_R {
        E802WUP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable BLE host wakeup interrupt for CPU2"]
    #[inline(always)]
    pub fn eblewup(&self) -> EBLEWUP_R {
        EBLEWUP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Wakeup pin WKUP5 for CPU2"]
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable Wakeup pin WKUP4 for CPU2"]
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3 for CPU2"]
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2 for CPU2"]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1 for CPU2"]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Enable internal wakeup line for CPU2"]
    #[inline(always)]
    pub fn eiwul(&mut self) -> EIWUL_W {
        EIWUL_W { w: self }
    }
    #[doc = "Bit 12 - Apply pull-up and pull-down configuration for CPU2"]
    #[inline(always)]
    pub fn apc(&mut self) -> APC_W {
        APC_W { w: self }
    }
    #[doc = "Bit 10 - Enable 802.15.4 host wakeup interrupt for CPU2"]
    #[inline(always)]
    pub fn e802wup(&mut self) -> E802WUP_W {
        E802WUP_W { w: self }
    }
    #[doc = "Bit 9 - Enable BLE host wakeup interrupt for CPU2"]
    #[inline(always)]
    pub fn eblewup(&mut self) -> EBLEWUP_W {
        EBLEWUP_W { w: self }
    }
    #[doc = "Bit 4 - Enable Wakeup pin WKUP5 for CPU2"]
    #[inline(always)]
    pub fn ewup5(&mut self) -> EWUP5_W {
        EWUP5_W { w: self }
    }
    #[doc = "Bit 3 - Enable Wakeup pin WKUP4 for CPU2"]
    #[inline(always)]
    pub fn ewup4(&mut self) -> EWUP4_W {
        EWUP4_W { w: self }
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3 for CPU2"]
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W {
        EWUP3_W { w: self }
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2 for CPU2"]
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W {
        EWUP2_W { w: self }
    }
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1 for CPU2"]
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W {
        EWUP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 Power control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2cr3](index.html) module"]
pub struct C2CR3_SPEC;
impl crate::RegisterSpec for C2CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2cr3::R](R) reader structure"]
impl crate::Readable for C2CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2cr3::W](W) writer structure"]
impl crate::Writable for C2CR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2CR3 to value 0x8000"]
impl crate::Resettable for C2CR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
