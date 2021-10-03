#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Address error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRERRF_A {
    #[doc = "0: No error"]
    NOERROR = 0,
    #[doc = "1: Address access is out of range (unmapped address)"]
    ERROR = 1,
}
impl From<ADDRERRF_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRERRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRERRF` reader - Address error flag"]
pub struct ADDRERRF_R(crate::FieldReader<bool, ADDRERRF_A>);
impl ADDRERRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDRERRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRERRF_A {
        match self.bits {
            false => ADDRERRF_A::NOERROR,
            true => ADDRERRF_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == ADDRERRF_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == ADDRERRF_A::ERROR
    }
}
impl core::ops::Deref for ADDRERRF_R {
    type Target = crate::FieldReader<bool, ADDRERRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PKA RAM error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMERRF_A {
    #[doc = "0: No error"]
    NOERROR = 0,
    #[doc = "1: An AHB access to the PKA RAM occurred while the PKA core was computing and using its internal RAM (AHB PKA_RAM access are not allowed while PKA operation is in progress)"]
    ERROR = 1,
}
impl From<RAMERRF_A> for bool {
    #[inline(always)]
    fn from(variant: RAMERRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMERRF` reader - PKA RAM error flag"]
pub struct RAMERRF_R(crate::FieldReader<bool, RAMERRF_A>);
impl RAMERRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAMERRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMERRF_A {
        match self.bits {
            false => RAMERRF_A::NOERROR,
            true => RAMERRF_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == RAMERRF_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == RAMERRF_A::ERROR
    }
}
impl core::ops::Deref for RAMERRF_R {
    type Target = crate::FieldReader<bool, RAMERRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PKA End of Operation flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROCENDF_A {
    #[doc = "0: Operation in progress"]
    INPROGRESS = 0,
    #[doc = "1: PKA operation is completed - set when BUSY is deasserted"]
    COMPLETED = 1,
}
impl From<PROCENDF_A> for bool {
    #[inline(always)]
    fn from(variant: PROCENDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROCENDF` reader - PKA End of Operation flag"]
pub struct PROCENDF_R(crate::FieldReader<bool, PROCENDF_A>);
impl PROCENDF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROCENDF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROCENDF_A {
        match self.bits {
            false => PROCENDF_A::INPROGRESS,
            true => PROCENDF_A::COMPLETED,
        }
    }
    #[doc = "Checks if the value of the field is `INPROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        **self == PROCENDF_A::INPROGRESS
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        **self == PROCENDF_A::COMPLETED
    }
}
impl core::ops::Deref for PROCENDF_R {
    type Target = crate::FieldReader<bool, PROCENDF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PKA operation is in progressThis bit is set to 1 whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: No operation in pgoress"]
    IDLE = 0,
    #[doc = "1: Operation in progress"]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - PKA operation is in progressThis bit is set to 1 whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started."]
pub struct BUSY_R(crate::FieldReader<bool, BUSY_A>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::IDLE,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == BUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == BUSY_A::BUSY
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 20 - Address error flag"]
    #[inline(always)]
    pub fn addrerrf(&self) -> ADDRERRF_R {
        ADDRERRF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PKA RAM error flag"]
    #[inline(always)]
    pub fn ramerrf(&self) -> RAMERRF_R {
        RAMERRF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PKA End of Operation flag"]
    #[inline(always)]
    pub fn procendf(&self) -> PROCENDF_R {
        PROCENDF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PKA operation is in progressThis bit is set to 1 whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
