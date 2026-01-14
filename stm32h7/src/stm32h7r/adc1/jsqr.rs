///Register `JSQR` reader
pub type R = crate::R<JSQRrs>;
///Register `JSQR` writer
pub type W = crate::W<JSQRrs>;
///Field `JL` reader - Injected channel sequence length These bits are written by software to define the total number of conversions in the injected channel conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
pub type JL_R = crate::FieldReader;
///Field `JL` writer - Injected channel sequence length These bits are written by software to define the total number of conversions in the injected channel conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
pub type JL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `JEXTSEL` reader - External Trigger Selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
pub type JEXTSEL_R = crate::FieldReader;
///Field `JEXTSEL` writer - External Trigger Selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `JEXTEN` reader - External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing). Note: If JQM = 1 and if the Queue of Context becomes empty, the software and hardware triggers of the injected sequence are both internally disabled (refer to Section 25.4.21: Queue of context for injected conversions)
pub type JEXTEN_R = crate::FieldReader;
///Field `JEXTEN` writer - External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing). Note: If JQM = 1 and if the Queue of Context becomes empty, the software and hardware triggers of the injected sequence are both internally disabled (refer to Section 25.4.21: Queue of context for injected conversions)
pub type JEXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `JSQ1` reader - 1st conversion in the injected sequence These bits are written by software with the channel number (0 to 18) assigned as the 1st in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
pub type JSQ1_R = crate::FieldReader;
///Field `JSQ1` writer - 1st conversion in the injected sequence These bits are written by software with the channel number (0 to 18) assigned as the 1st in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
pub type JSQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `JSQ2` reader - 2nd conversion in the injected sequence These bits are written by software with the channel number (0 to 18) assigned as the 2nd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
pub type JSQ2_R = crate::FieldReader;
///Field `JSQ2` writer - 2nd conversion in the injected sequence These bits are written by software with the channel number (0 to 18) assigned as the 2nd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
pub type JSQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `JSQ3` reader - 3rd conversion in the injected sequence These bits are written by software with the channel number (0 to 18) assigned as the 3rd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
pub type JSQ3_R = crate::FieldReader;
///Field `JSQ3` writer - 3rd conversion in the injected sequence These bits are written by software with the channel number (0 to 18) assigned as the 3rd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
pub type JSQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `JSQ4` reader - 4th conversion in the injected sequence These bits are written by software with the channel number (0 to 18) assigned as the 4th in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
pub type JSQ4_R = crate::FieldReader;
///Field `JSQ4` writer - 4th conversion in the injected sequence These bits are written by software with the channel number (0 to 18) assigned as the 4th in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
pub type JSQ4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:1 - Injected channel sequence length These bits are written by software to define the total number of conversions in the injected channel conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:6 - External Trigger Selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    ///Bits 7:8 - External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing). Note: If JQM = 1 and if the Queue of Context becomes empty, the software and hardware triggers of the injected sequence are both internally disabled (refer to Section 25.4.21: Queue of context for injected conversions)
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bits 9:13 - 1st conversion in the injected sequence These bits are written by software with the channel number (0 to 18) assigned as the 1st in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ1_R {
        JSQ1_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    ///Bits 15:19 - 2nd conversion in the injected sequence These bits are written by software with the channel number (0 to 18) assigned as the 2nd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 21:25 - 3rd conversion in the injected sequence These bits are written by software with the channel number (0 to 18) assigned as the 3rd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bits 27:31 - 4th conversion in the injected sequence These bits are written by software with the channel number (0 to 18) assigned as the 4th in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ4_R {
        JSQ4_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JSQR")
            .field("jl", &self.jl())
            .field("jextsel", &self.jextsel())
            .field("jexten", &self.jexten())
            .field("jsq1", &self.jsq1())
            .field("jsq2", &self.jsq2())
            .field("jsq3", &self.jsq3())
            .field("jsq4", &self.jsq4())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Injected channel sequence length These bits are written by software to define the total number of conversions in the injected channel conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
    #[inline(always)]
    pub fn jl(&mut self) -> JL_W<'_, JSQRrs> {
        JL_W::new(self, 0)
    }
    ///Bits 2:6 - External Trigger Selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W<'_, JSQRrs> {
        JEXTSEL_W::new(self, 2)
    }
    ///Bits 7:8 - External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing). Note: If JQM = 1 and if the Queue of Context becomes empty, the software and hardware triggers of the injected sequence are both internally disabled (refer to Section 25.4.21: Queue of context for injected conversions)
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W<'_, JSQRrs> {
        JEXTEN_W::new(self, 7)
    }
    ///Bits 9:13 - 1st conversion in the injected sequence These bits are written by software with the channel number (0 to 18) assigned as the 1st in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
    #[inline(always)]
    pub fn jsq1(&mut self) -> JSQ1_W<'_, JSQRrs> {
        JSQ1_W::new(self, 9)
    }
    ///Bits 15:19 - 2nd conversion in the injected sequence These bits are written by software with the channel number (0 to 18) assigned as the 2nd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
    #[inline(always)]
    pub fn jsq2(&mut self) -> JSQ2_W<'_, JSQRrs> {
        JSQ2_W::new(self, 15)
    }
    ///Bits 21:25 - 3rd conversion in the injected sequence These bits are written by software with the channel number (0 to 18) assigned as the 3rd in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
    #[inline(always)]
    pub fn jsq3(&mut self) -> JSQ3_W<'_, JSQRrs> {
        JSQ3_W::new(self, 21)
    }
    ///Bits 27:31 - 4th conversion in the injected sequence These bits are written by software with the channel number (0 to 18) assigned as the 4th in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures that no injected conversion is ongoing).
    #[inline(always)]
    pub fn jsq4(&mut self) -> JSQ4_W<'_, JSQRrs> {
        JSQ4_W::new(self, 27)
    }
}
/**ADC injected sequence register

You can [`read`](crate::Reg::read) this register and get [`jsqr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jsqr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#ADC1:JSQR)*/
pub struct JSQRrs;
impl crate::RegisterSpec for JSQRrs {
    type Ux = u32;
}
///`read()` method returns [`jsqr::R`](R) reader structure
impl crate::Readable for JSQRrs {}
///`write(|w| ..)` method takes [`jsqr::W`](W) writer structure
impl crate::Writable for JSQRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JSQR to value 0
impl crate::Resettable for JSQRrs {}
