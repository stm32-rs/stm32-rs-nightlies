///Register `BOOT_PRG` reader
pub type R = crate::R<BOOT_PRGrs>;
///Field `BOOT_CM_ADD0` reader - Boot address 0
pub type BOOT_CM_ADD0_R = crate::FieldReader<u16>;
///Field `BOOT_CM_ADD1` reader - Boot address 1
pub type BOOT_CM_ADD1_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Boot address 0
    #[inline(always)]
    pub fn boot_cm_add0(&self) -> BOOT_CM_ADD0_R {
        BOOT_CM_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Boot address 1
    #[inline(always)]
    pub fn boot_cm_add1(&self) -> BOOT_CM_ADD1_R {
        BOOT_CM_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_PRG")
            .field("boot_cm_add0", &self.boot_cm_add0())
            .field("boot_cm_add1", &self.boot_cm_add1())
            .finish()
    }
}
/**FLASH register with boot address

You can [`read`](crate::Reg::read) this register and get [`boot_prg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FLASH:BOOT_PRG)*/
pub struct BOOT_PRGrs;
impl crate::RegisterSpec for BOOT_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`boot_prg::R`](R) reader structure
impl crate::Readable for BOOT_PRGrs {}
///`reset()` method sets BOOT_PRG to value 0
impl crate::Resettable for BOOT_PRGrs {}
