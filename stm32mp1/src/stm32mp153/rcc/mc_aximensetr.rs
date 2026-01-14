///Register `MC_AXIMENSETR` reader
pub type R = crate::R<MC_AXIMENSETRrs>;
///Register `MC_AXIMENSETR` writer
pub type W = crate::W<MC_AXIMENSETRrs>;
///Field `SYSRAMEN` reader - SYSRAMEN
pub type SYSRAMEN_R = crate::BitReader;
///Field `SYSRAMEN` writer - SYSRAMEN
pub type SYSRAMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SYSRAMEN
    #[inline(always)]
    pub fn sysramen(&self) -> SYSRAMEN_R {
        SYSRAMEN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MC_AXIMENSETR")
            .field("sysramen", &self.sysramen())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSRAMEN
    #[inline(always)]
    pub fn sysramen(&mut self) -> SYSRAMEN_W<'_, MC_AXIMENSETRrs> {
        SYSRAMEN_W::new(self, 0)
    }
}
/**This register is used to set the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_aximensetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_aximensetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AXIMENSETR)*/
pub struct MC_AXIMENSETRrs;
impl crate::RegisterSpec for MC_AXIMENSETRrs {
    type Ux = u32;
}
///`read()` method returns [`mc_aximensetr::R`](R) reader structure
impl crate::Readable for MC_AXIMENSETRrs {}
///`write(|w| ..)` method takes [`mc_aximensetr::W`](W) writer structure
impl crate::Writable for MC_AXIMENSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MC_AXIMENSETR to value 0
impl crate::Resettable for MC_AXIMENSETRrs {}
