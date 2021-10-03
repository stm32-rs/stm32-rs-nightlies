#[doc = "Register `ICACHE_CRR0` reader"]
pub struct R(crate::R<ICACHE_CRR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_CRR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_CRR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_CRR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_CRR0` writer"]
pub struct W(crate::W<ICACHE_CRR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_CRR0_SPEC>;
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
impl From<crate::W<ICACHE_CRR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_CRR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASEADDR` reader - BASEADDR"]
pub struct BASEADDR_R(crate::FieldReader<u8, u8>);
impl BASEADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        BASEADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BASEADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BASEADDR` writer - BASEADDR"]
pub struct BASEADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> BASEADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `RSIZE` reader - RSIZE"]
pub struct RSIZE_R(crate::FieldReader<u8, u8>);
impl RSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSIZE` writer - RSIZE"]
pub struct RSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `REN` reader - REN"]
pub struct REN_R(crate::FieldReader<bool, bool>);
impl REN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REN` writer - REN"]
pub struct REN_W<'a> {
    w: &'a mut W,
}
impl<'a> REN_W<'a> {
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
#[doc = "Field `REMAPADDR` reader - REMAPADDR"]
pub struct REMAPADDR_R(crate::FieldReader<u16, u16>);
impl REMAPADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        REMAPADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REMAPADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REMAPADDR` writer - REMAPADDR"]
pub struct REMAPADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> REMAPADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | ((value as u32 & 0x07ff) << 16);
        self.w
    }
}
#[doc = "Field `MSTSEL` reader - MSTSEL"]
pub struct MSTSEL_R(crate::FieldReader<bool, bool>);
impl MSTSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTSEL` writer - MSTSEL"]
pub struct MSTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `HBURST` reader - HBURST"]
pub struct HBURST_R(crate::FieldReader<bool, bool>);
impl HBURST_R {
    pub(crate) fn new(bits: bool) -> Self {
        HBURST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBURST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HBURST` writer - HBURST"]
pub struct HBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> HBURST_W<'a> {
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
    #[doc = "Bits 0:7 - BASEADDR"]
    #[inline(always)]
    pub fn baseaddr(&self) -> BASEADDR_R {
        BASEADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 9:11 - RSIZE"]
    #[inline(always)]
    pub fn rsize(&self) -> RSIZE_R {
        RSIZE_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 15 - REN"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - REMAPADDR"]
    #[inline(always)]
    pub fn remapaddr(&self) -> REMAPADDR_R {
        REMAPADDR_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - MSTSEL"]
    #[inline(always)]
    pub fn mstsel(&self) -> MSTSEL_R {
        MSTSEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - HBURST"]
    #[inline(always)]
    pub fn hburst(&self) -> HBURST_R {
        HBURST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - BASEADDR"]
    #[inline(always)]
    pub fn baseaddr(&mut self) -> BASEADDR_W {
        BASEADDR_W { w: self }
    }
    #[doc = "Bits 9:11 - RSIZE"]
    #[inline(always)]
    pub fn rsize(&mut self) -> RSIZE_W {
        RSIZE_W { w: self }
    }
    #[doc = "Bit 15 - REN"]
    #[inline(always)]
    pub fn ren(&mut self) -> REN_W {
        REN_W { w: self }
    }
    #[doc = "Bits 16:26 - REMAPADDR"]
    #[inline(always)]
    pub fn remapaddr(&mut self) -> REMAPADDR_W {
        REMAPADDR_W { w: self }
    }
    #[doc = "Bit 28 - MSTSEL"]
    #[inline(always)]
    pub fn mstsel(&mut self) -> MSTSEL_W {
        MSTSEL_W { w: self }
    }
    #[doc = "Bit 31 - HBURST"]
    #[inline(always)]
    pub fn hburst(&mut self) -> HBURST_W {
        HBURST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICACHE region configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_crr0](index.html) module"]
pub struct ICACHE_CRR0_SPEC;
impl crate::RegisterSpec for ICACHE_CRR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_crr0::R](R) reader structure"]
impl crate::Readable for ICACHE_CRR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_crr0::W](W) writer structure"]
impl crate::Writable for ICACHE_CRR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICACHE_CRR0 to value 0x0200"]
impl crate::Resettable for ICACHE_CRR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
