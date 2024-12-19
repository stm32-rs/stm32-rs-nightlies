///Register `PRIVCFGR` reader
pub type R = crate::R<PRIVCFGRrs>;
///Register `PRIVCFGR` writer
pub type W = crate::W<PRIVCFGRrs>;
/**privileged state of channel x

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV0 {
    ///0: Channel is unprivileged
    Unprivileged = 0,
    ///1: Channel is privileged
    Privileged = 1,
}
impl From<PRIV0> for bool {
    #[inline(always)]
    fn from(variant: PRIV0) -> Self {
        variant as u8 != 0
    }
}
///Field `PRIV0` reader - privileged state of channel x
pub type PRIV0_R = crate::BitReader<PRIV0>;
impl PRIV0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRIV0 {
        match self.bits {
            false => PRIV0::Unprivileged,
            true => PRIV0::Privileged,
        }
    }
    ///Channel is unprivileged
    #[inline(always)]
    pub fn is_unprivileged(&self) -> bool {
        *self == PRIV0::Unprivileged
    }
    ///Channel is privileged
    #[inline(always)]
    pub fn is_privileged(&self) -> bool {
        *self == PRIV0::Privileged
    }
}
///Field `PRIV0` writer - privileged state of channel x
pub type PRIV0_W<'a, REG> = crate::BitWriter<'a, REG, PRIV0>;
impl<'a, REG> PRIV0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel is unprivileged
    #[inline(always)]
    pub fn unprivileged(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV0::Unprivileged)
    }
    ///Channel is privileged
    #[inline(always)]
    pub fn privileged(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV0::Privileged)
    }
}
///Field `PRIV1` reader - privileged state of channel x
pub use PRIV0_R as PRIV1_R;
///Field `PRIV2` reader - privileged state of channel x
pub use PRIV0_R as PRIV2_R;
///Field `PRIV3` reader - privileged state of channel x
pub use PRIV0_R as PRIV3_R;
///Field `PRIV4` reader - privileged state of channel x
pub use PRIV0_R as PRIV4_R;
///Field `PRIV5` reader - privileged state of channel x
pub use PRIV0_R as PRIV5_R;
///Field `PRIV6` reader - privileged state of channel x
pub use PRIV0_R as PRIV6_R;
///Field `PRIV7` reader - privileged state of channel x
pub use PRIV0_R as PRIV7_R;
///Field `PRIV1` writer - privileged state of channel x
pub use PRIV0_W as PRIV1_W;
///Field `PRIV2` writer - privileged state of channel x
pub use PRIV0_W as PRIV2_W;
///Field `PRIV3` writer - privileged state of channel x
pub use PRIV0_W as PRIV3_W;
///Field `PRIV4` writer - privileged state of channel x
pub use PRIV0_W as PRIV4_W;
///Field `PRIV5` writer - privileged state of channel x
pub use PRIV0_W as PRIV5_W;
///Field `PRIV6` writer - privileged state of channel x
pub use PRIV0_W as PRIV6_W;
///Field `PRIV7` writer - privileged state of channel x
pub use PRIV0_W as PRIV7_W;
impl R {
    ///Bit 0 - privileged state of channel x
    #[inline(always)]
    pub fn priv0(&self) -> PRIV0_R {
        PRIV0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - privileged state of channel x
    #[inline(always)]
    pub fn priv1(&self) -> PRIV1_R {
        PRIV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - privileged state of channel x
    #[inline(always)]
    pub fn priv2(&self) -> PRIV2_R {
        PRIV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - privileged state of channel x
    #[inline(always)]
    pub fn priv3(&self) -> PRIV3_R {
        PRIV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - privileged state of channel x
    #[inline(always)]
    pub fn priv4(&self) -> PRIV4_R {
        PRIV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - privileged state of channel x
    #[inline(always)]
    pub fn priv5(&self) -> PRIV5_R {
        PRIV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - privileged state of channel x
    #[inline(always)]
    pub fn priv6(&self) -> PRIV6_R {
        PRIV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - privileged state of channel x
    #[inline(always)]
    pub fn priv7(&self) -> PRIV7_R {
        PRIV7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR")
            .field("priv0", &self.priv0())
            .field("priv1", &self.priv1())
            .field("priv2", &self.priv2())
            .field("priv3", &self.priv3())
            .field("priv4", &self.priv4())
            .field("priv5", &self.priv5())
            .field("priv6", &self.priv6())
            .field("priv7", &self.priv7())
            .finish()
    }
}
impl W {
    ///Bit 0 - privileged state of channel x
    #[inline(always)]
    pub fn priv0(&mut self) -> PRIV0_W<PRIVCFGRrs> {
        PRIV0_W::new(self, 0)
    }
    ///Bit 1 - privileged state of channel x
    #[inline(always)]
    pub fn priv1(&mut self) -> PRIV1_W<PRIVCFGRrs> {
        PRIV1_W::new(self, 1)
    }
    ///Bit 2 - privileged state of channel x
    #[inline(always)]
    pub fn priv2(&mut self) -> PRIV2_W<PRIVCFGRrs> {
        PRIV2_W::new(self, 2)
    }
    ///Bit 3 - privileged state of channel x
    #[inline(always)]
    pub fn priv3(&mut self) -> PRIV3_W<PRIVCFGRrs> {
        PRIV3_W::new(self, 3)
    }
    ///Bit 4 - privileged state of channel x
    #[inline(always)]
    pub fn priv4(&mut self) -> PRIV4_W<PRIVCFGRrs> {
        PRIV4_W::new(self, 4)
    }
    ///Bit 5 - privileged state of channel x
    #[inline(always)]
    pub fn priv5(&mut self) -> PRIV5_W<PRIVCFGRrs> {
        PRIV5_W::new(self, 5)
    }
    ///Bit 6 - privileged state of channel x
    #[inline(always)]
    pub fn priv6(&mut self) -> PRIV6_W<PRIVCFGRrs> {
        PRIV6_W::new(self, 6)
    }
    ///Bit 7 - privileged state of channel x
    #[inline(always)]
    pub fn priv7(&mut self) -> PRIV7_W<PRIVCFGRrs> {
        PRIV7_W::new(self, 7)
    }
}
/**GPDMA privileged configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#GPDMA1:PRIVCFGR)*/
pub struct PRIVCFGRrs;
impl crate::RegisterSpec for PRIVCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr::R`](R) reader structure
impl crate::Readable for PRIVCFGRrs {}
///`write(|w| ..)` method takes [`privcfgr::W`](W) writer structure
impl crate::Writable for PRIVCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRIVCFGR to value 0
impl crate::Resettable for PRIVCFGRrs {
    const RESET_VALUE: u32 = 0;
}
