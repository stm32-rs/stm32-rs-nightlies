#[doc = "Register `DR_01` reader"]
pub struct R(crate::R<DR_01_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_01_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_01_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_01_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PE` reader - Parity Error bit"]
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
#[doc = "Field `V` reader - Validity bit"]
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
#[doc = "Field `U` reader - User bit"]
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
#[doc = "Field `C` reader - Channel Status bit"]
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
#[doc = "Field `PT` reader - Preamble Type"]
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
#[doc = "Field `DR` reader - Data value"]
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
impl R {
    #[doc = "Bit 0 - Parity Error bit"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Validity bit"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - User bit"]
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel Status bit"]
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Preamble Type"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:31 - Data value"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
#[doc = "Data input register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr_01](index.html) module"]
pub struct DR_01_SPEC;
impl crate::RegisterSpec for DR_01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr_01::R](R) reader structure"]
impl crate::Readable for DR_01_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DR_01 to value 0"]
impl crate::Resettable for DR_01_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
