///Register `DIFSEL` reader
pub type R = crate::R<DIFSELrs>;
///Register `DIFSEL` writer
pub type W = crate::W<DIFSELrs>;
/**Differential mode for channels 0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIFSEL_0 {
    ///0: Input channel is configured in single-ended mode
    SingleEnded = 0,
    ///1: Input channel is configured in differential mode
    Differential = 1,
}
impl From<DIFSEL_0> for bool {
    #[inline(always)]
    fn from(variant: DIFSEL_0) -> Self {
        variant as u8 != 0
    }
}
///Field `DIFSEL_0` reader - Differential mode for channels 0
pub type DIFSEL_0_R = crate::BitReader<DIFSEL_0>;
impl DIFSEL_0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIFSEL_0 {
        match self.bits {
            false => DIFSEL_0::SingleEnded,
            true => DIFSEL_0::Differential,
        }
    }
    ///Input channel is configured in single-ended mode
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        *self == DIFSEL_0::SingleEnded
    }
    ///Input channel is configured in differential mode
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == DIFSEL_0::Differential
    }
}
///Field `DIFSEL_0` writer - Differential mode for channels 0
pub type DIFSEL_0_W<'a, REG> = crate::BitWriter<'a, REG, DIFSEL_0>;
impl<'a, REG> DIFSEL_0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input channel is configured in single-ended mode
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut crate::W<REG> {
        self.variant(DIFSEL_0::SingleEnded)
    }
    ///Input channel is configured in differential mode
    #[inline(always)]
    pub fn differential(self) -> &'a mut crate::W<REG> {
        self.variant(DIFSEL_0::Differential)
    }
}
///Field `DIFSEL_1` reader - Differential mode for channels 0
pub use DIFSEL_0_R as DIFSEL_1_R;
///Field `DIFSEL_2` reader - Differential mode for channels 0
pub use DIFSEL_0_R as DIFSEL_2_R;
///Field `DIFSEL_3` reader - Differential mode for channels 0
pub use DIFSEL_0_R as DIFSEL_3_R;
///Field `DIFSEL_4` reader - Differential mode for channels 0
pub use DIFSEL_0_R as DIFSEL_4_R;
///Field `DIFSEL_5` reader - Differential mode for channels 0
pub use DIFSEL_0_R as DIFSEL_5_R;
///Field `DIFSEL_6` reader - Differential mode for channels 0
pub use DIFSEL_0_R as DIFSEL_6_R;
///Field `DIFSEL_7` reader - Differential mode for channels 0
pub use DIFSEL_0_R as DIFSEL_7_R;
///Field `DIFSEL_8` reader - Differential mode for channels 0
pub use DIFSEL_0_R as DIFSEL_8_R;
///Field `DIFSEL_9` reader - Differential mode for channels 0
pub use DIFSEL_0_R as DIFSEL_9_R;
///Field `DIFSEL_10` reader - Differential mode for channels 0
pub use DIFSEL_0_R as DIFSEL_10_R;
///Field `DIFSEL_11` reader - Differential mode for channels 0
pub use DIFSEL_0_R as DIFSEL_11_R;
///Field `DIFSEL_12` reader - Differential mode for channels 0
pub use DIFSEL_0_R as DIFSEL_12_R;
///Field `DIFSEL_13` reader - Differential mode for channels 0
pub use DIFSEL_0_R as DIFSEL_13_R;
///Field `DIFSEL_14` reader - Differential mode for channels 0
pub use DIFSEL_0_R as DIFSEL_14_R;
///Field `DIFSEL_15` reader - Differential mode for channels 0
pub use DIFSEL_0_R as DIFSEL_15_R;
///Field `DIFSEL_16` reader - Differential mode for channels 0
pub use DIFSEL_0_R as DIFSEL_16_R;
///Field `DIFSEL_17` reader - Differential mode for channels 0
pub use DIFSEL_0_R as DIFSEL_17_R;
///Field `DIFSEL_18` reader - Differential mode for channels 0
pub use DIFSEL_0_R as DIFSEL_18_R;
///Field `DIFSEL_1` writer - Differential mode for channels 0
pub use DIFSEL_0_W as DIFSEL_1_W;
///Field `DIFSEL_2` writer - Differential mode for channels 0
pub use DIFSEL_0_W as DIFSEL_2_W;
///Field `DIFSEL_3` writer - Differential mode for channels 0
pub use DIFSEL_0_W as DIFSEL_3_W;
///Field `DIFSEL_4` writer - Differential mode for channels 0
pub use DIFSEL_0_W as DIFSEL_4_W;
///Field `DIFSEL_5` writer - Differential mode for channels 0
pub use DIFSEL_0_W as DIFSEL_5_W;
///Field `DIFSEL_6` writer - Differential mode for channels 0
pub use DIFSEL_0_W as DIFSEL_6_W;
///Field `DIFSEL_7` writer - Differential mode for channels 0
pub use DIFSEL_0_W as DIFSEL_7_W;
///Field `DIFSEL_8` writer - Differential mode for channels 0
pub use DIFSEL_0_W as DIFSEL_8_W;
///Field `DIFSEL_9` writer - Differential mode for channels 0
pub use DIFSEL_0_W as DIFSEL_9_W;
///Field `DIFSEL_10` writer - Differential mode for channels 0
pub use DIFSEL_0_W as DIFSEL_10_W;
///Field `DIFSEL_11` writer - Differential mode for channels 0
pub use DIFSEL_0_W as DIFSEL_11_W;
///Field `DIFSEL_12` writer - Differential mode for channels 0
pub use DIFSEL_0_W as DIFSEL_12_W;
///Field `DIFSEL_13` writer - Differential mode for channels 0
pub use DIFSEL_0_W as DIFSEL_13_W;
///Field `DIFSEL_14` writer - Differential mode for channels 0
pub use DIFSEL_0_W as DIFSEL_14_W;
///Field `DIFSEL_15` writer - Differential mode for channels 0
pub use DIFSEL_0_W as DIFSEL_15_W;
///Field `DIFSEL_16` writer - Differential mode for channels 0
pub use DIFSEL_0_W as DIFSEL_16_W;
///Field `DIFSEL_17` writer - Differential mode for channels 0
pub use DIFSEL_0_W as DIFSEL_17_W;
///Field `DIFSEL_18` writer - Differential mode for channels 0
pub use DIFSEL_0_W as DIFSEL_18_W;
impl R {
    ///Bit 0 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_0(&self) -> DIFSEL_0_R {
        DIFSEL_0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_1(&self) -> DIFSEL_1_R {
        DIFSEL_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_2(&self) -> DIFSEL_2_R {
        DIFSEL_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_3(&self) -> DIFSEL_3_R {
        DIFSEL_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_4(&self) -> DIFSEL_4_R {
        DIFSEL_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_5(&self) -> DIFSEL_5_R {
        DIFSEL_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_6(&self) -> DIFSEL_6_R {
        DIFSEL_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_7(&self) -> DIFSEL_7_R {
        DIFSEL_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_8(&self) -> DIFSEL_8_R {
        DIFSEL_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_9(&self) -> DIFSEL_9_R {
        DIFSEL_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_10(&self) -> DIFSEL_10_R {
        DIFSEL_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_11(&self) -> DIFSEL_11_R {
        DIFSEL_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_12(&self) -> DIFSEL_12_R {
        DIFSEL_12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_13(&self) -> DIFSEL_13_R {
        DIFSEL_13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_14(&self) -> DIFSEL_14_R {
        DIFSEL_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_15(&self) -> DIFSEL_15_R {
        DIFSEL_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_16(&self) -> DIFSEL_16_R {
        DIFSEL_16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_17(&self) -> DIFSEL_17_R {
        DIFSEL_17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_18(&self) -> DIFSEL_18_R {
        DIFSEL_18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIFSEL")
            .field("difsel_0", &self.difsel_0())
            .field("difsel_1", &self.difsel_1())
            .field("difsel_2", &self.difsel_2())
            .field("difsel_3", &self.difsel_3())
            .field("difsel_4", &self.difsel_4())
            .field("difsel_5", &self.difsel_5())
            .field("difsel_6", &self.difsel_6())
            .field("difsel_7", &self.difsel_7())
            .field("difsel_8", &self.difsel_8())
            .field("difsel_9", &self.difsel_9())
            .field("difsel_10", &self.difsel_10())
            .field("difsel_11", &self.difsel_11())
            .field("difsel_12", &self.difsel_12())
            .field("difsel_13", &self.difsel_13())
            .field("difsel_14", &self.difsel_14())
            .field("difsel_15", &self.difsel_15())
            .field("difsel_16", &self.difsel_16())
            .field("difsel_17", &self.difsel_17())
            .field("difsel_18", &self.difsel_18())
            .finish()
    }
}
impl W {
    ///Bit 0 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_0(&mut self) -> DIFSEL_0_W<DIFSELrs> {
        DIFSEL_0_W::new(self, 0)
    }
    ///Bit 1 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_1(&mut self) -> DIFSEL_1_W<DIFSELrs> {
        DIFSEL_1_W::new(self, 1)
    }
    ///Bit 2 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_2(&mut self) -> DIFSEL_2_W<DIFSELrs> {
        DIFSEL_2_W::new(self, 2)
    }
    ///Bit 3 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_3(&mut self) -> DIFSEL_3_W<DIFSELrs> {
        DIFSEL_3_W::new(self, 3)
    }
    ///Bit 4 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_4(&mut self) -> DIFSEL_4_W<DIFSELrs> {
        DIFSEL_4_W::new(self, 4)
    }
    ///Bit 5 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_5(&mut self) -> DIFSEL_5_W<DIFSELrs> {
        DIFSEL_5_W::new(self, 5)
    }
    ///Bit 6 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_6(&mut self) -> DIFSEL_6_W<DIFSELrs> {
        DIFSEL_6_W::new(self, 6)
    }
    ///Bit 7 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_7(&mut self) -> DIFSEL_7_W<DIFSELrs> {
        DIFSEL_7_W::new(self, 7)
    }
    ///Bit 8 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_8(&mut self) -> DIFSEL_8_W<DIFSELrs> {
        DIFSEL_8_W::new(self, 8)
    }
    ///Bit 9 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_9(&mut self) -> DIFSEL_9_W<DIFSELrs> {
        DIFSEL_9_W::new(self, 9)
    }
    ///Bit 10 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_10(&mut self) -> DIFSEL_10_W<DIFSELrs> {
        DIFSEL_10_W::new(self, 10)
    }
    ///Bit 11 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_11(&mut self) -> DIFSEL_11_W<DIFSELrs> {
        DIFSEL_11_W::new(self, 11)
    }
    ///Bit 12 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_12(&mut self) -> DIFSEL_12_W<DIFSELrs> {
        DIFSEL_12_W::new(self, 12)
    }
    ///Bit 13 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_13(&mut self) -> DIFSEL_13_W<DIFSELrs> {
        DIFSEL_13_W::new(self, 13)
    }
    ///Bit 14 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_14(&mut self) -> DIFSEL_14_W<DIFSELrs> {
        DIFSEL_14_W::new(self, 14)
    }
    ///Bit 15 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_15(&mut self) -> DIFSEL_15_W<DIFSELrs> {
        DIFSEL_15_W::new(self, 15)
    }
    ///Bit 16 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_16(&mut self) -> DIFSEL_16_W<DIFSELrs> {
        DIFSEL_16_W::new(self, 16)
    }
    ///Bit 17 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_17(&mut self) -> DIFSEL_17_W<DIFSELrs> {
        DIFSEL_17_W::new(self, 17)
    }
    ///Bit 18 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_18(&mut self) -> DIFSEL_18_W<DIFSELrs> {
        DIFSEL_18_W::new(self, 18)
    }
}
/**Differential Mode Selection Register 2

You can [`read`](crate::Reg::read) this register and get [`difsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`difsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G471xx.html#ADC1:DIFSEL)*/
pub struct DIFSELrs;
impl crate::RegisterSpec for DIFSELrs {
    type Ux = u32;
}
///`read()` method returns [`difsel::R`](R) reader structure
impl crate::Readable for DIFSELrs {}
///`write(|w| ..)` method takes [`difsel::W`](W) writer structure
impl crate::Writable for DIFSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DIFSEL to value 0
impl crate::Resettable for DIFSELrs {
    const RESET_VALUE: u32 = 0;
}
