#[doc = "Register `FMC_CSQCFGR2` reader"]
pub type R = crate::R<FMC_CSQCFGR2rs>;
#[doc = "Register `FMC_CSQCFGR2` writer"]
pub type W = crate::W<FMC_CSQCFGR2rs>;
#[doc = "Field `SQSDTEN` reader - SQSDTEN"]
pub type SQSDTEN_R = crate::BitReader;
#[doc = "Field `SQSDTEN` writer - SQSDTEN"]
pub type SQSDTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCMD2EN` reader - RCMD2EN"]
pub type RCMD2EN_R = crate::BitReader;
#[doc = "Field `RCMD2EN` writer - RCMD2EN"]
pub type RCMD2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEN` reader - DMASEN"]
pub type DMASEN_R = crate::BitReader;
#[doc = "Field `DMASEN` writer - DMASEN"]
pub type DMASEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCMD1` reader - RCMD1"]
pub type RCMD1_R = crate::FieldReader;
#[doc = "Field `RCMD1` writer - RCMD1"]
pub type RCMD1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RCMD2` reader - RCMD2"]
pub type RCMD2_R = crate::FieldReader;
#[doc = "Field `RCMD2` writer - RCMD2"]
pub type RCMD2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RCMD1T` reader - RCMD1T"]
pub type RCMD1T_R = crate::BitReader;
#[doc = "Field `RCMD1T` writer - RCMD1T"]
pub type RCMD1T_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCMD2T` reader - RCMD2T"]
pub type RCMD2T_R = crate::BitReader;
#[doc = "Field `RCMD2T` writer - RCMD2T"]
pub type RCMD2T_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SQSDTEN"]
    #[inline(always)]
    pub fn sqsdten(&self) -> SQSDTEN_R {
        SQSDTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RCMD2EN"]
    #[inline(always)]
    pub fn rcmd2en(&self) -> RCMD2EN_R {
        RCMD2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMASEN"]
    #[inline(always)]
    pub fn dmasen(&self) -> DMASEN_R {
        DMASEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - RCMD1"]
    #[inline(always)]
    pub fn rcmd1(&self) -> RCMD1_R {
        RCMD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RCMD2"]
    #[inline(always)]
    pub fn rcmd2(&self) -> RCMD2_R {
        RCMD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - RCMD1T"]
    #[inline(always)]
    pub fn rcmd1t(&self) -> RCMD1T_R {
        RCMD1T_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RCMD2T"]
    #[inline(always)]
    pub fn rcmd2t(&self) -> RCMD2T_R {
        RCMD2T_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SQSDTEN"]
    #[inline(always)]
    #[must_use]
    pub fn sqsdten(&mut self) -> SQSDTEN_W<FMC_CSQCFGR2rs> {
        SQSDTEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - RCMD2EN"]
    #[inline(always)]
    #[must_use]
    pub fn rcmd2en(&mut self) -> RCMD2EN_W<FMC_CSQCFGR2rs> {
        RCMD2EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMASEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmasen(&mut self) -> DMASEN_W<FMC_CSQCFGR2rs> {
        DMASEN_W::new(self, 2)
    }
    #[doc = "Bits 8:15 - RCMD1"]
    #[inline(always)]
    #[must_use]
    pub fn rcmd1(&mut self) -> RCMD1_W<FMC_CSQCFGR2rs> {
        RCMD1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - RCMD2"]
    #[inline(always)]
    #[must_use]
    pub fn rcmd2(&mut self) -> RCMD2_W<FMC_CSQCFGR2rs> {
        RCMD2_W::new(self, 16)
    }
    #[doc = "Bit 24 - RCMD1T"]
    #[inline(always)]
    #[must_use]
    pub fn rcmd1t(&mut self) -> RCMD1T_W<FMC_CSQCFGR2rs> {
        RCMD1T_W::new(self, 24)
    }
    #[doc = "Bit 25 - RCMD2T"]
    #[inline(always)]
    #[must_use]
    pub fn rcmd2t(&mut self) -> RCMD2T_W<FMC_CSQCFGR2rs> {
        RCMD2T_W::new(self, 25)
    }
}
#[doc = "This register is used to configure the command sequencer to issue random read/ write commands to read/ write data by sector and automatically read/write data from NAND Flash memory at a programmable address offset. This is useful when performing a sector read/write operation followed by an ECC read/write operation in the NAND Flash spare area.The command sequencer generates the random commands untill all the sectors are read/written. .\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_csqcfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_csqcfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_CSQCFGR2rs;
impl crate::RegisterSpec for FMC_CSQCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_csqcfgr2::R`](R) reader structure"]
impl crate::Readable for FMC_CSQCFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`fmc_csqcfgr2::W`](W) writer structure"]
impl crate::Writable for FMC_CSQCFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMC_CSQCFGR2 to value 0"]
impl crate::Resettable for FMC_CSQCFGR2rs {
    const RESET_VALUE: u32 = 0;
}
