///Register `SWIER1` reader
pub type R = crate::R<SWIER1rs>;
///Register `SWIER1` writer
pub type W = crate::W<SWIER1rs>;
/**Software interrupt on event x

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFTWARE_INTERRUPT {
    ///1: Generates an interrupt request
    Pend = 1,
}
impl From<SOFTWARE_INTERRUPT> for bool {
    #[inline(always)]
    fn from(variant: SOFTWARE_INTERRUPT) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI0` reader - Software interrupt on event x
pub type SWI0_R = crate::BitReader<SOFTWARE_INTERRUPT>;
impl SWI0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SOFTWARE_INTERRUPT> {
        match self.bits {
            true => Some(SOFTWARE_INTERRUPT::Pend),
            _ => None,
        }
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SOFTWARE_INTERRUPT::Pend
    }
}
///Field `SWI0` writer - Software interrupt on event x
pub type SWI0_W<'a, REG> = crate::BitWriter<'a, REG, SOFTWARE_INTERRUPT>;
impl<'a, REG> SWI0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SOFTWARE_INTERRUPT::Pend)
    }
}
///Field `SWI1` reader - Software interrupt on event x
pub use SWI0_R as SWI1_R;
///Field `SWI2` reader - Software interrupt on event x
pub use SWI0_R as SWI2_R;
///Field `SWI3` reader - Software interrupt on event x
pub use SWI0_R as SWI3_R;
///Field `SWI4` reader - Software interrupt on event x
pub use SWI0_R as SWI4_R;
///Field `SWI5` reader - Software interrupt on event x
pub use SWI0_R as SWI5_R;
///Field `SWI6` reader - Software interrupt on event x
pub use SWI0_R as SWI6_R;
///Field `SWI7` reader - Software interrupt on event x
pub use SWI0_R as SWI7_R;
///Field `SWI8` reader - Software interrupt on event x
pub use SWI0_R as SWI8_R;
///Field `SWI9` reader - Software interrupt on event x
pub use SWI0_R as SWI9_R;
///Field `SWI10` reader - Software interrupt on event x
pub use SWI0_R as SWI10_R;
///Field `SWI11` reader - Software interrupt on event x
pub use SWI0_R as SWI11_R;
///Field `SWI12` reader - Software interrupt on event x
pub use SWI0_R as SWI12_R;
///Field `SWI13` reader - Software interrupt on event x
pub use SWI0_R as SWI13_R;
///Field `SWI14` reader - Software interrupt on event x
pub use SWI0_R as SWI14_R;
///Field `SWI15` reader - Software interrupt on event x
pub use SWI0_R as SWI15_R;
///Field `SWI16` reader - Software interrupt on event x
pub use SWI0_R as SWI16_R;
///Field `SWI1` writer - Software interrupt on event x
pub use SWI0_W as SWI1_W;
///Field `SWI2` writer - Software interrupt on event x
pub use SWI0_W as SWI2_W;
///Field `SWI3` writer - Software interrupt on event x
pub use SWI0_W as SWI3_W;
///Field `SWI4` writer - Software interrupt on event x
pub use SWI0_W as SWI4_W;
///Field `SWI5` writer - Software interrupt on event x
pub use SWI0_W as SWI5_W;
///Field `SWI6` writer - Software interrupt on event x
pub use SWI0_W as SWI6_W;
///Field `SWI7` writer - Software interrupt on event x
pub use SWI0_W as SWI7_W;
///Field `SWI8` writer - Software interrupt on event x
pub use SWI0_W as SWI8_W;
///Field `SWI9` writer - Software interrupt on event x
pub use SWI0_W as SWI9_W;
///Field `SWI10` writer - Software interrupt on event x
pub use SWI0_W as SWI10_W;
///Field `SWI11` writer - Software interrupt on event x
pub use SWI0_W as SWI11_W;
///Field `SWI12` writer - Software interrupt on event x
pub use SWI0_W as SWI12_W;
///Field `SWI13` writer - Software interrupt on event x
pub use SWI0_W as SWI13_W;
///Field `SWI14` writer - Software interrupt on event x
pub use SWI0_W as SWI14_W;
///Field `SWI15` writer - Software interrupt on event x
pub use SWI0_W as SWI15_W;
///Field `SWI16` writer - Software interrupt on event x
pub use SWI0_W as SWI16_W;
impl R {
    ///Bit 0 - Software interrupt on event x
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Software interrupt on event x
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Software interrupt on event x
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Software interrupt on event x
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Software interrupt on event x
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Software interrupt on event x
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Software interrupt on event x
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Software interrupt on event x
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Software interrupt on event x
    #[inline(always)]
    pub fn swi8(&self) -> SWI8_R {
        SWI8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Software interrupt on event x
    #[inline(always)]
    pub fn swi9(&self) -> SWI9_R {
        SWI9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Software interrupt on event x
    #[inline(always)]
    pub fn swi10(&self) -> SWI10_R {
        SWI10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Software interrupt on event x
    #[inline(always)]
    pub fn swi11(&self) -> SWI11_R {
        SWI11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Software interrupt on event x
    #[inline(always)]
    pub fn swi12(&self) -> SWI12_R {
        SWI12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Software interrupt on event x
    #[inline(always)]
    pub fn swi13(&self) -> SWI13_R {
        SWI13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Software interrupt on event x
    #[inline(always)]
    pub fn swi14(&self) -> SWI14_R {
        SWI14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Software interrupt on event x
    #[inline(always)]
    pub fn swi15(&self) -> SWI15_R {
        SWI15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Software interrupt on event x
    #[inline(always)]
    pub fn swi16(&self) -> SWI16_R {
        SWI16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER1")
            .field("swi0", &self.swi0())
            .field("swi1", &self.swi1())
            .field("swi2", &self.swi2())
            .field("swi3", &self.swi3())
            .field("swi4", &self.swi4())
            .field("swi5", &self.swi5())
            .field("swi6", &self.swi6())
            .field("swi7", &self.swi7())
            .field("swi8", &self.swi8())
            .field("swi9", &self.swi9())
            .field("swi10", &self.swi10())
            .field("swi11", &self.swi11())
            .field("swi12", &self.swi12())
            .field("swi13", &self.swi13())
            .field("swi14", &self.swi14())
            .field("swi15", &self.swi15())
            .field("swi16", &self.swi16())
            .finish()
    }
}
impl W {
    ///Bit 0 - Software interrupt on event x
    #[inline(always)]
    pub fn swi0(&mut self) -> SWI0_W<SWIER1rs> {
        SWI0_W::new(self, 0)
    }
    ///Bit 1 - Software interrupt on event x
    #[inline(always)]
    pub fn swi1(&mut self) -> SWI1_W<SWIER1rs> {
        SWI1_W::new(self, 1)
    }
    ///Bit 2 - Software interrupt on event x
    #[inline(always)]
    pub fn swi2(&mut self) -> SWI2_W<SWIER1rs> {
        SWI2_W::new(self, 2)
    }
    ///Bit 3 - Software interrupt on event x
    #[inline(always)]
    pub fn swi3(&mut self) -> SWI3_W<SWIER1rs> {
        SWI3_W::new(self, 3)
    }
    ///Bit 4 - Software interrupt on event x
    #[inline(always)]
    pub fn swi4(&mut self) -> SWI4_W<SWIER1rs> {
        SWI4_W::new(self, 4)
    }
    ///Bit 5 - Software interrupt on event x
    #[inline(always)]
    pub fn swi5(&mut self) -> SWI5_W<SWIER1rs> {
        SWI5_W::new(self, 5)
    }
    ///Bit 6 - Software interrupt on event x
    #[inline(always)]
    pub fn swi6(&mut self) -> SWI6_W<SWIER1rs> {
        SWI6_W::new(self, 6)
    }
    ///Bit 7 - Software interrupt on event x
    #[inline(always)]
    pub fn swi7(&mut self) -> SWI7_W<SWIER1rs> {
        SWI7_W::new(self, 7)
    }
    ///Bit 8 - Software interrupt on event x
    #[inline(always)]
    pub fn swi8(&mut self) -> SWI8_W<SWIER1rs> {
        SWI8_W::new(self, 8)
    }
    ///Bit 9 - Software interrupt on event x
    #[inline(always)]
    pub fn swi9(&mut self) -> SWI9_W<SWIER1rs> {
        SWI9_W::new(self, 9)
    }
    ///Bit 10 - Software interrupt on event x
    #[inline(always)]
    pub fn swi10(&mut self) -> SWI10_W<SWIER1rs> {
        SWI10_W::new(self, 10)
    }
    ///Bit 11 - Software interrupt on event x
    #[inline(always)]
    pub fn swi11(&mut self) -> SWI11_W<SWIER1rs> {
        SWI11_W::new(self, 11)
    }
    ///Bit 12 - Software interrupt on event x
    #[inline(always)]
    pub fn swi12(&mut self) -> SWI12_W<SWIER1rs> {
        SWI12_W::new(self, 12)
    }
    ///Bit 13 - Software interrupt on event x
    #[inline(always)]
    pub fn swi13(&mut self) -> SWI13_W<SWIER1rs> {
        SWI13_W::new(self, 13)
    }
    ///Bit 14 - Software interrupt on event x
    #[inline(always)]
    pub fn swi14(&mut self) -> SWI14_W<SWIER1rs> {
        SWI14_W::new(self, 14)
    }
    ///Bit 15 - Software interrupt on event x
    #[inline(always)]
    pub fn swi15(&mut self) -> SWI15_W<SWIER1rs> {
        SWI15_W::new(self, 15)
    }
    ///Bit 16 - Software interrupt on event x
    #[inline(always)]
    pub fn swi16(&mut self) -> SWI16_W<SWIER1rs> {
        SWI16_W::new(self, 16)
    }
}
/**EXTI software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#EXTI:SWIER1)*/
pub struct SWIER1rs;
impl crate::RegisterSpec for SWIER1rs {
    type Ux = u32;
}
///`read()` method returns [`swier1::R`](R) reader structure
impl crate::Readable for SWIER1rs {}
///`write(|w| ..)` method takes [`swier1::W`](W) writer structure
impl crate::Writable for SWIER1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWIER1 to value 0
impl crate::Resettable for SWIER1rs {}
