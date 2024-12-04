///Register `HWCFGR2` reader
pub type R = crate::R<HWCFGR2rs>;
///Field `CFG1` reader - CFG1
pub type CFG1_R = crate::FieldReader;
///Field `CFG2` reader - CFG2
pub type CFG2_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - CFG1
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - CFG2
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR2")
            .field("cfg1", &self.cfg1())
            .field("cfg2", &self.cfg2())
            .finish()
    }
}
/**USART Hardware Configuration register 2

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#USART1:HWCFGR2)*/
pub struct HWCFGR2rs;
impl crate::RegisterSpec for HWCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr2::R`](R) reader structure
impl crate::Readable for HWCFGR2rs {}
///`reset()` method sets HWCFGR2 to value 0x14
impl crate::Resettable for HWCFGR2rs {
    const RESET_VALUE: u32 = 0x14;
}
