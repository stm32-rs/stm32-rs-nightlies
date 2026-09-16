///Register `BOOTCR` reader
pub type R = crate::R<BOOTCRrs>;
///Register `BOOTCR` writer
pub type W = crate::W<BOOTCRrs>;
///Field `BOOT0_PD` reader - BOOT0 pin pull-down disable
pub type BOOT0_PD_R = crate::BitReader;
///Field `BOOT0_PD` writer - BOOT0 pin pull-down disable
pub type BOOT0_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOT1_PD` reader - BOOT1 pin pull-down disable
pub type BOOT1_PD_R = crate::BitReader;
///Field `BOOT1_PD` writer - BOOT1 pin pull-down disable
pub type BOOT1_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - BOOT0 pin pull-down disable
    #[inline(always)]
    pub fn boot0_pd(&self) -> BOOT0_PD_R {
        BOOT0_PD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BOOT1 pin pull-down disable
    #[inline(always)]
    pub fn boot1_pd(&self) -> BOOT1_PD_R {
        BOOT1_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOTCR")
            .field("boot0_pd", &self.boot0_pd())
            .field("boot1_pd", &self.boot1_pd())
            .finish()
    }
}
impl W {
    ///Bit 0 - BOOT0 pin pull-down disable
    #[inline(always)]
    pub fn boot0_pd(&mut self) -> BOOT0_PD_W<'_, BOOTCRrs> {
        BOOT0_PD_W::new(self, 0)
    }
    ///Bit 1 - BOOT1 pin pull-down disable
    #[inline(always)]
    pub fn boot1_pd(&mut self) -> BOOT1_PD_W<'_, BOOTCRrs> {
        BOOT1_PD_W::new(self, 1)
    }
}
/**SYSCFG boot pin control register

You can [`read`](crate::Reg::read) this register and get [`bootcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SYSCFG:BOOTCR)*/
pub struct BOOTCRrs;
impl crate::RegisterSpec for BOOTCRrs {
    type Ux = u32;
}
///`read()` method returns [`bootcr::R`](R) reader structure
impl crate::Readable for BOOTCRrs {}
///`write(|w| ..)` method takes [`bootcr::W`](W) writer structure
impl crate::Writable for BOOTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BOOTCR to value 0
impl crate::Resettable for BOOTCRrs {}
