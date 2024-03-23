#[doc = "Register `DTS_SR` reader"]
pub type R = crate::R<DTS_SRrs>;
#[doc = "Field `TS1_ITEF` reader - TS1_ITEF"]
pub type TS1_ITEF_R = crate::BitReader;
#[doc = "Field `TS1_ITLF` reader - TS1_ITLF"]
pub type TS1_ITLF_R = crate::BitReader;
#[doc = "Field `TS1_ITHF` reader - TS1_ITHF"]
pub type TS1_ITHF_R = crate::BitReader;
#[doc = "Field `TS1_AITEF` reader - TS1_AITEF"]
pub type TS1_AITEF_R = crate::BitReader;
#[doc = "Field `TS1_AITLF` reader - TS1_AITLF"]
pub type TS1_AITLF_R = crate::BitReader;
#[doc = "Field `TS1_AITHF` reader - TS1_AITHF"]
pub type TS1_AITHF_R = crate::BitReader;
#[doc = "Field `TS1_RDY` reader - TS1_RDY"]
pub type TS1_RDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TS1_ITEF"]
    #[inline(always)]
    pub fn ts1_itef(&self) -> TS1_ITEF_R {
        TS1_ITEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TS1_ITLF"]
    #[inline(always)]
    pub fn ts1_itlf(&self) -> TS1_ITLF_R {
        TS1_ITLF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TS1_ITHF"]
    #[inline(always)]
    pub fn ts1_ithf(&self) -> TS1_ITHF_R {
        TS1_ITHF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - TS1_AITEF"]
    #[inline(always)]
    pub fn ts1_aitef(&self) -> TS1_AITEF_R {
        TS1_AITEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TS1_AITLF"]
    #[inline(always)]
    pub fn ts1_aitlf(&self) -> TS1_AITLF_R {
        TS1_AITLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TS1_AITHF"]
    #[inline(always)]
    pub fn ts1_aithf(&self) -> TS1_AITHF_R {
        TS1_AITHF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 15 - TS1_RDY"]
    #[inline(always)]
    pub fn ts1_rdy(&self) -> TS1_RDY_R {
        TS1_RDY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Temperature sensor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts_sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTS_SRrs;
impl crate::RegisterSpec for DTS_SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dts_sr::R`](R) reader structure"]
impl crate::Readable for DTS_SRrs {}
#[doc = "`reset()` method sets DTS_SR to value 0"]
impl crate::Resettable for DTS_SRrs {
    const RESET_VALUE: u32 = 0;
}
