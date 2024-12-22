///Register `SYSCFG_ITLINE21` reader
pub type R = crate::R<SYSCFG_ITLINE21rs>;
///Field `TSC_MCE` reader - TSC max count error interrupt request pending
pub type TSC_MCE_R = crate::BitReader;
///Field `TSC_EOA` reader - TSC end of acquisition interrupt request pending
pub type TSC_EOA_R = crate::BitReader;
impl R {
    ///Bit 0 - TSC max count error interrupt request pending
    #[inline(always)]
    pub fn tsc_mce(&self) -> TSC_MCE_R {
        TSC_MCE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TSC end of acquisition interrupt request pending
    #[inline(always)]
    pub fn tsc_eoa(&self) -> TSC_EOA_R {
        TSC_EOA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE21")
            .field("tsc_mce", &self.tsc_mce())
            .field("tsc_eoa", &self.tsc_eoa())
            .finish()
    }
}
/**SYSCFG interrupt line 21 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline21::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#SYSCFG:SYSCFG_ITLINE21)*/
pub struct SYSCFG_ITLINE21rs;
impl crate::RegisterSpec for SYSCFG_ITLINE21rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline21::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE21rs {}
///`reset()` method sets SYSCFG_ITLINE21 to value 0
impl crate::Resettable for SYSCFG_ITLINE21rs {
    const RESET_VALUE: u32 = 0;
}
