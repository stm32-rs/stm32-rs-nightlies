///Register `NSBOOTR_CUR` reader
pub type R = crate::R<NSBOOTR_CURrs>;
///Field `NSBOOT_LOCK` reader - A field locking the values of SWAP_BANK, and NSBOOTADD settings.
pub type NSBOOT_LOCK_R = crate::FieldReader;
///Field `NSBOOTADD` reader - Non secure unique boot entry address These bits reflect the Non secure UBE address
pub type NSBOOTADD_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:7 - A field locking the values of SWAP_BANK, and NSBOOTADD settings.
    #[inline(always)]
    pub fn nsboot_lock(&self) -> NSBOOT_LOCK_R {
        NSBOOT_LOCK_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:31 - Non secure unique boot entry address These bits reflect the Non secure UBE address
    #[inline(always)]
    pub fn nsbootadd(&self) -> NSBOOTADD_R {
        NSBOOTADD_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NSBOOTR_CUR")
            .field("nsboot_lock", &self.nsboot_lock())
            .field("nsbootadd", &self.nsbootadd())
            .finish()
    }
}
/**FLASH non-secure boot register

You can [`read`](crate::Reg::read) this register and get [`nsbootr_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#FLASH:NSBOOTR_CUR)*/
pub struct NSBOOTR_CURrs;
impl crate::RegisterSpec for NSBOOTR_CURrs {
    type Ux = u32;
}
///`read()` method returns [`nsbootr_cur::R`](R) reader structure
impl crate::Readable for NSBOOTR_CURrs {}
///`reset()` method sets NSBOOTR_CUR to value 0
impl crate::Resettable for NSBOOTR_CURrs {}
