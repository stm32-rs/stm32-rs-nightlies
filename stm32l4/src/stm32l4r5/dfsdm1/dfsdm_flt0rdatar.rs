#[doc = "Register `DFSDM_FLT0RDATAR` reader"]
pub type R = crate::R<DFSDM_FLT0RDATARrs>;
#[doc = "Field `RDATACH` reader - Regular channel most recently converted"]
pub type RDATACH_R = crate::FieldReader;
#[doc = "Field `RPEND` reader - Regular channel pending data"]
pub type RPEND_R = crate::BitReader;
#[doc = "Field `RDATA` reader - Regular channel conversion data"]
pub type RDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - Regular channel most recently converted"]
    #[inline(always)]
    pub fn rdatach(&self) -> RDATACH_R {
        RDATACH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Regular channel pending data"]
    #[inline(always)]
    pub fn rpend(&self) -> RPEND_R {
        RPEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Regular channel conversion data"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "data register for the regular channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0rdatar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT0RDATARrs;
impl crate::RegisterSpec for DFSDM_FLT0RDATARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt0rdatar::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT0RDATARrs {}
#[doc = "`reset()` method sets DFSDM_FLT0RDATAR to value 0"]
impl crate::Resettable for DFSDM_FLT0RDATARrs {
    const RESET_VALUE: u32 = 0;
}
