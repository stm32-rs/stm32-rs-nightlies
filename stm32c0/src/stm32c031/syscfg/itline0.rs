///Register `ITLINE0` reader
pub type R = crate::R<ITLINE0rs>;
///Field `WWDG` reader - Window watchdog interrupt pending flag
pub type WWDG_R = crate::BitReader;
impl R {
    ///Bit 0 - Window watchdog interrupt pending flag
    #[inline(always)]
    pub fn wwdg(&self) -> WWDG_R {
        WWDG_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE0")
            .field("wwdg", &self.wwdg())
            .finish()
    }
}
/**SYSCFG interrupt line 0 status register

You can [`read`](crate::Reg::read) this register and get [`itline0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE0)*/
pub struct ITLINE0rs;
impl crate::RegisterSpec for ITLINE0rs {
    type Ux = u32;
}
///`read()` method returns [`itline0::R`](R) reader structure
impl crate::Readable for ITLINE0rs {}
///`reset()` method sets ITLINE0 to value 0
impl crate::Resettable for ITLINE0rs {
    const RESET_VALUE: u32 = 0;
}
