///Register `DIFSEL` reader
pub type R = crate::R<DIFSELrs>;
///Register `DIFSEL` writer
pub type W = crate::W<DIFSELrs>;
/**Differential mode for channels 15 to 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIFSEL_10 {
    ///0: Input channel is configured in single-ended mode
    SingleEnded = 0,
    ///1: Input channel is configured in differential mode
    Differential = 1,
}
impl From<DIFSEL_10> for bool {
    #[inline(always)]
    fn from(variant: DIFSEL_10) -> Self {
        variant as u8 != 0
    }
}
///Field `DIFSEL_10` reader - Differential mode for channels 15 to 1
pub type DIFSEL_10_R = crate::BitReader<DIFSEL_10>;
impl DIFSEL_10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIFSEL_10 {
        match self.bits {
            false => DIFSEL_10::SingleEnded,
            true => DIFSEL_10::Differential,
        }
    }
    ///Input channel is configured in single-ended mode
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        *self == DIFSEL_10::SingleEnded
    }
    ///Input channel is configured in differential mode
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == DIFSEL_10::Differential
    }
}
///Field `DIFSEL_10` writer - Differential mode for channels 15 to 1
pub type DIFSEL_10_W<'a, REG> = crate::BitWriter<'a, REG, DIFSEL_10>;
impl<'a, REG> DIFSEL_10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input channel is configured in single-ended mode
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut crate::W<REG> {
        self.variant(DIFSEL_10::SingleEnded)
    }
    ///Input channel is configured in differential mode
    #[inline(always)]
    pub fn differential(self) -> &'a mut crate::W<REG> {
        self.variant(DIFSEL_10::Differential)
    }
}
///Field `DIFSEL_11` reader - Differential mode for channels 15 to 1
pub use DIFSEL_10_R as DIFSEL_11_R;
///Field `DIFSEL_12` reader - Differential mode for channels 15 to 1
pub use DIFSEL_10_R as DIFSEL_12_R;
///Field `DIFSEL_13` reader - Differential mode for channels 15 to 1
pub use DIFSEL_10_R as DIFSEL_13_R;
///Field `DIFSEL_14` reader - Differential mode for channels 15 to 1
pub use DIFSEL_10_R as DIFSEL_14_R;
///Field `DIFSEL_15` reader - Differential mode for channels 15 to 1
pub use DIFSEL_10_R as DIFSEL_15_R;
///Field `DIFSEL_16` reader - Differential mode for channels 15 to 1
pub use DIFSEL_10_R as DIFSEL_16_R;
///Field `DIFSEL_17` reader - Differential mode for channels 15 to 1
pub use DIFSEL_10_R as DIFSEL_17_R;
///Field `DIFSEL_18` reader - Differential mode for channels 15 to 1
pub use DIFSEL_10_R as DIFSEL_18_R;
///Field `DIFSEL_19` reader - Differential mode for channels 15 to 1
pub use DIFSEL_10_R as DIFSEL_19_R;
///Field `DIFSEL_110` reader - Differential mode for channels 15 to 1
pub use DIFSEL_10_R as DIFSEL_110_R;
///Field `DIFSEL_111` reader - Differential mode for channels 15 to 1
pub use DIFSEL_10_R as DIFSEL_111_R;
///Field `DIFSEL_112` reader - Differential mode for channels 15 to 1
pub use DIFSEL_10_R as DIFSEL_112_R;
///Field `DIFSEL_113` reader - Differential mode for channels 15 to 1
pub use DIFSEL_10_R as DIFSEL_113_R;
///Field `DIFSEL_114` reader - Differential mode for channels 15 to 1
pub use DIFSEL_10_R as DIFSEL_114_R;
///Field `DIFSEL_115` reader - Differential mode for channels 15 to 1
pub use DIFSEL_10_R as DIFSEL_115_R;
///Field `DIFSEL_116` reader - Differential mode for channels 15 to 1
pub use DIFSEL_10_R as DIFSEL_116_R;
///Field `DIFSEL_117` reader - Differential mode for channels 15 to 1
pub use DIFSEL_10_R as DIFSEL_117_R;
///Field `DIFSEL_11` writer - Differential mode for channels 15 to 1
pub use DIFSEL_10_W as DIFSEL_11_W;
///Field `DIFSEL_12` writer - Differential mode for channels 15 to 1
pub use DIFSEL_10_W as DIFSEL_12_W;
///Field `DIFSEL_13` writer - Differential mode for channels 15 to 1
pub use DIFSEL_10_W as DIFSEL_13_W;
///Field `DIFSEL_14` writer - Differential mode for channels 15 to 1
pub use DIFSEL_10_W as DIFSEL_14_W;
///Field `DIFSEL_15` writer - Differential mode for channels 15 to 1
pub use DIFSEL_10_W as DIFSEL_15_W;
///Field `DIFSEL_16` writer - Differential mode for channels 15 to 1
pub use DIFSEL_10_W as DIFSEL_16_W;
///Field `DIFSEL_17` writer - Differential mode for channels 15 to 1
pub use DIFSEL_10_W as DIFSEL_17_W;
///Field `DIFSEL_18` writer - Differential mode for channels 15 to 1
pub use DIFSEL_10_W as DIFSEL_18_W;
///Field `DIFSEL_19` writer - Differential mode for channels 15 to 1
pub use DIFSEL_10_W as DIFSEL_19_W;
///Field `DIFSEL_110` writer - Differential mode for channels 15 to 1
pub use DIFSEL_10_W as DIFSEL_110_W;
///Field `DIFSEL_111` writer - Differential mode for channels 15 to 1
pub use DIFSEL_10_W as DIFSEL_111_W;
///Field `DIFSEL_112` writer - Differential mode for channels 15 to 1
pub use DIFSEL_10_W as DIFSEL_112_W;
///Field `DIFSEL_113` writer - Differential mode for channels 15 to 1
pub use DIFSEL_10_W as DIFSEL_113_W;
///Field `DIFSEL_114` writer - Differential mode for channels 15 to 1
pub use DIFSEL_10_W as DIFSEL_114_W;
///Field `DIFSEL_115` writer - Differential mode for channels 15 to 1
pub use DIFSEL_10_W as DIFSEL_115_W;
///Field `DIFSEL_116` writer - Differential mode for channels 15 to 1
pub use DIFSEL_10_W as DIFSEL_116_W;
///Field `DIFSEL_117` writer - Differential mode for channels 15 to 1
pub use DIFSEL_10_W as DIFSEL_117_W;
impl R {
    ///Bit 1 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_10(&self) -> DIFSEL_10_R {
        DIFSEL_10_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_11(&self) -> DIFSEL_11_R {
        DIFSEL_11_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_12(&self) -> DIFSEL_12_R {
        DIFSEL_12_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_13(&self) -> DIFSEL_13_R {
        DIFSEL_13_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_14(&self) -> DIFSEL_14_R {
        DIFSEL_14_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_15(&self) -> DIFSEL_15_R {
        DIFSEL_15_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_16(&self) -> DIFSEL_16_R {
        DIFSEL_16_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_17(&self) -> DIFSEL_17_R {
        DIFSEL_17_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_18(&self) -> DIFSEL_18_R {
        DIFSEL_18_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_19(&self) -> DIFSEL_19_R {
        DIFSEL_19_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_110(&self) -> DIFSEL_110_R {
        DIFSEL_110_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_111(&self) -> DIFSEL_111_R {
        DIFSEL_111_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_112(&self) -> DIFSEL_112_R {
        DIFSEL_112_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_113(&self) -> DIFSEL_113_R {
        DIFSEL_113_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_114(&self) -> DIFSEL_114_R {
        DIFSEL_114_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_115(&self) -> DIFSEL_115_R {
        DIFSEL_115_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_116(&self) -> DIFSEL_116_R {
        DIFSEL_116_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_117(&self) -> DIFSEL_117_R {
        DIFSEL_117_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIFSEL")
            .field("difsel_10", &self.difsel_10())
            .field("difsel_11", &self.difsel_11())
            .field("difsel_12", &self.difsel_12())
            .field("difsel_13", &self.difsel_13())
            .field("difsel_14", &self.difsel_14())
            .field("difsel_15", &self.difsel_15())
            .field("difsel_16", &self.difsel_16())
            .field("difsel_17", &self.difsel_17())
            .field("difsel_18", &self.difsel_18())
            .field("difsel_19", &self.difsel_19())
            .field("difsel_110", &self.difsel_110())
            .field("difsel_111", &self.difsel_111())
            .field("difsel_112", &self.difsel_112())
            .field("difsel_113", &self.difsel_113())
            .field("difsel_114", &self.difsel_114())
            .field("difsel_115", &self.difsel_115())
            .field("difsel_116", &self.difsel_116())
            .field("difsel_117", &self.difsel_117())
            .finish()
    }
}
impl W {
    ///Bit 1 - Differential mode for channels 15 to 1
    #[inline(always)]
    #[must_use]
    pub fn difsel_10(&mut self) -> DIFSEL_10_W<DIFSELrs> {
        DIFSEL_10_W::new(self, 1)
    }
    ///Bit 2 - Differential mode for channels 15 to 1
    #[inline(always)]
    #[must_use]
    pub fn difsel_11(&mut self) -> DIFSEL_11_W<DIFSELrs> {
        DIFSEL_11_W::new(self, 2)
    }
    ///Bit 3 - Differential mode for channels 15 to 1
    #[inline(always)]
    #[must_use]
    pub fn difsel_12(&mut self) -> DIFSEL_12_W<DIFSELrs> {
        DIFSEL_12_W::new(self, 3)
    }
    ///Bit 4 - Differential mode for channels 15 to 1
    #[inline(always)]
    #[must_use]
    pub fn difsel_13(&mut self) -> DIFSEL_13_W<DIFSELrs> {
        DIFSEL_13_W::new(self, 4)
    }
    ///Bit 5 - Differential mode for channels 15 to 1
    #[inline(always)]
    #[must_use]
    pub fn difsel_14(&mut self) -> DIFSEL_14_W<DIFSELrs> {
        DIFSEL_14_W::new(self, 5)
    }
    ///Bit 6 - Differential mode for channels 15 to 1
    #[inline(always)]
    #[must_use]
    pub fn difsel_15(&mut self) -> DIFSEL_15_W<DIFSELrs> {
        DIFSEL_15_W::new(self, 6)
    }
    ///Bit 7 - Differential mode for channels 15 to 1
    #[inline(always)]
    #[must_use]
    pub fn difsel_16(&mut self) -> DIFSEL_16_W<DIFSELrs> {
        DIFSEL_16_W::new(self, 7)
    }
    ///Bit 8 - Differential mode for channels 15 to 1
    #[inline(always)]
    #[must_use]
    pub fn difsel_17(&mut self) -> DIFSEL_17_W<DIFSELrs> {
        DIFSEL_17_W::new(self, 8)
    }
    ///Bit 9 - Differential mode for channels 15 to 1
    #[inline(always)]
    #[must_use]
    pub fn difsel_18(&mut self) -> DIFSEL_18_W<DIFSELrs> {
        DIFSEL_18_W::new(self, 9)
    }
    ///Bit 10 - Differential mode for channels 15 to 1
    #[inline(always)]
    #[must_use]
    pub fn difsel_19(&mut self) -> DIFSEL_19_W<DIFSELrs> {
        DIFSEL_19_W::new(self, 10)
    }
    ///Bit 11 - Differential mode for channels 15 to 1
    #[inline(always)]
    #[must_use]
    pub fn difsel_110(&mut self) -> DIFSEL_110_W<DIFSELrs> {
        DIFSEL_110_W::new(self, 11)
    }
    ///Bit 12 - Differential mode for channels 15 to 1
    #[inline(always)]
    #[must_use]
    pub fn difsel_111(&mut self) -> DIFSEL_111_W<DIFSELrs> {
        DIFSEL_111_W::new(self, 12)
    }
    ///Bit 13 - Differential mode for channels 15 to 1
    #[inline(always)]
    #[must_use]
    pub fn difsel_112(&mut self) -> DIFSEL_112_W<DIFSELrs> {
        DIFSEL_112_W::new(self, 13)
    }
    ///Bit 14 - Differential mode for channels 15 to 1
    #[inline(always)]
    #[must_use]
    pub fn difsel_113(&mut self) -> DIFSEL_113_W<DIFSELrs> {
        DIFSEL_113_W::new(self, 14)
    }
    ///Bit 15 - Differential mode for channels 15 to 1
    #[inline(always)]
    #[must_use]
    pub fn difsel_114(&mut self) -> DIFSEL_114_W<DIFSELrs> {
        DIFSEL_114_W::new(self, 15)
    }
    ///Bit 16 - Differential mode for channels 15 to 1
    #[inline(always)]
    #[must_use]
    pub fn difsel_115(&mut self) -> DIFSEL_115_W<DIFSELrs> {
        DIFSEL_115_W::new(self, 16)
    }
    ///Bit 17 - Differential mode for channels 15 to 1
    #[inline(always)]
    #[must_use]
    pub fn difsel_116(&mut self) -> DIFSEL_116_W<DIFSELrs> {
        DIFSEL_116_W::new(self, 17)
    }
    ///Bit 18 - Differential mode for channels 15 to 1
    #[inline(always)]
    #[must_use]
    pub fn difsel_117(&mut self) -> DIFSEL_117_W<DIFSELrs> {
        DIFSEL_117_W::new(self, 18)
    }
}
/**Differential Mode Selection Register 2

You can [`read`](crate::Reg::read) this register and get [`difsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`difsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#ADC1:DIFSEL)*/
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
