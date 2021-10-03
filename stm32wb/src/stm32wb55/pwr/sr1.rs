#[doc = "Register `SR1` reader"]
pub struct R(crate::R<SR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WUFI` reader - Internal Wakeup interrupt flag"]
pub struct WUFI_R(crate::FieldReader<bool, bool>);
impl WUFI_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUFI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUFI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C2HF` reader - CPU2 Hold interrupt flag"]
pub struct C2HF_R(crate::FieldReader<bool, bool>);
impl C2HF_R {
    pub(crate) fn new(bits: bool) -> Self {
        C2HF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2HF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AF802` reader - 802.15.4 end of activity interrupt flag"]
pub struct AF802_R(crate::FieldReader<bool, bool>);
impl AF802_R {
    pub(crate) fn new(bits: bool) -> Self {
        AF802_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AF802_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEAF` reader - BLE end of activity interrupt flag"]
pub struct BLEAF_R(crate::FieldReader<bool, bool>);
impl BLEAF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEAF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEAF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRPEF` reader - Enable critical radio phase end of activity interrupt flag"]
pub struct CRPEF_R(crate::FieldReader<bool, bool>);
impl CRPEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRPEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRPEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `_802WUF` reader - 802.15.4 wakeup interrupt flag"]
pub struct _802WUF_R(crate::FieldReader<bool, bool>);
impl _802WUF_R {
    pub(crate) fn new(bits: bool) -> Self {
        _802WUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for _802WUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEWUF` reader - BLE wakeup interrupt flag"]
pub struct BLEWUF_R(crate::FieldReader<bool, bool>);
impl BLEWUF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEWUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEWUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BORHF` reader - BORH interrupt flag"]
pub struct BORHF_R(crate::FieldReader<bool, bool>);
impl BORHF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BORHF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BORHF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDFBF` reader - Step Down converter forced in Bypass interrupt flag"]
pub struct SDFBF_R(crate::FieldReader<bool, bool>);
impl SDFBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDFBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDFBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWUF5` reader - Wakeup flag 5"]
pub struct CWUF5_R(crate::FieldReader<bool, bool>);
impl CWUF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CWUF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWUF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWUF4` reader - Wakeup flag 4"]
pub struct CWUF4_R(crate::FieldReader<bool, bool>);
impl CWUF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CWUF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWUF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWUF3` reader - Wakeup flag 3"]
pub struct CWUF3_R(crate::FieldReader<bool, bool>);
impl CWUF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CWUF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWUF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWUF2` reader - Wakeup flag 2"]
pub struct CWUF2_R(crate::FieldReader<bool, bool>);
impl CWUF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CWUF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWUF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWUF1` reader - Wakeup flag 1"]
pub struct CWUF1_R(crate::FieldReader<bool, bool>);
impl CWUF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CWUF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWUF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 15 - Internal Wakeup interrupt flag"]
    #[inline(always)]
    pub fn wufi(&self) -> WUFI_R {
        WUFI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CPU2 Hold interrupt flag"]
    #[inline(always)]
    pub fn c2hf(&self) -> C2HF_R {
        C2HF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 802.15.4 end of activity interrupt flag"]
    #[inline(always)]
    pub fn af802(&self) -> AF802_R {
        AF802_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BLE end of activity interrupt flag"]
    #[inline(always)]
    pub fn bleaf(&self) -> BLEAF_R {
        BLEAF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable critical radio phase end of activity interrupt flag"]
    #[inline(always)]
    pub fn crpef(&self) -> CRPEF_R {
        CRPEF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 802.15.4 wakeup interrupt flag"]
    #[inline(always)]
    pub fn _802wuf(&self) -> _802WUF_R {
        _802WUF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BLE wakeup interrupt flag"]
    #[inline(always)]
    pub fn blewuf(&self) -> BLEWUF_R {
        BLEWUF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BORH interrupt flag"]
    #[inline(always)]
    pub fn borhf(&self) -> BORHF_R {
        BORHF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Step Down converter forced in Bypass interrupt flag"]
    #[inline(always)]
    pub fn sdfbf(&self) -> SDFBF_R {
        SDFBF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup flag 5"]
    #[inline(always)]
    pub fn cwuf5(&self) -> CWUF5_R {
        CWUF5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wakeup flag 4"]
    #[inline(always)]
    pub fn cwuf4(&self) -> CWUF4_R {
        CWUF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup flag 3"]
    #[inline(always)]
    pub fn cwuf3(&self) -> CWUF3_R {
        CWUF3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup flag 2"]
    #[inline(always)]
    pub fn cwuf2(&self) -> CWUF2_R {
        CWUF2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Wakeup flag 1"]
    #[inline(always)]
    pub fn cwuf1(&self) -> CWUF1_R {
        CWUF1_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Power status register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr1](index.html) module"]
pub struct SR1_SPEC;
impl crate::RegisterSpec for SR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr1::R](R) reader structure"]
impl crate::Readable for SR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for SR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
