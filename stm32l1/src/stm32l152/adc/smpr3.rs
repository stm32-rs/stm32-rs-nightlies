///Register `SMPR3` reader
pub type R = crate::R<SMPR3rs>;
///Register `SMPR3` writer
pub type W = crate::W<SMPR3rs>;
///Field `SMP(0-9)` reader - Channel %s sample time selection
pub type SMP_R = crate::FieldReader;
///Field `SMP(0-9)` writer - Channel %s sample time selection
pub type SMP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Channel (0-9) sample time selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SMP0` field.</div>
    #[inline(always)]
    pub fn smp(&self, n: u8) -> SMP_R {
        #[allow(clippy::no_effect)]
        [(); 10][n as usize];
        SMP_R::new(((self.bits >> (n * 3)) & 7) as u8)
    }
    ///Iterator for array of:
    ///Channel (0-9) sample time selection
    #[inline(always)]
    pub fn smp_iter(&self) -> impl Iterator<Item = SMP_R> + '_ {
        (0..10).map(move |n| SMP_R::new(((self.bits >> (n * 3)) & 7) as u8))
    }
    ///Bits 0:2 - Channel 0 sample time selection
    #[inline(always)]
    pub fn smp0(&self) -> SMP_R {
        SMP_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - Channel 1 sample time selection
    #[inline(always)]
    pub fn smp1(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - Channel 2 sample time selection
    #[inline(always)]
    pub fn smp2(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - Channel 3 sample time selection
    #[inline(always)]
    pub fn smp3(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - Channel 4 sample time selection
    #[inline(always)]
    pub fn smp4(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - Channel 5 sample time selection
    #[inline(always)]
    pub fn smp5(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - Channel 6 sample time selection
    #[inline(always)]
    pub fn smp6(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - Channel 7 sample time selection
    #[inline(always)]
    pub fn smp7(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:26 - Channel 8 sample time selection
    #[inline(always)]
    pub fn smp8(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - Channel 9 sample time selection
    #[inline(always)]
    pub fn smp9(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR3")
            .field("smp0", &self.smp0())
            .field("smp1", &self.smp1())
            .field("smp2", &self.smp2())
            .field("smp3", &self.smp3())
            .field("smp4", &self.smp4())
            .field("smp5", &self.smp5())
            .field("smp6", &self.smp6())
            .field("smp7", &self.smp7())
            .field("smp8", &self.smp8())
            .field("smp9", &self.smp9())
            .finish()
    }
}
impl W {
    ///Channel (0-9) sample time selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SMP0` field.</div>
    #[inline(always)]
    pub fn smp(&mut self, n: u8) -> SMP_W<SMPR3rs> {
        #[allow(clippy::no_effect)]
        [(); 10][n as usize];
        SMP_W::new(self, n * 3)
    }
    ///Bits 0:2 - Channel 0 sample time selection
    #[inline(always)]
    pub fn smp0(&mut self) -> SMP_W<SMPR3rs> {
        SMP_W::new(self, 0)
    }
    ///Bits 3:5 - Channel 1 sample time selection
    #[inline(always)]
    pub fn smp1(&mut self) -> SMP_W<SMPR3rs> {
        SMP_W::new(self, 3)
    }
    ///Bits 6:8 - Channel 2 sample time selection
    #[inline(always)]
    pub fn smp2(&mut self) -> SMP_W<SMPR3rs> {
        SMP_W::new(self, 6)
    }
    ///Bits 9:11 - Channel 3 sample time selection
    #[inline(always)]
    pub fn smp3(&mut self) -> SMP_W<SMPR3rs> {
        SMP_W::new(self, 9)
    }
    ///Bits 12:14 - Channel 4 sample time selection
    #[inline(always)]
    pub fn smp4(&mut self) -> SMP_W<SMPR3rs> {
        SMP_W::new(self, 12)
    }
    ///Bits 15:17 - Channel 5 sample time selection
    #[inline(always)]
    pub fn smp5(&mut self) -> SMP_W<SMPR3rs> {
        SMP_W::new(self, 15)
    }
    ///Bits 18:20 - Channel 6 sample time selection
    #[inline(always)]
    pub fn smp6(&mut self) -> SMP_W<SMPR3rs> {
        SMP_W::new(self, 18)
    }
    ///Bits 21:23 - Channel 7 sample time selection
    #[inline(always)]
    pub fn smp7(&mut self) -> SMP_W<SMPR3rs> {
        SMP_W::new(self, 21)
    }
    ///Bits 24:26 - Channel 8 sample time selection
    #[inline(always)]
    pub fn smp8(&mut self) -> SMP_W<SMPR3rs> {
        SMP_W::new(self, 24)
    }
    ///Bits 27:29 - Channel 9 sample time selection
    #[inline(always)]
    pub fn smp9(&mut self) -> SMP_W<SMPR3rs> {
        SMP_W::new(self, 27)
    }
}
/**sample time register 3

You can [`read`](crate::Reg::read) this register and get [`smpr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#ADC:SMPR3)*/
pub struct SMPR3rs;
impl crate::RegisterSpec for SMPR3rs {
    type Ux = u32;
}
///`read()` method returns [`smpr3::R`](R) reader structure
impl crate::Readable for SMPR3rs {}
///`write(|w| ..)` method takes [`smpr3::W`](W) writer structure
impl crate::Writable for SMPR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SMPR3 to value 0
impl crate::Resettable for SMPR3rs {
    const RESET_VALUE: u32 = 0;
}