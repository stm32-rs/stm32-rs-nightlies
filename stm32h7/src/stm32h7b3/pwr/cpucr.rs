#[doc = "Register `CPUCR` reader"]
pub type R = crate::R<CPUCRrs>;
#[doc = "Register `CPUCR` writer"]
pub type W = crate::W<CPUCRrs>;
#[doc = "Field `RETDS_CD` reader - RETDS_CD"]
pub type RETDS_CD_R = crate::BitReader;
#[doc = "Field `RETDS_CD` writer - RETDS_CD"]
pub type RETDS_CD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDDS_SRD` reader - PDDS_SRD"]
pub type PDDS_SRD_R = crate::BitReader;
#[doc = "Field `PDDS_SRD` writer - PDDS_SRD"]
pub type PDDS_SRD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPF` reader - STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU1 CSSF bit."]
pub type STOPF_R = crate::BitReader;
#[doc = "Field `SBF` reader - System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU1 CSSF bit"]
pub type SBF_R = crate::BitReader;
#[doc = "Field `CSSF` reader - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
pub type CSSF_R = crate::BitReader;
#[doc = "Field `CSSF` writer - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
pub type CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUN_SRD` reader - RUN_SRD"]
pub type RUN_SRD_R = crate::BitReader;
#[doc = "Field `RUN_SRD` writer - RUN_SRD"]
pub type RUN_SRD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RETDS_CD"]
    #[inline(always)]
    pub fn retds_cd(&self) -> RETDS_CD_R {
        RETDS_CD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PDDS_SRD"]
    #[inline(always)]
    pub fn pdds_srd(&self) -> PDDS_SRD_R {
        PDDS_SRD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU1 CSSF bit."]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU1 CSSF bit"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - RUN_SRD"]
    #[inline(always)]
    pub fn run_srd(&self) -> RUN_SRD_R {
        RUN_SRD_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RETDS_CD"]
    #[inline(always)]
    #[must_use]
    pub fn retds_cd(&mut self) -> RETDS_CD_W<CPUCRrs> {
        RETDS_CD_W::new(self, 0)
    }
    #[doc = "Bit 2 - PDDS_SRD"]
    #[inline(always)]
    #[must_use]
    pub fn pdds_srd(&mut self) -> PDDS_SRD_W<CPUCRrs> {
        PDDS_SRD_W::new(self, 2)
    }
    #[doc = "Bit 9 - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn cssf(&mut self) -> CSSF_W<CPUCRrs> {
        CSSF_W::new(self, 9)
    }
    #[doc = "Bit 11 - RUN_SRD"]
    #[inline(always)]
    #[must_use]
    pub fn run_srd(&mut self) -> RUN_SRD_W<CPUCRrs> {
        RUN_SRD_W::new(self, 11)
    }
}
#[doc = "This register allows controlling CPU1 power.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpucr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpucr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPUCRrs;
impl crate::RegisterSpec for CPUCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpucr::R`](R) reader structure"]
impl crate::Readable for CPUCRrs {}
#[doc = "`write(|w| ..)` method takes [`cpucr::W`](W) writer structure"]
impl crate::Writable for CPUCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUCR to value 0"]
impl crate::Resettable for CPUCRrs {
    const RESET_VALUE: u32 = 0;
}
