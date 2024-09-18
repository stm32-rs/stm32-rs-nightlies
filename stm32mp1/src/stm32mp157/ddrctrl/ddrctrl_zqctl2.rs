///Register `DDRCTRL_ZQCTL2` reader
pub type R = crate::R<DDRCTRL_ZQCTL2rs>;
///Register `DDRCTRL_ZQCTL2` writer
pub type W = crate::W<DDRCTRL_ZQCTL2rs>;
///Field `ZQ_RESET` reader - ZQ_RESET
pub type ZQ_RESET_R = crate::BitReader;
///Field `ZQ_RESET` writer - ZQ_RESET
pub type ZQ_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ZQ_RESET
    #[inline(always)]
    pub fn zq_reset(&self) -> ZQ_RESET_R {
        ZQ_RESET_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRCTRL_ZQCTL2")
            .field("zq_reset", &self.zq_reset())
            .finish()
    }
}
impl W {
    ///Bit 0 - ZQ_RESET
    #[inline(always)]
    #[must_use]
    pub fn zq_reset(&mut self) -> ZQ_RESET_W<DDRCTRL_ZQCTL2rs> {
        ZQ_RESET_W::new(self, 0)
    }
}
/**DDRCTRL ZQ control register 2

You can [`read`](crate::Reg::read) this register and get [`ddrctrl_zqctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrctrl_zqctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DDRCTRL_ZQCTL2)*/
pub struct DDRCTRL_ZQCTL2rs;
impl crate::RegisterSpec for DDRCTRL_ZQCTL2rs {
    type Ux = u32;
}
///`read()` method returns [`ddrctrl_zqctl2::R`](R) reader structure
impl crate::Readable for DDRCTRL_ZQCTL2rs {}
///`write(|w| ..)` method takes [`ddrctrl_zqctl2::W`](W) writer structure
impl crate::Writable for DDRCTRL_ZQCTL2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRCTRL_ZQCTL2 to value 0
impl crate::Resettable for DDRCTRL_ZQCTL2rs {
    const RESET_VALUE: u32 = 0;
}
