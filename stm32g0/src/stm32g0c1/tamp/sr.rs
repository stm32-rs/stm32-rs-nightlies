#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `TAMP1F` reader - TAMP1 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP1 input."]
pub type TAMP1F_R = crate::BitReader;
#[doc = "Field `TAMP2F` reader - TAMP2 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP2 input."]
pub type TAMP2F_R = crate::BitReader;
#[doc = "Field `ITAMP3F` reader - LSE monitoring tamper detection flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 3."]
pub type ITAMP3F_R = crate::BitReader;
#[doc = "Field `ITAMP4F` reader - HSE monitoring tamper detection flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 4."]
pub type ITAMP4F_R = crate::BitReader;
#[doc = "Field `ITAMP5F` reader - RTC calendar overflow tamper detection flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 5."]
pub type ITAMP5F_R = crate::BitReader;
#[doc = "Field `ITAMP6F` reader - ST manufacturer readout tamper detection flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 6."]
pub type ITAMP6F_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TAMP1 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP1 input."]
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2 detection flag This flag is set by hardware when a tamper detection event is detected on the TAMP2 input."]
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 18 - LSE monitoring tamper detection flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 3."]
    #[inline(always)]
    pub fn itamp3f(&self) -> ITAMP3F_R {
        ITAMP3F_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSE monitoring tamper detection flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 4."]
    #[inline(always)]
    pub fn itamp4f(&self) -> ITAMP4F_R {
        ITAMP4F_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RTC calendar overflow tamper detection flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 5."]
    #[inline(always)]
    pub fn itamp5f(&self) -> ITAMP5F_R {
        ITAMP5F_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ST manufacturer readout tamper detection flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 6."]
    #[inline(always)]
    pub fn itamp6f(&self) -> ITAMP6F_R {
        ITAMP6F_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "TAMP status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
