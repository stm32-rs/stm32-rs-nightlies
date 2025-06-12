///Register `BOOTSR` reader
pub type R = crate::R<BOOTSRrs>;
///Field `BOOT0` reader - BOOT0 pin value
pub type BOOT0_R = crate::BitReader;
///Field `BOOT1` reader - BOOT1 pin value
pub type BOOT1_R = crate::BitReader;
impl R {
    ///Bit 0 - BOOT0 pin value
    #[inline(always)]
    pub fn boot0(&self) -> BOOT0_R {
        BOOT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BOOT1 pin value
    #[inline(always)]
    pub fn boot1(&self) -> BOOT1_R {
        BOOT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOTSR")
            .field("boot0", &self.boot0())
            .field("boot1", &self.boot1())
            .finish()
    }
}
/**SYSCFG boot pin status register

You can [`read`](crate::Reg::read) this register and get [`bootsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#SYSCFG:BOOTSR)*/
pub struct BOOTSRrs;
impl crate::RegisterSpec for BOOTSRrs {
    type Ux = u32;
}
///`read()` method returns [`bootsr::R`](R) reader structure
impl crate::Readable for BOOTSRrs {}
///`reset()` method sets BOOTSR to value 0
impl crate::Resettable for BOOTSRrs {}
