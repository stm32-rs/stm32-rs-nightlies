#[doc = "Register `SPDIFRX_FMT0_DR` reader"]
pub struct R(crate::R<SPDIFRX_FMT0_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDIFRX_FMT0_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDIFRX_FMT0_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDIFRX_FMT0_DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DR` reader - DR"]
pub struct DR_R(crate::FieldReader<u32, u32>);
impl DR_R {
    pub(crate) fn new(bits: u32) -> Self {
        DR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE` reader - PE"]
pub struct PE_R(crate::FieldReader<bool, bool>);
impl PE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `V` reader - V"]
pub struct V_R(crate::FieldReader<bool, bool>);
impl V_R {
    pub(crate) fn new(bits: bool) -> Self {
        V_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for V_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U` reader - U"]
pub struct U_R(crate::FieldReader<bool, bool>);
impl U_R {
    pub(crate) fn new(bits: bool) -> Self {
        U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C` reader - C"]
pub struct C_R(crate::FieldReader<bool, bool>);
impl C_R {
    pub(crate) fn new(bits: bool) -> Self {
        C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PT` reader - PT"]
pub struct PT_R(crate::FieldReader<u8, u8>);
impl PT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - DR"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - PE"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - V"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - U"]
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - C"]
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - PT"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
#[doc = "This register can take 3 different formats according to DRFMT. Here is the format when DRFMT = 00:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_fmt0_dr](index.html) module"]
pub struct SPDIFRX_FMT0_DR_SPEC;
impl crate::RegisterSpec for SPDIFRX_FMT0_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spdifrx_fmt0_dr::R](R) reader structure"]
impl crate::Readable for SPDIFRX_FMT0_DR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPDIFRX_FMT0_DR to value 0"]
impl crate::Resettable for SPDIFRX_FMT0_DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
