///Register `MP_GCR` reader
pub type R = crate::R<MP_GCRrs>;
///Register `MP_GCR` writer
pub type W = crate::W<MP_GCRrs>;
///Field `BOOT_MCU` reader - BOOT_MCU
pub type BOOT_MCU_R = crate::BitReader;
///Field `BOOT_MCU` writer - BOOT_MCU
pub type BOOT_MCU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - BOOT_MCU
    #[inline(always)]
    pub fn boot_mcu(&self) -> BOOT_MCU_R {
        BOOT_MCU_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_GCR")
            .field("boot_mcu", &self.boot_mcu())
            .finish()
    }
}
impl W {
    ///Bit 0 - BOOT_MCU
    #[inline(always)]
    pub fn boot_mcu(&mut self) -> BOOT_MCU_W<'_, MP_GCRrs> {
        BOOT_MCU_W::new(self, 0)
    }
}
/**The register contains global control bits. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MP_GCR)*/
pub struct MP_GCRrs;
impl crate::RegisterSpec for MP_GCRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_gcr::R`](R) reader structure
impl crate::Readable for MP_GCRrs {}
///`write(|w| ..)` method takes [`mp_gcr::W`](W) writer structure
impl crate::Writable for MP_GCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_GCR to value 0
impl crate::Resettable for MP_GCRrs {}
