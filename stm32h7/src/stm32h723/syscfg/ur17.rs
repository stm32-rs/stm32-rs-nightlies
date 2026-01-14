///Register `UR17` reader
pub type R = crate::R<UR17rs>;
///Field `IO_HSLV` reader - I/O high speed / low voltage
pub type IO_HSLV_R = crate::BitReader;
///Field `TCM_AXI_SHARED_CFG` reader - ITCM-RAM/AXI-SRAM size
pub type TCM_AXI_SHARED_CFG_R = crate::FieldReader;
impl R {
    ///Bit 0 - I/O high speed / low voltage
    #[inline(always)]
    pub fn io_hslv(&self) -> IO_HSLV_R {
        IO_HSLV_R::new((self.bits & 1) != 0)
    }
    ///Bits 16:17 - ITCM-RAM/AXI-SRAM size
    #[inline(always)]
    pub fn tcm_axi_shared_cfg(&self) -> TCM_AXI_SHARED_CFG_R {
        TCM_AXI_SHARED_CFG_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR17")
            .field("io_hslv", &self.io_hslv())
            .field("tcm_axi_shared_cfg", &self.tcm_axi_shared_cfg())
            .finish()
    }
}
/**SYSCFG user register 17

You can [`read`](crate::Reg::read) this register and get [`ur17::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#SYSCFG:UR17)*/
pub struct UR17rs;
impl crate::RegisterSpec for UR17rs {
    type Ux = u32;
}
///`read()` method returns [`ur17::R`](R) reader structure
impl crate::Readable for UR17rs {}
///`reset()` method sets UR17 to value 0
impl crate::Resettable for UR17rs {}
