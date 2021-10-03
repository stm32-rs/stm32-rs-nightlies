#[doc = "Register `PCROP1ER` reader"]
pub struct R(crate::R<PCROP1ER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP1ER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP1ER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP1ER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCROP1ER` writer"]
pub struct W(crate::W<PCROP1ER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCROP1ER_SPEC>;
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
impl From<crate::W<PCROP1ER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCROP1ER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCROP1_END` reader - Bank 1 PCROP area end offset"]
pub struct PCROP1_END_R(crate::FieldReader<u16, u16>);
impl PCROP1_END_R {
    pub(crate) fn new(bits: u16) -> Self {
        PCROP1_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCROP1_END_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCROP1_END` writer - Bank 1 PCROP area end offset"]
pub struct PCROP1_END_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP1_END_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `PCROP_RDP` reader - PCROP area preserved when RDP level decreased"]
pub struct PCROP_RDP_R(crate::FieldReader<bool, bool>);
impl PCROP_RDP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCROP_RDP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCROP_RDP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCROP_RDP` writer - PCROP area preserved when RDP level decreased"]
pub struct PCROP_RDP_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP_RDP_W<'a> {
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
    #[doc = "Bits 0:15 - Bank 1 PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop1_end(&self) -> PCROP1_END_R {
        PCROP1_END_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bank 1 PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop1_end(&mut self) -> PCROP1_END_W {
        PCROP1_END_W { w: self }
    }
    #[doc = "Bit 31 - PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W {
        PCROP_RDP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Bank 1 PCROP End address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrop1er](index.html) module"]
pub struct PCROP1ER_SPEC;
impl crate::RegisterSpec for PCROP1ER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcrop1er::R](R) reader structure"]
impl crate::Readable for PCROP1ER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcrop1er::W](W) writer structure"]
impl crate::Writable for PCROP1ER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCROP1ER to value 0x0fff_0000"]
impl crate::Resettable for PCROP1ER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_0000
    }
}
