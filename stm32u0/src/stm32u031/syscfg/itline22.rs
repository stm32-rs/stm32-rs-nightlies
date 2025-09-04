///Register `ITLINE22` reader
pub type R = crate::R<ITLINE22rs>;
///Field `LCD` reader - LCD interrupt request pending
pub type LCD_R = crate::BitReader;
impl R {
    ///Bit 0 - LCD interrupt request pending
    #[inline(always)]
    pub fn lcd(&self) -> LCD_R {
        LCD_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE22")
            .field("lcd", &self.lcd())
            .finish()
    }
}
/**SYSCFG interrupt line 22 status register

You can [`read`](crate::Reg::read) this register and get [`itline22::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#SYSCFG:ITLINE22)*/
pub struct ITLINE22rs;
impl crate::RegisterSpec for ITLINE22rs {
    type Ux = u32;
}
///`read()` method returns [`itline22::R`](R) reader structure
impl crate::Readable for ITLINE22rs {}
///`reset()` method sets ITLINE22 to value 0
impl crate::Resettable for ITLINE22rs {}
