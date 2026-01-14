///Register `SMPR0` reader
pub type R = crate::R<SMPR0rs>;
///Register `SMPR0` writer
pub type W = crate::W<SMPR0rs>;
///Field `SMP(30-31)` reader - Channel %s sample time selection
pub type SMP_R = crate::FieldReader;
///Field `SMP(30-31)` writer - Channel %s sample time selection
pub type SMP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Channel (30-31) sample time selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SMP30` field.</div>
    #[inline(always)]
    pub fn smp(&self, n: u8) -> SMP_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SMP_R::new(((self.bits >> (n * 3)) & 7) as u8)
    }
    ///Iterator for array of:
    ///Channel (30-31) sample time selection
    #[inline(always)]
    pub fn smp_iter(&self) -> impl Iterator<Item = SMP_R> + '_ {
        (0..2).map(move |n| SMP_R::new(((self.bits >> (n * 3)) & 7) as u8))
    }
    ///Bits 0:2 - Channel 30 sample time selection
    #[inline(always)]
    pub fn smp30(&self) -> SMP_R {
        SMP_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - Channel 31 sample time selection
    #[inline(always)]
    pub fn smp31(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR0")
            .field("smp30", &self.smp30())
            .field("smp31", &self.smp31())
            .finish()
    }
}
impl W {
    ///Channel (30-31) sample time selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SMP30` field.</div>
    #[inline(always)]
    pub fn smp(&mut self, n: u8) -> SMP_W<'_, SMPR0rs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SMP_W::new(self, n * 3)
    }
    ///Bits 0:2 - Channel 30 sample time selection
    #[inline(always)]
    pub fn smp30(&mut self) -> SMP_W<'_, SMPR0rs> {
        SMP_W::new(self, 0)
    }
    ///Bits 3:5 - Channel 31 sample time selection
    #[inline(always)]
    pub fn smp31(&mut self) -> SMP_W<'_, SMPR0rs> {
        SMP_W::new(self, 3)
    }
}
/**sample time register 0

You can [`read`](crate::Reg::read) this register and get [`smpr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#ADC:SMPR0)*/
pub struct SMPR0rs;
impl crate::RegisterSpec for SMPR0rs {
    type Ux = u32;
}
///`read()` method returns [`smpr0::R`](R) reader structure
impl crate::Readable for SMPR0rs {}
///`write(|w| ..)` method takes [`smpr0::W`](W) writer structure
impl crate::Writable for SMPR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMPR0 to value 0
impl crate::Resettable for SMPR0rs {}
