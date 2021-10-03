#[doc = "Register `ETH_MACL4A1R` reader"]
pub struct R(crate::R<ETH_MACL4A1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACL4A1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACL4A1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACL4A1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACL4A1R` writer"]
pub struct W(crate::W<ETH_MACL4A1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACL4A1R_SPEC>;
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
impl From<crate::W<ETH_MACL4A1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACL4A1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L4SP1` reader - L4SP1"]
pub struct L4SP1_R(crate::FieldReader<u16, u16>);
impl L4SP1_R {
    pub(crate) fn new(bits: u16) -> Self {
        L4SP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L4SP1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L4SP1` writer - L4SP1"]
pub struct L4SP1_W<'a> {
    w: &'a mut W,
}
impl<'a> L4SP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `L4DP1` reader - L4DP1"]
pub struct L4DP1_R(crate::FieldReader<u16, u16>);
impl L4DP1_R {
    pub(crate) fn new(bits: u16) -> Self {
        L4DP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L4DP1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L4DP1` writer - L4DP1"]
pub struct L4DP1_W<'a> {
    w: &'a mut W,
}
impl<'a> L4DP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - L4SP1"]
    #[inline(always)]
    pub fn l4sp1(&self) -> L4SP1_R {
        L4SP1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - L4DP1"]
    #[inline(always)]
    pub fn l4dp1(&self) -> L4DP1_R {
        L4DP1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - L4SP1"]
    #[inline(always)]
    pub fn l4sp1(&mut self) -> L4SP1_W {
        L4SP1_W { w: self }
    }
    #[doc = "Bits 16:31 - L4DP1"]
    #[inline(always)]
    pub fn l4dp1(&mut self) -> L4DP1_W {
        L4DP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Layer 4 Address 0 register and registers 580 through 667 are reserved (RO with default value) if Enable Layer 3 and Layer 4 Packet Filter option is not selected while configuring the core. You can configure the Layer 3 and Layer 4 Address Registers to be double-synchronized by selecting the Synchronize Layer 3 and Layer 4 Address Registers to Rx Clock Domain option while configuring the core. When you select this option, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Layer 3 and Layer 4 Address Registers are written. For proper synchronization updates, you should perform consecutive writes to same Layer 3 and Layer 4 Address Registers after at least four clock cycles delay of the destination clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macl4a1r](index.html) module"]
pub struct ETH_MACL4A1R_SPEC;
impl crate::RegisterSpec for ETH_MACL4A1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macl4a1r::R](R) reader structure"]
impl crate::Readable for ETH_MACL4A1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macl4a1r::W](W) writer structure"]
impl crate::Writable for ETH_MACL4A1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACL4A1R to value 0"]
impl crate::Resettable for ETH_MACL4A1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
