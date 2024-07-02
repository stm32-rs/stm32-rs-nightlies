///Register `RCC_DSICKSELR` reader
pub type R = crate::R<RCC_DSICKSELRrs>;
///Register `RCC_DSICKSELR` writer
pub type W = crate::W<RCC_DSICKSELRrs>;
///Field `DSISRC` reader - DSISRC
pub type DSISRC_R = crate::BitReader;
///Field `DSISRC` writer - DSISRC
pub type DSISRC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DSISRC
    #[inline(always)]
    pub fn dsisrc(&self) -> DSISRC_R {
        DSISRC_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_DSICKSELR")
            .field("dsisrc", &self.dsisrc())
            .finish()
    }
}
impl W {
    ///Bit 0 - DSISRC
    #[inline(always)]
    #[must_use]
    pub fn dsisrc(&mut self) -> DSISRC_W<RCC_DSICKSELRrs> {
        DSISRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the DSI block.

You can [`read`](crate::Reg::read) this register and get [`rcc_dsickselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_dsickselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:RCC_DSICKSELR)*/
pub struct RCC_DSICKSELRrs;
impl crate::RegisterSpec for RCC_DSICKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_dsickselr::R`](R) reader structure
impl crate::Readable for RCC_DSICKSELRrs {}
///`write(|w| ..)` method takes [`rcc_dsickselr::W`](W) writer structure
impl crate::Writable for RCC_DSICKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_DSICKSELR to value 0
impl crate::Resettable for RCC_DSICKSELRrs {
    const RESET_VALUE: u32 = 0;
}
