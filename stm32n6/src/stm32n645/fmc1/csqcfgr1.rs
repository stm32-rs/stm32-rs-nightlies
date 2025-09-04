///Register `CSQCFGR1` reader
pub type R = crate::R<CSQCFGR1rs>;
///Register `CSQCFGR1` writer
pub type W = crate::W<CSQCFGR1rs>;
///Field `CMD2EN` reader - Command cycle 2 Enable
pub type CMD2EN_R = crate::BitReader;
///Field `CMD2EN` writer - Command cycle 2 Enable
pub type CMD2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMADEN` reader - Command sequencer DMA request data enable
pub type DMADEN_R = crate::BitReader;
///Field `DMADEN` writer - Command sequencer DMA request data enable
pub type DMADEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACYNBR` reader - Address Cycle number
pub type ACYNBR_R = crate::FieldReader;
///Field `ACYNBR` writer - Address Cycle number
pub type ACYNBR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CMD1` reader - Command 1 sequencer
pub type CMD1_R = crate::FieldReader;
///Field `CMD1` writer - Command 1 sequencer
pub type CMD1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CMD2` reader - Command 2 sequencer
pub type CMD2_R = crate::FieldReader;
///Field `CMD2` writer - Command 2 sequencer
pub type CMD2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CMD1T` reader - Command 1 Sequencer timings
pub type CMD1T_R = crate::BitReader;
///Field `CMD1T` writer - Command 1 Sequencer timings
pub type CMD1T_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD2T` reader - Command 2 Sequencer timings
pub type CMD2T_R = crate::BitReader;
///Field `CMD2T` writer - Command 2 Sequencer timings
pub type CMD2T_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Command cycle 2 Enable
    #[inline(always)]
    pub fn cmd2en(&self) -> CMD2EN_R {
        CMD2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Command sequencer DMA request data enable
    #[inline(always)]
    pub fn dmaden(&self) -> DMADEN_R {
        DMADEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:6 - Address Cycle number
    #[inline(always)]
    pub fn acynbr(&self) -> ACYNBR_R {
        ACYNBR_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:15 - Command 1 sequencer
    #[inline(always)]
    pub fn cmd1(&self) -> CMD1_R {
        CMD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Command 2 sequencer
    #[inline(always)]
    pub fn cmd2(&self) -> CMD2_R {
        CMD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - Command 1 Sequencer timings
    #[inline(always)]
    pub fn cmd1t(&self) -> CMD1T_R {
        CMD1T_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Command 2 Sequencer timings
    #[inline(always)]
    pub fn cmd2t(&self) -> CMD2T_R {
        CMD2T_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSQCFGR1")
            .field("cmd2en", &self.cmd2en())
            .field("dmaden", &self.dmaden())
            .field("acynbr", &self.acynbr())
            .field("cmd1", &self.cmd1())
            .field("cmd2", &self.cmd2())
            .field("cmd1t", &self.cmd1t())
            .field("cmd2t", &self.cmd2t())
            .finish()
    }
}
impl W {
    ///Bit 1 - Command cycle 2 Enable
    #[inline(always)]
    pub fn cmd2en(&mut self) -> CMD2EN_W<CSQCFGR1rs> {
        CMD2EN_W::new(self, 1)
    }
    ///Bit 2 - Command sequencer DMA request data enable
    #[inline(always)]
    pub fn dmaden(&mut self) -> DMADEN_W<CSQCFGR1rs> {
        DMADEN_W::new(self, 2)
    }
    ///Bits 4:6 - Address Cycle number
    #[inline(always)]
    pub fn acynbr(&mut self) -> ACYNBR_W<CSQCFGR1rs> {
        ACYNBR_W::new(self, 4)
    }
    ///Bits 8:15 - Command 1 sequencer
    #[inline(always)]
    pub fn cmd1(&mut self) -> CMD1_W<CSQCFGR1rs> {
        CMD1_W::new(self, 8)
    }
    ///Bits 16:23 - Command 2 sequencer
    #[inline(always)]
    pub fn cmd2(&mut self) -> CMD2_W<CSQCFGR1rs> {
        CMD2_W::new(self, 16)
    }
    ///Bit 24 - Command 1 Sequencer timings
    #[inline(always)]
    pub fn cmd1t(&mut self) -> CMD1T_W<CSQCFGR1rs> {
        CMD1T_W::new(self, 24)
    }
    ///Bit 25 - Command 2 Sequencer timings
    #[inline(always)]
    pub fn cmd2t(&mut self) -> CMD2T_W<CSQCFGR1rs> {
        CMD2T_W::new(self, 25)
    }
}
/**FMC NAND command sequencer configuration register 1

You can [`read`](crate::Reg::read) this register and get [`csqcfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqcfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#FMC1:CSQCFGR1)*/
pub struct CSQCFGR1rs;
impl crate::RegisterSpec for CSQCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`csqcfgr1::R`](R) reader structure
impl crate::Readable for CSQCFGR1rs {}
///`write(|w| ..)` method takes [`csqcfgr1::W`](W) writer structure
impl crate::Writable for CSQCFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSQCFGR1 to value 0
impl crate::Resettable for CSQCFGR1rs {}
