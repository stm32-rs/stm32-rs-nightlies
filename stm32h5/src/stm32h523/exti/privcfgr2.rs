///Register `PRIVCFGR2` reader
pub type R = crate::R<PRIVCFGR2rs>;
///Register `PRIVCFGR2` writer
pub type W = crate::W<PRIVCFGR2rs>;
/**Security enable on event input x

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENT_PRIVILEGE {
    ///0: Event privilege disabled
    Unprivileged = 0,
    ///1: Event privilege enabled
    Privileged = 1,
}
impl From<EVENT_PRIVILEGE> for bool {
    #[inline(always)]
    fn from(variant: EVENT_PRIVILEGE) -> Self {
        variant as u8 != 0
    }
}
///Field `PRIV32` reader - Security enable on event input x
pub type PRIV32_R = crate::BitReader<EVENT_PRIVILEGE>;
impl PRIV32_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EVENT_PRIVILEGE {
        match self.bits {
            false => EVENT_PRIVILEGE::Unprivileged,
            true => EVENT_PRIVILEGE::Privileged,
        }
    }
    ///Event privilege disabled
    #[inline(always)]
    pub fn is_unprivileged(&self) -> bool {
        *self == EVENT_PRIVILEGE::Unprivileged
    }
    ///Event privilege enabled
    #[inline(always)]
    pub fn is_privileged(&self) -> bool {
        *self == EVENT_PRIVILEGE::Privileged
    }
}
///Field `PRIV32` writer - Security enable on event input x
pub type PRIV32_W<'a, REG> = crate::BitWriter<'a, REG, EVENT_PRIVILEGE>;
impl<'a, REG> PRIV32_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Event privilege disabled
    #[inline(always)]
    pub fn unprivileged(self) -> &'a mut crate::W<REG> {
        self.variant(EVENT_PRIVILEGE::Unprivileged)
    }
    ///Event privilege enabled
    #[inline(always)]
    pub fn privileged(self) -> &'a mut crate::W<REG> {
        self.variant(EVENT_PRIVILEGE::Privileged)
    }
}
///Field `PRIV33` reader - Security enable on event input x
pub use PRIV32_R as PRIV33_R;
///Field `PRIV34` reader - Security enable on event input x
pub use PRIV32_R as PRIV34_R;
///Field `PRIV35` reader - Security enable on event input x
pub use PRIV32_R as PRIV35_R;
///Field `PRIV36` reader - Security enable on event input x
pub use PRIV32_R as PRIV36_R;
///Field `PRIV37` reader - Security enable on event input x
pub use PRIV32_R as PRIV37_R;
///Field `PRIV38` reader - Security enable on event input x
pub use PRIV32_R as PRIV38_R;
///Field `PRIV39` reader - Security enable on event input x
pub use PRIV32_R as PRIV39_R;
///Field `PRIV40` reader - Security enable on event input x
pub use PRIV32_R as PRIV40_R;
///Field `PRIV41` reader - Security enable on event input x
pub use PRIV32_R as PRIV41_R;
///Field `PRIV42` reader - Security enable on event input x
pub use PRIV32_R as PRIV42_R;
///Field `PRIV43` reader - Security enable on event input x
pub use PRIV32_R as PRIV43_R;
///Field `PRIV44` reader - Security enable on event input x
pub use PRIV32_R as PRIV44_R;
///Field `PRIV45` reader - Security enable on event input x
pub use PRIV32_R as PRIV45_R;
///Field `PRIV46` reader - Security enable on event input x
pub use PRIV32_R as PRIV46_R;
///Field `PRIV47` reader - Security enable on event input x
pub use PRIV32_R as PRIV47_R;
///Field `PRIV48` reader - Security enable on event input x
pub use PRIV32_R as PRIV48_R;
///Field `PRIV49` reader - Security enable on event input x
pub use PRIV32_R as PRIV49_R;
///Field `PRIV50` reader - Security enable on event input x
pub use PRIV32_R as PRIV50_R;
///Field `PRIV51` reader - Security enable on event input x
pub use PRIV32_R as PRIV51_R;
///Field `PRIV52` reader - Security enable on event input x
pub use PRIV32_R as PRIV52_R;
///Field `PRIV53` reader - Security enable on event input x
pub use PRIV32_R as PRIV53_R;
///Field `PRIV54` reader - Security enable on event input x
pub use PRIV32_R as PRIV54_R;
///Field `PRIV55` reader - Security enable on event input x
pub use PRIV32_R as PRIV55_R;
///Field `PRIV56` reader - Security enable on event input x
pub use PRIV32_R as PRIV56_R;
///Field `PRIV57` reader - Security enable on event input x
pub use PRIV32_R as PRIV57_R;
///Field `PRIV33` writer - Security enable on event input x
pub use PRIV32_W as PRIV33_W;
///Field `PRIV34` writer - Security enable on event input x
pub use PRIV32_W as PRIV34_W;
///Field `PRIV35` writer - Security enable on event input x
pub use PRIV32_W as PRIV35_W;
///Field `PRIV36` writer - Security enable on event input x
pub use PRIV32_W as PRIV36_W;
///Field `PRIV37` writer - Security enable on event input x
pub use PRIV32_W as PRIV37_W;
///Field `PRIV38` writer - Security enable on event input x
pub use PRIV32_W as PRIV38_W;
///Field `PRIV39` writer - Security enable on event input x
pub use PRIV32_W as PRIV39_W;
///Field `PRIV40` writer - Security enable on event input x
pub use PRIV32_W as PRIV40_W;
///Field `PRIV41` writer - Security enable on event input x
pub use PRIV32_W as PRIV41_W;
///Field `PRIV42` writer - Security enable on event input x
pub use PRIV32_W as PRIV42_W;
///Field `PRIV43` writer - Security enable on event input x
pub use PRIV32_W as PRIV43_W;
///Field `PRIV44` writer - Security enable on event input x
pub use PRIV32_W as PRIV44_W;
///Field `PRIV45` writer - Security enable on event input x
pub use PRIV32_W as PRIV45_W;
///Field `PRIV46` writer - Security enable on event input x
pub use PRIV32_W as PRIV46_W;
///Field `PRIV47` writer - Security enable on event input x
pub use PRIV32_W as PRIV47_W;
///Field `PRIV48` writer - Security enable on event input x
pub use PRIV32_W as PRIV48_W;
///Field `PRIV49` writer - Security enable on event input x
pub use PRIV32_W as PRIV49_W;
///Field `PRIV50` writer - Security enable on event input x
pub use PRIV32_W as PRIV50_W;
///Field `PRIV51` writer - Security enable on event input x
pub use PRIV32_W as PRIV51_W;
///Field `PRIV52` writer - Security enable on event input x
pub use PRIV32_W as PRIV52_W;
///Field `PRIV53` writer - Security enable on event input x
pub use PRIV32_W as PRIV53_W;
///Field `PRIV54` writer - Security enable on event input x
pub use PRIV32_W as PRIV54_W;
///Field `PRIV55` writer - Security enable on event input x
pub use PRIV32_W as PRIV55_W;
///Field `PRIV56` writer - Security enable on event input x
pub use PRIV32_W as PRIV56_W;
///Field `PRIV57` writer - Security enable on event input x
pub use PRIV32_W as PRIV57_W;
impl R {
    ///Bit 0 - Security enable on event input x
    #[inline(always)]
    pub fn priv32(&self) -> PRIV32_R {
        PRIV32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Security enable on event input x
    #[inline(always)]
    pub fn priv33(&self) -> PRIV33_R {
        PRIV33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Security enable on event input x
    #[inline(always)]
    pub fn priv34(&self) -> PRIV34_R {
        PRIV34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Security enable on event input x
    #[inline(always)]
    pub fn priv35(&self) -> PRIV35_R {
        PRIV35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Security enable on event input x
    #[inline(always)]
    pub fn priv36(&self) -> PRIV36_R {
        PRIV36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Security enable on event input x
    #[inline(always)]
    pub fn priv37(&self) -> PRIV37_R {
        PRIV37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Security enable on event input x
    #[inline(always)]
    pub fn priv38(&self) -> PRIV38_R {
        PRIV38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Security enable on event input x
    #[inline(always)]
    pub fn priv39(&self) -> PRIV39_R {
        PRIV39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Security enable on event input x
    #[inline(always)]
    pub fn priv40(&self) -> PRIV40_R {
        PRIV40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Security enable on event input x
    #[inline(always)]
    pub fn priv41(&self) -> PRIV41_R {
        PRIV41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Security enable on event input x
    #[inline(always)]
    pub fn priv42(&self) -> PRIV42_R {
        PRIV42_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Security enable on event input x
    #[inline(always)]
    pub fn priv43(&self) -> PRIV43_R {
        PRIV43_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Security enable on event input x
    #[inline(always)]
    pub fn priv44(&self) -> PRIV44_R {
        PRIV44_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Security enable on event input x
    #[inline(always)]
    pub fn priv45(&self) -> PRIV45_R {
        PRIV45_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Security enable on event input x
    #[inline(always)]
    pub fn priv46(&self) -> PRIV46_R {
        PRIV46_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Security enable on event input x
    #[inline(always)]
    pub fn priv47(&self) -> PRIV47_R {
        PRIV47_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Security enable on event input x
    #[inline(always)]
    pub fn priv48(&self) -> PRIV48_R {
        PRIV48_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Security enable on event input x
    #[inline(always)]
    pub fn priv49(&self) -> PRIV49_R {
        PRIV49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Security enable on event input x
    #[inline(always)]
    pub fn priv50(&self) -> PRIV50_R {
        PRIV50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Security enable on event input x
    #[inline(always)]
    pub fn priv51(&self) -> PRIV51_R {
        PRIV51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Security enable on event input x
    #[inline(always)]
    pub fn priv52(&self) -> PRIV52_R {
        PRIV52_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Security enable on event input x
    #[inline(always)]
    pub fn priv53(&self) -> PRIV53_R {
        PRIV53_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Security enable on event input x
    #[inline(always)]
    pub fn priv54(&self) -> PRIV54_R {
        PRIV54_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Security enable on event input x
    #[inline(always)]
    pub fn priv55(&self) -> PRIV55_R {
        PRIV55_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Security enable on event input x
    #[inline(always)]
    pub fn priv56(&self) -> PRIV56_R {
        PRIV56_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Security enable on event input x
    #[inline(always)]
    pub fn priv57(&self) -> PRIV57_R {
        PRIV57_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR2")
            .field("priv32", &self.priv32())
            .field("priv33", &self.priv33())
            .field("priv34", &self.priv34())
            .field("priv35", &self.priv35())
            .field("priv36", &self.priv36())
            .field("priv37", &self.priv37())
            .field("priv38", &self.priv38())
            .field("priv39", &self.priv39())
            .field("priv40", &self.priv40())
            .field("priv41", &self.priv41())
            .field("priv42", &self.priv42())
            .field("priv43", &self.priv43())
            .field("priv44", &self.priv44())
            .field("priv45", &self.priv45())
            .field("priv46", &self.priv46())
            .field("priv47", &self.priv47())
            .field("priv48", &self.priv48())
            .field("priv49", &self.priv49())
            .field("priv50", &self.priv50())
            .field("priv51", &self.priv51())
            .field("priv52", &self.priv52())
            .field("priv53", &self.priv53())
            .field("priv54", &self.priv54())
            .field("priv55", &self.priv55())
            .field("priv56", &self.priv56())
            .field("priv57", &self.priv57())
            .finish()
    }
}
impl W {
    ///Bit 0 - Security enable on event input x
    #[inline(always)]
    pub fn priv32(&mut self) -> PRIV32_W<'_, PRIVCFGR2rs> {
        PRIV32_W::new(self, 0)
    }
    ///Bit 1 - Security enable on event input x
    #[inline(always)]
    pub fn priv33(&mut self) -> PRIV33_W<'_, PRIVCFGR2rs> {
        PRIV33_W::new(self, 1)
    }
    ///Bit 2 - Security enable on event input x
    #[inline(always)]
    pub fn priv34(&mut self) -> PRIV34_W<'_, PRIVCFGR2rs> {
        PRIV34_W::new(self, 2)
    }
    ///Bit 3 - Security enable on event input x
    #[inline(always)]
    pub fn priv35(&mut self) -> PRIV35_W<'_, PRIVCFGR2rs> {
        PRIV35_W::new(self, 3)
    }
    ///Bit 4 - Security enable on event input x
    #[inline(always)]
    pub fn priv36(&mut self) -> PRIV36_W<'_, PRIVCFGR2rs> {
        PRIV36_W::new(self, 4)
    }
    ///Bit 5 - Security enable on event input x
    #[inline(always)]
    pub fn priv37(&mut self) -> PRIV37_W<'_, PRIVCFGR2rs> {
        PRIV37_W::new(self, 5)
    }
    ///Bit 6 - Security enable on event input x
    #[inline(always)]
    pub fn priv38(&mut self) -> PRIV38_W<'_, PRIVCFGR2rs> {
        PRIV38_W::new(self, 6)
    }
    ///Bit 7 - Security enable on event input x
    #[inline(always)]
    pub fn priv39(&mut self) -> PRIV39_W<'_, PRIVCFGR2rs> {
        PRIV39_W::new(self, 7)
    }
    ///Bit 8 - Security enable on event input x
    #[inline(always)]
    pub fn priv40(&mut self) -> PRIV40_W<'_, PRIVCFGR2rs> {
        PRIV40_W::new(self, 8)
    }
    ///Bit 9 - Security enable on event input x
    #[inline(always)]
    pub fn priv41(&mut self) -> PRIV41_W<'_, PRIVCFGR2rs> {
        PRIV41_W::new(self, 9)
    }
    ///Bit 10 - Security enable on event input x
    #[inline(always)]
    pub fn priv42(&mut self) -> PRIV42_W<'_, PRIVCFGR2rs> {
        PRIV42_W::new(self, 10)
    }
    ///Bit 11 - Security enable on event input x
    #[inline(always)]
    pub fn priv43(&mut self) -> PRIV43_W<'_, PRIVCFGR2rs> {
        PRIV43_W::new(self, 11)
    }
    ///Bit 12 - Security enable on event input x
    #[inline(always)]
    pub fn priv44(&mut self) -> PRIV44_W<'_, PRIVCFGR2rs> {
        PRIV44_W::new(self, 12)
    }
    ///Bit 13 - Security enable on event input x
    #[inline(always)]
    pub fn priv45(&mut self) -> PRIV45_W<'_, PRIVCFGR2rs> {
        PRIV45_W::new(self, 13)
    }
    ///Bit 14 - Security enable on event input x
    #[inline(always)]
    pub fn priv46(&mut self) -> PRIV46_W<'_, PRIVCFGR2rs> {
        PRIV46_W::new(self, 14)
    }
    ///Bit 15 - Security enable on event input x
    #[inline(always)]
    pub fn priv47(&mut self) -> PRIV47_W<'_, PRIVCFGR2rs> {
        PRIV47_W::new(self, 15)
    }
    ///Bit 16 - Security enable on event input x
    #[inline(always)]
    pub fn priv48(&mut self) -> PRIV48_W<'_, PRIVCFGR2rs> {
        PRIV48_W::new(self, 16)
    }
    ///Bit 17 - Security enable on event input x
    #[inline(always)]
    pub fn priv49(&mut self) -> PRIV49_W<'_, PRIVCFGR2rs> {
        PRIV49_W::new(self, 17)
    }
    ///Bit 18 - Security enable on event input x
    #[inline(always)]
    pub fn priv50(&mut self) -> PRIV50_W<'_, PRIVCFGR2rs> {
        PRIV50_W::new(self, 18)
    }
    ///Bit 19 - Security enable on event input x
    #[inline(always)]
    pub fn priv51(&mut self) -> PRIV51_W<'_, PRIVCFGR2rs> {
        PRIV51_W::new(self, 19)
    }
    ///Bit 20 - Security enable on event input x
    #[inline(always)]
    pub fn priv52(&mut self) -> PRIV52_W<'_, PRIVCFGR2rs> {
        PRIV52_W::new(self, 20)
    }
    ///Bit 21 - Security enable on event input x
    #[inline(always)]
    pub fn priv53(&mut self) -> PRIV53_W<'_, PRIVCFGR2rs> {
        PRIV53_W::new(self, 21)
    }
    ///Bit 22 - Security enable on event input x
    #[inline(always)]
    pub fn priv54(&mut self) -> PRIV54_W<'_, PRIVCFGR2rs> {
        PRIV54_W::new(self, 22)
    }
    ///Bit 23 - Security enable on event input x
    #[inline(always)]
    pub fn priv55(&mut self) -> PRIV55_W<'_, PRIVCFGR2rs> {
        PRIV55_W::new(self, 23)
    }
    ///Bit 24 - Security enable on event input x
    #[inline(always)]
    pub fn priv56(&mut self) -> PRIV56_W<'_, PRIVCFGR2rs> {
        PRIV56_W::new(self, 24)
    }
    ///Bit 25 - Security enable on event input x
    #[inline(always)]
    pub fn priv57(&mut self) -> PRIV57_W<'_, PRIVCFGR2rs> {
        PRIV57_W::new(self, 25)
    }
}
/**EXTI privilege configuration register 2

You can [`read`](crate::Reg::read) this register and get [`privcfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#EXTI:PRIVCFGR2)*/
pub struct PRIVCFGR2rs;
impl crate::RegisterSpec for PRIVCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr2::R`](R) reader structure
impl crate::Readable for PRIVCFGR2rs {}
///`write(|w| ..)` method takes [`privcfgr2::W`](W) writer structure
impl crate::Writable for PRIVCFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR2 to value 0
impl crate::Resettable for PRIVCFGR2rs {}
