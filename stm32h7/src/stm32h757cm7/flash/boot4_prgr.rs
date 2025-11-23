///Register `BOOT4_PRGR` reader
pub type R = crate::R<BOOT4_PRGRrs>;
///Register `BOOT4_PRGR` writer
pub type W = crate::W<BOOT4_PRGRrs>;
///Field `BOOT_CM4_ADD0` reader - Arm Cortex-M4 boot address 0 configuration
pub type BOOT_CM4_ADD0_R = crate::FieldReader<u16>;
///Field `BOOT_CM4_ADD0` writer - Arm Cortex-M4 boot address 0 configuration
pub type BOOT_CM4_ADD0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `BOOT_CM4_ADD1` reader - Arm Cortex-M4 boot address 1 configuration
pub type BOOT_CM4_ADD1_R = crate::FieldReader<u16>;
///Field `BOOT_CM4_ADD1` writer - Arm Cortex-M4 boot address 1 configuration
pub type BOOT_CM4_ADD1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Arm Cortex-M4 boot address 0 configuration
    #[inline(always)]
    pub fn boot_cm4_add0(&self) -> BOOT_CM4_ADD0_R {
        BOOT_CM4_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Arm Cortex-M4 boot address 1 configuration
    #[inline(always)]
    pub fn boot_cm4_add1(&self) -> BOOT_CM4_ADD1_R {
        BOOT_CM4_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT4_PRGR")
            .field("boot_cm4_add1", &self.boot_cm4_add1())
            .field("boot_cm4_add0", &self.boot_cm4_add0())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Arm Cortex-M4 boot address 0 configuration
    #[inline(always)]
    pub fn boot_cm4_add0(&mut self) -> BOOT_CM4_ADD0_W<'_, BOOT4_PRGRrs> {
        BOOT_CM4_ADD0_W::new(self, 0)
    }
    ///Bits 16:31 - Arm Cortex-M4 boot address 1 configuration
    #[inline(always)]
    pub fn boot_cm4_add1(&mut self) -> BOOT_CM4_ADD1_W<'_, BOOT4_PRGRrs> {
        BOOT_CM4_ADD1_W::new(self, 16)
    }
}
/**FLASH register boot address for Arm Cortex-M4 core

You can [`read`](crate::Reg::read) this register and get [`boot4_prgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot4_prgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#FLASH:BOOT4_PRGR)*/
pub struct BOOT4_PRGRrs;
impl crate::RegisterSpec for BOOT4_PRGRrs {
    type Ux = u32;
}
///`read()` method returns [`boot4_prgr::R`](R) reader structure
impl crate::Readable for BOOT4_PRGRrs {}
///`write(|w| ..)` method takes [`boot4_prgr::W`](W) writer structure
impl crate::Writable for BOOT4_PRGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BOOT4_PRGR to value 0
impl crate::Resettable for BOOT4_PRGRrs {}
