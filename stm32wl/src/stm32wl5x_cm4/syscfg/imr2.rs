///Register `IMR2` reader
pub type R = crate::R<IMR2rs>;
///Register `IMR2` writer
pub type W = crate::W<IMR2rs>;
///Field `PVM3IM` reader - PVM3IM
pub type PVM3IM_R = crate::BitReader;
///Field `PVM3IM` writer - PVM3IM
pub type PVM3IM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDIM` reader - PVDIM
pub type PVDIM_R = crate::BitReader;
///Field `PVDIM` writer - PVDIM
pub type PVDIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 18 - PVM3IM
    #[inline(always)]
    pub fn pvm3im(&self) -> PVM3IM_R {
        PVM3IM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - PVDIM
    #[inline(always)]
    pub fn pvdim(&self) -> PVDIM_R {
        PVDIM_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR2")
            .field("pvm3im", &self.pvm3im())
            .field("pvdim", &self.pvdim())
            .finish()
    }
}
impl W {
    ///Bit 18 - PVM3IM
    #[inline(always)]
    pub fn pvm3im(&mut self) -> PVM3IM_W<'_, IMR2rs> {
        PVM3IM_W::new(self, 18)
    }
    ///Bit 20 - PVDIM
    #[inline(always)]
    pub fn pvdim(&mut self) -> PVDIM_W<'_, IMR2rs> {
        PVDIM_W::new(self, 20)
    }
}
/**SYSCFG CPU1 interrupt mask register 2

You can [`read`](crate::Reg::read) this register and get [`imr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#SYSCFG:IMR2)*/
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
