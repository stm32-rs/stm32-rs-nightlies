#[doc = "Register `FDCAN_TEST` reader"]
pub struct R(crate::R<FDCAN_TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LBCK` reader - Loop Back mode"]
pub struct LBCK_R(crate::FieldReader<bool, bool>);
impl LBCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LBCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LBCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX` reader - Loop Back mode"]
pub struct TX_R(crate::FieldReader<u8, u8>);
impl TX_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX` reader - Control of Transmit Pin"]
pub struct RX_R(crate::FieldReader<bool, bool>);
impl RX_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - Loop Back mode"]
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Loop Back mode"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Control of Transmit Pin"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "FDCAN Test Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_test](index.html) module"]
pub struct FDCAN_TEST_SPEC;
impl crate::RegisterSpec for FDCAN_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_test::R](R) reader structure"]
impl crate::Readable for FDCAN_TEST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_TEST to value 0"]
impl crate::Resettable for FDCAN_TEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
