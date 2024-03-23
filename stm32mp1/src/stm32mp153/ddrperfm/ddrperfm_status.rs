#[doc = "Register `DDRPERFM_STATUS` reader"]
pub type R = crate::R<DDRPERFM_STATUSrs>;
#[doc = "Field `COVF` reader - COVF"]
pub type COVF_R = crate::FieldReader;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `TOVF` reader - TOVF"]
pub type TOVF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - COVF"]
    #[inline(always)]
    pub fn covf(&self) -> COVF_R {
        COVF_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - TOVF"]
    #[inline(always)]
    pub fn tovf(&self) -> TOVF_R {
        TOVF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "DDRPERFM status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPERFM_STATUSrs;
impl crate::RegisterSpec for DDRPERFM_STATUSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrperfm_status::R`](R) reader structure"]
impl crate::Readable for DDRPERFM_STATUSrs {}
#[doc = "`reset()` method sets DDRPERFM_STATUS to value 0"]
impl crate::Resettable for DDRPERFM_STATUSrs {
    const RESET_VALUE: u32 = 0;
}
