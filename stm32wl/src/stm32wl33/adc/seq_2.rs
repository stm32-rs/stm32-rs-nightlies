///Register `SEQ_2` reader
pub type R = crate::R<SEQ_2rs>;
///Register `SEQ_2` writer
pub type W = crate::W<SEQ_2rs>;
///Field `SEQ8` reader - SEQ8\[3:0\]: channel number code for 9th conversion of the sequence 0000: VINM\[0\] to ADC single negative input 0001: VINM\[1\] to ADC single negative input 0010: VINM\[2\] to ADC single negative input 0011: VINM\[3\] to ADC single negative input 0100: VINP\[0\] to ADC single positive input 0101: VINP\[1\] to ADC single positive input 0110: VINP\[2\] to ADC single positive input 0111: VINP\[3\] to ADC single positive input 1000: VINP\[0\]-VINM\[0\] to ADC differential input 1001: VINP\[1\]-VINM\[1\] to ADC differential input 1010: VINP\[2\]-VINM\[2\] to ADC differential input 1011: VINP\[3\]-VINM\[3\] to ADC differential input 1100: VBAT - Battery level detector 1101: Temperature sensor 111x: reserved
pub type SEQ8_R = crate::FieldReader;
///Field `SEQ8` writer - SEQ8\[3:0\]: channel number code for 9th conversion of the sequence 0000: VINM\[0\] to ADC single negative input 0001: VINM\[1\] to ADC single negative input 0010: VINM\[2\] to ADC single negative input 0011: VINM\[3\] to ADC single negative input 0100: VINP\[0\] to ADC single positive input 0101: VINP\[1\] to ADC single positive input 0110: VINP\[2\] to ADC single positive input 0111: VINP\[3\] to ADC single positive input 1000: VINP\[0\]-VINM\[0\] to ADC differential input 1001: VINP\[1\]-VINM\[1\] to ADC differential input 1010: VINP\[2\]-VINM\[2\] to ADC differential input 1011: VINP\[3\]-VINM\[3\] to ADC differential input 1100: VBAT - Battery level detector 1101: Temperature sensor 111x: reserved
pub type SEQ8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ9` reader - SEQ9\[3:0\]: channel number code for 10th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ9_R = crate::FieldReader;
///Field `SEQ9` writer - SEQ9\[3:0\]: channel number code for 10th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ9_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ10` reader - SEQ10\[3:0\]: channel number code for 11th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ10_R = crate::FieldReader;
///Field `SEQ10` writer - SEQ10\[3:0\]: channel number code for 11th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ11` reader - SEQ11\[3:0\]: channel number code for 12th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ11_R = crate::FieldReader;
///Field `SEQ11` writer - SEQ11\[3:0\]: channel number code for 12th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ12` reader - SEQ12\[3:0\]: channel number code for 13th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ12_R = crate::FieldReader;
///Field `SEQ12` writer - SEQ12\[3:0\]: channel number code for 13th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ12_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ13` reader - SEQ13\[3:0\]: channel number code for 14th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ13_R = crate::FieldReader;
///Field `SEQ13` writer - SEQ13\[3:0\]: channel number code for 14th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ13_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ14` reader - SEQ14\[3:0\]: channel number code for 15th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ14_R = crate::FieldReader;
///Field `SEQ14` writer - SEQ14\[3:0\]: channel number code for 15th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ14_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEQ15` reader - SEQ15\[3:0\]: channel number code for 16th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ15_R = crate::FieldReader;
///Field `SEQ15` writer - SEQ15\[3:0\]: channel number code for 16th conversion of the sequence. See SEQ0 for code detail.
pub type SEQ15_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - SEQ8\[3:0\]: channel number code for 9th conversion of the sequence 0000: VINM\[0\] to ADC single negative input 0001: VINM\[1\] to ADC single negative input 0010: VINM\[2\] to ADC single negative input 0011: VINM\[3\] to ADC single negative input 0100: VINP\[0\] to ADC single positive input 0101: VINP\[1\] to ADC single positive input 0110: VINP\[2\] to ADC single positive input 0111: VINP\[3\] to ADC single positive input 1000: VINP\[0\]-VINM\[0\] to ADC differential input 1001: VINP\[1\]-VINM\[1\] to ADC differential input 1010: VINP\[2\]-VINM\[2\] to ADC differential input 1011: VINP\[3\]-VINM\[3\] to ADC differential input 1100: VBAT - Battery level detector 1101: Temperature sensor 111x: reserved
    #[inline(always)]
    pub fn seq8(&self) -> SEQ8_R {
        SEQ8_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - SEQ9\[3:0\]: channel number code for 10th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq9(&self) -> SEQ9_R {
        SEQ9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - SEQ10\[3:0\]: channel number code for 11th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq10(&self) -> SEQ10_R {
        SEQ10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - SEQ11\[3:0\]: channel number code for 12th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq11(&self) -> SEQ11_R {
        SEQ11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - SEQ12\[3:0\]: channel number code for 13th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq12(&self) -> SEQ12_R {
        SEQ12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - SEQ13\[3:0\]: channel number code for 14th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq13(&self) -> SEQ13_R {
        SEQ13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - SEQ14\[3:0\]: channel number code for 15th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq14(&self) -> SEQ14_R {
        SEQ14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - SEQ15\[3:0\]: channel number code for 16th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq15(&self) -> SEQ15_R {
        SEQ15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEQ_2")
            .field("seq8", &self.seq8())
            .field("seq9", &self.seq9())
            .field("seq10", &self.seq10())
            .field("seq11", &self.seq11())
            .field("seq12", &self.seq12())
            .field("seq13", &self.seq13())
            .field("seq14", &self.seq14())
            .field("seq15", &self.seq15())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - SEQ8\[3:0\]: channel number code for 9th conversion of the sequence 0000: VINM\[0\] to ADC single negative input 0001: VINM\[1\] to ADC single negative input 0010: VINM\[2\] to ADC single negative input 0011: VINM\[3\] to ADC single negative input 0100: VINP\[0\] to ADC single positive input 0101: VINP\[1\] to ADC single positive input 0110: VINP\[2\] to ADC single positive input 0111: VINP\[3\] to ADC single positive input 1000: VINP\[0\]-VINM\[0\] to ADC differential input 1001: VINP\[1\]-VINM\[1\] to ADC differential input 1010: VINP\[2\]-VINM\[2\] to ADC differential input 1011: VINP\[3\]-VINM\[3\] to ADC differential input 1100: VBAT - Battery level detector 1101: Temperature sensor 111x: reserved
    #[inline(always)]
    pub fn seq8(&mut self) -> SEQ8_W<'_, SEQ_2rs> {
        SEQ8_W::new(self, 0)
    }
    ///Bits 4:7 - SEQ9\[3:0\]: channel number code for 10th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq9(&mut self) -> SEQ9_W<'_, SEQ_2rs> {
        SEQ9_W::new(self, 4)
    }
    ///Bits 8:11 - SEQ10\[3:0\]: channel number code for 11th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq10(&mut self) -> SEQ10_W<'_, SEQ_2rs> {
        SEQ10_W::new(self, 8)
    }
    ///Bits 12:15 - SEQ11\[3:0\]: channel number code for 12th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq11(&mut self) -> SEQ11_W<'_, SEQ_2rs> {
        SEQ11_W::new(self, 12)
    }
    ///Bits 16:19 - SEQ12\[3:0\]: channel number code for 13th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq12(&mut self) -> SEQ12_W<'_, SEQ_2rs> {
        SEQ12_W::new(self, 16)
    }
    ///Bits 20:23 - SEQ13\[3:0\]: channel number code for 14th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq13(&mut self) -> SEQ13_W<'_, SEQ_2rs> {
        SEQ13_W::new(self, 20)
    }
    ///Bits 24:27 - SEQ14\[3:0\]: channel number code for 15th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq14(&mut self) -> SEQ14_W<'_, SEQ_2rs> {
        SEQ14_W::new(self, 24)
    }
    ///Bits 28:31 - SEQ15\[3:0\]: channel number code for 16th conversion of the sequence. See SEQ0 for code detail.
    #[inline(always)]
    pub fn seq15(&mut self) -> SEQ15_W<'_, SEQ_2rs> {
        SEQ15_W::new(self, 28)
    }
}
/**SEQ_2 register

You can [`read`](crate::Reg::read) this register and get [`seq_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#ADC:SEQ_2)*/
pub struct SEQ_2rs;
impl crate::RegisterSpec for SEQ_2rs {
    type Ux = u32;
}
///`read()` method returns [`seq_2::R`](R) reader structure
impl crate::Readable for SEQ_2rs {}
///`write(|w| ..)` method takes [`seq_2::W`](W) writer structure
impl crate::Writable for SEQ_2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SEQ_2 to value 0
impl crate::Resettable for SEQ_2rs {}
