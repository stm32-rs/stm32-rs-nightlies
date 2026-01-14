///Register `FTSR1` reader
pub type R = crate::R<FTSR1rs>;
///Register `FTSR1` writer
pub type W = crate::W<FTSR1rs>;
/**Falling trigger event configuration of line 0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FALLING_TRIGGER {
    ///0: Falling edge trigger is disabled
    Disabled = 0,
    ///1: Falling edge trigger is enabled
    Enabled = 1,
}
impl From<FALLING_TRIGGER> for bool {
    #[inline(always)]
    fn from(variant: FALLING_TRIGGER) -> Self {
        variant as u8 != 0
    }
}
///Field `FT0` reader - Falling trigger event configuration of line 0
pub type FT0_R = crate::BitReader<FALLING_TRIGGER>;
impl FT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FALLING_TRIGGER {
        match self.bits {
            false => FALLING_TRIGGER::Disabled,
            true => FALLING_TRIGGER::Enabled,
        }
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FALLING_TRIGGER::Disabled
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FALLING_TRIGGER::Enabled
    }
}
///Field `FT0` writer - Falling trigger event configuration of line 0
pub type FT0_W<'a, REG> = crate::BitWriter<'a, REG, FALLING_TRIGGER>;
impl<'a, REG> FT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FALLING_TRIGGER::Disabled)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FALLING_TRIGGER::Enabled)
    }
}
///Field `FT1` reader - Falling trigger event configuration of line 1
pub use FT0_R as FT1_R;
///Field `FT2` reader - Falling trigger event configuration of line 2
pub use FT0_R as FT2_R;
///Field `FT3` reader - Falling trigger event configuration of line 3
pub use FT0_R as FT3_R;
///Field `FT4` reader - Falling trigger event configuration of line 4
pub use FT0_R as FT4_R;
///Field `FT5` reader - Falling trigger event configuration of line 5
pub use FT0_R as FT5_R;
///Field `FT6` reader - Falling trigger event configuration of line 6
pub use FT0_R as FT6_R;
///Field `FT7` reader - Falling trigger event configuration of line 7
pub use FT0_R as FT7_R;
///Field `FT8` reader - Falling trigger event configuration of line 8
pub use FT0_R as FT8_R;
///Field `FT9` reader - Falling trigger event configuration of line 9
pub use FT0_R as FT9_R;
///Field `FT10` reader - Falling trigger event configuration of line 10
pub use FT0_R as FT10_R;
///Field `FT11` reader - Falling trigger event configuration of line 11
pub use FT0_R as FT11_R;
///Field `FT12` reader - Falling trigger event configuration of line 12
pub use FT0_R as FT12_R;
///Field `FT13` reader - Falling trigger event configuration of line 13
pub use FT0_R as FT13_R;
///Field `FT14` reader - Falling trigger event configuration of line 14
pub use FT0_R as FT14_R;
///Field `FT15` reader - Falling trigger event configuration of line 15
pub use FT0_R as FT15_R;
///Field `FT16` reader - Falling trigger event configuration of line 16
pub use FT0_R as FT16_R;
///Field `FT17` reader - Falling trigger event configuration of line 17
pub use FT0_R as FT17_R;
///Field `FT19` reader - Falling trigger event configuration of line 19
pub use FT0_R as FT19_R;
///Field `FT20` reader - Falling trigger event configuration of line 20
pub use FT0_R as FT20_R;
///Field `FT21` reader - Falling trigger event configuration of line 21
pub use FT0_R as FT21_R;
///Field `FT22` reader - Falling trigger event configuration of line 22
pub use FT0_R as FT22_R;
///Field `FT29` reader - Falling trigger event configuration of line 29
pub use FT0_R as FT29_R;
///Field `FT30` reader - Falling trigger event configuration of line 30
pub use FT0_R as FT30_R;
///Field `FT31` reader - Falling trigger event configuration of line 31
pub use FT0_R as FT31_R;
///Field `FT1` writer - Falling trigger event configuration of line 1
pub use FT0_W as FT1_W;
///Field `FT2` writer - Falling trigger event configuration of line 2
pub use FT0_W as FT2_W;
///Field `FT3` writer - Falling trigger event configuration of line 3
pub use FT0_W as FT3_W;
///Field `FT4` writer - Falling trigger event configuration of line 4
pub use FT0_W as FT4_W;
///Field `FT5` writer - Falling trigger event configuration of line 5
pub use FT0_W as FT5_W;
///Field `FT6` writer - Falling trigger event configuration of line 6
pub use FT0_W as FT6_W;
///Field `FT7` writer - Falling trigger event configuration of line 7
pub use FT0_W as FT7_W;
///Field `FT8` writer - Falling trigger event configuration of line 8
pub use FT0_W as FT8_W;
///Field `FT9` writer - Falling trigger event configuration of line 9
pub use FT0_W as FT9_W;
///Field `FT10` writer - Falling trigger event configuration of line 10
pub use FT0_W as FT10_W;
///Field `FT11` writer - Falling trigger event configuration of line 11
pub use FT0_W as FT11_W;
///Field `FT12` writer - Falling trigger event configuration of line 12
pub use FT0_W as FT12_W;
///Field `FT13` writer - Falling trigger event configuration of line 13
pub use FT0_W as FT13_W;
///Field `FT14` writer - Falling trigger event configuration of line 14
pub use FT0_W as FT14_W;
///Field `FT15` writer - Falling trigger event configuration of line 15
pub use FT0_W as FT15_W;
///Field `FT16` writer - Falling trigger event configuration of line 16
pub use FT0_W as FT16_W;
///Field `FT17` writer - Falling trigger event configuration of line 17
pub use FT0_W as FT17_W;
///Field `FT19` writer - Falling trigger event configuration of line 19
pub use FT0_W as FT19_W;
///Field `FT20` writer - Falling trigger event configuration of line 20
pub use FT0_W as FT20_W;
///Field `FT21` writer - Falling trigger event configuration of line 21
pub use FT0_W as FT21_W;
///Field `FT22` writer - Falling trigger event configuration of line 22
pub use FT0_W as FT22_W;
///Field `FT29` writer - Falling trigger event configuration of line 29
pub use FT0_W as FT29_W;
///Field `FT30` writer - Falling trigger event configuration of line 30
pub use FT0_W as FT30_W;
///Field `FT31` writer - Falling trigger event configuration of line 31
pub use FT0_W as FT31_W;
impl R {
    ///Bit 0 - Falling trigger event configuration of line 0
    #[inline(always)]
    pub fn ft0(&self) -> FT0_R {
        FT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Falling trigger event configuration of line 1
    #[inline(always)]
    pub fn ft1(&self) -> FT1_R {
        FT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Falling trigger event configuration of line 2
    #[inline(always)]
    pub fn ft2(&self) -> FT2_R {
        FT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Falling trigger event configuration of line 3
    #[inline(always)]
    pub fn ft3(&self) -> FT3_R {
        FT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Falling trigger event configuration of line 4
    #[inline(always)]
    pub fn ft4(&self) -> FT4_R {
        FT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Falling trigger event configuration of line 5
    #[inline(always)]
    pub fn ft5(&self) -> FT5_R {
        FT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Falling trigger event configuration of line 6
    #[inline(always)]
    pub fn ft6(&self) -> FT6_R {
        FT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Falling trigger event configuration of line 7
    #[inline(always)]
    pub fn ft7(&self) -> FT7_R {
        FT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Falling trigger event configuration of line 8
    #[inline(always)]
    pub fn ft8(&self) -> FT8_R {
        FT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Falling trigger event configuration of line 9
    #[inline(always)]
    pub fn ft9(&self) -> FT9_R {
        FT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Falling trigger event configuration of line 10
    #[inline(always)]
    pub fn ft10(&self) -> FT10_R {
        FT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Falling trigger event configuration of line 11
    #[inline(always)]
    pub fn ft11(&self) -> FT11_R {
        FT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Falling trigger event configuration of line 12
    #[inline(always)]
    pub fn ft12(&self) -> FT12_R {
        FT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Falling trigger event configuration of line 13
    #[inline(always)]
    pub fn ft13(&self) -> FT13_R {
        FT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Falling trigger event configuration of line 14
    #[inline(always)]
    pub fn ft14(&self) -> FT14_R {
        FT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Falling trigger event configuration of line 15
    #[inline(always)]
    pub fn ft15(&self) -> FT15_R {
        FT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Falling trigger event configuration of line 16
    #[inline(always)]
    pub fn ft16(&self) -> FT16_R {
        FT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Falling trigger event configuration of line 17
    #[inline(always)]
    pub fn ft17(&self) -> FT17_R {
        FT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Falling trigger event configuration of line 19
    #[inline(always)]
    pub fn ft19(&self) -> FT19_R {
        FT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Falling trigger event configuration of line 20
    #[inline(always)]
    pub fn ft20(&self) -> FT20_R {
        FT20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Falling trigger event configuration of line 21
    #[inline(always)]
    pub fn ft21(&self) -> FT21_R {
        FT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Falling trigger event configuration of line 22
    #[inline(always)]
    pub fn ft22(&self) -> FT22_R {
        FT22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 29 - Falling trigger event configuration of line 29
    #[inline(always)]
    pub fn ft29(&self) -> FT29_R {
        FT29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Falling trigger event configuration of line 30
    #[inline(always)]
    pub fn ft30(&self) -> FT30_R {
        FT30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Falling trigger event configuration of line 31
    #[inline(always)]
    pub fn ft31(&self) -> FT31_R {
        FT31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FTSR1")
            .field("ft0", &self.ft0())
            .field("ft1", &self.ft1())
            .field("ft2", &self.ft2())
            .field("ft3", &self.ft3())
            .field("ft4", &self.ft4())
            .field("ft5", &self.ft5())
            .field("ft6", &self.ft6())
            .field("ft7", &self.ft7())
            .field("ft8", &self.ft8())
            .field("ft9", &self.ft9())
            .field("ft10", &self.ft10())
            .field("ft11", &self.ft11())
            .field("ft12", &self.ft12())
            .field("ft13", &self.ft13())
            .field("ft14", &self.ft14())
            .field("ft15", &self.ft15())
            .field("ft16", &self.ft16())
            .field("ft19", &self.ft19())
            .field("ft20", &self.ft20())
            .field("ft21", &self.ft21())
            .field("ft22", &self.ft22())
            .field("ft17", &self.ft17())
            .field("ft29", &self.ft29())
            .field("ft30", &self.ft30())
            .field("ft31", &self.ft31())
            .finish()
    }
}
impl W {
    ///Bit 0 - Falling trigger event configuration of line 0
    #[inline(always)]
    pub fn ft0(&mut self) -> FT0_W<'_, FTSR1rs> {
        FT0_W::new(self, 0)
    }
    ///Bit 1 - Falling trigger event configuration of line 1
    #[inline(always)]
    pub fn ft1(&mut self) -> FT1_W<'_, FTSR1rs> {
        FT1_W::new(self, 1)
    }
    ///Bit 2 - Falling trigger event configuration of line 2
    #[inline(always)]
    pub fn ft2(&mut self) -> FT2_W<'_, FTSR1rs> {
        FT2_W::new(self, 2)
    }
    ///Bit 3 - Falling trigger event configuration of line 3
    #[inline(always)]
    pub fn ft3(&mut self) -> FT3_W<'_, FTSR1rs> {
        FT3_W::new(self, 3)
    }
    ///Bit 4 - Falling trigger event configuration of line 4
    #[inline(always)]
    pub fn ft4(&mut self) -> FT4_W<'_, FTSR1rs> {
        FT4_W::new(self, 4)
    }
    ///Bit 5 - Falling trigger event configuration of line 5
    #[inline(always)]
    pub fn ft5(&mut self) -> FT5_W<'_, FTSR1rs> {
        FT5_W::new(self, 5)
    }
    ///Bit 6 - Falling trigger event configuration of line 6
    #[inline(always)]
    pub fn ft6(&mut self) -> FT6_W<'_, FTSR1rs> {
        FT6_W::new(self, 6)
    }
    ///Bit 7 - Falling trigger event configuration of line 7
    #[inline(always)]
    pub fn ft7(&mut self) -> FT7_W<'_, FTSR1rs> {
        FT7_W::new(self, 7)
    }
    ///Bit 8 - Falling trigger event configuration of line 8
    #[inline(always)]
    pub fn ft8(&mut self) -> FT8_W<'_, FTSR1rs> {
        FT8_W::new(self, 8)
    }
    ///Bit 9 - Falling trigger event configuration of line 9
    #[inline(always)]
    pub fn ft9(&mut self) -> FT9_W<'_, FTSR1rs> {
        FT9_W::new(self, 9)
    }
    ///Bit 10 - Falling trigger event configuration of line 10
    #[inline(always)]
    pub fn ft10(&mut self) -> FT10_W<'_, FTSR1rs> {
        FT10_W::new(self, 10)
    }
    ///Bit 11 - Falling trigger event configuration of line 11
    #[inline(always)]
    pub fn ft11(&mut self) -> FT11_W<'_, FTSR1rs> {
        FT11_W::new(self, 11)
    }
    ///Bit 12 - Falling trigger event configuration of line 12
    #[inline(always)]
    pub fn ft12(&mut self) -> FT12_W<'_, FTSR1rs> {
        FT12_W::new(self, 12)
    }
    ///Bit 13 - Falling trigger event configuration of line 13
    #[inline(always)]
    pub fn ft13(&mut self) -> FT13_W<'_, FTSR1rs> {
        FT13_W::new(self, 13)
    }
    ///Bit 14 - Falling trigger event configuration of line 14
    #[inline(always)]
    pub fn ft14(&mut self) -> FT14_W<'_, FTSR1rs> {
        FT14_W::new(self, 14)
    }
    ///Bit 15 - Falling trigger event configuration of line 15
    #[inline(always)]
    pub fn ft15(&mut self) -> FT15_W<'_, FTSR1rs> {
        FT15_W::new(self, 15)
    }
    ///Bit 16 - Falling trigger event configuration of line 16
    #[inline(always)]
    pub fn ft16(&mut self) -> FT16_W<'_, FTSR1rs> {
        FT16_W::new(self, 16)
    }
    ///Bit 17 - Falling trigger event configuration of line 17
    #[inline(always)]
    pub fn ft17(&mut self) -> FT17_W<'_, FTSR1rs> {
        FT17_W::new(self, 17)
    }
    ///Bit 19 - Falling trigger event configuration of line 19
    #[inline(always)]
    pub fn ft19(&mut self) -> FT19_W<'_, FTSR1rs> {
        FT19_W::new(self, 19)
    }
    ///Bit 20 - Falling trigger event configuration of line 20
    #[inline(always)]
    pub fn ft20(&mut self) -> FT20_W<'_, FTSR1rs> {
        FT20_W::new(self, 20)
    }
    ///Bit 21 - Falling trigger event configuration of line 21
    #[inline(always)]
    pub fn ft21(&mut self) -> FT21_W<'_, FTSR1rs> {
        FT21_W::new(self, 21)
    }
    ///Bit 22 - Falling trigger event configuration of line 22
    #[inline(always)]
    pub fn ft22(&mut self) -> FT22_W<'_, FTSR1rs> {
        FT22_W::new(self, 22)
    }
    ///Bit 29 - Falling trigger event configuration of line 29
    #[inline(always)]
    pub fn ft29(&mut self) -> FT29_W<'_, FTSR1rs> {
        FT29_W::new(self, 29)
    }
    ///Bit 30 - Falling trigger event configuration of line 30
    #[inline(always)]
    pub fn ft30(&mut self) -> FT30_W<'_, FTSR1rs> {
        FT30_W::new(self, 30)
    }
    ///Bit 31 - Falling trigger event configuration of line 31
    #[inline(always)]
    pub fn ft31(&mut self) -> FT31_W<'_, FTSR1rs> {
        FT31_W::new(self, 31)
    }
}
/**Falling Trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#EXTI:FTSR1)*/
pub struct FTSR1rs;
impl crate::RegisterSpec for FTSR1rs {
    type Ux = u32;
}
///`read()` method returns [`ftsr1::R`](R) reader structure
impl crate::Readable for FTSR1rs {}
///`write(|w| ..)` method takes [`ftsr1::W`](W) writer structure
impl crate::Writable for FTSR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FTSR1 to value 0
impl crate::Resettable for FTSR1rs {}
