///Register `CSQCFGR2` reader
pub type R = crate::R<CSQCFGR2rs>;
///Register `CSQCFGR2` writer
pub type W = crate::W<CSQCFGR2rs>;
///Field `SQSDTEN` reader - SQSDTEN
pub type SQSDTEN_R = crate::BitReader;
///Field `SQSDTEN` writer - SQSDTEN
pub type SQSDTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCMD2EN` reader - RCMD2EN
pub type RCMD2EN_R = crate::BitReader;
///Field `RCMD2EN` writer - RCMD2EN
pub type RCMD2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMASEN` reader - DMASEN
pub type DMASEN_R = crate::BitReader;
///Field `DMASEN` writer - DMASEN
pub type DMASEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCMD1` reader - RCMD1
pub type RCMD1_R = crate::FieldReader;
///Field `RCMD1` writer - RCMD1
pub type RCMD1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RCMD2` reader - RCMD2
pub type RCMD2_R = crate::FieldReader;
///Field `RCMD2` writer - RCMD2
pub type RCMD2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RCMD1T` reader - RCMD1T
pub type RCMD1T_R = crate::BitReader;
///Field `RCMD1T` writer - RCMD1T
pub type RCMD1T_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCMD2T` reader - RCMD2T
pub type RCMD2T_R = crate::BitReader;
///Field `RCMD2T` writer - RCMD2T
pub type RCMD2T_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SQSDTEN
    #[inline(always)]
    pub fn sqsdten(&self) -> SQSDTEN_R {
        SQSDTEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RCMD2EN
    #[inline(always)]
    pub fn rcmd2en(&self) -> RCMD2EN_R {
        RCMD2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMASEN
    #[inline(always)]
    pub fn dmasen(&self) -> DMASEN_R {
        DMASEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 8:15 - RCMD1
    #[inline(always)]
    pub fn rcmd1(&self) -> RCMD1_R {
        RCMD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - RCMD2
    #[inline(always)]
    pub fn rcmd2(&self) -> RCMD2_R {
        RCMD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - RCMD1T
    #[inline(always)]
    pub fn rcmd1t(&self) -> RCMD1T_R {
        RCMD1T_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - RCMD2T
    #[inline(always)]
    pub fn rcmd2t(&self) -> RCMD2T_R {
        RCMD2T_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSQCFGR2")
            .field("sqsdten", &self.sqsdten())
            .field("rcmd2en", &self.rcmd2en())
            .field("dmasen", &self.dmasen())
            .field("rcmd1", &self.rcmd1())
            .field("rcmd2", &self.rcmd2())
            .field("rcmd1t", &self.rcmd1t())
            .field("rcmd2t", &self.rcmd2t())
            .finish()
    }
}
impl W {
    ///Bit 0 - SQSDTEN
    #[inline(always)]
    pub fn sqsdten(&mut self) -> SQSDTEN_W<'_, CSQCFGR2rs> {
        SQSDTEN_W::new(self, 0)
    }
    ///Bit 1 - RCMD2EN
    #[inline(always)]
    pub fn rcmd2en(&mut self) -> RCMD2EN_W<'_, CSQCFGR2rs> {
        RCMD2EN_W::new(self, 1)
    }
    ///Bit 2 - DMASEN
    #[inline(always)]
    pub fn dmasen(&mut self) -> DMASEN_W<'_, CSQCFGR2rs> {
        DMASEN_W::new(self, 2)
    }
    ///Bits 8:15 - RCMD1
    #[inline(always)]
    pub fn rcmd1(&mut self) -> RCMD1_W<'_, CSQCFGR2rs> {
        RCMD1_W::new(self, 8)
    }
    ///Bits 16:23 - RCMD2
    #[inline(always)]
    pub fn rcmd2(&mut self) -> RCMD2_W<'_, CSQCFGR2rs> {
        RCMD2_W::new(self, 16)
    }
    ///Bit 24 - RCMD1T
    #[inline(always)]
    pub fn rcmd1t(&mut self) -> RCMD1T_W<'_, CSQCFGR2rs> {
        RCMD1T_W::new(self, 24)
    }
    ///Bit 25 - RCMD2T
    #[inline(always)]
    pub fn rcmd2t(&mut self) -> RCMD2T_W<'_, CSQCFGR2rs> {
        RCMD2T_W::new(self, 25)
    }
}
/**This register is used to configure the command sequencer to issue random read/ write commands to read/ write data by sector and automatically read/write data from NAND Flash memory at a programmable address offset. This is useful when performing a sector read/write operation followed by an ECC read/write operation in the NAND Flash spare area.The command sequencer generates the random commands untill all the sectors are read/written. .

You can [`read`](crate::Reg::read) this register and get [`csqcfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqcfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:CSQCFGR2)*/
pub struct CSQCFGR2rs;
impl crate::RegisterSpec for CSQCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`csqcfgr2::R`](R) reader structure
impl crate::Readable for CSQCFGR2rs {}
///`write(|w| ..)` method takes [`csqcfgr2::W`](W) writer structure
impl crate::Writable for CSQCFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSQCFGR2 to value 0
impl crate::Resettable for CSQCFGR2rs {}
