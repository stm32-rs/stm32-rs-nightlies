///Register `DIFSEL` reader
pub type R = crate::R<DIFSELrs>;
///Register `DIFSEL` writer
pub type W = crate::W<DIFSELrs>;
/**Differential mode for channels 0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIFSEL0 {
    ///0: Input channel is configured in single-ended mode
    SingleEnded = 0,
    ///1: Input channel is configured in differential mode
    Differential = 1,
}
impl From<DIFSEL0> for bool {
    #[inline(always)]
    fn from(variant: DIFSEL0) -> Self {
        variant as u8 != 0
    }
}
///Field `DIFSEL0` reader - Differential mode for channels 0
pub type DIFSEL0_R = crate::BitReader<DIFSEL0>;
impl DIFSEL0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIFSEL0 {
        match self.bits {
            false => DIFSEL0::SingleEnded,
            true => DIFSEL0::Differential,
        }
    }
    ///Input channel is configured in single-ended mode
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        *self == DIFSEL0::SingleEnded
    }
    ///Input channel is configured in differential mode
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == DIFSEL0::Differential
    }
}
///Field `DIFSEL0` writer - Differential mode for channels 0
pub type DIFSEL0_W<'a, REG> = crate::BitWriter<'a, REG, DIFSEL0>;
impl<'a, REG> DIFSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input channel is configured in single-ended mode
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut crate::W<REG> {
        self.variant(DIFSEL0::SingleEnded)
    }
    ///Input channel is configured in differential mode
    #[inline(always)]
    pub fn differential(self) -> &'a mut crate::W<REG> {
        self.variant(DIFSEL0::Differential)
    }
}
///Field `DIFSEL1` reader - Differential mode for channels 0
pub use DIFSEL0_R as DIFSEL1_R;
///Field `DIFSEL2` reader - Differential mode for channels 0
pub use DIFSEL0_R as DIFSEL2_R;
///Field `DIFSEL3` reader - Differential mode for channels 0
pub use DIFSEL0_R as DIFSEL3_R;
///Field `DIFSEL4` reader - Differential mode for channels 0
pub use DIFSEL0_R as DIFSEL4_R;
///Field `DIFSEL5` reader - Differential mode for channels 0
pub use DIFSEL0_R as DIFSEL5_R;
///Field `DIFSEL6` reader - Differential mode for channels 0
pub use DIFSEL0_R as DIFSEL6_R;
///Field `DIFSEL7` reader - Differential mode for channels 0
pub use DIFSEL0_R as DIFSEL7_R;
///Field `DIFSEL8` reader - Differential mode for channels 0
pub use DIFSEL0_R as DIFSEL8_R;
///Field `DIFSEL9` reader - Differential mode for channels 0
pub use DIFSEL0_R as DIFSEL9_R;
///Field `DIFSEL10` reader - Differential mode for channels 0
pub use DIFSEL0_R as DIFSEL10_R;
///Field `DIFSEL11` reader - Differential mode for channels 0
pub use DIFSEL0_R as DIFSEL11_R;
///Field `DIFSEL12` reader - Differential mode for channels 0
pub use DIFSEL0_R as DIFSEL12_R;
///Field `DIFSEL13` reader - Differential mode for channels 0
pub use DIFSEL0_R as DIFSEL13_R;
///Field `DIFSEL14` reader - Differential mode for channels 0
pub use DIFSEL0_R as DIFSEL14_R;
///Field `DIFSEL15` reader - Differential mode for channels 0
pub use DIFSEL0_R as DIFSEL15_R;
///Field `DIFSEL16` reader - Differential mode for channels 0
pub use DIFSEL0_R as DIFSEL16_R;
///Field `DIFSEL17` reader - Differential mode for channels 0
pub use DIFSEL0_R as DIFSEL17_R;
///Field `DIFSEL18` reader - Differential mode for channels 0
pub use DIFSEL0_R as DIFSEL18_R;
///Field `DIFSEL1` writer - Differential mode for channels 0
pub use DIFSEL0_W as DIFSEL1_W;
///Field `DIFSEL2` writer - Differential mode for channels 0
pub use DIFSEL0_W as DIFSEL2_W;
///Field `DIFSEL3` writer - Differential mode for channels 0
pub use DIFSEL0_W as DIFSEL3_W;
///Field `DIFSEL4` writer - Differential mode for channels 0
pub use DIFSEL0_W as DIFSEL4_W;
///Field `DIFSEL5` writer - Differential mode for channels 0
pub use DIFSEL0_W as DIFSEL5_W;
///Field `DIFSEL6` writer - Differential mode for channels 0
pub use DIFSEL0_W as DIFSEL6_W;
///Field `DIFSEL7` writer - Differential mode for channels 0
pub use DIFSEL0_W as DIFSEL7_W;
///Field `DIFSEL8` writer - Differential mode for channels 0
pub use DIFSEL0_W as DIFSEL8_W;
///Field `DIFSEL9` writer - Differential mode for channels 0
pub use DIFSEL0_W as DIFSEL9_W;
///Field `DIFSEL10` writer - Differential mode for channels 0
pub use DIFSEL0_W as DIFSEL10_W;
///Field `DIFSEL11` writer - Differential mode for channels 0
pub use DIFSEL0_W as DIFSEL11_W;
///Field `DIFSEL12` writer - Differential mode for channels 0
pub use DIFSEL0_W as DIFSEL12_W;
///Field `DIFSEL13` writer - Differential mode for channels 0
pub use DIFSEL0_W as DIFSEL13_W;
///Field `DIFSEL14` writer - Differential mode for channels 0
pub use DIFSEL0_W as DIFSEL14_W;
///Field `DIFSEL15` writer - Differential mode for channels 0
pub use DIFSEL0_W as DIFSEL15_W;
///Field `DIFSEL16` writer - Differential mode for channels 0
pub use DIFSEL0_W as DIFSEL16_W;
///Field `DIFSEL17` writer - Differential mode for channels 0
pub use DIFSEL0_W as DIFSEL17_W;
///Field `DIFSEL18` writer - Differential mode for channels 0
pub use DIFSEL0_W as DIFSEL18_W;
impl R {
    ///Bit 0 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel0(&self) -> DIFSEL0_R {
        DIFSEL0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel1(&self) -> DIFSEL1_R {
        DIFSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel2(&self) -> DIFSEL2_R {
        DIFSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel3(&self) -> DIFSEL3_R {
        DIFSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel4(&self) -> DIFSEL4_R {
        DIFSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel5(&self) -> DIFSEL5_R {
        DIFSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel6(&self) -> DIFSEL6_R {
        DIFSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel7(&self) -> DIFSEL7_R {
        DIFSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel8(&self) -> DIFSEL8_R {
        DIFSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel9(&self) -> DIFSEL9_R {
        DIFSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel10(&self) -> DIFSEL10_R {
        DIFSEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel11(&self) -> DIFSEL11_R {
        DIFSEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel12(&self) -> DIFSEL12_R {
        DIFSEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel13(&self) -> DIFSEL13_R {
        DIFSEL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel14(&self) -> DIFSEL14_R {
        DIFSEL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel15(&self) -> DIFSEL15_R {
        DIFSEL15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel16(&self) -> DIFSEL16_R {
        DIFSEL16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel17(&self) -> DIFSEL17_R {
        DIFSEL17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel18(&self) -> DIFSEL18_R {
        DIFSEL18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIFSEL")
            .field("difsel0", &self.difsel0())
            .field("difsel1", &self.difsel1())
            .field("difsel2", &self.difsel2())
            .field("difsel3", &self.difsel3())
            .field("difsel4", &self.difsel4())
            .field("difsel5", &self.difsel5())
            .field("difsel6", &self.difsel6())
            .field("difsel7", &self.difsel7())
            .field("difsel8", &self.difsel8())
            .field("difsel9", &self.difsel9())
            .field("difsel10", &self.difsel10())
            .field("difsel11", &self.difsel11())
            .field("difsel12", &self.difsel12())
            .field("difsel13", &self.difsel13())
            .field("difsel14", &self.difsel14())
            .field("difsel15", &self.difsel15())
            .field("difsel16", &self.difsel16())
            .field("difsel17", &self.difsel17())
            .field("difsel18", &self.difsel18())
            .finish()
    }
}
impl W {
    ///Bit 0 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel0(&mut self) -> DIFSEL0_W<DIFSELrs> {
        DIFSEL0_W::new(self, 0)
    }
    ///Bit 1 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel1(&mut self) -> DIFSEL1_W<DIFSELrs> {
        DIFSEL1_W::new(self, 1)
    }
    ///Bit 2 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel2(&mut self) -> DIFSEL2_W<DIFSELrs> {
        DIFSEL2_W::new(self, 2)
    }
    ///Bit 3 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel3(&mut self) -> DIFSEL3_W<DIFSELrs> {
        DIFSEL3_W::new(self, 3)
    }
    ///Bit 4 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel4(&mut self) -> DIFSEL4_W<DIFSELrs> {
        DIFSEL4_W::new(self, 4)
    }
    ///Bit 5 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel5(&mut self) -> DIFSEL5_W<DIFSELrs> {
        DIFSEL5_W::new(self, 5)
    }
    ///Bit 6 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel6(&mut self) -> DIFSEL6_W<DIFSELrs> {
        DIFSEL6_W::new(self, 6)
    }
    ///Bit 7 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel7(&mut self) -> DIFSEL7_W<DIFSELrs> {
        DIFSEL7_W::new(self, 7)
    }
    ///Bit 8 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel8(&mut self) -> DIFSEL8_W<DIFSELrs> {
        DIFSEL8_W::new(self, 8)
    }
    ///Bit 9 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel9(&mut self) -> DIFSEL9_W<DIFSELrs> {
        DIFSEL9_W::new(self, 9)
    }
    ///Bit 10 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel10(&mut self) -> DIFSEL10_W<DIFSELrs> {
        DIFSEL10_W::new(self, 10)
    }
    ///Bit 11 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel11(&mut self) -> DIFSEL11_W<DIFSELrs> {
        DIFSEL11_W::new(self, 11)
    }
    ///Bit 12 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel12(&mut self) -> DIFSEL12_W<DIFSELrs> {
        DIFSEL12_W::new(self, 12)
    }
    ///Bit 13 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel13(&mut self) -> DIFSEL13_W<DIFSELrs> {
        DIFSEL13_W::new(self, 13)
    }
    ///Bit 14 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel14(&mut self) -> DIFSEL14_W<DIFSELrs> {
        DIFSEL14_W::new(self, 14)
    }
    ///Bit 15 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel15(&mut self) -> DIFSEL15_W<DIFSELrs> {
        DIFSEL15_W::new(self, 15)
    }
    ///Bit 16 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel16(&mut self) -> DIFSEL16_W<DIFSELrs> {
        DIFSEL16_W::new(self, 16)
    }
    ///Bit 17 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel17(&mut self) -> DIFSEL17_W<DIFSELrs> {
        DIFSEL17_W::new(self, 17)
    }
    ///Bit 18 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel18(&mut self) -> DIFSEL18_W<DIFSELrs> {
        DIFSEL18_W::new(self, 18)
    }
}
/**Differential Mode Selection Register 2

You can [`read`](crate::Reg::read) this register and get [`difsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`difsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484xx.html#ADC1:DIFSEL)*/
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
