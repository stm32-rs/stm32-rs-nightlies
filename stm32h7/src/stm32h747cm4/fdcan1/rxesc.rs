#[doc = "Register `RXESC` reader"]
pub struct R(crate::R<RXESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXESC` writer"]
pub struct W(crate::W<RXESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXESC_SPEC>;
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
impl From<crate::W<RXESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F0DS` reader - Rx FIFO 1 Data Field Size:"]
pub struct F0DS_R(crate::FieldReader<u8, u8>);
impl F0DS_R {
    pub(crate) fn new(bits: u8) -> Self {
        F0DS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0DS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F0DS` writer - Rx FIFO 1 Data Field Size:"]
pub struct F0DS_W<'a> {
    w: &'a mut W,
}
impl<'a> F0DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `F1DS` reader - Rx FIFO 0 Data Field Size:"]
pub struct F1DS_R(crate::FieldReader<u8, u8>);
impl F1DS_R {
    pub(crate) fn new(bits: u8) -> Self {
        F1DS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1DS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1DS` writer - Rx FIFO 0 Data Field Size:"]
pub struct F1DS_W<'a> {
    w: &'a mut W,
}
impl<'a> F1DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `RBDS` reader - Rx Buffer Data Field Size:"]
pub struct RBDS_R(crate::FieldReader<u8, u8>);
impl RBDS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RBDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBDS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBDS` writer - Rx Buffer Data Field Size:"]
pub struct RBDS_W<'a> {
    w: &'a mut W,
}
impl<'a> RBDS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Rx FIFO 1 Data Field Size:"]
    #[inline(always)]
    pub fn f0ds(&self) -> F0DS_R {
        F0DS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Rx FIFO 0 Data Field Size:"]
    #[inline(always)]
    pub fn f1ds(&self) -> F1DS_R {
        F1DS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Rx Buffer Data Field Size:"]
    #[inline(always)]
    pub fn rbds(&self) -> RBDS_R {
        RBDS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Rx FIFO 1 Data Field Size:"]
    #[inline(always)]
    pub fn f0ds(&mut self) -> F0DS_W {
        F0DS_W { w: self }
    }
    #[doc = "Bits 4:6 - Rx FIFO 0 Data Field Size:"]
    #[inline(always)]
    pub fn f1ds(&mut self) -> F1DS_W {
        F1DS_W { w: self }
    }
    #[doc = "Bits 8:10 - Rx Buffer Data Field Size:"]
    #[inline(always)]
    pub fn rbds(&mut self) -> RBDS_W {
        RBDS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Rx Buffer Element Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxesc](index.html) module"]
pub struct RXESC_SPEC;
impl crate::RegisterSpec for RXESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxesc::R](R) reader structure"]
impl crate::Readable for RXESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxesc::W](W) writer structure"]
impl crate::Writable for RXESC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXESC to value 0"]
impl crate::Resettable for RXESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
