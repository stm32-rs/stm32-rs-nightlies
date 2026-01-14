///Register `ITLINE21` reader
pub type R = crate::R<ITLINE21rs>;
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
        f.debug_struct("ITLINE21")
            .field("tsc_mce", &self.tsc_mce())
            .field("tsc_eoa", &self.tsc_eoa())
            .finish()
    }
}
/**SYSCFG interrupt line 21 status register

You can [`read`](crate::Reg::read) this register and get [`itline21::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#SYSCFG:ITLINE21)*/
pub struct ITLINE21rs;
impl crate::RegisterSpec for ITLINE21rs {
    type Ux = u32;
}
///`read()` method returns [`itline21::R`](R) reader structure
impl crate::Readable for ITLINE21rs {}
///`reset()` method sets ITLINE21 to value 0
impl crate::Resettable for ITLINE21rs {}
