#[doc = "Register `DDRCTRL_PCFGQOS0_0` reader"]
pub struct R(crate::R<DDRCTRL_PCFGQOS0_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PCFGQOS0_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PCFGQOS0_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PCFGQOS0_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_PCFGQOS0_0` writer"]
pub struct W(crate::W<DDRCTRL_PCFGQOS0_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_PCFGQOS0_0_SPEC>;
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
impl From<crate::W<DDRCTRL_PCFGQOS0_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_PCFGQOS0_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RQOS_MAP_LEVEL1` reader - RQOS_MAP_LEVEL1"]
pub struct RQOS_MAP_LEVEL1_R(crate::FieldReader<u8, u8>);
impl RQOS_MAP_LEVEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RQOS_MAP_LEVEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RQOS_MAP_LEVEL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RQOS_MAP_LEVEL1` writer - RQOS_MAP_LEVEL1"]
pub struct RQOS_MAP_LEVEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> RQOS_MAP_LEVEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `RQOS_MAP_LEVEL2` reader - RQOS_MAP_LEVEL2"]
pub struct RQOS_MAP_LEVEL2_R(crate::FieldReader<u8, u8>);
impl RQOS_MAP_LEVEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        RQOS_MAP_LEVEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RQOS_MAP_LEVEL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RQOS_MAP_LEVEL2` writer - RQOS_MAP_LEVEL2"]
pub struct RQOS_MAP_LEVEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> RQOS_MAP_LEVEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RQOS_MAP_REGION0` reader - RQOS_MAP_REGION0"]
pub struct RQOS_MAP_REGION0_R(crate::FieldReader<u8, u8>);
impl RQOS_MAP_REGION0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RQOS_MAP_REGION0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RQOS_MAP_REGION0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RQOS_MAP_REGION0` writer - RQOS_MAP_REGION0"]
pub struct RQOS_MAP_REGION0_W<'a> {
    w: &'a mut W,
}
impl<'a> RQOS_MAP_REGION0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `RQOS_MAP_REGION1` reader - RQOS_MAP_REGION1"]
pub struct RQOS_MAP_REGION1_R(crate::FieldReader<u8, u8>);
impl RQOS_MAP_REGION1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RQOS_MAP_REGION1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RQOS_MAP_REGION1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RQOS_MAP_REGION1` writer - RQOS_MAP_REGION1"]
pub struct RQOS_MAP_REGION1_W<'a> {
    w: &'a mut W,
}
impl<'a> RQOS_MAP_REGION1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `RQOS_MAP_REGION2` reader - RQOS_MAP_REGION2"]
pub struct RQOS_MAP_REGION2_R(crate::FieldReader<u8, u8>);
impl RQOS_MAP_REGION2_R {
    pub(crate) fn new(bits: u8) -> Self {
        RQOS_MAP_REGION2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RQOS_MAP_REGION2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RQOS_MAP_REGION2` writer - RQOS_MAP_REGION2"]
pub struct RQOS_MAP_REGION2_W<'a> {
    w: &'a mut W,
}
impl<'a> RQOS_MAP_REGION2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - RQOS_MAP_LEVEL1"]
    #[inline(always)]
    pub fn rqos_map_level1(&self) -> RQOS_MAP_LEVEL1_R {
        RQOS_MAP_LEVEL1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - RQOS_MAP_LEVEL2"]
    #[inline(always)]
    pub fn rqos_map_level2(&self) -> RQOS_MAP_LEVEL2_R {
        RQOS_MAP_LEVEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - RQOS_MAP_REGION0"]
    #[inline(always)]
    pub fn rqos_map_region0(&self) -> RQOS_MAP_REGION0_R {
        RQOS_MAP_REGION0_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - RQOS_MAP_REGION1"]
    #[inline(always)]
    pub fn rqos_map_region1(&self) -> RQOS_MAP_REGION1_R {
        RQOS_MAP_REGION1_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - RQOS_MAP_REGION2"]
    #[inline(always)]
    pub fn rqos_map_region2(&self) -> RQOS_MAP_REGION2_R {
        RQOS_MAP_REGION2_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RQOS_MAP_LEVEL1"]
    #[inline(always)]
    pub fn rqos_map_level1(&mut self) -> RQOS_MAP_LEVEL1_W {
        RQOS_MAP_LEVEL1_W { w: self }
    }
    #[doc = "Bits 8:11 - RQOS_MAP_LEVEL2"]
    #[inline(always)]
    pub fn rqos_map_level2(&mut self) -> RQOS_MAP_LEVEL2_W {
        RQOS_MAP_LEVEL2_W { w: self }
    }
    #[doc = "Bits 16:17 - RQOS_MAP_REGION0"]
    #[inline(always)]
    pub fn rqos_map_region0(&mut self) -> RQOS_MAP_REGION0_W {
        RQOS_MAP_REGION0_W { w: self }
    }
    #[doc = "Bits 20:21 - RQOS_MAP_REGION1"]
    #[inline(always)]
    pub fn rqos_map_region1(&mut self) -> RQOS_MAP_REGION1_W {
        RQOS_MAP_REGION1_W { w: self }
    }
    #[doc = "Bits 24:25 - RQOS_MAP_REGION2"]
    #[inline(always)]
    pub fn rqos_map_region2(&mut self) -> RQOS_MAP_REGION2_W {
        RQOS_MAP_REGION2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL port 0 read Q0S configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pcfgqos0_0](index.html) module"]
pub struct DDRCTRL_PCFGQOS0_0_SPEC;
impl crate::RegisterSpec for DDRCTRL_PCFGQOS0_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_pcfgqos0_0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_PCFGQOS0_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pcfgqos0_0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_PCFGQOS0_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_PCFGQOS0_0 to value 0x0200_0e00"]
impl crate::Resettable for DDRCTRL_PCFGQOS0_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0e00
    }
}
