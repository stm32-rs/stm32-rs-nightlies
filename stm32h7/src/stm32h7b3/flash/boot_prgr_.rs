///Register `BOOT_PRGR_` reader
pub type R = crate::R<BOOT_PRGR_rs>;
///Field `BOOT_ADD0` reader - Boot address 0
pub type BOOT_ADD0_R = crate::FieldReader<u16>;
///Field `BOOT_ADD1` reader - Boot address 1
pub type BOOT_ADD1_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Boot address 0
    #[inline(always)]
    pub fn boot_add0(&self) -> BOOT_ADD0_R {
        BOOT_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Boot address 1
    #[inline(always)]
    pub fn boot_add1(&self) -> BOOT_ADD1_R {
        BOOT_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_PRGR_")
            .field("boot_add0", &self.boot_add0())
            .field("boot_add1", &self.boot_add1())
            .finish()
    }
}
/**FLASH register with boot address

You can [`read`](crate::Reg::read) this register and get [`boot_prgr_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#FLASH:BOOT_PRGR_)*/
pub struct BOOT_PRGR_rs;
impl crate::RegisterSpec for BOOT_PRGR_rs {
    type Ux = u32;
}
///`read()` method returns [`boot_prgr_::R`](R) reader structure
impl crate::Readable for BOOT_PRGR_rs {}
///`reset()` method sets BOOT_PRGR_ to value 0
impl crate::Resettable for BOOT_PRGR_rs {}
