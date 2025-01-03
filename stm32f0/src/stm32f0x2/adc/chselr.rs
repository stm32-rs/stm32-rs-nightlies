///Register `CHSELR` reader
pub type R = crate::R<CHSELRrs>;
///Register `CHSELR` writer
pub type W = crate::W<CHSELRrs>;
/**Channel-x selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL0 {
    ///0: Input Channel is not selected for conversion
    NotSelected = 0,
    ///1: Input Channel is selected for conversion
    Selected = 1,
}
impl From<CHSEL0> for bool {
    #[inline(always)]
    fn from(variant: CHSEL0) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSEL0` reader - Channel-x selection
pub type CHSEL0_R = crate::BitReader<CHSEL0>;
impl CHSEL0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL0 {
        match self.bits {
            false => CHSEL0::NotSelected,
            true => CHSEL0::Selected,
        }
    }
    ///Input Channel is not selected for conversion
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == CHSEL0::NotSelected
    }
    ///Input Channel is selected for conversion
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == CHSEL0::Selected
    }
}
///Field `CHSEL0` writer - Channel-x selection
pub type CHSEL0_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL0>;
impl<'a, REG> CHSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input Channel is not selected for conversion
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL0::NotSelected)
    }
    ///Input Channel is selected for conversion
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL0::Selected)
    }
}
///Field `CHSEL1` reader - Channel-x selection
pub use CHSEL0_R as CHSEL1_R;
///Field `CHSEL2` reader - Channel-x selection
pub use CHSEL0_R as CHSEL2_R;
///Field `CHSEL3` reader - Channel-x selection
pub use CHSEL0_R as CHSEL3_R;
///Field `CHSEL4` reader - Channel-x selection
pub use CHSEL0_R as CHSEL4_R;
///Field `CHSEL5` reader - Channel-x selection
pub use CHSEL0_R as CHSEL5_R;
///Field `CHSEL6` reader - Channel-x selection
pub use CHSEL0_R as CHSEL6_R;
///Field `CHSEL7` reader - Channel-x selection
pub use CHSEL0_R as CHSEL7_R;
///Field `CHSEL8` reader - Channel-x selection
pub use CHSEL0_R as CHSEL8_R;
///Field `CHSEL9` reader - Channel-x selection
pub use CHSEL0_R as CHSEL9_R;
///Field `CHSEL10` reader - Channel-x selection
pub use CHSEL0_R as CHSEL10_R;
///Field `CHSEL11` reader - Channel-x selection
pub use CHSEL0_R as CHSEL11_R;
///Field `CHSEL12` reader - Channel-x selection
pub use CHSEL0_R as CHSEL12_R;
///Field `CHSEL13` reader - Channel-x selection
pub use CHSEL0_R as CHSEL13_R;
///Field `CHSEL14` reader - Channel-x selection
pub use CHSEL0_R as CHSEL14_R;
///Field `CHSEL15` reader - Channel-x selection
pub use CHSEL0_R as CHSEL15_R;
///Field `CHSEL16` reader - Channel-x selection
pub use CHSEL0_R as CHSEL16_R;
///Field `CHSEL17` reader - Channel-x selection
pub use CHSEL0_R as CHSEL17_R;
///Field `CHSEL18` reader - Channel-x selection
pub use CHSEL0_R as CHSEL18_R;
///Field `CHSEL1` writer - Channel-x selection
pub use CHSEL0_W as CHSEL1_W;
///Field `CHSEL2` writer - Channel-x selection
pub use CHSEL0_W as CHSEL2_W;
///Field `CHSEL3` writer - Channel-x selection
pub use CHSEL0_W as CHSEL3_W;
///Field `CHSEL4` writer - Channel-x selection
pub use CHSEL0_W as CHSEL4_W;
///Field `CHSEL5` writer - Channel-x selection
pub use CHSEL0_W as CHSEL5_W;
///Field `CHSEL6` writer - Channel-x selection
pub use CHSEL0_W as CHSEL6_W;
///Field `CHSEL7` writer - Channel-x selection
pub use CHSEL0_W as CHSEL7_W;
///Field `CHSEL8` writer - Channel-x selection
pub use CHSEL0_W as CHSEL8_W;
///Field `CHSEL9` writer - Channel-x selection
pub use CHSEL0_W as CHSEL9_W;
///Field `CHSEL10` writer - Channel-x selection
pub use CHSEL0_W as CHSEL10_W;
///Field `CHSEL11` writer - Channel-x selection
pub use CHSEL0_W as CHSEL11_W;
///Field `CHSEL12` writer - Channel-x selection
pub use CHSEL0_W as CHSEL12_W;
///Field `CHSEL13` writer - Channel-x selection
pub use CHSEL0_W as CHSEL13_W;
///Field `CHSEL14` writer - Channel-x selection
pub use CHSEL0_W as CHSEL14_W;
///Field `CHSEL15` writer - Channel-x selection
pub use CHSEL0_W as CHSEL15_W;
///Field `CHSEL16` writer - Channel-x selection
pub use CHSEL0_W as CHSEL16_W;
///Field `CHSEL17` writer - Channel-x selection
pub use CHSEL0_W as CHSEL17_W;
///Field `CHSEL18` writer - Channel-x selection
pub use CHSEL0_W as CHSEL18_W;
impl R {
    ///Bit 0 - Channel-x selection
    #[inline(always)]
    pub fn chsel0(&self) -> CHSEL0_R {
        CHSEL0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel-x selection
    #[inline(always)]
    pub fn chsel1(&self) -> CHSEL1_R {
        CHSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel-x selection
    #[inline(always)]
    pub fn chsel2(&self) -> CHSEL2_R {
        CHSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel-x selection
    #[inline(always)]
    pub fn chsel3(&self) -> CHSEL3_R {
        CHSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Channel-x selection
    #[inline(always)]
    pub fn chsel4(&self) -> CHSEL4_R {
        CHSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Channel-x selection
    #[inline(always)]
    pub fn chsel5(&self) -> CHSEL5_R {
        CHSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Channel-x selection
    #[inline(always)]
    pub fn chsel6(&self) -> CHSEL6_R {
        CHSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Channel-x selection
    #[inline(always)]
    pub fn chsel7(&self) -> CHSEL7_R {
        CHSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Channel-x selection
    #[inline(always)]
    pub fn chsel8(&self) -> CHSEL8_R {
        CHSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Channel-x selection
    #[inline(always)]
    pub fn chsel9(&self) -> CHSEL9_R {
        CHSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Channel-x selection
    #[inline(always)]
    pub fn chsel10(&self) -> CHSEL10_R {
        CHSEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Channel-x selection
    #[inline(always)]
    pub fn chsel11(&self) -> CHSEL11_R {
        CHSEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Channel-x selection
    #[inline(always)]
    pub fn chsel12(&self) -> CHSEL12_R {
        CHSEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Channel-x selection
    #[inline(always)]
    pub fn chsel13(&self) -> CHSEL13_R {
        CHSEL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Channel-x selection
    #[inline(always)]
    pub fn chsel14(&self) -> CHSEL14_R {
        CHSEL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Channel-x selection
    #[inline(always)]
    pub fn chsel15(&self) -> CHSEL15_R {
        CHSEL15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Channel-x selection
    #[inline(always)]
    pub fn chsel16(&self) -> CHSEL16_R {
        CHSEL16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Channel-x selection
    #[inline(always)]
    pub fn chsel17(&self) -> CHSEL17_R {
        CHSEL17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Channel-x selection
    #[inline(always)]
    pub fn chsel18(&self) -> CHSEL18_R {
        CHSEL18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHSELR")
            .field("chsel0", &self.chsel0())
            .field("chsel18", &self.chsel18())
            .field("chsel17", &self.chsel17())
            .field("chsel16", &self.chsel16())
            .field("chsel15", &self.chsel15())
            .field("chsel14", &self.chsel14())
            .field("chsel13", &self.chsel13())
            .field("chsel12", &self.chsel12())
            .field("chsel11", &self.chsel11())
            .field("chsel10", &self.chsel10())
            .field("chsel9", &self.chsel9())
            .field("chsel8", &self.chsel8())
            .field("chsel7", &self.chsel7())
            .field("chsel6", &self.chsel6())
            .field("chsel5", &self.chsel5())
            .field("chsel4", &self.chsel4())
            .field("chsel3", &self.chsel3())
            .field("chsel2", &self.chsel2())
            .field("chsel1", &self.chsel1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Channel-x selection
    #[inline(always)]
    pub fn chsel0(&mut self) -> CHSEL0_W<CHSELRrs> {
        CHSEL0_W::new(self, 0)
    }
    ///Bit 1 - Channel-x selection
    #[inline(always)]
    pub fn chsel1(&mut self) -> CHSEL1_W<CHSELRrs> {
        CHSEL1_W::new(self, 1)
    }
    ///Bit 2 - Channel-x selection
    #[inline(always)]
    pub fn chsel2(&mut self) -> CHSEL2_W<CHSELRrs> {
        CHSEL2_W::new(self, 2)
    }
    ///Bit 3 - Channel-x selection
    #[inline(always)]
    pub fn chsel3(&mut self) -> CHSEL3_W<CHSELRrs> {
        CHSEL3_W::new(self, 3)
    }
    ///Bit 4 - Channel-x selection
    #[inline(always)]
    pub fn chsel4(&mut self) -> CHSEL4_W<CHSELRrs> {
        CHSEL4_W::new(self, 4)
    }
    ///Bit 5 - Channel-x selection
    #[inline(always)]
    pub fn chsel5(&mut self) -> CHSEL5_W<CHSELRrs> {
        CHSEL5_W::new(self, 5)
    }
    ///Bit 6 - Channel-x selection
    #[inline(always)]
    pub fn chsel6(&mut self) -> CHSEL6_W<CHSELRrs> {
        CHSEL6_W::new(self, 6)
    }
    ///Bit 7 - Channel-x selection
    #[inline(always)]
    pub fn chsel7(&mut self) -> CHSEL7_W<CHSELRrs> {
        CHSEL7_W::new(self, 7)
    }
    ///Bit 8 - Channel-x selection
    #[inline(always)]
    pub fn chsel8(&mut self) -> CHSEL8_W<CHSELRrs> {
        CHSEL8_W::new(self, 8)
    }
    ///Bit 9 - Channel-x selection
    #[inline(always)]
    pub fn chsel9(&mut self) -> CHSEL9_W<CHSELRrs> {
        CHSEL9_W::new(self, 9)
    }
    ///Bit 10 - Channel-x selection
    #[inline(always)]
    pub fn chsel10(&mut self) -> CHSEL10_W<CHSELRrs> {
        CHSEL10_W::new(self, 10)
    }
    ///Bit 11 - Channel-x selection
    #[inline(always)]
    pub fn chsel11(&mut self) -> CHSEL11_W<CHSELRrs> {
        CHSEL11_W::new(self, 11)
    }
    ///Bit 12 - Channel-x selection
    #[inline(always)]
    pub fn chsel12(&mut self) -> CHSEL12_W<CHSELRrs> {
        CHSEL12_W::new(self, 12)
    }
    ///Bit 13 - Channel-x selection
    #[inline(always)]
    pub fn chsel13(&mut self) -> CHSEL13_W<CHSELRrs> {
        CHSEL13_W::new(self, 13)
    }
    ///Bit 14 - Channel-x selection
    #[inline(always)]
    pub fn chsel14(&mut self) -> CHSEL14_W<CHSELRrs> {
        CHSEL14_W::new(self, 14)
    }
    ///Bit 15 - Channel-x selection
    #[inline(always)]
    pub fn chsel15(&mut self) -> CHSEL15_W<CHSELRrs> {
        CHSEL15_W::new(self, 15)
    }
    ///Bit 16 - Channel-x selection
    #[inline(always)]
    pub fn chsel16(&mut self) -> CHSEL16_W<CHSELRrs> {
        CHSEL16_W::new(self, 16)
    }
    ///Bit 17 - Channel-x selection
    #[inline(always)]
    pub fn chsel17(&mut self) -> CHSEL17_W<CHSELRrs> {
        CHSEL17_W::new(self, 17)
    }
    ///Bit 18 - Channel-x selection
    #[inline(always)]
    pub fn chsel18(&mut self) -> CHSEL18_W<CHSELRrs> {
        CHSEL18_W::new(self, 18)
    }
}
/**channel selection register

You can [`read`](crate::Reg::read) this register and get [`chselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#ADC:CHSELR)*/
pub struct CHSELRrs;
impl crate::RegisterSpec for CHSELRrs {
    type Ux = u32;
}
///`read()` method returns [`chselr::R`](R) reader structure
impl crate::Readable for CHSELRrs {}
///`write(|w| ..)` method takes [`chselr::W`](W) writer structure
impl crate::Writable for CHSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CHSELR to value 0
impl crate::Resettable for CHSELRrs {
    const RESET_VALUE: u32 = 0;
}
