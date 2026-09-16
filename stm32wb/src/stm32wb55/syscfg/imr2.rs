///Register `IMR2` reader
pub type R = crate::R<IMR2rs>;
///Register `IMR2` writer
pub type W = crate::W<IMR2rs>;
/**Peripheral PVM1 interrupt mask to CPU1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVM1IM {
    ///0: Peripheral interrupt forwarded to CPU1
    Unmasked = 0,
    ///1: Peripheral interrupt to CPU1 masked
    Masked = 1,
}
impl From<PVM1IM> for bool {
    #[inline(always)]
    fn from(variant: PVM1IM) -> Self {
        variant as u8 != 0
    }
}
///Field `PVM1IM` reader - Peripheral PVM1 interrupt mask to CPU1
pub type PVM1IM_R = crate::BitReader<PVM1IM>;
impl PVM1IM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVM1IM {
        match self.bits {
            false => PVM1IM::Unmasked,
            true => PVM1IM::Masked,
        }
    }
    ///Peripheral interrupt forwarded to CPU1
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == PVM1IM::Unmasked
    }
    ///Peripheral interrupt to CPU1 masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PVM1IM::Masked
    }
}
///Field `PVM1IM` writer - Peripheral PVM1 interrupt mask to CPU1
pub type PVM1IM_W<'a, REG> = crate::BitWriter<'a, REG, PVM1IM>;
impl<'a, REG> PVM1IM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral interrupt forwarded to CPU1
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(PVM1IM::Unmasked)
    }
    ///Peripheral interrupt to CPU1 masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(PVM1IM::Masked)
    }
}
///Field `PVM3IM` reader - Peripheral PVM3 interrupt mask to CPU1
pub use PVM1IM_R as PVM3IM_R;
///Field `PVDIM` reader - Peripheral PVD interrupt mask to CPU1
pub use PVM1IM_R as PVDIM_R;
///Field `PVM3IM` writer - Peripheral PVM3 interrupt mask to CPU1
pub use PVM1IM_W as PVM3IM_W;
///Field `PVDIM` writer - Peripheral PVD interrupt mask to CPU1
pub use PVM1IM_W as PVDIM_W;
impl R {
    ///Bit 16 - Peripheral PVM1 interrupt mask to CPU1
    #[inline(always)]
    pub fn pvm1im(&self) -> PVM1IM_R {
        PVM1IM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Peripheral PVM3 interrupt mask to CPU1
    #[inline(always)]
    pub fn pvm3im(&self) -> PVM3IM_R {
        PVM3IM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - Peripheral PVD interrupt mask to CPU1
    #[inline(always)]
    pub fn pvdim(&self) -> PVDIM_R {
        PVDIM_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR2")
            .field("pvm1im", &self.pvm1im())
            .field("pvm3im", &self.pvm3im())
            .field("pvdim", &self.pvdim())
            .finish()
    }
}
impl W {
    ///Bit 16 - Peripheral PVM1 interrupt mask to CPU1
    #[inline(always)]
    pub fn pvm1im(&mut self) -> PVM1IM_W<'_, IMR2rs> {
        PVM1IM_W::new(self, 16)
    }
    ///Bit 18 - Peripheral PVM3 interrupt mask to CPU1
    #[inline(always)]
    pub fn pvm3im(&mut self) -> PVM3IM_W<'_, IMR2rs> {
        PVM3IM_W::new(self, 18)
    }
    ///Bit 20 - Peripheral PVD interrupt mask to CPU1
    #[inline(always)]
    pub fn pvdim(&mut self) -> PVDIM_W<'_, IMR2rs> {
        PVDIM_W::new(self, 20)
    }
}
/**CPU1 interrupt mask register 2

You can [`read`](crate::Reg::read) this register and get [`imr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:IMR2)*/
pub struct IMR2rs;
impl crate::RegisterSpec for IMR2rs {
    type Ux = u32;
}
///`read()` method returns [`imr2::R`](R) reader structure
impl crate::Readable for IMR2rs {}
///`write(|w| ..)` method takes [`imr2::W`](W) writer structure
impl crate::Writable for IMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IMR2 to value 0
impl crate::Resettable for IMR2rs {}
