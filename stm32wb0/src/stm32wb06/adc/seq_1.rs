///Register `SEQ_1` reader
pub type R = crate::R<SEQ_1rs>;
///Register `SEQ_1` writer
pub type W = crate::W<SEQ_1rs>;
///Field `SEQ0` reader - channel number code for first conversion of the sequence
pub type SEQ0_R = crate::FieldReader;
///Field `SEQ0` writer - channel number code for first conversion of the sequence
pub type SEQ0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ1` reader - channel number code for second conversion of the sequence.
pub type SEQ1_R = crate::FieldReader;
///Field `SEQ1` writer - channel number code for second conversion of the sequence.
pub type SEQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ2` reader - channel number code for 3rd conversion of the sequence.
pub type SEQ2_R = crate::FieldReader;
///Field `SEQ2` writer - channel number code for 3rd conversion of the sequence.
pub type SEQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ3` reader - channel number code for 4th conversion of the sequence.
pub type SEQ3_R = crate::FieldReader;
///Field `SEQ3` writer - channel number code for 4th conversion of the sequence.
pub type SEQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ4` reader - channel number code for 5th conversion of the sequence.
pub type SEQ4_R = crate::FieldReader;
///Field `SEQ4` writer - channel number code for 5th conversion of the sequence.
pub type SEQ4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ5` reader - channel number code for 6th conversion of the sequence.
pub type SEQ5_R = crate::FieldReader;
///Field `SEQ5` writer - channel number code for 6th conversion of the sequence.
pub type SEQ5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ6` reader - channel number code for 7th conversion of the sequence.
pub type SEQ6_R = crate::FieldReader;
///Field `SEQ6` writer - channel number code for 7th conversion of the sequence.
pub type SEQ6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ7` reader - channel number code for 8th conversion of the sequence.
pub type SEQ7_R = crate::FieldReader;
///Field `SEQ7` writer - channel number code for 8th conversion of the sequence.
pub type SEQ7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - channel number code for first conversion of the sequence
    #[inline(always)]
    pub fn seq0(&self) -> SEQ0_R {
        SEQ0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - channel number code for second conversion of the sequence.
    #[inline(always)]
    pub fn seq1(&self) -> SEQ1_R {
        SEQ1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - channel number code for 3rd conversion of the sequence.
    #[inline(always)]
    pub fn seq2(&self) -> SEQ2_R {
        SEQ2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - channel number code for 4th conversion of the sequence.
    #[inline(always)]
    pub fn seq3(&self) -> SEQ3_R {
        SEQ3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - channel number code for 5th conversion of the sequence.
    #[inline(always)]
    pub fn seq4(&self) -> SEQ4_R {
        SEQ4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - channel number code for 6th conversion of the sequence.
    #[inline(always)]
    pub fn seq5(&self) -> SEQ5_R {
        SEQ5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - channel number code for 7th conversion of the sequence.
    #[inline(always)]
    pub fn seq6(&self) -> SEQ6_R {
        SEQ6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - channel number code for 8th conversion of the sequence.
    #[inline(always)]
    pub fn seq7(&self) -> SEQ7_R {
        SEQ7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEQ_1")
            .field("seq7", &self.seq7())
            .field("seq6", &self.seq6())
            .field("seq5", &self.seq5())
            .field("seq4", &self.seq4())
            .field("seq3", &self.seq3())
            .field("seq2", &self.seq2())
            .field("seq1", &self.seq1())
            .field("seq0", &self.seq0())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - channel number code for first conversion of the sequence
    #[inline(always)]
    pub fn seq0(&mut self) -> SEQ0_W<'_, SEQ_1rs> {
        SEQ0_W::new(self, 0)
    }
    ///Bits 4:7 - channel number code for second conversion of the sequence.
    #[inline(always)]
    pub fn seq1(&mut self) -> SEQ1_W<'_, SEQ_1rs> {
        SEQ1_W::new(self, 4)
    }
    ///Bits 8:11 - channel number code for 3rd conversion of the sequence.
    #[inline(always)]
    pub fn seq2(&mut self) -> SEQ2_W<'_, SEQ_1rs> {
        SEQ2_W::new(self, 8)
    }
    ///Bits 12:15 - channel number code for 4th conversion of the sequence.
    #[inline(always)]
    pub fn seq3(&mut self) -> SEQ3_W<'_, SEQ_1rs> {
        SEQ3_W::new(self, 12)
    }
    ///Bits 16:19 - channel number code for 5th conversion of the sequence.
    #[inline(always)]
    pub fn seq4(&mut self) -> SEQ4_W<'_, SEQ_1rs> {
        SEQ4_W::new(self, 16)
    }
    ///Bits 20:23 - channel number code for 6th conversion of the sequence.
    #[inline(always)]
    pub fn seq5(&mut self) -> SEQ5_W<'_, SEQ_1rs> {
        SEQ5_W::new(self, 20)
    }
    ///Bits 24:27 - channel number code for 7th conversion of the sequence.
    #[inline(always)]
    pub fn seq6(&mut self) -> SEQ6_W<'_, SEQ_1rs> {
        SEQ6_W::new(self, 24)
    }
    ///Bits 28:31 - channel number code for 8th conversion of the sequence.
    #[inline(always)]
    pub fn seq7(&mut self) -> SEQ7_W<'_, SEQ_1rs> {
        SEQ7_W::new(self, 28)
    }
}
/**ADC regular sequence configuration register 1

You can [`read`](crate::Reg::read) this register and get [`seq_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#ADC:SEQ_1)*/
pub struct SEQ_1rs;
impl crate::RegisterSpec for SEQ_1rs {
    type Ux = u32;
}
///`read()` method returns [`seq_1::R`](R) reader structure
impl crate::Readable for SEQ_1rs {}
///`write(|w| ..)` method takes [`seq_1::W`](W) writer structure
impl crate::Writable for SEQ_1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SEQ_1 to value 0
impl crate::Resettable for SEQ_1rs {}
