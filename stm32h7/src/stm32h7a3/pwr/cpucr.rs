///Register `CPUCR` reader
pub type R = crate::R<CPUCRrs>;
///Register `CPUCR` writer
pub type W = crate::W<CPUCRrs>;
///Field `RETDS_CD` reader - RETDS_CD
pub type RETDS_CD_R = crate::BitReader;
///Field `RETDS_CD` writer - RETDS_CD
pub type RETDS_CD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDDS_SRD` reader - PDDS_SRD
pub type PDDS_SRD_R = crate::BitReader;
///Field `PDDS_SRD` writer - PDDS_SRD
pub type PDDS_SRD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOPF` reader - STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU1 CSSF bit.
pub type STOPF_R = crate::BitReader;
///Field `SBF` reader - System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU1 CSSF bit
pub type SBF_R = crate::BitReader;
///Field `CSSF` reader - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware.
pub type CSSF_R = crate::BitReader;
///Field `CSSF` writer - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware.
pub type CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RUN_SRD` reader - RUN_SRD
pub type RUN_SRD_R = crate::BitReader;
///Field `RUN_SRD` writer - RUN_SRD
pub type RUN_SRD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RETDS_CD
    #[inline(always)]
    pub fn retds_cd(&self) -> RETDS_CD_R {
        RETDS_CD_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - PDDS_SRD
    #[inline(always)]
    pub fn pdds_srd(&self) -> PDDS_SRD_R {
        PDDS_SRD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU1 CSSF bit.
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU1 CSSF bit
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware.
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - RUN_SRD
    #[inline(always)]
    pub fn run_srd(&self) -> RUN_SRD_R {
        RUN_SRD_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUCR")
            .field("retds_cd", &self.retds_cd())
            .field("pdds_srd", &self.pdds_srd())
            .field("stopf", &self.stopf())
            .field("sbf", &self.sbf())
            .field("cssf", &self.cssf())
            .field("run_srd", &self.run_srd())
            .finish()
    }
}
impl W {
    ///Bit 0 - RETDS_CD
    #[inline(always)]
    pub fn retds_cd(&mut self) -> RETDS_CD_W<'_, CPUCRrs> {
        RETDS_CD_W::new(self, 0)
    }
    ///Bit 2 - PDDS_SRD
    #[inline(always)]
    pub fn pdds_srd(&mut self) -> PDDS_SRD_W<'_, CPUCRrs> {
        PDDS_SRD_W::new(self, 2)
    }
    ///Bit 9 - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware.
    #[inline(always)]
    pub fn cssf(&mut self) -> CSSF_W<'_, CPUCRrs> {
        CSSF_W::new(self, 9)
    }
    ///Bit 11 - RUN_SRD
    #[inline(always)]
    pub fn run_srd(&mut self) -> RUN_SRD_W<'_, CPUCRrs> {
        RUN_SRD_W::new(self, 11)
    }
}
/**This register allows controlling CPU1 power.

You can [`read`](crate::Reg::read) this register and get [`cpucr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#PWR:CPUCR)*/
pub struct CPUCRrs;
impl crate::RegisterSpec for CPUCRrs {
    type Ux = u32;
}
///`read()` method returns [`cpucr::R`](R) reader structure
impl crate::Readable for CPUCRrs {}
///`write(|w| ..)` method takes [`cpucr::W`](W) writer structure
impl crate::Writable for CPUCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CPUCR to value 0
impl crate::Resettable for CPUCRrs {}
