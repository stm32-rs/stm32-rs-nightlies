///Register `SMPR2` reader
pub type R = crate::R<SMPR2rs>;
///Register `SMPR2` writer
pub type W = crate::W<SMPR2rs>;
/**Channel %s sample time selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP10 {
    ///0: 5 ADC clock cycles
    Cycles5 = 0,
    ///1: 6 ADC clock cycles
    Cycles6 = 1,
    ///2: 12 ADC clock cycles
    Cycles12 = 2,
    ///3: 20 ADC clock cycles
    Cycles20 = 3,
    ///4: 36 ADC clock cycles
    Cycles36 = 4,
    ///5: 68 ADC clock cycles
    Cycles68 = 5,
    ///6: 391 ADC clock cycles
    Cycles391 = 6,
    ///7: 814 ADC clock cycles
    Cycles814 = 7,
}
impl From<SMP10> for u8 {
    #[inline(always)]
    fn from(variant: SMP10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP10 {
    type Ux = u8;
}
impl crate::IsEnum for SMP10 {}
///Field `SMP(10-19)` reader - Channel %s sample time selection
pub type SMP_R = crate::FieldReader<SMP10>;
impl SMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMP10 {
        match self.bits {
            0 => SMP10::Cycles5,
            1 => SMP10::Cycles6,
            2 => SMP10::Cycles12,
            3 => SMP10::Cycles20,
            4 => SMP10::Cycles36,
            5 => SMP10::Cycles68,
            6 => SMP10::Cycles391,
            7 => SMP10::Cycles814,
            _ => unreachable!(),
        }
    }
    ///5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles5(&self) -> bool {
        *self == SMP10::Cycles5
    }
    ///6 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles6(&self) -> bool {
        *self == SMP10::Cycles6
    }
    ///12 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles12(&self) -> bool {
        *self == SMP10::Cycles12
    }
    ///20 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles20(&self) -> bool {
        *self == SMP10::Cycles20
    }
    ///36 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles36(&self) -> bool {
        *self == SMP10::Cycles36
    }
    ///68 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles68(&self) -> bool {
        *self == SMP10::Cycles68
    }
    ///391 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles391(&self) -> bool {
        *self == SMP10::Cycles391
    }
    ///814 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles814(&self) -> bool {
        *self == SMP10::Cycles814
    }
}
///Field `SMP(10-19)` writer - Channel %s sample time selection
pub type SMP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP10, crate::Safe>;
impl<'a, REG> SMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///5 ADC clock cycles
    #[inline(always)]
    pub fn cycles5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles5)
    }
    ///6 ADC clock cycles
    #[inline(always)]
    pub fn cycles6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles6)
    }
    ///12 ADC clock cycles
    #[inline(always)]
    pub fn cycles12(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles12)
    }
    ///20 ADC clock cycles
    #[inline(always)]
    pub fn cycles20(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles20)
    }
    ///36 ADC clock cycles
    #[inline(always)]
    pub fn cycles36(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles36)
    }
    ///68 ADC clock cycles
    #[inline(always)]
    pub fn cycles68(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles68)
    }
    ///391 ADC clock cycles
    #[inline(always)]
    pub fn cycles391(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles391)
    }
    ///814 ADC clock cycles
    #[inline(always)]
    pub fn cycles814(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10::Cycles814)
    }
}
impl R {
    ///Channel (10-19) sample time selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SMP10` field.</div>
    #[inline(always)]
    pub fn smp(&self, n: u8) -> SMP_R {
        #[allow(clippy::no_effect)]
        [(); 10][n as usize];
        SMP_R::new(((self.bits >> (n * 3)) & 7) as u8)
    }
    ///Iterator for array of:
    ///Channel (10-19) sample time selection
    #[inline(always)]
    pub fn smp_iter(&self) -> impl Iterator<Item = SMP_R> + '_ {
        (0..10).map(move |n| SMP_R::new(((self.bits >> (n * 3)) & 7) as u8))
    }
    ///Bits 0:2 - Channel 10 sample time selection
    #[inline(always)]
    pub fn smp10(&self) -> SMP_R {
        SMP_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - Channel 11 sample time selection
    #[inline(always)]
    pub fn smp11(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - Channel 12 sample time selection
    #[inline(always)]
    pub fn smp12(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - Channel 13 sample time selection
    #[inline(always)]
    pub fn smp13(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - Channel 14 sample time selection
    #[inline(always)]
    pub fn smp14(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - Channel 15 sample time selection
    #[inline(always)]
    pub fn smp15(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - Channel 16 sample time selection
    #[inline(always)]
    pub fn smp16(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - Channel 17 sample time selection
    #[inline(always)]
    pub fn smp17(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:26 - Channel 18 sample time selection
    #[inline(always)]
    pub fn smp18(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - Channel 19 sample time selection
    #[inline(always)]
    pub fn smp19(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR2")
            .field("smp10", &self.smp10())
            .field("smp11", &self.smp11())
            .field("smp12", &self.smp12())
            .field("smp13", &self.smp13())
            .field("smp14", &self.smp14())
            .field("smp15", &self.smp15())
            .field("smp16", &self.smp16())
            .field("smp17", &self.smp17())
            .field("smp18", &self.smp18())
            .field("smp19", &self.smp19())
            .finish()
    }
}
impl W {
    ///Channel (10-19) sample time selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SMP10` field.</div>
    #[inline(always)]
    pub fn smp(&mut self, n: u8) -> SMP_W<SMPR2rs> {
        #[allow(clippy::no_effect)]
        [(); 10][n as usize];
        SMP_W::new(self, n * 3)
    }
    ///Bits 0:2 - Channel 10 sample time selection
    #[inline(always)]
    pub fn smp10(&mut self) -> SMP_W<SMPR2rs> {
        SMP_W::new(self, 0)
    }
    ///Bits 3:5 - Channel 11 sample time selection
    #[inline(always)]
    pub fn smp11(&mut self) -> SMP_W<SMPR2rs> {
        SMP_W::new(self, 3)
    }
    ///Bits 6:8 - Channel 12 sample time selection
    #[inline(always)]
    pub fn smp12(&mut self) -> SMP_W<SMPR2rs> {
        SMP_W::new(self, 6)
    }
    ///Bits 9:11 - Channel 13 sample time selection
    #[inline(always)]
    pub fn smp13(&mut self) -> SMP_W<SMPR2rs> {
        SMP_W::new(self, 9)
    }
    ///Bits 12:14 - Channel 14 sample time selection
    #[inline(always)]
    pub fn smp14(&mut self) -> SMP_W<SMPR2rs> {
        SMP_W::new(self, 12)
    }
    ///Bits 15:17 - Channel 15 sample time selection
    #[inline(always)]
    pub fn smp15(&mut self) -> SMP_W<SMPR2rs> {
        SMP_W::new(self, 15)
    }
    ///Bits 18:20 - Channel 16 sample time selection
    #[inline(always)]
    pub fn smp16(&mut self) -> SMP_W<SMPR2rs> {
        SMP_W::new(self, 18)
    }
    ///Bits 21:23 - Channel 17 sample time selection
    #[inline(always)]
    pub fn smp17(&mut self) -> SMP_W<SMPR2rs> {
        SMP_W::new(self, 21)
    }
    ///Bits 24:26 - Channel 18 sample time selection
    #[inline(always)]
    pub fn smp18(&mut self) -> SMP_W<SMPR2rs> {
        SMP_W::new(self, 24)
    }
    ///Bits 27:29 - Channel 19 sample time selection
    #[inline(always)]
    pub fn smp19(&mut self) -> SMP_W<SMPR2rs> {
        SMP_W::new(self, 27)
    }
}
/**ADC sample time register 2

You can [`read`](crate::Reg::read) this register and get [`smpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#ADC1:SMPR2)*/
pub struct SMPR2rs;
impl crate::RegisterSpec for SMPR2rs {
    type Ux = u32;
}
///`read()` method returns [`smpr2::R`](R) reader structure
impl crate::Readable for SMPR2rs {}
///`write(|w| ..)` method takes [`smpr2::W`](W) writer structure
impl crate::Writable for SMPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SMPR2 to value 0
impl crate::Resettable for SMPR2rs {
    const RESET_VALUE: u32 = 0;
}