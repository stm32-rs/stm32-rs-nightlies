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
#[doc = "TAMP1F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1F_A {
    #[doc = "0: No tamper detected"]
    IDLE = 0,
    #[doc = "1: Tamper detected"]
    TAMPER = 1,
}
impl From<TAMP1F_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1F` reader - TAMP1F"]
pub struct TAMP1F_R(crate::FieldReader<bool, TAMP1F_A>);
impl TAMP1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1F_A {
        match self.bits {
            false => TAMP1F_A::IDLE,
            true => TAMP1F_A::TAMPER,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == TAMP1F_A::IDLE
    }
    #[doc = "Checks if the value of the field is `TAMPER`"]
    #[inline(always)]
    pub fn is_tamper(&self) -> bool {
        **self == TAMP1F_A::TAMPER
    }
}
impl core::ops::Deref for TAMP1F_R {
    type Target = crate::FieldReader<bool, TAMP1F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "TAMP2F"]
pub type TAMP2F_A = TAMP1F_A;
#[doc = "Field `TAMP2F` reader - TAMP2F"]
pub type TAMP2F_R = TAMP1F_R;
#[doc = "TAMP3F"]
pub type TAMP3F_A = TAMP1F_A;
#[doc = "Field `TAMP3F` reader - TAMP3F"]
pub type TAMP3F_R = TAMP1F_R;
#[doc = "ITAMP3F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITAMP3F_A {
    #[doc = "0: No tamper detected"]
    IDLE = 0,
    #[doc = "1: Internal tamper detected"]
    TAMPER = 1,
}
impl From<ITAMP3F_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP3F` reader - ITAMP3F"]
pub struct ITAMP3F_R(crate::FieldReader<bool, ITAMP3F_A>);
impl ITAMP3F_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP3F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITAMP3F_A {
        match self.bits {
            false => ITAMP3F_A::IDLE,
            true => ITAMP3F_A::TAMPER,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == ITAMP3F_A::IDLE
    }
    #[doc = "Checks if the value of the field is `TAMPER`"]
    #[inline(always)]
    pub fn is_tamper(&self) -> bool {
        **self == ITAMP3F_A::TAMPER
    }
}
impl core::ops::Deref for ITAMP3F_R {
    type Target = crate::FieldReader<bool, ITAMP3F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ITAMP5F"]
pub type ITAMP5F_A = ITAMP3F_A;
#[doc = "Field `ITAMP5F` reader - ITAMP5F"]
pub type ITAMP5F_R = ITAMP3F_R;
#[doc = "ITAMP6F"]
pub type ITAMP6F_A = ITAMP3F_A;
#[doc = "Field `ITAMP6F` reader - ITAMP6F"]
pub type ITAMP6F_R = ITAMP3F_R;
#[doc = "ITAMP8F"]
pub type ITAMP8F_A = ITAMP3F_A;
#[doc = "Field `ITAMP8F` reader - ITAMP8F"]
pub type ITAMP8F_R = ITAMP3F_R;
impl R {
    #[doc = "Bit 0 - TAMP1F"]
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TAMP2F"]
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TAMP3F"]
    #[inline(always)]
    pub fn tamp3f(&self) -> TAMP3F_R {
        TAMP3F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ITAMP3F"]
    #[inline(always)]
    pub fn itamp3f(&self) -> ITAMP3F_R {
        ITAMP3F_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ITAMP5F"]
    #[inline(always)]
    pub fn itamp5f(&self) -> ITAMP5F_R {
        ITAMP5F_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ITAMP6F"]
    #[inline(always)]
    pub fn itamp6f(&self) -> ITAMP6F_R {
        ITAMP6F_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ITAMP8F"]
    #[inline(always)]
    pub fn itamp8f(&self) -> ITAMP8F_R {
        ITAMP8F_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
#[doc = "TAMP status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
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
