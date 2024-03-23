#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Computation complete flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCF {
    #[doc = "0: Computation complete"]
    Complete = 0,
    #[doc = "1: Computation not complete"]
    NotComplete = 1,
}
impl From<CCF> for bool {
    #[inline(always)]
    fn from(variant: CCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCF` reader - Computation complete flag"]
pub type CCF_R = crate::BitReader<CCF>;
impl CCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCF {
        match self.bits {
            false => CCF::Complete,
            true => CCF::NotComplete,
        }
    }
    #[doc = "Computation complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == CCF::Complete
    }
    #[doc = "Computation not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == CCF::NotComplete
    }
}
#[doc = "Read error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERR {
    #[doc = "0: Read error not detected"]
    NoError = 0,
    #[doc = "1: Read error detected"]
    Error = 1,
}
impl From<RDERR> for bool {
    #[inline(always)]
    fn from(variant: RDERR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDERR` reader - Read error flag"]
pub type RDERR_R = crate::BitReader<RDERR>;
impl RDERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDERR {
        match self.bits {
            false => RDERR::NoError,
            true => RDERR::Error,
        }
    }
    #[doc = "Read error not detected"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RDERR::NoError
    }
    #[doc = "Read error detected"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RDERR::Error
    }
}
#[doc = "Write error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRERR {
    #[doc = "0: Write error not detected"]
    NoError = 0,
    #[doc = "1: Write error detected"]
    Error = 1,
}
impl From<WRERR> for bool {
    #[inline(always)]
    fn from(variant: WRERR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRERR` reader - Write error flag"]
pub type WRERR_R = crate::BitReader<WRERR>;
impl WRERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WRERR {
        match self.bits {
            false => WRERR::NoError,
            true => WRERR::Error,
        }
    }
    #[doc = "Write error not detected"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WRERR::NoError
    }
    #[doc = "Write error detected"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WRERR::Error
    }
}
impl R {
    #[doc = "Bit 0 - Computation complete flag"]
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read error flag"]
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write error flag"]
    #[inline(always)]
    pub fn wrerr(&self) -> WRERR_R {
        WRERR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
