///Register `SMPR1` reader
pub type R = crate::R<SMPR1rs>;
///Register `SMPR1` writer
pub type W = crate::W<SMPR1rs>;
///Field `SMP(20-29)` reader - Channel %s sample time selection
pub type SMP_R = crate::FieldReader;
///Field `SMP(20-29)` writer - Channel %s sample time selection
pub type SMP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Channel (20-29) sample time selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SMP20` field.</div>
    #[inline(always)]
    pub fn smp(&self, n: u8) -> SMP_R {
        #[allow(clippy::no_effect)]
        [(); 10][n as usize];
        SMP_R::new(((self.bits >> (n * 3)) & 7) as u8)
    }
    ///Iterator for array of:
    ///Channel (20-29) sample time selection
    #[inline(always)]
    pub fn smp_iter(&self) -> impl Iterator<Item = SMP_R> + '_ {
        (0..10).map(move |n| SMP_R::new(((self.bits >> (n * 3)) & 7) as u8))
    }
    ///Bits 0:2 - Channel 20 sample time selection
    #[inline(always)]
    pub fn smp20(&self) -> SMP_R {
        SMP_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - Channel 21 sample time selection
    #[inline(always)]
    pub fn smp21(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - Channel 22 sample time selection
    #[inline(always)]
    pub fn smp22(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - Channel 23 sample time selection
    #[inline(always)]
    pub fn smp23(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - Channel 24 sample time selection
    #[inline(always)]
    pub fn smp24(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - Channel 25 sample time selection
    #[inline(always)]
    pub fn smp25(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - Channel 26 sample time selection
    #[inline(always)]
    pub fn smp26(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - Channel 27 sample time selection
    #[inline(always)]
    pub fn smp27(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:26 - Channel 28 sample time selection
    #[inline(always)]
    pub fn smp28(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - Channel 29 sample time selection
    #[inline(always)]
    pub fn smp29(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR1")
            .field("smp20", &self.smp20())
            .field("smp21", &self.smp21())
            .field("smp22", &self.smp22())
            .field("smp23", &self.smp23())
            .field("smp24", &self.smp24())
            .field("smp25", &self.smp25())
            .field("smp26", &self.smp26())
            .field("smp27", &self.smp27())
            .field("smp28", &self.smp28())
            .field("smp29", &self.smp29())
            .finish()
    }
}
impl W {
    ///Channel (20-29) sample time selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SMP20` field.</div>
    #[inline(always)]
    pub fn smp(&mut self, n: u8) -> SMP_W<'_, SMPR1rs> {
        #[allow(clippy::no_effect)]
        [(); 10][n as usize];
        SMP_W::new(self, n * 3)
    }
    ///Bits 0:2 - Channel 20 sample time selection
    #[inline(always)]
    pub fn smp20(&mut self) -> SMP_W<'_, SMPR1rs> {
        SMP_W::new(self, 0)
    }
    ///Bits 3:5 - Channel 21 sample time selection
    #[inline(always)]
    pub fn smp21(&mut self) -> SMP_W<'_, SMPR1rs> {
        SMP_W::new(self, 3)
    }
    ///Bits 6:8 - Channel 22 sample time selection
    #[inline(always)]
    pub fn smp22(&mut self) -> SMP_W<'_, SMPR1rs> {
        SMP_W::new(self, 6)
    }
    ///Bits 9:11 - Channel 23 sample time selection
    #[inline(always)]
    pub fn smp23(&mut self) -> SMP_W<'_, SMPR1rs> {
        SMP_W::new(self, 9)
    }
    ///Bits 12:14 - Channel 24 sample time selection
    #[inline(always)]
    pub fn smp24(&mut self) -> SMP_W<'_, SMPR1rs> {
        SMP_W::new(self, 12)
    }
    ///Bits 15:17 - Channel 25 sample time selection
    #[inline(always)]
    pub fn smp25(&mut self) -> SMP_W<'_, SMPR1rs> {
        SMP_W::new(self, 15)
    }
    ///Bits 18:20 - Channel 26 sample time selection
    #[inline(always)]
    pub fn smp26(&mut self) -> SMP_W<'_, SMPR1rs> {
        SMP_W::new(self, 18)
    }
    ///Bits 21:23 - Channel 27 sample time selection
    #[inline(always)]
    pub fn smp27(&mut self) -> SMP_W<'_, SMPR1rs> {
        SMP_W::new(self, 21)
    }
    ///Bits 24:26 - Channel 28 sample time selection
    #[inline(always)]
    pub fn smp28(&mut self) -> SMP_W<'_, SMPR1rs> {
        SMP_W::new(self, 24)
    }
    ///Bits 27:29 - Channel 29 sample time selection
    #[inline(always)]
    pub fn smp29(&mut self) -> SMP_W<'_, SMPR1rs> {
        SMP_W::new(self, 27)
    }
}
/**sample time register 1

You can [`read`](crate::Reg::read) this register and get [`smpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#ADC:SMPR1)*/
pub struct SMPR1rs;
impl crate::RegisterSpec for SMPR1rs {
    type Ux = u32;
}
///`read()` method returns [`smpr1::R`](R) reader structure
impl crate::Readable for SMPR1rs {}
///`write(|w| ..)` method takes [`smpr1::W`](W) writer structure
impl crate::Writable for SMPR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMPR1 to value 0
impl crate::Resettable for SMPR1rs {}
