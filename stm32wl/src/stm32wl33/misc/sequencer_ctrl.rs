///Register `SEQUENCER_CTRL` reader
pub type R = crate::R<SEQUENCER_CTRLrs>;
///Register `SEQUENCER_CTRL` writer
pub type W = crate::W<SEQUENCER_CTRLrs>;
///Field `GEN_SEQ_TRIGGER` writer - Action bit: write 1 to generate a trigger event on Sequencer.
pub type GEN_SEQ_TRIGGER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISABLE_SEQ` reader - Enable/disable the Sequencer
pub type DISABLE_SEQ_R = crate::BitReader;
///Field `DISABLE_SEQ` writer - Enable/disable the Sequencer
pub type DISABLE_SEQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Enable/disable the Sequencer
    #[inline(always)]
    pub fn disable_seq(&self) -> DISABLE_SEQ_R {
        DISABLE_SEQ_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEQUENCER_CTRL")
            .field("disable_seq", &self.disable_seq())
            .finish()
    }
}
impl W {
    ///Bit 0 - Action bit: write 1 to generate a trigger event on Sequencer.
    #[inline(always)]
    pub fn gen_seq_trigger(&mut self) -> GEN_SEQ_TRIGGER_W<'_, SEQUENCER_CTRLrs> {
        GEN_SEQ_TRIGGER_W::new(self, 0)
    }
    ///Bit 1 - Enable/disable the Sequencer
    #[inline(always)]
    pub fn disable_seq(&mut self) -> DISABLE_SEQ_W<'_, SEQUENCER_CTRLrs> {
        DISABLE_SEQ_W::new(self, 1)
    }
}
/**SEQUENCER_CTRL register

You can [`read`](crate::Reg::read) this register and get [`sequencer_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sequencer_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MISC:SEQUENCER_CTRL)*/
pub struct SEQUENCER_CTRLrs;
impl crate::RegisterSpec for SEQUENCER_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`sequencer_ctrl::R`](R) reader structure
impl crate::Readable for SEQUENCER_CTRLrs {}
///`write(|w| ..)` method takes [`sequencer_ctrl::W`](W) writer structure
impl crate::Writable for SEQUENCER_CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SEQUENCER_CTRL to value 0
impl crate::Resettable for SEQUENCER_CTRLrs {}
