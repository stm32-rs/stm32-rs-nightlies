///Register `CHSELRMOD0` reader
pub type R = crate::R<CHSELRMOD0rs>;
///Register `CHSELRMOD0` writer
pub type W = crate::W<CHSELRMOD0rs>;
/**CHSEL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL0 {
    ///0: Input channel x is not selected for conversion
    Disabled = 0,
    ///1: Input channel x is selected for conversion
    Enabled = 1,
}
impl From<CHSEL0> for bool {
    #[inline(always)]
    fn from(variant: CHSEL0) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL0` reader - CHSEL
pub type CHSEL0_R = crate::BitReader<CHSEL0>;
impl CHSEL0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL0 {
        match self.bits {
            false => CHSEL0::Disabled,
            true => CHSEL0::Enabled,
        }
    }
    ///Input channel x is not selected for conversion
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHSEL0::Disabled
    }
    ///Input channel x is selected for conversion
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHSEL0::Enabled
    }
}
///Field `CHSEL0` writer - CHSEL
pub type CHSEL0_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL0>;
impl<'a, REG> CHSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input channel x is not selected for conversion
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL0::Disabled)
    }
    ///Input channel x is selected for conversion
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL0::Enabled)
    }
}
///Field `CHSEL1` reader - CHSEL
pub use CHSEL0_R as CHSEL1_R;
///Field `CHSEL2` reader - CHSEL
pub use CHSEL0_R as CHSEL2_R;
///Field `CHSEL3` reader - CHSEL
pub use CHSEL0_R as CHSEL3_R;
///Field `CHSEL4` reader - CHSEL
pub use CHSEL0_R as CHSEL4_R;
///Field `CHSEL5` reader - CHSEL
pub use CHSEL0_R as CHSEL5_R;
///Field `CHSEL6` reader - CHSEL
pub use CHSEL0_R as CHSEL6_R;
///Field `CHSEL7` reader - CHSEL
pub use CHSEL0_R as CHSEL7_R;
///Field `CHSEL8` reader - CHSEL
pub use CHSEL0_R as CHSEL8_R;
///Field `CHSEL9` reader - CHSEL
pub use CHSEL0_R as CHSEL9_R;
///Field `CHSEL10` reader - CHSEL
pub use CHSEL0_R as CHSEL10_R;
///Field `CHSEL11` reader - CHSEL
pub use CHSEL0_R as CHSEL11_R;
///Field `CHSEL12` reader - CHSEL
pub use CHSEL0_R as CHSEL12_R;
///Field `CHSEL13` reader - CHSEL
pub use CHSEL0_R as CHSEL13_R;
///Field `CHSEL14` reader - CHSEL
pub use CHSEL0_R as CHSEL14_R;
///Field `CHSEL15` reader - CHSEL
pub use CHSEL0_R as CHSEL15_R;
///Field `CHSEL16` reader - CHSEL
pub use CHSEL0_R as CHSEL16_R;
///Field `CHSEL17` reader - CHSEL
pub use CHSEL0_R as CHSEL17_R;
///Field `CHSEL18` reader - CHSEL
pub use CHSEL0_R as CHSEL18_R;
///Field `CHSEL19` reader - CHSEL
pub use CHSEL0_R as CHSEL19_R;
///Field `CHSEL20` reader - CHSEL
pub use CHSEL0_R as CHSEL20_R;
///Field `CHSEL21` reader - CHSEL
pub use CHSEL0_R as CHSEL21_R;
///Field `CHSEL22` reader - CHSEL
pub use CHSEL0_R as CHSEL22_R;
///Field `CHSEL23` reader - CHSEL
pub use CHSEL0_R as CHSEL23_R;
///Field `CHSEL1` writer - CHSEL
pub use CHSEL0_W as CHSEL1_W;
///Field `CHSEL2` writer - CHSEL
pub use CHSEL0_W as CHSEL2_W;
///Field `CHSEL3` writer - CHSEL
pub use CHSEL0_W as CHSEL3_W;
///Field `CHSEL4` writer - CHSEL
pub use CHSEL0_W as CHSEL4_W;
///Field `CHSEL5` writer - CHSEL
pub use CHSEL0_W as CHSEL5_W;
///Field `CHSEL6` writer - CHSEL
pub use CHSEL0_W as CHSEL6_W;
///Field `CHSEL7` writer - CHSEL
pub use CHSEL0_W as CHSEL7_W;
///Field `CHSEL8` writer - CHSEL
pub use CHSEL0_W as CHSEL8_W;
///Field `CHSEL9` writer - CHSEL
pub use CHSEL0_W as CHSEL9_W;
///Field `CHSEL10` writer - CHSEL
pub use CHSEL0_W as CHSEL10_W;
///Field `CHSEL11` writer - CHSEL
pub use CHSEL0_W as CHSEL11_W;
///Field `CHSEL12` writer - CHSEL
pub use CHSEL0_W as CHSEL12_W;
///Field `CHSEL13` writer - CHSEL
pub use CHSEL0_W as CHSEL13_W;
///Field `CHSEL14` writer - CHSEL
pub use CHSEL0_W as CHSEL14_W;
///Field `CHSEL15` writer - CHSEL
pub use CHSEL0_W as CHSEL15_W;
///Field `CHSEL16` writer - CHSEL
pub use CHSEL0_W as CHSEL16_W;
///Field `CHSEL17` writer - CHSEL
pub use CHSEL0_W as CHSEL17_W;
///Field `CHSEL18` writer - CHSEL
pub use CHSEL0_W as CHSEL18_W;
///Field `CHSEL19` writer - CHSEL
pub use CHSEL0_W as CHSEL19_W;
///Field `CHSEL20` writer - CHSEL
pub use CHSEL0_W as CHSEL20_W;
///Field `CHSEL21` writer - CHSEL
pub use CHSEL0_W as CHSEL21_W;
///Field `CHSEL22` writer - CHSEL
pub use CHSEL0_W as CHSEL22_W;
///Field `CHSEL23` writer - CHSEL
pub use CHSEL0_W as CHSEL23_W;
impl R {
    ///Bit 0 - CHSEL
    #[inline(always)]
    pub fn chsel0(&self) -> CHSEL0_R {
        CHSEL0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CHSEL
    #[inline(always)]
    pub fn chsel1(&self) -> CHSEL1_R {
        CHSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CHSEL
    #[inline(always)]
    pub fn chsel2(&self) -> CHSEL2_R {
        CHSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CHSEL
    #[inline(always)]
    pub fn chsel3(&self) -> CHSEL3_R {
        CHSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CHSEL
    #[inline(always)]
    pub fn chsel4(&self) -> CHSEL4_R {
        CHSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CHSEL
    #[inline(always)]
    pub fn chsel5(&self) -> CHSEL5_R {
        CHSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CHSEL
    #[inline(always)]
    pub fn chsel6(&self) -> CHSEL6_R {
        CHSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CHSEL
    #[inline(always)]
    pub fn chsel7(&self) -> CHSEL7_R {
        CHSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CHSEL
    #[inline(always)]
    pub fn chsel8(&self) -> CHSEL8_R {
        CHSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CHSEL
    #[inline(always)]
    pub fn chsel9(&self) -> CHSEL9_R {
        CHSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CHSEL
    #[inline(always)]
    pub fn chsel10(&self) -> CHSEL10_R {
        CHSEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CHSEL
    #[inline(always)]
    pub fn chsel11(&self) -> CHSEL11_R {
        CHSEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CHSEL
    #[inline(always)]
    pub fn chsel12(&self) -> CHSEL12_R {
        CHSEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CHSEL
    #[inline(always)]
    pub fn chsel13(&self) -> CHSEL13_R {
        CHSEL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CHSEL
    #[inline(always)]
    pub fn chsel14(&self) -> CHSEL14_R {
        CHSEL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CHSEL
    #[inline(always)]
    pub fn chsel15(&self) -> CHSEL15_R {
        CHSEL15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CHSEL
    #[inline(always)]
    pub fn chsel16(&self) -> CHSEL16_R {
        CHSEL16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CHSEL
    #[inline(always)]
    pub fn chsel17(&self) -> CHSEL17_R {
        CHSEL17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CHSEL
    #[inline(always)]
    pub fn chsel18(&self) -> CHSEL18_R {
        CHSEL18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CHSEL
    #[inline(always)]
    pub fn chsel19(&self) -> CHSEL19_R {
        CHSEL19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - CHSEL
    #[inline(always)]
    pub fn chsel20(&self) -> CHSEL20_R {
        CHSEL20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CHSEL
    #[inline(always)]
    pub fn chsel21(&self) -> CHSEL21_R {
        CHSEL21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - CHSEL
    #[inline(always)]
    pub fn chsel22(&self) -> CHSEL22_R {
        CHSEL22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CHSEL
    #[inline(always)]
    pub fn chsel23(&self) -> CHSEL23_R {
        CHSEL23_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHSELRMOD0")
            .field("chsel0", &self.chsel0())
            .field("chsel1", &self.chsel1())
            .field("chsel2", &self.chsel2())
            .field("chsel3", &self.chsel3())
            .field("chsel4", &self.chsel4())
            .field("chsel5", &self.chsel5())
            .field("chsel6", &self.chsel6())
            .field("chsel7", &self.chsel7())
            .field("chsel8", &self.chsel8())
            .field("chsel9", &self.chsel9())
            .field("chsel10", &self.chsel10())
            .field("chsel11", &self.chsel11())
            .field("chsel12", &self.chsel12())
            .field("chsel13", &self.chsel13())
            .field("chsel14", &self.chsel14())
            .field("chsel15", &self.chsel15())
            .field("chsel16", &self.chsel16())
            .field("chsel17", &self.chsel17())
            .field("chsel18", &self.chsel18())
            .field("chsel19", &self.chsel19())
            .field("chsel20", &self.chsel20())
            .field("chsel21", &self.chsel21())
            .field("chsel22", &self.chsel22())
            .field("chsel23", &self.chsel23())
            .finish()
    }
}
impl W {
    ///Bit 0 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel0(&mut self) -> CHSEL0_W<CHSELRMOD0rs> {
        CHSEL0_W::new(self, 0)
    }
    ///Bit 1 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel1(&mut self) -> CHSEL1_W<CHSELRMOD0rs> {
        CHSEL1_W::new(self, 1)
    }
    ///Bit 2 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel2(&mut self) -> CHSEL2_W<CHSELRMOD0rs> {
        CHSEL2_W::new(self, 2)
    }
    ///Bit 3 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel3(&mut self) -> CHSEL3_W<CHSELRMOD0rs> {
        CHSEL3_W::new(self, 3)
    }
    ///Bit 4 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel4(&mut self) -> CHSEL4_W<CHSELRMOD0rs> {
        CHSEL4_W::new(self, 4)
    }
    ///Bit 5 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel5(&mut self) -> CHSEL5_W<CHSELRMOD0rs> {
        CHSEL5_W::new(self, 5)
    }
    ///Bit 6 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel6(&mut self) -> CHSEL6_W<CHSELRMOD0rs> {
        CHSEL6_W::new(self, 6)
    }
    ///Bit 7 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel7(&mut self) -> CHSEL7_W<CHSELRMOD0rs> {
        CHSEL7_W::new(self, 7)
    }
    ///Bit 8 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel8(&mut self) -> CHSEL8_W<CHSELRMOD0rs> {
        CHSEL8_W::new(self, 8)
    }
    ///Bit 9 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel9(&mut self) -> CHSEL9_W<CHSELRMOD0rs> {
        CHSEL9_W::new(self, 9)
    }
    ///Bit 10 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel10(&mut self) -> CHSEL10_W<CHSELRMOD0rs> {
        CHSEL10_W::new(self, 10)
    }
    ///Bit 11 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel11(&mut self) -> CHSEL11_W<CHSELRMOD0rs> {
        CHSEL11_W::new(self, 11)
    }
    ///Bit 12 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel12(&mut self) -> CHSEL12_W<CHSELRMOD0rs> {
        CHSEL12_W::new(self, 12)
    }
    ///Bit 13 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel13(&mut self) -> CHSEL13_W<CHSELRMOD0rs> {
        CHSEL13_W::new(self, 13)
    }
    ///Bit 14 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel14(&mut self) -> CHSEL14_W<CHSELRMOD0rs> {
        CHSEL14_W::new(self, 14)
    }
    ///Bit 15 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel15(&mut self) -> CHSEL15_W<CHSELRMOD0rs> {
        CHSEL15_W::new(self, 15)
    }
    ///Bit 16 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel16(&mut self) -> CHSEL16_W<CHSELRMOD0rs> {
        CHSEL16_W::new(self, 16)
    }
    ///Bit 17 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel17(&mut self) -> CHSEL17_W<CHSELRMOD0rs> {
        CHSEL17_W::new(self, 17)
    }
    ///Bit 18 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel18(&mut self) -> CHSEL18_W<CHSELRMOD0rs> {
        CHSEL18_W::new(self, 18)
    }
    ///Bit 19 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel19(&mut self) -> CHSEL19_W<CHSELRMOD0rs> {
        CHSEL19_W::new(self, 19)
    }
    ///Bit 20 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel20(&mut self) -> CHSEL20_W<CHSELRMOD0rs> {
        CHSEL20_W::new(self, 20)
    }
    ///Bit 21 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel21(&mut self) -> CHSEL21_W<CHSELRMOD0rs> {
        CHSEL21_W::new(self, 21)
    }
    ///Bit 22 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel22(&mut self) -> CHSEL22_W<CHSELRMOD0rs> {
        CHSEL22_W::new(self, 22)
    }
    ///Bit 23 - CHSEL
    #[inline(always)]
    #[must_use]
    pub fn chsel23(&mut self) -> CHSEL23_W<CHSELRMOD0rs> {
        CHSEL23_W::new(self, 23)
    }
}
/**ADC channel selection register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`chselrmod0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselrmod0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#ADC4:CHSELRMOD0)*/
pub struct CHSELRMOD0rs;
impl crate::RegisterSpec for CHSELRMOD0rs {
    type Ux = u32;
}
///`read()` method returns [`chselrmod0::R`](R) reader structure
impl crate::Readable for CHSELRMOD0rs {}
///`write(|w| ..)` method takes [`chselrmod0::W`](W) writer structure
impl crate::Writable for CHSELRMOD0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CHSELRMOD0 to value 0
impl crate::Resettable for CHSELRMOD0rs {
    const RESET_VALUE: u32 = 0;
}
