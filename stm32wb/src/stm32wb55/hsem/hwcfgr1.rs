///Register `HWCFGR1` reader
pub type R = crate::R<HWCFGR1rs>;
///Field `NBSEM` reader - Hardware Configuration number of semaphores
pub type NBSEM_R = crate::FieldReader;
///Field `NBINT` reader - Hardware Configuration number of interrupts supported number of master IDs
pub type NBINT_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Hardware Configuration number of semaphores
    #[inline(always)]
    pub fn nbsem(&self) -> NBSEM_R {
        NBSEM_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Hardware Configuration number of interrupts supported number of master IDs
    #[inline(always)]
    pub fn nbint(&self) -> NBINT_R {
        NBINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR1")
            .field("nbint", &self.nbint())
            .field("nbsem", &self.nbsem())
            .finish()
    }
}
/**Semaphore hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#HSEM:HWCFGR1)*/
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
