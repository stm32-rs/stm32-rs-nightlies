///Register `SEQ_1` reader
pub type R = crate::R<SEQ_1rs>;
///Register `SEQ_1` writer
pub type W = crate::W<SEQ_1rs>;
///Field `SEQ0` reader - SEQ0\[3:0\]: channel number code for first conversion of the sequence 0000: VINM\[0\] to ADC single negative input 0001: VINM\[1\] to ADC single negative input 0010: VINM\[2\] to ADC single negative input 0011: VINM\[3\] to ADC single negative input 0100: VINP\[0\] to ADC single positive input 0101: VINP\[1\] to ADC single positive input 0110: VINP\[2\] to ADC single positive input 0111: VINP\[3\] to ADC single positive input 1000: VINP\[0\]-VINM\[0\] to ADC differential input 1001: VINP\[1\]-VINM\[1\] to ADC differential input 1010: VINP\[2\]-VINM\[2\] to ADC differential input 1011: VINP\[3\]-VINM\[3\] to ADC differential input 1100: VBAT Battery level detector 1101: Temperature sensor 111x: reserved
pub type SEQ0_R = crate::FieldReader;
///Field `SEQ0` writer - SEQ0\[3:0\]: channel number code for first conversion of the sequence 0000: VINM\[0\] to ADC single negative input 0001: VINM\[1\] to ADC single negative input 0010: VINM\[2\] to ADC single negative input 0011: VINM\[3\] to ADC single negative input 0100: VINP\[0\] to ADC single positive input 0101: VINP\[1\] to ADC single positive input 0110: VINP\[2\] to ADC single positive input 0111: VINP\[3\] to ADC single positive input 1000: VINP\[0\]-VINM\[0\] to ADC differential input 1001: VINP\[1\]-VINM\[1\] to ADC differential input 1010: VINP\[2\]-VINM\[2\] to ADC differential input 1011: VINP\[3\]-VINM\[3\] to ADC differential input 1100: VBAT Battery level detector 1101: Temperature sensor 111x: reserved
pub type SEQ0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ1` reader - SEQ1\[3:0\]: channel number code for second conversion of the sequence. See SEQ0 for code detail.
pub type SEQ1_R = crate::FieldReader;
///Field `SEQ1` writer - SEQ1\[3:0\]: channel number code for second conversion of the sequence. See SEQ0 for code detail.
pub type SEQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ2` reader - SEQ2\[3:0\]: channel number code for 3rd conversion of the sequence. See SEQ0 for code detail.
pub type SEQ2_R = crate::FieldReader;
///Field `SEQ2` writer - SEQ2\[3:0\]: channel number code for 3rd conversion of the sequence. See SEQ0 for code detail.
pub type SEQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ3` reader - SEQ3\[3:0\]: channel number code for 4th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ3_R = crate::FieldReader;
///Field `SEQ3` writer - SEQ3\[3:0\]: channel number code for 4th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ4` reader - SEQ4\[3:0\]: channel number code for 5th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ4_R = crate::FieldReader;
///Field `SEQ4` writer - SEQ4\[3:0\]: channel number code for 5th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ5` reader - SEQ5\[3:0\]: channel number code for 6th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ5_R = crate::FieldReader;
///Field `SEQ5` writer - SEQ5\[3:0\]: channel number code for 6th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ6` reader - SEQ6\[3:0\]: channel number code for 7th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ6_R = crate::FieldReader;
///Field `SEQ6` writer - SEQ6\[3:0\]: channel number code for 7th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ7` reader - SEQ7\[3:0\]: channel number code for 8th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ7_R = crate::FieldReader;
///Field `SEQ7` writer - SEQ7\[3:0\]: channel number code for 8th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - SEQ0\[3:0\]: channel number code for first conversion of the sequence 0000: VINM\[0\] to ADC single negative input 0001: VINM\[1\] to ADC single negative input 0010: VINM\[2\] to ADC single negative input 0011: VINM\[3\] to ADC single negative input 0100: VINP\[0\] to ADC single positive input 0101: VINP\[1\] to ADC single positive input 0110: VINP\[2\] to ADC single positive input 0111: VINP\[3\] to ADC single positive input 1000: VINP\[0\]-VINM\[0\] to ADC differential input 1001: VINP\[1\]-VINM\[1\] to ADC differential input 1010: VINP\[2\]-VINM\[2\] to ADC differential input 1011: VINP\[3\]-VINM\[3\] to ADC differential input 1100: VBAT Battery level detector 1101: Temperature sensor 111x: reserved
    #[inline(always)]
    pub fn seq0(&self) -> SEQ0_R {
        SEQ0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - SEQ1\[3:0\]: channel number code for second conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq1(&self) -> SEQ1_R {
        SEQ1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - SEQ2\[3:0\]: channel number code for 3rd conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq2(&self) -> SEQ2_R {
        SEQ2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - SEQ3\[3:0\]: channel number code for 4th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq3(&self) -> SEQ3_R {
        SEQ3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - SEQ4\[3:0\]: channel number code for 5th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq4(&self) -> SEQ4_R {
        SEQ4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - SEQ5\[3:0\]: channel number code for 6th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq5(&self) -> SEQ5_R {
        SEQ5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - SEQ6\[3:0\]: channel number code for 7th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq6(&self) -> SEQ6_R {
        SEQ6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - SEQ7\[3:0\]: channel number code for 8th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq7(&self) -> SEQ7_R {
        SEQ7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEQ_1")
            .field("seq0", &self.seq0())
            .field("seq1", &self.seq1())
            .field("seq2", &self.seq2())
            .field("seq3", &self.seq3())
            .field("seq4", &self.seq4())
            .field("seq5", &self.seq5())
            .field("seq6", &self.seq6())
            .field("seq7", &self.seq7())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - SEQ0\[3:0\]: channel number code for first conversion of the sequence 0000: VINM\[0\] to ADC single negative input 0001: VINM\[1\] to ADC single negative input 0010: VINM\[2\] to ADC single negative input 0011: VINM\[3\] to ADC single negative input 0100: VINP\[0\] to ADC single positive input 0101: VINP\[1\] to ADC single positive input 0110: VINP\[2\] to ADC single positive input 0111: VINP\[3\] to ADC single positive input 1000: VINP\[0\]-VINM\[0\] to ADC differential input 1001: VINP\[1\]-VINM\[1\] to ADC differential input 1010: VINP\[2\]-VINM\[2\] to ADC differential input 1011: VINP\[3\]-VINM\[3\] to ADC differential input 1100: VBAT Battery level detector 1101: Temperature sensor 111x: reserved
    #[inline(always)]
    pub fn seq0(&mut self) -> SEQ0_W<'_, SEQ_1rs> {
        SEQ0_W::new(self, 0)
    }
    ///Bits 4:7 - SEQ1\[3:0\]: channel number code for second conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq1(&mut self) -> SEQ1_W<'_, SEQ_1rs> {
        SEQ1_W::new(self, 4)
    }
    ///Bits 8:11 - SEQ2\[3:0\]: channel number code for 3rd conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq2(&mut self) -> SEQ2_W<'_, SEQ_1rs> {
        SEQ2_W::new(self, 8)
    }
    ///Bits 12:15 - SEQ3\[3:0\]: channel number code for 4th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq3(&mut self) -> SEQ3_W<'_, SEQ_1rs> {
        SEQ3_W::new(self, 12)
    }
    ///Bits 16:19 - SEQ4\[3:0\]: channel number code for 5th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq4(&mut self) -> SEQ4_W<'_, SEQ_1rs> {
        SEQ4_W::new(self, 16)
    }
    ///Bits 20:23 - SEQ5\[3:0\]: channel number code for 6th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq5(&mut self) -> SEQ5_W<'_, SEQ_1rs> {
        SEQ5_W::new(self, 20)
    }
    ///Bits 24:27 - SEQ6\[3:0\]: channel number code for 7th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq6(&mut self) -> SEQ6_W<'_, SEQ_1rs> {
        SEQ6_W::new(self, 24)
    }
    ///Bits 28:31 - SEQ7\[3:0\]: channel number code for 8th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq7(&mut self) -> SEQ7_W<'_, SEQ_1rs> {
        SEQ7_W::new(self, 28)
    }
}
/**SEQ_1 register

You can [`read`](crate::Reg::read) this register and get [`seq_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#ADC:SEQ_1)*/
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
