///Register `DDR3_MR2` reader
pub type R = crate::R<DDR3_MR2rs>;
///Register `DDR3_MR2` writer
pub type W = crate::W<DDR3_MR2rs>;
///Field `PASR` reader - PASR
pub type PASR_R = crate::FieldReader;
///Field `PASR` writer - PASR
pub type PASR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CWL` reader - CWL
pub type CWL_R = crate::FieldReader;
///Field `CWL` writer - CWL
pub type CWL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ASR` reader - ASR
pub type ASR_R = crate::BitReader;
///Field `ASR` writer - ASR
pub type ASR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRT` reader - SRT
pub type SRT_R = crate::BitReader;
///Field `SRT` writer - SRT
pub type SRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTTWR` reader - RTTWR
pub type RTTWR_R = crate::FieldReader;
///Field `RTTWR` writer - RTTWR
pub type RTTWR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - PASR
    #[inline(always)]
    pub fn pasr(&self) -> PASR_R {
        PASR_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - CWL
    #[inline(always)]
    pub fn cwl(&self) -> CWL_R {
        CWL_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bit 6 - ASR
    #[inline(always)]
    pub fn asr(&self) -> ASR_R {
        ASR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SRT
    #[inline(always)]
    pub fn srt(&self) -> SRT_R {
        SRT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 9:10 - RTTWR
    #[inline(always)]
    pub fn rttwr(&self) -> RTTWR_R {
        RTTWR_R::new(((self.bits >> 9) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDR3_MR2")
            .field("pasr", &self.pasr())
            .field("cwl", &self.cwl())
            .field("asr", &self.asr())
            .field("srt", &self.srt())
            .field("rttwr", &self.rttwr())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - PASR
    #[inline(always)]
    pub fn pasr(&mut self) -> PASR_W<'_, DDR3_MR2rs> {
        PASR_W::new(self, 0)
    }
    ///Bits 3:5 - CWL
    #[inline(always)]
    pub fn cwl(&mut self) -> CWL_W<'_, DDR3_MR2rs> {
        CWL_W::new(self, 3)
    }
    ///Bit 6 - ASR
    #[inline(always)]
    pub fn asr(&mut self) -> ASR_W<'_, DDR3_MR2rs> {
        ASR_W::new(self, 6)
    }
    ///Bit 7 - SRT
    #[inline(always)]
    pub fn srt(&mut self) -> SRT_W<'_, DDR3_MR2rs> {
        SRT_W::new(self, 7)
    }
    ///Bits 9:10 - RTTWR
    #[inline(always)]
    pub fn rttwr(&mut self) -> RTTWR_W<'_, DDR3_MR2rs> {
        RTTWR_W::new(self, 9)
    }
}
/**DDRPHYC MR2 register for DDR3

You can [`read`](crate::Reg::read) this register and get [`ddr3_mr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddr3_mr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDR3_MR2)*/
pub struct DDR3_MR2rs;
impl crate::RegisterSpec for DDR3_MR2rs {
    type Ux = u16;
}
///`read()` method returns [`ddr3_mr2::R`](R) reader structure
impl crate::Readable for DDR3_MR2rs {}
///`write(|w| ..)` method takes [`ddr3_mr2::W`](W) writer structure
impl crate::Writable for DDR3_MR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DDR3_MR2 to value 0
impl crate::Resettable for DDR3_MR2rs {}
