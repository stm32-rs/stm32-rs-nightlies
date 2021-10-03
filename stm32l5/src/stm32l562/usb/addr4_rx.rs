#[doc = "Register `ADDR4_RX` reader"]
pub struct R(crate::R<ADDR4_RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR4_RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR4_RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR4_RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR4_RX` writer"]
pub struct W(crate::W<ADDR4_RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR4_RX_SPEC>;
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
impl From<crate::W<ADDR4_RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR4_RX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR4_RX` reader - Reception buffer address"]
pub struct ADDR4_RX_R(crate::FieldReader<u16, u16>);
impl ADDR4_RX_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADDR4_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR4_RX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR4_RX` writer - Reception buffer address"]
pub struct ADDR4_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR4_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 1)) | ((value as u16 & 0x7fff) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:15 - Reception buffer address"]
    #[inline(always)]
    pub fn addr4_rx(&self) -> ADDR4_RX_R {
        ADDR4_RX_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:15 - Reception buffer address"]
    #[inline(always)]
    pub fn addr4_rx(&mut self) -> ADDR4_RX_W {
        ADDR4_RX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reception buffer address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr4_rx](index.html) module"]
pub struct ADDR4_RX_SPEC;
impl crate::RegisterSpec for ADDR4_RX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [addr4_rx::R](R) reader structure"]
impl crate::Readable for ADDR4_RX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr4_rx::W](W) writer structure"]
impl crate::Writable for ADDR4_RX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDR4_RX to value 0"]
impl crate::Resettable for ADDR4_RX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
