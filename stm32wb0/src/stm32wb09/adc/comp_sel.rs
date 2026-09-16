///Register `COMP_SEL` reader
pub type R = crate::R<COMP_SELrs>;
///Register `COMP_SEL` writer
pub type W = crate::W<COMP_SELrs>;
///Field `OFFSET_GAIN0` reader - OFFSET_GAIN0\[1:0\]: gain / offset used in ADC single negative mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
pub type OFFSET_GAIN0_R = crate::FieldReader;
///Field `OFFSET_GAIN0` writer - OFFSET_GAIN0\[1:0\]: gain / offset used in ADC single negative mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
pub type OFFSET_GAIN0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OFFSET_GAIN1` reader - OFFSET_GAIN1\[1:0\]: gain / offset used in ADC single positive mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
pub type OFFSET_GAIN1_R = crate::FieldReader;
///Field `OFFSET_GAIN1` writer - OFFSET_GAIN1\[1:0\]: gain / offset used in ADC single positive mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
pub type OFFSET_GAIN1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OFFSET_GAIN2` reader - OFFSET_GAIN2\[1:0\]: gain / offset used in ADC differential mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
pub type OFFSET_GAIN2_R = crate::FieldReader;
///Field `OFFSET_GAIN2` writer - OFFSET_GAIN2\[1:0\]: gain / offset used in ADC differential mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
pub type OFFSET_GAIN2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OFFSET_GAIN3` reader - OFFSET_GAIN3\[1:0\]: gain / offset used in ADC single negative mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
pub type OFFSET_GAIN3_R = crate::FieldReader;
///Field `OFFSET_GAIN3` writer - OFFSET_GAIN3\[1:0\]: gain / offset used in ADC single negative mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
pub type OFFSET_GAIN3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OFFSET_GAIN4` reader - OFFSET_GAIN4\[1:0\]: gain / offset used in ADC single positive mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
pub type OFFSET_GAIN4_R = crate::FieldReader;
///Field `OFFSET_GAIN4` writer - OFFSET_GAIN4\[1:0\]: gain / offset used in ADC single positive mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
pub type OFFSET_GAIN4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OFFSET_GAIN5` reader - OFFSET_GAIN5\[1:0\]: gain / offset used in ADC differential mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
pub type OFFSET_GAIN5_R = crate::FieldReader;
///Field `OFFSET_GAIN5` writer - OFFSET_GAIN5\[1:0\]: gain / offset used in ADC differential mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
pub type OFFSET_GAIN5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OFFSET_GAIN6` reader - OFFSET_GAIN6\[1:0\]: gain / offset used in ADC single negative mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
pub type OFFSET_GAIN6_R = crate::FieldReader;
///Field `OFFSET_GAIN6` writer - OFFSET_GAIN6\[1:0\]: gain / offset used in ADC single negative mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
pub type OFFSET_GAIN6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OFFSET_GAIN7` reader - OFFSET_GAIN7\[1:0\]: gain / offset used in ADC single positive mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
pub type OFFSET_GAIN7_R = crate::FieldReader;
///Field `OFFSET_GAIN7` writer - OFFSET_GAIN7\[1:0\]: gain / offset used in ADC single positive mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
pub type OFFSET_GAIN7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OFFSET_GAIN8` reader - OFFSET_GAIN8\[1:0\]: gain / offset used in ADC differential mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
pub type OFFSET_GAIN8_R = crate::FieldReader;
///Field `OFFSET_GAIN8` writer - OFFSET_GAIN8\[1:0\]: gain / offset used in ADC differential mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
pub type OFFSET_GAIN8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - OFFSET_GAIN0\[1:0\]: gain / offset used in ADC single negative mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
    #[inline(always)]
    pub fn offset_gain0(&self) -> OFFSET_GAIN0_R {
        OFFSET_GAIN0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - OFFSET_GAIN1\[1:0\]: gain / offset used in ADC single positive mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
    #[inline(always)]
    pub fn offset_gain1(&self) -> OFFSET_GAIN1_R {
        OFFSET_GAIN1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - OFFSET_GAIN2\[1:0\]: gain / offset used in ADC differential mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
    #[inline(always)]
    pub fn offset_gain2(&self) -> OFFSET_GAIN2_R {
        OFFSET_GAIN2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - OFFSET_GAIN3\[1:0\]: gain / offset used in ADC single negative mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
    #[inline(always)]
    pub fn offset_gain3(&self) -> OFFSET_GAIN3_R {
        OFFSET_GAIN3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - OFFSET_GAIN4\[1:0\]: gain / offset used in ADC single positive mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
    #[inline(always)]
    pub fn offset_gain4(&self) -> OFFSET_GAIN4_R {
        OFFSET_GAIN4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - OFFSET_GAIN5\[1:0\]: gain / offset used in ADC differential mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
    #[inline(always)]
    pub fn offset_gain5(&self) -> OFFSET_GAIN5_R {
        OFFSET_GAIN5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - OFFSET_GAIN6\[1:0\]: gain / offset used in ADC single negative mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
    #[inline(always)]
    pub fn offset_gain6(&self) -> OFFSET_GAIN6_R {
        OFFSET_GAIN6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - OFFSET_GAIN7\[1:0\]: gain / offset used in ADC single positive mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
    #[inline(always)]
    pub fn offset_gain7(&self) -> OFFSET_GAIN7_R {
        OFFSET_GAIN7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - OFFSET_GAIN8\[1:0\]: gain / offset used in ADC differential mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
    #[inline(always)]
    pub fn offset_gain8(&self) -> OFFSET_GAIN8_R {
        OFFSET_GAIN8_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP_SEL")
            .field("offset_gain0", &self.offset_gain0())
            .field("offset_gain1", &self.offset_gain1())
            .field("offset_gain2", &self.offset_gain2())
            .field("offset_gain3", &self.offset_gain3())
            .field("offset_gain4", &self.offset_gain4())
            .field("offset_gain5", &self.offset_gain5())
            .field("offset_gain6", &self.offset_gain6())
            .field("offset_gain7", &self.offset_gain7())
            .field("offset_gain8", &self.offset_gain8())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - OFFSET_GAIN0\[1:0\]: gain / offset used in ADC single negative mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
    #[inline(always)]
    pub fn offset_gain0(&mut self) -> OFFSET_GAIN0_W<'_, COMP_SELrs> {
        OFFSET_GAIN0_W::new(self, 0)
    }
    ///Bits 2:3 - OFFSET_GAIN1\[1:0\]: gain / offset used in ADC single positive mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
    #[inline(always)]
    pub fn offset_gain1(&mut self) -> OFFSET_GAIN1_W<'_, COMP_SELrs> {
        OFFSET_GAIN1_W::new(self, 2)
    }
    ///Bits 4:5 - OFFSET_GAIN2\[1:0\]: gain / offset used in ADC differential mode with Vinput range = 1.2V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
    #[inline(always)]
    pub fn offset_gain2(&mut self) -> OFFSET_GAIN2_W<'_, COMP_SELrs> {
        OFFSET_GAIN2_W::new(self, 4)
    }
    ///Bits 6:7 - OFFSET_GAIN3\[1:0\]: gain / offset used in ADC single negative mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
    #[inline(always)]
    pub fn offset_gain3(&mut self) -> OFFSET_GAIN3_W<'_, COMP_SELrs> {
        OFFSET_GAIN3_W::new(self, 6)
    }
    ///Bits 8:9 - OFFSET_GAIN4\[1:0\]: gain / offset used in ADC single positive mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
    #[inline(always)]
    pub fn offset_gain4(&mut self) -> OFFSET_GAIN4_W<'_, COMP_SELrs> {
        OFFSET_GAIN4_W::new(self, 8)
    }
    ///Bits 10:11 - OFFSET_GAIN5\[1:0\]: gain / offset used in ADC differential mode with Vinput range = 2.4V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
    #[inline(always)]
    pub fn offset_gain5(&mut self) -> OFFSET_GAIN5_W<'_, COMP_SELrs> {
        OFFSET_GAIN5_W::new(self, 10)
    }
    ///Bits 12:13 - OFFSET_GAIN6\[1:0\]: gain / offset used in ADC single negative mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
    #[inline(always)]
    pub fn offset_gain6(&mut self) -> OFFSET_GAIN6_W<'_, COMP_SELrs> {
        OFFSET_GAIN6_W::new(self, 12)
    }
    ///Bits 14:15 - OFFSET_GAIN7\[1:0\]: gain / offset used in ADC single positive mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
    #[inline(always)]
    pub fn offset_gain7(&mut self) -> OFFSET_GAIN7_W<'_, COMP_SELrs> {
        OFFSET_GAIN7_W::new(self, 14)
    }
    ///Bits 16:17 - OFFSET_GAIN8\[1:0\]: gain / offset used in ADC differential mode with Vinput range = 3.6V: 00: OFFSET1 and GAIN1 from COMP_1 01: OFFSET2 and GAIN2 from COMP_2 10: OFFSET3 and GAIN3 from COMP_3 11: OFFSET4 and GAIN4 from COMP_4
    #[inline(always)]
    pub fn offset_gain8(&mut self) -> OFFSET_GAIN8_W<'_, COMP_SELrs> {
        OFFSET_GAIN8_W::new(self, 16)
    }
}
/**COMP_SEL register

You can [`read`](crate::Reg::read) this register and get [`comp_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#ADC:COMP_SEL)*/
pub struct COMP_SELrs;
impl crate::RegisterSpec for COMP_SELrs {
    type Ux = u32;
}
///`read()` method returns [`comp_sel::R`](R) reader structure
impl crate::Readable for COMP_SELrs {}
///`write(|w| ..)` method takes [`comp_sel::W`](W) writer structure
impl crate::Writable for COMP_SELrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP_SEL to value 0
impl crate::Resettable for COMP_SELrs {}
