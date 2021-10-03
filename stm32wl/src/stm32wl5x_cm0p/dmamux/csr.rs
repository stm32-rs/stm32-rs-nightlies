#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Synchronization overrun event flag"]
pub type SOF13_A = SOF10_A;
#[doc = "Field `SOF13` reader - Synchronization overrun event flag"]
pub type SOF13_R = SOF10_R;
#[doc = "SOF12"]
pub type SOF12_A = SOF10_A;
#[doc = "Field `SOF12` reader - SOF12"]
pub type SOF12_R = SOF10_R;
#[doc = "SOF11"]
pub type SOF11_A = SOF10_A;
#[doc = "Field `SOF11` reader - SOF11"]
pub type SOF11_R = SOF10_R;
#[doc = "SOF10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF10_A {
    #[doc = "0: No synchronization event occured on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ"]
    NOSYNCEVENT = 0,
    #[doc = "1: Synchronization event occured on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ"]
    SYNCEVENT = 1,
}
impl From<SOF10_A> for bool {
    #[inline(always)]
    fn from(variant: SOF10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF10` reader - SOF10"]
pub struct SOF10_R(crate::FieldReader<bool, SOF10_A>);
impl SOF10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOF10_A {
        match self.bits {
            false => SOF10_A::NOSYNCEVENT,
            true => SOF10_A::SYNCEVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOSYNCEVENT`"]
    #[inline(always)]
    pub fn is_no_sync_event(&self) -> bool {
        **self == SOF10_A::NOSYNCEVENT
    }
    #[doc = "Checks if the value of the field is `SYNCEVENT`"]
    #[inline(always)]
    pub fn is_sync_event(&self) -> bool {
        **self == SOF10_A::SYNCEVENT
    }
}
impl core::ops::Deref for SOF10_R {
    type Target = crate::FieldReader<bool, SOF10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SOF9"]
pub type SOF9_A = SOF0_A;
#[doc = "Field `SOF9` reader - SOF9"]
pub type SOF9_R = SOF0_R;
#[doc = "SOF8"]
pub type SOF8_A = SOF0_A;
#[doc = "Field `SOF8` reader - SOF8"]
pub type SOF8_R = SOF0_R;
#[doc = "SOF7"]
pub type SOF7_A = SOF0_A;
#[doc = "Field `SOF7` reader - SOF7"]
pub type SOF7_R = SOF0_R;
#[doc = "SOF6"]
pub type SOF6_A = SOF0_A;
#[doc = "Field `SOF6` reader - SOF6"]
pub type SOF6_R = SOF0_R;
#[doc = "SOF5"]
pub type SOF5_A = SOF0_A;
#[doc = "Field `SOF5` reader - SOF5"]
pub type SOF5_R = SOF0_R;
#[doc = "SOF4"]
pub type SOF4_A = SOF0_A;
#[doc = "Field `SOF4` reader - SOF4"]
pub type SOF4_R = SOF0_R;
#[doc = "SOF3"]
pub type SOF3_A = SOF0_A;
#[doc = "Field `SOF3` reader - SOF3"]
pub type SOF3_R = SOF0_R;
#[doc = "SOF2"]
pub type SOF2_A = SOF0_A;
#[doc = "Field `SOF2` reader - SOF2"]
pub type SOF2_R = SOF0_R;
#[doc = "SOF1"]
pub type SOF1_A = SOF0_A;
#[doc = "Field `SOF1` reader - SOF1"]
pub type SOF1_R = SOF0_R;
#[doc = "SOF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF0_A {
    #[doc = "0: No synchronization event occured on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ"]
    NOSYNCEVENT = 0,
    #[doc = "1: Synchronization event occured on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ"]
    SYNCEVENT = 1,
}
impl From<SOF0_A> for bool {
    #[inline(always)]
    fn from(variant: SOF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF0` reader - SOF0"]
pub struct SOF0_R(crate::FieldReader<bool, SOF0_A>);
impl SOF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOF0_A {
        match self.bits {
            false => SOF0_A::NOSYNCEVENT,
            true => SOF0_A::SYNCEVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOSYNCEVENT`"]
    #[inline(always)]
    pub fn is_no_sync_event(&self) -> bool {
        **self == SOF0_A::NOSYNCEVENT
    }
    #[doc = "Checks if the value of the field is `SYNCEVENT`"]
    #[inline(always)]
    pub fn is_sync_event(&self) -> bool {
        **self == SOF0_A::SYNCEVENT
    }
}
impl core::ops::Deref for SOF0_R {
    type Target = crate::FieldReader<bool, SOF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 13 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof13(&self) -> SOF13_R {
        SOF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SOF12"]
    #[inline(always)]
    pub fn sof12(&self) -> SOF12_R {
        SOF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SOF11"]
    #[inline(always)]
    pub fn sof11(&self) -> SOF11_R {
        SOF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SOF10"]
    #[inline(always)]
    pub fn sof10(&self) -> SOF10_R {
        SOF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SOF9"]
    #[inline(always)]
    pub fn sof9(&self) -> SOF9_R {
        SOF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SOF8"]
    #[inline(always)]
    pub fn sof8(&self) -> SOF8_R {
        SOF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SOF7"]
    #[inline(always)]
    pub fn sof7(&self) -> SOF7_R {
        SOF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SOF6"]
    #[inline(always)]
    pub fn sof6(&self) -> SOF6_R {
        SOF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SOF5"]
    #[inline(always)]
    pub fn sof5(&self) -> SOF5_R {
        SOF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SOF4"]
    #[inline(always)]
    pub fn sof4(&self) -> SOF4_R {
        SOF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SOF3"]
    #[inline(always)]
    pub fn sof3(&self) -> SOF3_R {
        SOF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SOF2"]
    #[inline(always)]
    pub fn sof2(&self) -> SOF2_R {
        SOF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SOF1"]
    #[inline(always)]
    pub fn sof1(&self) -> SOF1_R {
        SOF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SOF0"]
    #[inline(always)]
    pub fn sof0(&self) -> SOF0_R {
        SOF0_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "request line multiplexer interrupt channel status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
