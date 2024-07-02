///Register `ITLINE18` reader
pub type R = crate::R<ITLINE18rs>;
///Field `LPTIM2` reader - LPTIM2
pub type LPTIM2_R = crate::BitReader;
impl R {
    ///Bit 1 - LPTIM2
    #[inline(always)]
    pub fn lptim2(&self) -> LPTIM2_R {
        LPTIM2_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE18")
            .field("lptim2", &self.lptim2())
            .finish()
    }
}
/**interrupt line 18 status register

You can [`read`](crate::Reg::read) this register and get [`itline18::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG_ITLINE:ITLINE18)*/
pub struct ITLINE18rs;
impl crate::RegisterSpec for ITLINE18rs {
    type Ux = u32;
}
///`read()` method returns [`itline18::R`](R) reader structure
impl crate::Readable for ITLINE18rs {}
///`reset()` method sets ITLINE18 to value 0
impl crate::Resettable for ITLINE18rs {
    const RESET_VALUE: u32 = 0;
}
