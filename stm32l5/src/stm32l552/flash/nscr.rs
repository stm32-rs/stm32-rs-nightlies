#[doc = "Register `NSCR` reader"]
pub struct R(crate::R<NSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NSCR` writer"]
pub struct W(crate::W<NSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSCR_SPEC>;
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
impl From<crate::W<NSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSPG` reader - NSPG"]
pub struct NSPG_R(crate::FieldReader<bool, bool>);
impl NSPG_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSPG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSPG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSPG` writer - NSPG"]
pub struct NSPG_W<'a> {
    w: &'a mut W,
}
impl<'a> NSPG_W<'a> {
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
#[doc = "Field `NSPER` reader - NSPER"]
pub struct NSPER_R(crate::FieldReader<bool, bool>);
impl NSPER_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSPER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSPER` writer - NSPER"]
pub struct NSPER_W<'a> {
    w: &'a mut W,
}
impl<'a> NSPER_W<'a> {
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
#[doc = "Field `NSMER1` reader - NSMER1"]
pub struct NSMER1_R(crate::FieldReader<bool, bool>);
impl NSMER1_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSMER1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSMER1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSMER1` writer - NSMER1"]
pub struct NSMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> NSMER1_W<'a> {
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
#[doc = "Field `NSPNB` reader - NSPNB"]
pub struct NSPNB_R(crate::FieldReader<u8, u8>);
impl NSPNB_R {
    pub(crate) fn new(bits: u8) -> Self {
        NSPNB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSPNB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSPNB` writer - NSPNB"]
pub struct NSPNB_W<'a> {
    w: &'a mut W,
}
impl<'a> NSPNB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 3)) | ((value as u32 & 0x7f) << 3);
        self.w
    }
}
#[doc = "Field `NSBKER` reader - NSBKER"]
pub struct NSBKER_R(crate::FieldReader<bool, bool>);
impl NSBKER_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSBKER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSBKER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSBKER` writer - NSBKER"]
pub struct NSBKER_W<'a> {
    w: &'a mut W,
}
impl<'a> NSBKER_W<'a> {
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
#[doc = "Field `NSMER2` reader - NSMER2"]
pub struct NSMER2_R(crate::FieldReader<bool, bool>);
impl NSMER2_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSMER2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSMER2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSMER2` writer - NSMER2"]
pub struct NSMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> NSMER2_W<'a> {
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
#[doc = "Field `NSSTRT` reader - Options modification start"]
pub struct NSSTRT_R(crate::FieldReader<bool, bool>);
impl NSSTRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSSTRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSSTRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSSTRT` writer - Options modification start"]
pub struct NSSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> NSSTRT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `OPTSTRT` reader - Options modification start"]
pub struct OPTSTRT_R(crate::FieldReader<bool, bool>);
impl OPTSTRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPTSTRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPTSTRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPTSTRT` writer - Options modification start"]
pub struct OPTSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTSTRT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `NSEOPIE` reader - NSEOPIE"]
pub struct NSEOPIE_R(crate::FieldReader<bool, bool>);
impl NSEOPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSEOPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSEOPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSEOPIE` writer - NSEOPIE"]
pub struct NSEOPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NSEOPIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `NSERRIE` reader - NSERRIE"]
pub struct NSERRIE_R(crate::FieldReader<bool, bool>);
impl NSERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSERRIE` writer - NSERRIE"]
pub struct NSERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NSERRIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `OBL_LAUNCH` reader - Force the option byte loading"]
pub struct OBL_LAUNCH_R(crate::FieldReader<bool, bool>);
impl OBL_LAUNCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        OBL_LAUNCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OBL_LAUNCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OBL_LAUNCH` writer - Force the option byte loading"]
pub struct OBL_LAUNCH_W<'a> {
    w: &'a mut W,
}
impl<'a> OBL_LAUNCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `OPTLOCK` reader - Options Lock"]
pub struct OPTLOCK_R(crate::FieldReader<bool, bool>);
impl OPTLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPTLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPTLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPTLOCK` writer - Options Lock"]
pub struct OPTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `NSLOCK` reader - NSLOCK"]
pub struct NSLOCK_R(crate::FieldReader<bool, bool>);
impl NSLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSLOCK` writer - NSLOCK"]
pub struct NSLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> NSLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - NSPG"]
    #[inline(always)]
    pub fn nspg(&self) -> NSPG_R {
        NSPG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NSPER"]
    #[inline(always)]
    pub fn nsper(&self) -> NSPER_R {
        NSPER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NSMER1"]
    #[inline(always)]
    pub fn nsmer1(&self) -> NSMER1_R {
        NSMER1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:9 - NSPNB"]
    #[inline(always)]
    pub fn nspnb(&self) -> NSPNB_R {
        NSPNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bit 11 - NSBKER"]
    #[inline(always)]
    pub fn nsbker(&self) -> NSBKER_R {
        NSBKER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - NSMER2"]
    #[inline(always)]
    pub fn nsmer2(&self) -> NSMER2_R {
        NSMER2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Options modification start"]
    #[inline(always)]
    pub fn nsstrt(&self) -> NSSTRT_R {
        NSSTRT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Options modification start"]
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - NSEOPIE"]
    #[inline(always)]
    pub fn nseopie(&self) -> NSEOPIE_R {
        NSEOPIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - NSERRIE"]
    #[inline(always)]
    pub fn nserrie(&self) -> NSERRIE_R {
        NSERRIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Force the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Options Lock"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - NSLOCK"]
    #[inline(always)]
    pub fn nslock(&self) -> NSLOCK_R {
        NSLOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NSPG"]
    #[inline(always)]
    pub fn nspg(&mut self) -> NSPG_W {
        NSPG_W { w: self }
    }
    #[doc = "Bit 1 - NSPER"]
    #[inline(always)]
    pub fn nsper(&mut self) -> NSPER_W {
        NSPER_W { w: self }
    }
    #[doc = "Bit 2 - NSMER1"]
    #[inline(always)]
    pub fn nsmer1(&mut self) -> NSMER1_W {
        NSMER1_W { w: self }
    }
    #[doc = "Bits 3:9 - NSPNB"]
    #[inline(always)]
    pub fn nspnb(&mut self) -> NSPNB_W {
        NSPNB_W { w: self }
    }
    #[doc = "Bit 11 - NSBKER"]
    #[inline(always)]
    pub fn nsbker(&mut self) -> NSBKER_W {
        NSBKER_W { w: self }
    }
    #[doc = "Bit 15 - NSMER2"]
    #[inline(always)]
    pub fn nsmer2(&mut self) -> NSMER2_W {
        NSMER2_W { w: self }
    }
    #[doc = "Bit 16 - Options modification start"]
    #[inline(always)]
    pub fn nsstrt(&mut self) -> NSSTRT_W {
        NSSTRT_W { w: self }
    }
    #[doc = "Bit 17 - Options modification start"]
    #[inline(always)]
    pub fn optstrt(&mut self) -> OPTSTRT_W {
        OPTSTRT_W { w: self }
    }
    #[doc = "Bit 24 - NSEOPIE"]
    #[inline(always)]
    pub fn nseopie(&mut self) -> NSEOPIE_W {
        NSEOPIE_W { w: self }
    }
    #[doc = "Bit 25 - NSERRIE"]
    #[inline(always)]
    pub fn nserrie(&mut self) -> NSERRIE_W {
        NSERRIE_W { w: self }
    }
    #[doc = "Bit 27 - Force the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W {
        OBL_LAUNCH_W { w: self }
    }
    #[doc = "Bit 30 - Options Lock"]
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W {
        OPTLOCK_W { w: self }
    }
    #[doc = "Bit 31 - NSLOCK"]
    #[inline(always)]
    pub fn nslock(&mut self) -> NSLOCK_W {
        NSLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash non-secure control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nscr](index.html) module"]
pub struct NSCR_SPEC;
impl crate::RegisterSpec for NSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nscr::R](R) reader structure"]
impl crate::Readable for NSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nscr::W](W) writer structure"]
impl crate::Writable for NSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NSCR to value 0xc000_0000"]
impl crate::Resettable for NSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc000_0000
    }
}
