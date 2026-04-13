///Register `MP_AXIMLPENSETR` reader
pub type R = crate::R<MP_AXIMLPENSETRrs>;
///Register `MP_AXIMLPENSETR` writer
pub type W = crate::W<MP_AXIMLPENSETRrs>;
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
        f.debug_struct("MP_AXIMLPENSETR")
            .field("sysramlpen", &self.sysramlpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSRAMLPEN
    #[inline(always)]
    pub fn sysramlpen(&mut self) -> SYSRAMLPEN_W<'_, MP_AXIMLPENSETRrs> {
        SYSRAMLPEN_W::new(self, 0)
    }
}
/**This register is used by the MPU

You can [`read`](crate::Reg::read) this register and get [`mp_aximlpensetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_aximlpensetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MP_AXIMLPENSETR)*/
pub struct MP_AXIMLPENSETRrs;
impl crate::RegisterSpec for MP_AXIMLPENSETRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_aximlpensetr::R`](R) reader structure
impl crate::Readable for MP_AXIMLPENSETRrs {}
///`write(|w| ..)` method takes [`mp_aximlpensetr::W`](W) writer structure
impl crate::Writable for MP_AXIMLPENSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_AXIMLPENSETR to value 0x01
impl crate::Resettable for MP_AXIMLPENSETRrs {
    const RESET_VALUE: u32 = 0x01;
}
