#[doc = "Register `DDRPHYC_ZQ0SR1` reader"]
pub type R = crate::R<DDRPHYC_ZQ0SR1rs>;
#[doc = "Field `ZPD` reader - ZPD"]
pub type ZPD_R = crate::FieldReader;
#[doc = "Field `ZPU` reader - ZPU"]
pub type ZPU_R = crate::FieldReader;
#[doc = "Field `OPD` reader - OPD"]
pub type OPD_R = crate::FieldReader;
#[doc = "Field `OPU` reader - OPU"]
pub type OPU_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - ZPD"]
    #[inline(always)]
    pub fn zpd(&self) -> ZPD_R {
        ZPD_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - ZPU"]
    #[inline(always)]
    pub fn zpu(&self) -> ZPU_R {
        ZPU_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - OPD"]
    #[inline(always)]
    pub fn opd(&self) -> OPD_R {
        OPD_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - OPU"]
    #[inline(always)]
    pub fn opu(&self) -> OPU_R {
        OPU_R::new((self.bits >> 6) & 3)
    }
}
#[doc = "DDRPHYC ZQ0S register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_zq0sr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_ZQ0SR1rs;
impl crate::RegisterSpec for DDRPHYC_ZQ0SR1rs {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ddrphyc_zq0sr1::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0SR1rs {}
#[doc = "`reset()` method sets DDRPHYC_ZQ0SR1 to value 0"]
impl crate::Resettable for DDRPHYC_ZQ0SR1rs {
    const RESET_VALUE: u8 = 0;
}
