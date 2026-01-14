///Register `CSQCFGR2` reader
pub type R = crate::R<CSQCFGR2rs>;
///Register `CSQCFGR2` writer
pub type W = crate::W<CSQCFGR2rs>;
///Field `SQSDTEN` reader - Sequencer spare data transfer enable
pub type SQSDTEN_R = crate::BitReader;
///Field `SQSDTEN` writer - Sequencer spare data transfer enable
pub type SQSDTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCMD2EN` reader - Random Command 2 sequencer enable
pub type RCMD2EN_R = crate::BitReader;
///Field `RCMD2EN` writer - Random Command 2 sequencer enable
pub type RCMD2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMASEN` reader - Command sequencer DMA request decoding status enable
pub type DMASEN_R = crate::BitReader;
///Field `DMASEN` writer - Command sequencer DMA request decoding status enable
pub type DMASEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCMD1` reader - Random Command 1 sequencer
pub type RCMD1_R = crate::FieldReader;
///Field `RCMD1` writer - Random Command 1 sequencer
pub type RCMD1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RCMD2` reader - Random Command 2 sequencer
pub type RCMD2_R = crate::FieldReader;
///Field `RCMD2` writer - Random Command 2 sequencer
pub type RCMD2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RCMD1T` reader - Command 1 sequencer timings
pub type RCMD1T_R = crate::BitReader;
///Field `RCMD1T` writer - Command 1 sequencer timings
pub type RCMD1T_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCMD2T` reader - Command 1 sequencer timings
pub type RCMD2T_R = crate::BitReader;
///Field `RCMD2T` writer - Command 1 sequencer timings
pub type RCMD2T_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Sequencer spare data transfer enable
    #[inline(always)]
    pub fn sqsdten(&self) -> SQSDTEN_R {
        SQSDTEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Random Command 2 sequencer enable
    #[inline(always)]
    pub fn rcmd2en(&self) -> RCMD2EN_R {
        RCMD2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Command sequencer DMA request decoding status enable
    #[inline(always)]
    pub fn dmasen(&self) -> DMASEN_R {
        DMASEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 8:15 - Random Command 1 sequencer
    #[inline(always)]
    pub fn rcmd1(&self) -> RCMD1_R {
        RCMD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Random Command 2 sequencer
    #[inline(always)]
    pub fn rcmd2(&self) -> RCMD2_R {
        RCMD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - Command 1 sequencer timings
    #[inline(always)]
    pub fn rcmd1t(&self) -> RCMD1T_R {
        RCMD1T_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Command 1 sequencer timings
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
    ///Bit 0 - Sequencer spare data transfer enable
    #[inline(always)]
    pub fn sqsdten(&mut self) -> SQSDTEN_W<'_, CSQCFGR2rs> {
        SQSDTEN_W::new(self, 0)
    }
    ///Bit 1 - Random Command 2 sequencer enable
    #[inline(always)]
    pub fn rcmd2en(&mut self) -> RCMD2EN_W<'_, CSQCFGR2rs> {
        RCMD2EN_W::new(self, 1)
    }
    ///Bit 2 - Command sequencer DMA request decoding status enable
    #[inline(always)]
    pub fn dmasen(&mut self) -> DMASEN_W<'_, CSQCFGR2rs> {
        DMASEN_W::new(self, 2)
    }
    ///Bits 8:15 - Random Command 1 sequencer
    #[inline(always)]
    pub fn rcmd1(&mut self) -> RCMD1_W<'_, CSQCFGR2rs> {
        RCMD1_W::new(self, 8)
    }
    ///Bits 16:23 - Random Command 2 sequencer
    #[inline(always)]
    pub fn rcmd2(&mut self) -> RCMD2_W<'_, CSQCFGR2rs> {
        RCMD2_W::new(self, 16)
    }
    ///Bit 24 - Command 1 sequencer timings
    #[inline(always)]
    pub fn rcmd1t(&mut self) -> RCMD1T_W<'_, CSQCFGR2rs> {
        RCMD1T_W::new(self, 24)
    }
    ///Bit 25 - Command 1 sequencer timings
    #[inline(always)]
    pub fn rcmd2t(&mut self) -> RCMD2T_W<'_, CSQCFGR2rs> {
        RCMD2T_W::new(self, 25)
    }
}
/**FMC NAND command sequencer configuration register 2

You can [`read`](crate::Reg::read) this register and get [`csqcfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqcfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#FMC1:CSQCFGR2)*/
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
