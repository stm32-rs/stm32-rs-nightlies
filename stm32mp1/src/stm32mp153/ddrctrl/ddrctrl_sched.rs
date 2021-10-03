#[doc = "Register `DDRCTRL_SCHED` reader"]
pub struct R(crate::R<DDRCTRL_SCHED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_SCHED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_SCHED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_SCHED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_SCHED` writer"]
pub struct W(crate::W<DDRCTRL_SCHED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_SCHED_SPEC>;
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
impl From<crate::W<DDRCTRL_SCHED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_SCHED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_LOW_PRI_N` reader - FORCE_LOW_PRI_N"]
pub struct FORCE_LOW_PRI_N_R(crate::FieldReader<bool, bool>);
impl FORCE_LOW_PRI_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_LOW_PRI_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_LOW_PRI_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_LOW_PRI_N` writer - FORCE_LOW_PRI_N"]
pub struct FORCE_LOW_PRI_N_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_LOW_PRI_N_W<'a> {
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
#[doc = "Field `PREFER_WRITE` reader - PREFER_WRITE"]
pub struct PREFER_WRITE_R(crate::FieldReader<bool, bool>);
impl PREFER_WRITE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PREFER_WRITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREFER_WRITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREFER_WRITE` writer - PREFER_WRITE"]
pub struct PREFER_WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFER_WRITE_W<'a> {
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
#[doc = "Field `PAGECLOSE` reader - PAGECLOSE"]
pub struct PAGECLOSE_R(crate::FieldReader<bool, bool>);
impl PAGECLOSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGECLOSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGECLOSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAGECLOSE` writer - PAGECLOSE"]
pub struct PAGECLOSE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGECLOSE_W<'a> {
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
#[doc = "Field `LPR_NUM_ENTRIES` reader - LPR_NUM_ENTRIES"]
pub struct LPR_NUM_ENTRIES_R(crate::FieldReader<u8, u8>);
impl LPR_NUM_ENTRIES_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPR_NUM_ENTRIES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPR_NUM_ENTRIES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPR_NUM_ENTRIES` writer - LPR_NUM_ENTRIES"]
pub struct LPR_NUM_ENTRIES_W<'a> {
    w: &'a mut W,
}
impl<'a> LPR_NUM_ENTRIES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `GO2CRITICAL_HYSTERESIS` reader - GO2CRITICAL_HYSTERESIS"]
pub struct GO2CRITICAL_HYSTERESIS_R(crate::FieldReader<u8, u8>);
impl GO2CRITICAL_HYSTERESIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        GO2CRITICAL_HYSTERESIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GO2CRITICAL_HYSTERESIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GO2CRITICAL_HYSTERESIS` writer - GO2CRITICAL_HYSTERESIS"]
pub struct GO2CRITICAL_HYSTERESIS_W<'a> {
    w: &'a mut W,
}
impl<'a> GO2CRITICAL_HYSTERESIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `RDWR_IDLE_GAP` reader - RDWR_IDLE_GAP"]
pub struct RDWR_IDLE_GAP_R(crate::FieldReader<u8, u8>);
impl RDWR_IDLE_GAP_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDWR_IDLE_GAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDWR_IDLE_GAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDWR_IDLE_GAP` writer - RDWR_IDLE_GAP"]
pub struct RDWR_IDLE_GAP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWR_IDLE_GAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | ((value as u32 & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FORCE_LOW_PRI_N"]
    #[inline(always)]
    pub fn force_low_pri_n(&self) -> FORCE_LOW_PRI_N_R {
        FORCE_LOW_PRI_N_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PREFER_WRITE"]
    #[inline(always)]
    pub fn prefer_write(&self) -> PREFER_WRITE_R {
        PREFER_WRITE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PAGECLOSE"]
    #[inline(always)]
    pub fn pageclose(&self) -> PAGECLOSE_R {
        PAGECLOSE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - LPR_NUM_ENTRIES"]
    #[inline(always)]
    pub fn lpr_num_entries(&self) -> LPR_NUM_ENTRIES_R {
        LPR_NUM_ENTRIES_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - GO2CRITICAL_HYSTERESIS"]
    #[inline(always)]
    pub fn go2critical_hysteresis(&self) -> GO2CRITICAL_HYSTERESIS_R {
        GO2CRITICAL_HYSTERESIS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - RDWR_IDLE_GAP"]
    #[inline(always)]
    pub fn rdwr_idle_gap(&self) -> RDWR_IDLE_GAP_R {
        RDWR_IDLE_GAP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FORCE_LOW_PRI_N"]
    #[inline(always)]
    pub fn force_low_pri_n(&mut self) -> FORCE_LOW_PRI_N_W {
        FORCE_LOW_PRI_N_W { w: self }
    }
    #[doc = "Bit 1 - PREFER_WRITE"]
    #[inline(always)]
    pub fn prefer_write(&mut self) -> PREFER_WRITE_W {
        PREFER_WRITE_W { w: self }
    }
    #[doc = "Bit 2 - PAGECLOSE"]
    #[inline(always)]
    pub fn pageclose(&mut self) -> PAGECLOSE_W {
        PAGECLOSE_W { w: self }
    }
    #[doc = "Bits 8:11 - LPR_NUM_ENTRIES"]
    #[inline(always)]
    pub fn lpr_num_entries(&mut self) -> LPR_NUM_ENTRIES_W {
        LPR_NUM_ENTRIES_W { w: self }
    }
    #[doc = "Bits 16:23 - GO2CRITICAL_HYSTERESIS"]
    #[inline(always)]
    pub fn go2critical_hysteresis(&mut self) -> GO2CRITICAL_HYSTERESIS_W {
        GO2CRITICAL_HYSTERESIS_W { w: self }
    }
    #[doc = "Bits 24:30 - RDWR_IDLE_GAP"]
    #[inline(always)]
    pub fn rdwr_idle_gap(&mut self) -> RDWR_IDLE_GAP_W {
        RDWR_IDLE_GAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL scheduler control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_sched](index.html) module"]
pub struct DDRCTRL_SCHED_SPEC;
impl crate::RegisterSpec for DDRCTRL_SCHED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_sched::R](R) reader structure"]
impl crate::Readable for DDRCTRL_SCHED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_sched::W](W) writer structure"]
impl crate::Writable for DDRCTRL_SCHED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_SCHED to value 0x0805"]
impl crate::Resettable for DDRCTRL_SCHED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0805
    }
}
