///Register `HWCFGR1` reader
pub type R = crate::R<HWCFGR1rs>;
///Field `NBSEM` reader - NBSEM
pub type NBSEM_R = crate::FieldReader;
///Field `NBINT` reader - NBINT
pub type NBINT_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - NBSEM
    #[inline(always)]
    pub fn nbsem(&self) -> NBSEM_R {
        NBSEM_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - NBINT
    #[inline(always)]
    pub fn nbint(&self) -> NBINT_R {
        NBINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR1")
            .field("nbsem", &self.nbsem())
            .field("nbint", &self.nbint())
            .finish()
    }
}
/**HSEM hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HWCFGR1)*/
pub struct HWCFGR1rs;
impl crate::RegisterSpec for HWCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr1::R`](R) reader structure
impl crate::Readable for HWCFGR1rs {}
///`reset()` method sets HWCFGR1 to value 0x0220
impl crate::Resettable for HWCFGR1rs {
    const RESET_VALUE: u32 = 0x0220;
}
