///Register `PWR_MPUCR` reader
pub type R = crate::R<PWR_MPUCRrs>;
///Register `PWR_MPUCR` writer
pub type W = crate::W<PWR_MPUCRrs>;
///Field `PDDS` reader - PDDS
pub type PDDS_R = crate::BitReader;
///Field `PDDS` writer - PDDS
pub type PDDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSTBYDIS` reader - CSTBYDIS
pub type CSTBYDIS_R = crate::BitReader;
///Field `CSTBYDIS` writer - CSTBYDIS
pub type CSTBYDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOPF` reader - STOPF
pub type STOPF_R = crate::BitReader;
///Field `SBF` reader - SBF
pub type SBF_R = crate::BitReader;
///Field `SBFMPU` reader - SBFMPU
pub type SBFMPU_R = crate::BitReader;
///Field `CSSF` reader - CSSF
pub type CSSF_R = crate::BitReader;
///Field `CSSF` writer - CSSF
pub type CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STANDBYWFIL2` reader - STANDBYWFIL2
pub type STANDBYWFIL2_R = crate::BitReader;
impl R {
    ///Bit 0 - PDDS
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - CSTBYDIS
    #[inline(always)]
    pub fn cstbydis(&self) -> CSTBYDIS_R {
        CSTBYDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - STOPF
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SBF
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SBFMPU
    #[inline(always)]
    pub fn sbfmpu(&self) -> SBFMPU_R {
        SBFMPU_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - CSSF
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 15 - STANDBYWFIL2
    #[inline(always)]
    pub fn standbywfil2(&self) -> STANDBYWFIL2_R {
        STANDBYWFIL2_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_MPUCR")
            .field("pdds", &self.pdds())
            .field("cstbydis", &self.cstbydis())
            .field("stopf", &self.stopf())
            .field("sbf", &self.sbf())
            .field("sbfmpu", &self.sbfmpu())
            .field("cssf", &self.cssf())
            .field("standbywfil2", &self.standbywfil2())
            .finish()
    }
}
impl W {
    ///Bit 0 - PDDS
    #[inline(always)]
    #[must_use]
    pub fn pdds(&mut self) -> PDDS_W<PWR_MPUCRrs> {
        PDDS_W::new(self, 0)
    }
    ///Bit 3 - CSTBYDIS
    #[inline(always)]
    #[must_use]
    pub fn cstbydis(&mut self) -> CSTBYDIS_W<PWR_MPUCRrs> {
        CSTBYDIS_W::new(self, 3)
    }
    ///Bit 9 - CSSF
    #[inline(always)]
    #[must_use]
    pub fn cssf(&mut self) -> CSSF_W<PWR_MPUCRrs> {
        CSSF_W::new(self, 9)
    }
}
/**See individual bits for reset condition. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.

You can [`read`](crate::Reg::read) this register and get [`pwr_mpucr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_mpucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#PWR:PWR_MPUCR)*/
pub struct PWR_MPUCRrs;
impl crate::RegisterSpec for PWR_MPUCRrs {
    type Ux = u32;
}
///`read()` method returns [`pwr_mpucr::R`](R) reader structure
impl crate::Readable for PWR_MPUCRrs {}
///`write(|w| ..)` method takes [`pwr_mpucr::W`](W) writer structure
impl crate::Writable for PWR_MPUCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWR_MPUCR to value 0
impl crate::Resettable for PWR_MPUCRrs {
    const RESET_VALUE: u32 = 0;
}
