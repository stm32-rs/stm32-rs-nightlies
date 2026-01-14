///Register `MC_AXIMLPENSETR` reader
pub type R = crate::R<MC_AXIMLPENSETRrs>;
///Register `MC_AXIMLPENSETR` writer
pub type W = crate::W<MC_AXIMLPENSETRrs>;
///Field `SYSRAMLPEN` reader - SYSRAMLPEN
pub type SYSRAMLPEN_R = crate::BitReader;
///Field `SYSRAMLPEN` writer - SYSRAMLPEN
pub type SYSRAMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SYSRAMLPEN
    #[inline(always)]
    pub fn sysramlpen(&self) -> SYSRAMLPEN_R {
        SYSRAMLPEN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MC_AXIMLPENSETR")
            .field("sysramlpen", &self.sysramlpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSRAMLPEN
    #[inline(always)]
    pub fn sysramlpen(&mut self) -> SYSRAMLPEN_W<'_, MC_AXIMLPENSETRrs> {
        SYSRAMLPEN_W::new(self, 0)
    }
}
/**This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`mc_aximlpensetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_aximlpensetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AXIMLPENSETR)*/
pub struct MC_AXIMLPENSETRrs;
impl crate::RegisterSpec for MC_AXIMLPENSETRrs {
    type Ux = u32;
}
///`read()` method returns [`mc_aximlpensetr::R`](R) reader structure
impl crate::Readable for MC_AXIMLPENSETRrs {}
///`write(|w| ..)` method takes [`mc_aximlpensetr::W`](W) writer structure
impl crate::Writable for MC_AXIMLPENSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MC_AXIMLPENSETR to value 0x01
impl crate::Resettable for MC_AXIMLPENSETRrs {
    const RESET_VALUE: u32 = 0x01;
}
