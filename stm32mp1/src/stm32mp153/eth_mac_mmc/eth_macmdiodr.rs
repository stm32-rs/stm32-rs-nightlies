#[doc = "Register `ETH_MACMDIODR` reader"]
pub struct R(crate::R<ETH_MACMDIODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACMDIODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACMDIODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACMDIODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACMDIODR` writer"]
pub struct W(crate::W<ETH_MACMDIODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACMDIODR_SPEC>;
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
impl From<crate::W<ETH_MACMDIODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACMDIODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GD` reader - GD"]
pub struct GD_R(crate::FieldReader<u16, u16>);
impl GD_R {
    pub(crate) fn new(bits: u16) -> Self {
        GD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GD` writer - GD"]
pub struct GD_W<'a> {
    w: &'a mut W,
}
impl<'a> GD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `RA` reader - RA"]
pub struct RA_R(crate::FieldReader<u16, u16>);
impl RA_R {
    pub(crate) fn new(bits: u16) -> Self {
        RA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA` writer - RA"]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - GD"]
    #[inline(always)]
    pub fn gd(&self) -> GD_R {
        GD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - RA"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GD"]
    #[inline(always)]
    pub fn gd(&mut self) -> GD_W {
        GD_W { w: self }
    }
    #[doc = "Bits 16:31 - RA"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The MDIO Data register stores the Write data to be written to the PHY register located at the address specified in ETH_MACMDIOAR. This register also stores the Read data from the PHY register located at the address specified by MDIO Address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macmdiodr](index.html) module"]
pub struct ETH_MACMDIODR_SPEC;
impl crate::RegisterSpec for ETH_MACMDIODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macmdiodr::R](R) reader structure"]
impl crate::Readable for ETH_MACMDIODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macmdiodr::W](W) writer structure"]
impl crate::Writable for ETH_MACMDIODR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACMDIODR to value 0"]
impl crate::Resettable for ETH_MACMDIODR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
