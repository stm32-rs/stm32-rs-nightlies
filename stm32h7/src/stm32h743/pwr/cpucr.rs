#[doc = "Register `CPUCR` reader"]
pub type R = crate::R<CPUCRrs>;
#[doc = "Register `CPUCR` writer"]
pub type W = crate::W<CPUCRrs>;
#[doc = "Field `PDDS_D1` reader - D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain."]
pub type PDDS_D1_R = crate::BitReader;
#[doc = "Field `PDDS_D1` writer - D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain."]
pub type PDDS_D1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDDS_D2` reader - D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain."]
pub type PDDS_D2_R = crate::BitReader;
#[doc = "Field `PDDS_D2` writer - D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain."]
pub type PDDS_D2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDDS_D3` reader - System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain."]
pub type PDDS_D3_R = crate::BitReader;
#[doc = "Field `PDDS_D3` writer - System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain."]
pub type PDDS_D3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPF` reader - STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU1 CSSF bit."]
pub type STOPF_R = crate::BitReader;
#[doc = "Field `SBF` reader - System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU1 CSSF bit"]
pub type SBF_R = crate::BitReader;
#[doc = "Field `SBF_D1` reader - D1 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D1 domain is no longer in DStandby mode."]
pub type SBF_D1_R = crate::BitReader;
#[doc = "Field `SBF_D2` reader - D2 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D2 domain is no longer in DStandby mode."]
pub type SBF_D2_R = crate::BitReader;
#[doc = "Field `CSSF` reader - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
pub type CSSF_R = crate::BitReader;
#[doc = "Field `CSSF` writer - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
pub type CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUN_D3` reader - Keep system D3 domain in Run mode regardless of the CPU sub-systems modes"]
pub type RUN_D3_R = crate::BitReader;
#[doc = "Field `RUN_D3` writer - Keep system D3 domain in Run mode regardless of the CPU sub-systems modes"]
pub type RUN_D3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain."]
    #[inline(always)]
    pub fn pdds_d1(&self) -> PDDS_D1_R {
        PDDS_D1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain."]
    #[inline(always)]
    pub fn pdds_d2(&self) -> PDDS_D2_R {
        PDDS_D2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain."]
    #[inline(always)]
    pub fn pdds_d3(&self) -> PDDS_D3_R {
        PDDS_D3_R::new(((self.bits >> 2) & 1) != 0)
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
    #[doc = "Bit 7 - D1 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D1 domain is no longer in DStandby mode."]
    #[inline(always)]
    pub fn sbf_d1(&self) -> SBF_D1_R {
        SBF_D1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - D2 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D2 domain is no longer in DStandby mode."]
    #[inline(always)]
    pub fn sbf_d2(&self) -> SBF_D2_R {
        SBF_D2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Keep system D3 domain in Run mode regardless of the CPU sub-systems modes"]
    #[inline(always)]
    pub fn run_d3(&self) -> RUN_D3_R {
        RUN_D3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain."]
    #[inline(always)]
    #[must_use]
    pub fn pdds_d1(&mut self) -> PDDS_D1_W<CPUCRrs> {
        PDDS_D1_W::new(self, 0)
    }
    #[doc = "Bit 1 - D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain."]
    #[inline(always)]
    #[must_use]
    pub fn pdds_d2(&mut self) -> PDDS_D2_W<CPUCRrs> {
        PDDS_D2_W::new(self, 1)
    }
    #[doc = "Bit 2 - System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain."]
    #[inline(always)]
    #[must_use]
    pub fn pdds_d3(&mut self) -> PDDS_D3_W<CPUCRrs> {
        PDDS_D3_W::new(self, 2)
    }
    #[doc = "Bit 9 - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn cssf(&mut self) -> CSSF_W<CPUCRrs> {
        CSSF_W::new(self, 9)
    }
    #[doc = "Bit 11 - Keep system D3 domain in Run mode regardless of the CPU sub-systems modes"]
    #[inline(always)]
    #[must_use]
    pub fn run_d3(&mut self) -> RUN_D3_W<CPUCRrs> {
        RUN_D3_W::new(self, 11)
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
