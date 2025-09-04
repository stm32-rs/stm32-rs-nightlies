///Register `SWIER` reader
pub type R = crate::R<SWIERrs>;
///Register `SWIER` writer
pub type W = crate::W<SWIERrs>;
/**Software Interrupt on line 0

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
///Field `SWI0` reader - Software Interrupt on line 0
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
///Field `SWI0` writer - Software Interrupt on line 0
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
///Field `SWI1` reader - Software Interrupt on line 1
pub use SWI0_R as SWI1_R;
///Field `SWI2` reader - Software Interrupt on line 2
pub use SWI0_R as SWI2_R;
///Field `SWI3` reader - Software Interrupt on line 3
pub use SWI0_R as SWI3_R;
///Field `SWI4` reader - Software Interrupt on line 4
pub use SWI0_R as SWI4_R;
///Field `SWI5` reader - Software Interrupt on line 5
pub use SWI0_R as SWI5_R;
///Field `SWI6` reader - Software Interrupt on line 6
pub use SWI0_R as SWI6_R;
///Field `SWI7` reader - Software Interrupt on line 7
pub use SWI0_R as SWI7_R;
///Field `SWI8` reader - Software Interrupt on line 8
pub use SWI0_R as SWI8_R;
///Field `SWI9` reader - Software Interrupt on line 9
pub use SWI0_R as SWI9_R;
///Field `SWI10` reader - Software Interrupt on line 10
pub use SWI0_R as SWI10_R;
///Field `SWI11` reader - Software Interrupt on line 11
pub use SWI0_R as SWI11_R;
///Field `SWI12` reader - Software Interrupt on line 12
pub use SWI0_R as SWI12_R;
///Field `SWI13` reader - Software Interrupt on line 13
pub use SWI0_R as SWI13_R;
///Field `SWI14` reader - Software Interrupt on line 14
pub use SWI0_R as SWI14_R;
///Field `SWI15` reader - Software Interrupt on line 15
pub use SWI0_R as SWI15_R;
///Field `SWI16` reader - Software Interrupt on line 16
pub use SWI0_R as SWI16_R;
///Field `SWI17` reader - Software Interrupt on line 17
pub use SWI0_R as SWI17_R;
///Field `SWI19` reader - Software Interrupt on line 19
pub use SWI0_R as SWI19_R;
///Field `SWI20` reader - Software Interrupt on line 20
pub use SWI0_R as SWI20_R;
///Field `SWI21` reader - Software Interrupt on line 21
pub use SWI0_R as SWI21_R;
///Field `SWI22` reader - Software Interrupt on line 22
pub use SWI0_R as SWI22_R;
///Field `SWI1` writer - Software Interrupt on line 1
pub use SWI0_W as SWI1_W;
///Field `SWI2` writer - Software Interrupt on line 2
pub use SWI0_W as SWI2_W;
///Field `SWI3` writer - Software Interrupt on line 3
pub use SWI0_W as SWI3_W;
///Field `SWI4` writer - Software Interrupt on line 4
pub use SWI0_W as SWI4_W;
///Field `SWI5` writer - Software Interrupt on line 5
pub use SWI0_W as SWI5_W;
///Field `SWI6` writer - Software Interrupt on line 6
pub use SWI0_W as SWI6_W;
///Field `SWI7` writer - Software Interrupt on line 7
pub use SWI0_W as SWI7_W;
///Field `SWI8` writer - Software Interrupt on line 8
pub use SWI0_W as SWI8_W;
///Field `SWI9` writer - Software Interrupt on line 9
pub use SWI0_W as SWI9_W;
///Field `SWI10` writer - Software Interrupt on line 10
pub use SWI0_W as SWI10_W;
///Field `SWI11` writer - Software Interrupt on line 11
pub use SWI0_W as SWI11_W;
///Field `SWI12` writer - Software Interrupt on line 12
pub use SWI0_W as SWI12_W;
///Field `SWI13` writer - Software Interrupt on line 13
pub use SWI0_W as SWI13_W;
///Field `SWI14` writer - Software Interrupt on line 14
pub use SWI0_W as SWI14_W;
///Field `SWI15` writer - Software Interrupt on line 15
pub use SWI0_W as SWI15_W;
///Field `SWI16` writer - Software Interrupt on line 16
pub use SWI0_W as SWI16_W;
///Field `SWI17` writer - Software Interrupt on line 17
pub use SWI0_W as SWI17_W;
///Field `SWI19` writer - Software Interrupt on line 19
pub use SWI0_W as SWI19_W;
///Field `SWI20` writer - Software Interrupt on line 20
pub use SWI0_W as SWI20_W;
///Field `SWI21` writer - Software Interrupt on line 21
pub use SWI0_W as SWI21_W;
///Field `SWI22` writer - Software Interrupt on line 22
pub use SWI0_W as SWI22_W;
impl R {
    ///Bit 0 - Software Interrupt on line 0
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Software Interrupt on line 1
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Software Interrupt on line 2
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Software Interrupt on line 3
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Software Interrupt on line 4
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Software Interrupt on line 5
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Software Interrupt on line 6
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Software Interrupt on line 7
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Software Interrupt on line 8
    #[inline(always)]
    pub fn swi8(&self) -> SWI8_R {
        SWI8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Software Interrupt on line 9
    #[inline(always)]
    pub fn swi9(&self) -> SWI9_R {
        SWI9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Software Interrupt on line 10
    #[inline(always)]
    pub fn swi10(&self) -> SWI10_R {
        SWI10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Software Interrupt on line 11
    #[inline(always)]
    pub fn swi11(&self) -> SWI11_R {
        SWI11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Software Interrupt on line 12
    #[inline(always)]
    pub fn swi12(&self) -> SWI12_R {
        SWI12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Software Interrupt on line 13
    #[inline(always)]
    pub fn swi13(&self) -> SWI13_R {
        SWI13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Software Interrupt on line 14
    #[inline(always)]
    pub fn swi14(&self) -> SWI14_R {
        SWI14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Software Interrupt on line 15
    #[inline(always)]
    pub fn swi15(&self) -> SWI15_R {
        SWI15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Software Interrupt on line 16
    #[inline(always)]
    pub fn swi16(&self) -> SWI16_R {
        SWI16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Software Interrupt on line 17
    #[inline(always)]
    pub fn swi17(&self) -> SWI17_R {
        SWI17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Software Interrupt on line 19
    #[inline(always)]
    pub fn swi19(&self) -> SWI19_R {
        SWI19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Software Interrupt on line 20
    #[inline(always)]
    pub fn swi20(&self) -> SWI20_R {
        SWI20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Software Interrupt on line 21
    #[inline(always)]
    pub fn swi21(&self) -> SWI21_R {
        SWI21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Software Interrupt on line 22
    #[inline(always)]
    pub fn swi22(&self) -> SWI22_R {
        SWI22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER")
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
            .field("swi17", &self.swi17())
            .field("swi19", &self.swi19())
            .field("swi20", &self.swi20())
            .field("swi21", &self.swi21())
            .field("swi22", &self.swi22())
            .finish()
    }
}
impl W {
    ///Bit 0 - Software Interrupt on line 0
    #[inline(always)]
    pub fn swi0(&mut self) -> SWI0_W<SWIERrs> {
        SWI0_W::new(self, 0)
    }
    ///Bit 1 - Software Interrupt on line 1
    #[inline(always)]
    pub fn swi1(&mut self) -> SWI1_W<SWIERrs> {
        SWI1_W::new(self, 1)
    }
    ///Bit 2 - Software Interrupt on line 2
    #[inline(always)]
    pub fn swi2(&mut self) -> SWI2_W<SWIERrs> {
        SWI2_W::new(self, 2)
    }
    ///Bit 3 - Software Interrupt on line 3
    #[inline(always)]
    pub fn swi3(&mut self) -> SWI3_W<SWIERrs> {
        SWI3_W::new(self, 3)
    }
    ///Bit 4 - Software Interrupt on line 4
    #[inline(always)]
    pub fn swi4(&mut self) -> SWI4_W<SWIERrs> {
        SWI4_W::new(self, 4)
    }
    ///Bit 5 - Software Interrupt on line 5
    #[inline(always)]
    pub fn swi5(&mut self) -> SWI5_W<SWIERrs> {
        SWI5_W::new(self, 5)
    }
    ///Bit 6 - Software Interrupt on line 6
    #[inline(always)]
    pub fn swi6(&mut self) -> SWI6_W<SWIERrs> {
        SWI6_W::new(self, 6)
    }
    ///Bit 7 - Software Interrupt on line 7
    #[inline(always)]
    pub fn swi7(&mut self) -> SWI7_W<SWIERrs> {
        SWI7_W::new(self, 7)
    }
    ///Bit 8 - Software Interrupt on line 8
    #[inline(always)]
    pub fn swi8(&mut self) -> SWI8_W<SWIERrs> {
        SWI8_W::new(self, 8)
    }
    ///Bit 9 - Software Interrupt on line 9
    #[inline(always)]
    pub fn swi9(&mut self) -> SWI9_W<SWIERrs> {
        SWI9_W::new(self, 9)
    }
    ///Bit 10 - Software Interrupt on line 10
    #[inline(always)]
    pub fn swi10(&mut self) -> SWI10_W<SWIERrs> {
        SWI10_W::new(self, 10)
    }
    ///Bit 11 - Software Interrupt on line 11
    #[inline(always)]
    pub fn swi11(&mut self) -> SWI11_W<SWIERrs> {
        SWI11_W::new(self, 11)
    }
    ///Bit 12 - Software Interrupt on line 12
    #[inline(always)]
    pub fn swi12(&mut self) -> SWI12_W<SWIERrs> {
        SWI12_W::new(self, 12)
    }
    ///Bit 13 - Software Interrupt on line 13
    #[inline(always)]
    pub fn swi13(&mut self) -> SWI13_W<SWIERrs> {
        SWI13_W::new(self, 13)
    }
    ///Bit 14 - Software Interrupt on line 14
    #[inline(always)]
    pub fn swi14(&mut self) -> SWI14_W<SWIERrs> {
        SWI14_W::new(self, 14)
    }
    ///Bit 15 - Software Interrupt on line 15
    #[inline(always)]
    pub fn swi15(&mut self) -> SWI15_W<SWIERrs> {
        SWI15_W::new(self, 15)
    }
    ///Bit 16 - Software Interrupt on line 16
    #[inline(always)]
    pub fn swi16(&mut self) -> SWI16_W<SWIERrs> {
        SWI16_W::new(self, 16)
    }
    ///Bit 17 - Software Interrupt on line 17
    #[inline(always)]
    pub fn swi17(&mut self) -> SWI17_W<SWIERrs> {
        SWI17_W::new(self, 17)
    }
    ///Bit 19 - Software Interrupt on line 19
    #[inline(always)]
    pub fn swi19(&mut self) -> SWI19_W<SWIERrs> {
        SWI19_W::new(self, 19)
    }
    ///Bit 20 - Software Interrupt on line 20
    #[inline(always)]
    pub fn swi20(&mut self) -> SWI20_W<SWIERrs> {
        SWI20_W::new(self, 20)
    }
    ///Bit 21 - Software Interrupt on line 21
    #[inline(always)]
    pub fn swi21(&mut self) -> SWI21_W<SWIERrs> {
        SWI21_W::new(self, 21)
    }
    ///Bit 22 - Software Interrupt on line 22
    #[inline(always)]
    pub fn swi22(&mut self) -> SWI22_W<SWIERrs> {
        SWI22_W::new(self, 22)
    }
}
/**Software interrupt event register (EXTI_SWIER)

You can [`read`](crate::Reg::read) this register and get [`swier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#EXTI:SWIER)*/
pub struct SWIERrs;
impl crate::RegisterSpec for SWIERrs {
    type Ux = u32;
}
///`read()` method returns [`swier::R`](R) reader structure
impl crate::Readable for SWIERrs {}
///`write(|w| ..)` method takes [`swier::W`](W) writer structure
impl crate::Writable for SWIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWIER to value 0
impl crate::Resettable for SWIERrs {}
