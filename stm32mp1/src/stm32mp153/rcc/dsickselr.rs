///Register `DSICKSELR` reader
pub type R = crate::R<DSICKSELRrs>;
///Register `DSICKSELR` writer
pub type W = crate::W<DSICKSELRrs>;
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
        f.debug_struct("DSICKSELR")
            .field("dsisrc", &self.dsisrc())
            .finish()
    }
}
impl W {
    ///Bit 0 - DSISRC
    #[inline(always)]
    pub fn dsisrc(&mut self) -> DSISRC_W<'_, DSICKSELRrs> {
        DSISRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the DSI block.

You can [`read`](crate::Reg::read) this register and get [`dsickselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsickselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:DSICKSELR)*/
pub struct DSICKSELRrs;
impl crate::RegisterSpec for DSICKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`dsickselr::R`](R) reader structure
impl crate::Readable for DSICKSELRrs {}
///`write(|w| ..)` method takes [`dsickselr::W`](W) writer structure
impl crate::Writable for DSICKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DSICKSELR to value 0
impl crate::Resettable for DSICKSELRrs {}
