///Register `COMP_SEL` reader
pub type R = crate::R<COMP_SELrs>;
///Register `COMP_SEL` writer
pub type W = crate::W<COMP_SELrs>;
///Field `GAIN_OFFSET0` reader - gain / offset used in ADC single negative mode with Vinput range = 1.2V
pub type GAIN_OFFSET0_R = crate::FieldReader;
///Field `GAIN_OFFSET0` writer - gain / offset used in ADC single negative mode with Vinput range = 1.2V
pub type GAIN_OFFSET0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `GAIN_OFFSET1` reader - gain / offset used in ADC single positive mode with Vinput range = 1.2V
pub type GAIN_OFFSET1_R = crate::FieldReader;
///Field `GAIN_OFFSET1` writer - gain / offset used in ADC single positive mode with Vinput range = 1.2V
pub type GAIN_OFFSET1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `GAIN_OFFSET2` reader - gain / offset used in ADC differential mode with Vinput range = 1.2V
pub type GAIN_OFFSET2_R = crate::FieldReader;
///Field `GAIN_OFFSET2` writer - gain / offset used in ADC differential mode with Vinput range = 1.2V
pub type GAIN_OFFSET2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `GAIN_OFFSET3` reader - gain / offset used in ADC single negative mode with Vinput range = 2.4V
pub type GAIN_OFFSET3_R = crate::FieldReader;
///Field `GAIN_OFFSET3` writer - gain / offset used in ADC single negative mode with Vinput range = 2.4V
pub type GAIN_OFFSET3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `GAIN_OFFSET4` reader - gain / offset used in ADC single positive mode with Vinput range = 2.4V
pub type GAIN_OFFSET4_R = crate::FieldReader;
///Field `GAIN_OFFSET4` writer - gain / offset used in ADC single positive mode with Vinput range = 2.4V
pub type GAIN_OFFSET4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `GAIN_OFFSET5` reader - gain / offset used in ADC differential mode with Vinput range = 2.4V
pub type GAIN_OFFSET5_R = crate::FieldReader;
///Field `GAIN_OFFSET5` writer - gain / offset used in ADC differential mode with Vinput range = 2.4V
pub type GAIN_OFFSET5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `GAIN_OFFSET6` reader - gain / offset used in ADC single negative mode with Vinput range = 3.6V
pub type GAIN_OFFSET6_R = crate::FieldReader;
///Field `GAIN_OFFSET6` writer - gain / offset used in ADC single negative mode with Vinput range = 3.6V
pub type GAIN_OFFSET6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `GAIN_OFFSET7` reader - gain / offset used in ADC single positive mode with Vinput range = 3.6V
pub type GAIN_OFFSET7_R = crate::FieldReader;
///Field `GAIN_OFFSET7` writer - gain / offset used in ADC single positive mode with Vinput range = 3.6V
pub type GAIN_OFFSET7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `GAIN_OFFSET8` reader - gain / offset used in ADC differential mode with Vinput range = 3.6V
pub type GAIN_OFFSET8_R = crate::FieldReader;
///Field `GAIN_OFFSET8` writer - gain / offset used in ADC differential mode with Vinput range = 3.6V
pub type GAIN_OFFSET8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - gain / offset used in ADC single negative mode with Vinput range = 1.2V
    #[inline(always)]
    pub fn gain_offset0(&self) -> GAIN_OFFSET0_R {
        GAIN_OFFSET0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - gain / offset used in ADC single positive mode with Vinput range = 1.2V
    #[inline(always)]
    pub fn gain_offset1(&self) -> GAIN_OFFSET1_R {
        GAIN_OFFSET1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - gain / offset used in ADC differential mode with Vinput range = 1.2V
    #[inline(always)]
    pub fn gain_offset2(&self) -> GAIN_OFFSET2_R {
        GAIN_OFFSET2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - gain / offset used in ADC single negative mode with Vinput range = 2.4V
    #[inline(always)]
    pub fn gain_offset3(&self) -> GAIN_OFFSET3_R {
        GAIN_OFFSET3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - gain / offset used in ADC single positive mode with Vinput range = 2.4V
    #[inline(always)]
    pub fn gain_offset4(&self) -> GAIN_OFFSET4_R {
        GAIN_OFFSET4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - gain / offset used in ADC differential mode with Vinput range = 2.4V
    #[inline(always)]
    pub fn gain_offset5(&self) -> GAIN_OFFSET5_R {
        GAIN_OFFSET5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - gain / offset used in ADC single negative mode with Vinput range = 3.6V
    #[inline(always)]
    pub fn gain_offset6(&self) -> GAIN_OFFSET6_R {
        GAIN_OFFSET6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - gain / offset used in ADC single positive mode with Vinput range = 3.6V
    #[inline(always)]
    pub fn gain_offset7(&self) -> GAIN_OFFSET7_R {
        GAIN_OFFSET7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - gain / offset used in ADC differential mode with Vinput range = 3.6V
    #[inline(always)]
    pub fn gain_offset8(&self) -> GAIN_OFFSET8_R {
        GAIN_OFFSET8_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP_SEL")
            .field("gain_offset8", &self.gain_offset8())
            .field("gain_offset7", &self.gain_offset7())
            .field("gain_offset6", &self.gain_offset6())
            .field("gain_offset5", &self.gain_offset5())
            .field("gain_offset4", &self.gain_offset4())
            .field("gain_offset3", &self.gain_offset3())
            .field("gain_offset2", &self.gain_offset2())
            .field("gain_offset1", &self.gain_offset1())
            .field("gain_offset0", &self.gain_offset0())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - gain / offset used in ADC single negative mode with Vinput range = 1.2V
    #[inline(always)]
    pub fn gain_offset0(&mut self) -> GAIN_OFFSET0_W<'_, COMP_SELrs> {
        GAIN_OFFSET0_W::new(self, 0)
    }
    ///Bits 2:3 - gain / offset used in ADC single positive mode with Vinput range = 1.2V
    #[inline(always)]
    pub fn gain_offset1(&mut self) -> GAIN_OFFSET1_W<'_, COMP_SELrs> {
        GAIN_OFFSET1_W::new(self, 2)
    }
    ///Bits 4:5 - gain / offset used in ADC differential mode with Vinput range = 1.2V
    #[inline(always)]
    pub fn gain_offset2(&mut self) -> GAIN_OFFSET2_W<'_, COMP_SELrs> {
        GAIN_OFFSET2_W::new(self, 4)
    }
    ///Bits 6:7 - gain / offset used in ADC single negative mode with Vinput range = 2.4V
    #[inline(always)]
    pub fn gain_offset3(&mut self) -> GAIN_OFFSET3_W<'_, COMP_SELrs> {
        GAIN_OFFSET3_W::new(self, 6)
    }
    ///Bits 8:9 - gain / offset used in ADC single positive mode with Vinput range = 2.4V
    #[inline(always)]
    pub fn gain_offset4(&mut self) -> GAIN_OFFSET4_W<'_, COMP_SELrs> {
        GAIN_OFFSET4_W::new(self, 8)
    }
    ///Bits 10:11 - gain / offset used in ADC differential mode with Vinput range = 2.4V
    #[inline(always)]
    pub fn gain_offset5(&mut self) -> GAIN_OFFSET5_W<'_, COMP_SELrs> {
        GAIN_OFFSET5_W::new(self, 10)
    }
    ///Bits 12:13 - gain / offset used in ADC single negative mode with Vinput range = 3.6V
    #[inline(always)]
    pub fn gain_offset6(&mut self) -> GAIN_OFFSET6_W<'_, COMP_SELrs> {
        GAIN_OFFSET6_W::new(self, 12)
    }
    ///Bits 14:15 - gain / offset used in ADC single positive mode with Vinput range = 3.6V
    #[inline(always)]
    pub fn gain_offset7(&mut self) -> GAIN_OFFSET7_W<'_, COMP_SELrs> {
        GAIN_OFFSET7_W::new(self, 14)
    }
    ///Bits 16:17 - gain / offset used in ADC differential mode with Vinput range = 3.6V
    #[inline(always)]
    pub fn gain_offset8(&mut self) -> GAIN_OFFSET8_W<'_, COMP_SELrs> {
        GAIN_OFFSET8_W::new(self, 16)
    }
}
/**ADC Gain and Offset selection values register

You can [`read`](crate::Reg::read) this register and get [`comp_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#ADC:COMP_SEL)*/
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
