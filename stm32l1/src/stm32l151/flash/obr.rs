#[doc = "Register `OBR` reader"]
pub struct R(crate::R<OBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDPRT` reader - Read protection"]
pub struct RDPRT_R(crate::FieldReader<u8, u8>);
impl RDPRT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDPRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDPRT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOR_LEV` reader - BOR_LEV"]
pub struct BOR_LEV_R(crate::FieldReader<u8, u8>);
impl BOR_LEV_R {
    pub(crate) fn new(bits: u8) -> Self {
        BOR_LEV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOR_LEV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWDG_SW` reader - IWDG_SW"]
pub struct IWDG_SW_R(crate::FieldReader<bool, bool>);
impl IWDG_SW_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDG_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWDG_SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nRTS_STOP` reader - nRTS_STOP"]
pub struct NRTS_STOP_R(crate::FieldReader<bool, bool>);
impl NRTS_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        NRTS_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRTS_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub struct NRST_STDBY_R(crate::FieldReader<bool, bool>);
impl NRST_STDBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        NRST_STDBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRST_STDBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFB2` reader - Boot From Bank 2"]
pub struct BFB2_R(crate::FieldReader<bool, bool>);
impl BFB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BFB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Read protection"]
    #[inline(always)]
    pub fn rdprt(&self) -> RDPRT_R {
        RDPRT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - BOR_LEV"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - IWDG_SW"]
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - nRTS_STOP"]
    #[inline(always)]
    pub fn n_rts_stop(&self) -> NRTS_STOP_R {
        NRTS_STOP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Boot From Bank 2"]
    #[inline(always)]
    pub fn bfb2(&self) -> BFB2_R {
        BFB2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
#[doc = "Option byte register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obr](index.html) module"]
pub struct OBR_SPEC;
impl crate::RegisterSpec for OBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [obr::R](R) reader structure"]
impl crate::Readable for OBR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OBR to value 0x00f8_0000"]
impl crate::Resettable for OBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00f8_0000
    }
}
