///Register `OPTSR2_CUR` reader
pub type R = crate::R<OPTSR2_CURrs>;
///Field `TCM_AXI_SHARED` reader - TCM RAM sharing status bit
pub type TCM_AXI_SHARED_R = crate::FieldReader;
///Field `CPUFREQ_BOOST` reader - CPU frequency boost status bit
pub type CPUFREQ_BOOST_R = crate::BitReader;
impl R {
    ///Bits 0:1 - TCM RAM sharing status bit
    #[inline(always)]
    pub fn tcm_axi_shared(&self) -> TCM_AXI_SHARED_R {
        TCM_AXI_SHARED_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - CPU frequency boost status bit
    #[inline(always)]
    pub fn cpufreq_boost(&self) -> CPUFREQ_BOOST_R {
        CPUFREQ_BOOST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTSR2_CUR")
            .field("tcm_axi_shared", &self.tcm_axi_shared())
            .field("cpufreq_boost", &self.cpufreq_boost())
            .finish()
    }
}
/**FLASH ECC fail address for bank 1

You can [`read`](crate::Reg::read) this register and get [`optsr2_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#FLASH:OPTSR2_CUR)*/
pub struct OPTSR2_CURrs;
impl crate::RegisterSpec for OPTSR2_CURrs {
    type Ux = u32;
}
///`read()` method returns [`optsr2_cur::R`](R) reader structure
impl crate::Readable for OPTSR2_CURrs {}
///`reset()` method sets OPTSR2_CUR to value 0
impl crate::Resettable for OPTSR2_CURrs {}
