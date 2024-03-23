#[doc = "Register `DDRPHYC_DDR3_MR2` reader"]
pub type R = crate::R<DDRPHYC_DDR3_MR2rs>;
#[doc = "Register `DDRPHYC_DDR3_MR2` writer"]
pub type W = crate::W<DDRPHYC_DDR3_MR2rs>;
#[doc = "Field `PASR` reader - PASR"]
pub type PASR_R = crate::FieldReader;
#[doc = "Field `PASR` writer - PASR"]
pub type PASR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CWL` reader - CWL"]
pub type CWL_R = crate::FieldReader;
#[doc = "Field `CWL` writer - CWL"]
pub type CWL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ASR` reader - ASR"]
pub type ASR_R = crate::BitReader;
#[doc = "Field `ASR` writer - ASR"]
pub type ASR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRT` reader - SRT"]
pub type SRT_R = crate::BitReader;
#[doc = "Field `SRT` writer - SRT"]
pub type SRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTTWR` reader - RTTWR"]
pub type RTTWR_R = crate::FieldReader;
#[doc = "Field `RTTWR` writer - RTTWR"]
pub type RTTWR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - PASR"]
    #[inline(always)]
    pub fn pasr(&self) -> PASR_R {
        PASR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - CWL"]
    #[inline(always)]
    pub fn cwl(&self) -> CWL_R {
        CWL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - ASR"]
    #[inline(always)]
    pub fn asr(&self) -> ASR_R {
        ASR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SRT"]
    #[inline(always)]
    pub fn srt(&self) -> SRT_R {
        SRT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 9:10 - RTTWR"]
    #[inline(always)]
    pub fn rttwr(&self) -> RTTWR_R {
        RTTWR_R::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PASR"]
    #[inline(always)]
    #[must_use]
    pub fn pasr(&mut self) -> PASR_W<DDRPHYC_DDR3_MR2rs> {
        PASR_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - CWL"]
    #[inline(always)]
    #[must_use]
    pub fn cwl(&mut self) -> CWL_W<DDRPHYC_DDR3_MR2rs> {
        CWL_W::new(self, 3)
    }
    #[doc = "Bit 6 - ASR"]
    #[inline(always)]
    #[must_use]
    pub fn asr(&mut self) -> ASR_W<DDRPHYC_DDR3_MR2rs> {
        ASR_W::new(self, 6)
    }
    #[doc = "Bit 7 - SRT"]
    #[inline(always)]
    #[must_use]
    pub fn srt(&mut self) -> SRT_W<DDRPHYC_DDR3_MR2rs> {
        SRT_W::new(self, 7)
    }
    #[doc = "Bits 9:10 - RTTWR"]
    #[inline(always)]
    #[must_use]
    pub fn rttwr(&mut self) -> RTTWR_W<DDRPHYC_DDR3_MR2rs> {
        RTTWR_W::new(self, 9)
    }
}
#[doc = "DDRPHYC MR2 register for DDR3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_ddr3_mr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_ddr3_mr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DDR3_MR2rs;
impl crate::RegisterSpec for DDRPHYC_DDR3_MR2rs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ddrphyc_ddr3_mr2::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DDR3_MR2rs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_ddr3_mr2::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_DDR3_MR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DDR3_MR2 to value 0"]
impl crate::Resettable for DDRPHYC_DDR3_MR2rs {
    const RESET_VALUE: u16 = 0;
}
