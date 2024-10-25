///Register `SMPR1` reader
pub type R = crate::R<SMPR1rs>;
///Register `SMPR1` writer
pub type W = crate::W<SMPR1rs>;
///Field `SMP20` reader - Channel 20 sampling time selection
pub type SMP20_R = crate::FieldReader;
///Field `SMP20` writer - Channel 20 sampling time selection
pub type SMP20_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP21` reader - Channel 21 sampling time selection
pub type SMP21_R = crate::FieldReader;
///Field `SMP21` writer - Channel 21 sampling time selection
pub type SMP21_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP22` reader - Channel 22 sampling time selection
pub type SMP22_R = crate::FieldReader;
///Field `SMP22` writer - Channel 22 sampling time selection
pub type SMP22_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP23` reader - Channel 23 sampling time selection
pub type SMP23_R = crate::FieldReader;
///Field `SMP23` writer - Channel 23 sampling time selection
pub type SMP23_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP24` reader - Channel 24 sampling time selection
pub type SMP24_R = crate::FieldReader;
///Field `SMP24` writer - Channel 24 sampling time selection
pub type SMP24_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP25` reader - Channel 25 sampling time selection
pub type SMP25_R = crate::FieldReader;
///Field `SMP25` writer - Channel 25 sampling time selection
pub type SMP25_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP26` reader - Channel 26 sampling time selection
pub type SMP26_R = crate::FieldReader;
///Field `SMP26` writer - Channel 26 sampling time selection
pub type SMP26_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP27` reader - Channel 27 sampling time selection
pub type SMP27_R = crate::FieldReader;
///Field `SMP27` writer - Channel 27 sampling time selection
pub type SMP27_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP28` reader - Channel 28 sampling time selection
pub type SMP28_R = crate::FieldReader;
///Field `SMP28` writer - Channel 28 sampling time selection
pub type SMP28_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SMP29` reader - Channel 29 sampling time selection
pub type SMP29_R = crate::FieldReader;
///Field `SMP29` writer - Channel 29 sampling time selection
pub type SMP29_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Channel 20 sampling time selection
    #[inline(always)]
    pub fn smp20(&self) -> SMP20_R {
        SMP20_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - Channel 21 sampling time selection
    #[inline(always)]
    pub fn smp21(&self) -> SMP21_R {
        SMP21_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - Channel 22 sampling time selection
    #[inline(always)]
    pub fn smp22(&self) -> SMP22_R {
        SMP22_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - Channel 23 sampling time selection
    #[inline(always)]
    pub fn smp23(&self) -> SMP23_R {
        SMP23_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - Channel 24 sampling time selection
    #[inline(always)]
    pub fn smp24(&self) -> SMP24_R {
        SMP24_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - Channel 25 sampling time selection
    #[inline(always)]
    pub fn smp25(&self) -> SMP25_R {
        SMP25_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - Channel 26 sampling time selection
    #[inline(always)]
    pub fn smp26(&self) -> SMP26_R {
        SMP26_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - Channel 27 sampling time selection
    #[inline(always)]
    pub fn smp27(&self) -> SMP27_R {
        SMP27_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:26 - Channel 28 sampling time selection
    #[inline(always)]
    pub fn smp28(&self) -> SMP28_R {
        SMP28_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - Channel 29 sampling time selection
    #[inline(always)]
    pub fn smp29(&self) -> SMP29_R {
        SMP29_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR1")
            .field("smp29", &self.smp29())
            .field("smp28", &self.smp28())
            .field("smp27", &self.smp27())
            .field("smp26", &self.smp26())
            .field("smp25", &self.smp25())
            .field("smp24", &self.smp24())
            .field("smp23", &self.smp23())
            .field("smp22", &self.smp22())
            .field("smp21", &self.smp21())
            .field("smp20", &self.smp20())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Channel 20 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp20(&mut self) -> SMP20_W<SMPR1rs> {
        SMP20_W::new(self, 0)
    }
    ///Bits 3:5 - Channel 21 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp21(&mut self) -> SMP21_W<SMPR1rs> {
        SMP21_W::new(self, 3)
    }
    ///Bits 6:8 - Channel 22 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp22(&mut self) -> SMP22_W<SMPR1rs> {
        SMP22_W::new(self, 6)
    }
    ///Bits 9:11 - Channel 23 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp23(&mut self) -> SMP23_W<SMPR1rs> {
        SMP23_W::new(self, 9)
    }
    ///Bits 12:14 - Channel 24 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp24(&mut self) -> SMP24_W<SMPR1rs> {
        SMP24_W::new(self, 12)
    }
    ///Bits 15:17 - Channel 25 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp25(&mut self) -> SMP25_W<SMPR1rs> {
        SMP25_W::new(self, 15)
    }
    ///Bits 18:20 - Channel 26 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp26(&mut self) -> SMP26_W<SMPR1rs> {
        SMP26_W::new(self, 18)
    }
    ///Bits 21:23 - Channel 27 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp27(&mut self) -> SMP27_W<SMPR1rs> {
        SMP27_W::new(self, 21)
    }
    ///Bits 24:26 - Channel 28 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp28(&mut self) -> SMP28_W<SMPR1rs> {
        SMP28_W::new(self, 24)
    }
    ///Bits 27:29 - Channel 29 sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp29(&mut self) -> SMP29_W<SMPR1rs> {
        SMP29_W::new(self, 27)
    }
}
/**sample time register 1

You can [`read`](crate::Reg::read) this register and get [`smpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#ADC:SMPR1)*/
pub struct SMPR1rs;
impl crate::RegisterSpec for SMPR1rs {
    type Ux = u32;
}
///`read()` method returns [`smpr1::R`](R) reader structure
impl crate::Readable for SMPR1rs {}
///`write(|w| ..)` method takes [`smpr1::W`](W) writer structure
impl crate::Writable for SMPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SMPR1 to value 0
impl crate::Resettable for SMPR1rs {
    const RESET_VALUE: u32 = 0;
}
