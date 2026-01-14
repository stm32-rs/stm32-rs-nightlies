///Register `BOOT7_CURR_` reader
pub type R = crate::R<BOOT7_CURR_rs>;
///Register `BOOT7_CURR_` writer
pub type W = crate::W<BOOT7_CURR_rs>;
///Field `BOOT_CM7_ADD0` reader - Arm Cortex-M7 boot address 0
pub type BOOT_CM7_ADD0_R = crate::FieldReader<u16>;
///Field `BOOT_CM7_ADD0` writer - Arm Cortex-M7 boot address 0
pub type BOOT_CM7_ADD0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `BOOT_CM7_ADD1` reader - Arm Cortex-M7 boot address 1
pub type BOOT_CM7_ADD1_R = crate::FieldReader<u16>;
///Field `BOOT_CM7_ADD1` writer - Arm Cortex-M7 boot address 1
pub type BOOT_CM7_ADD1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Arm Cortex-M7 boot address 0
    #[inline(always)]
    pub fn boot_cm7_add0(&self) -> BOOT_CM7_ADD0_R {
        BOOT_CM7_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Arm Cortex-M7 boot address 1
    #[inline(always)]
    pub fn boot_cm7_add1(&self) -> BOOT_CM7_ADD1_R {
        BOOT_CM7_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT7_CURR_")
            .field("boot_cm7_add1", &self.boot_cm7_add1())
            .field("boot_cm7_add0", &self.boot_cm7_add0())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Arm Cortex-M7 boot address 0
    #[inline(always)]
    pub fn boot_cm7_add0(&mut self) -> BOOT_CM7_ADD0_W<'_, BOOT7_CURR_rs> {
        BOOT_CM7_ADD0_W::new(self, 0)
    }
    ///Bits 16:31 - Arm Cortex-M7 boot address 1
    #[inline(always)]
    pub fn boot_cm7_add1(&mut self) -> BOOT_CM7_ADD1_W<'_, BOOT7_CURR_rs> {
        BOOT_CM7_ADD1_W::new(self, 16)
    }
}
/**FLASH register boot address for Arm Cortex-M7 core

You can [`read`](crate::Reg::read) this register and get [`boot7_curr_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot7_curr_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#FLASH:BOOT7_CURR_)*/
pub struct BOOT7_CURR_rs;
impl crate::RegisterSpec for BOOT7_CURR_rs {
    type Ux = u32;
}
///`read()` method returns [`boot7_curr_::R`](R) reader structure
impl crate::Readable for BOOT7_CURR_rs {}
///`write(|w| ..)` method takes [`boot7_curr_::W`](W) writer structure
impl crate::Writable for BOOT7_CURR_rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BOOT7_CURR_ to value 0
impl crate::Resettable for BOOT7_CURR_rs {}
