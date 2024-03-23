#[doc = "Register `DFSDM_FLT4RDATAR` reader"]
pub type R = crate::R<DFSDM_FLT4RDATARrs>;
#[doc = "Field `RDATACH` reader - RDATACH"]
pub type RDATACH_R = crate::FieldReader;
#[doc = "Field `RPEND` reader - RPEND"]
pub type RPEND_R = crate::BitReader;
#[doc = "Field `RDATA` reader - RDATA"]
pub type RDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - RDATACH"]
    #[inline(always)]
    pub fn rdatach(&self) -> RDATACH_R {
        RDATACH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - RPEND"]
    #[inline(always)]
    pub fn rpend(&self) -> RPEND_R {
        RPEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:31 - RDATA"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "DFSDM filter 4 data register for the regular channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt4rdatar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT4RDATARrs;
impl crate::RegisterSpec for DFSDM_FLT4RDATARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt4rdatar::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT4RDATARrs {}
#[doc = "`reset()` method sets DFSDM_FLT4RDATAR to value 0"]
impl crate::Resettable for DFSDM_FLT4RDATARrs {
    const RESET_VALUE: u32 = 0;
}
