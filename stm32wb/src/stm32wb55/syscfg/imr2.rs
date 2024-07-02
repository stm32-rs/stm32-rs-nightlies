///Register `IMR2` reader
pub type R = crate::R<IMR2rs>;
///Register `IMR2` writer
pub type W = crate::W<IMR2rs>;
///Field `PVM1IM` reader - Peripheral PVM1 interrupt mask to CPU1
pub type PVM1IM_R = crate::BitReader;
///Field `PVM1IM` writer - Peripheral PVM1 interrupt mask to CPU1
pub type PVM1IM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVM3IM` reader - Peripheral PVM3 interrupt mask to CPU1
pub type PVM3IM_R = crate::BitReader;
///Field `PVM3IM` writer - Peripheral PVM3 interrupt mask to CPU1
pub type PVM3IM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDIM` reader - Peripheral PVD interrupt mask to CPU1
pub type PVDIM_R = crate::BitReader;
///Field `PVDIM` writer - Peripheral PVD interrupt mask to CPU1
pub type PVDIM_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("pvm3im", &self.pvm3im())
            .field("pvm1im", &self.pvm1im())
            .field("pvdim", &self.pvdim())
            .finish()
    }
}
impl W {
    ///Bit 16 - Peripheral PVM1 interrupt mask to CPU1
    #[inline(always)]
    #[must_use]
    pub fn pvm1im(&mut self) -> PVM1IM_W<IMR2rs> {
        PVM1IM_W::new(self, 16)
    }
    ///Bit 18 - Peripheral PVM3 interrupt mask to CPU1
    #[inline(always)]
    #[must_use]
    pub fn pvm3im(&mut self) -> PVM3IM_W<IMR2rs> {
        PVM3IM_W::new(self, 18)
    }
    ///Bit 20 - Peripheral PVD interrupt mask to CPU1
    #[inline(always)]
    #[must_use]
    pub fn pvdim(&mut self) -> PVDIM_W<IMR2rs> {
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IMR2 to value 0
impl crate::Resettable for IMR2rs {
    const RESET_VALUE: u32 = 0;
}
