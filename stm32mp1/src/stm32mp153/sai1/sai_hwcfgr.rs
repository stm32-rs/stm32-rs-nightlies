///Register `SAI_HWCFGR` reader
pub type R = crate::R<SAI_HWCFGRrs>;
///Field `FIFO_SIZE` reader - FIFO_SIZE
pub type FIFO_SIZE_R = crate::FieldReader;
///Field `SPDIF_PDM` reader - SPDIF_PDM
pub type SPDIF_PDM_R = crate::FieldReader;
///Field `OPTION_REGOUT` reader - OPTION_REGOUT
pub type OPTION_REGOUT_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - FIFO_SIZE
    #[inline(always)]
    pub fn fifo_size(&self) -> FIFO_SIZE_R {
        FIFO_SIZE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - SPDIF_PDM
    #[inline(always)]
    pub fn spdif_pdm(&self) -> SPDIF_PDM_R {
        SPDIF_PDM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:19 - OPTION_REGOUT
    #[inline(always)]
    pub fn option_regout(&self) -> OPTION_REGOUT_R {
        OPTION_REGOUT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAI_HWCFGR")
            .field("fifo_size", &self.fifo_size())
            .field("spdif_pdm", &self.spdif_pdm())
            .field("option_regout", &self.option_regout())
            .finish()
    }
}
/**SAI hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`sai_hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SAI1:SAI_HWCFGR)*/
pub struct SAI_HWCFGRrs;
impl crate::RegisterSpec for SAI_HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`sai_hwcfgr::R`](R) reader structure
impl crate::Readable for SAI_HWCFGRrs {}
///`reset()` method sets SAI_HWCFGR to value 0x0108
impl crate::Resettable for SAI_HWCFGRrs {
    const RESET_VALUE: u32 = 0x0108;
}
