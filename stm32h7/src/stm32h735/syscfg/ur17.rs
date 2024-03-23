#[doc = "Register `UR17` reader"]
pub type R = crate::R<UR17rs>;
#[doc = "Field `IO_HSLV` reader - I/O high speed / low voltage"]
pub type IO_HSLV_R = crate::BitReader;
#[doc = "Field `TCM_AXI_SHARED_CFG` reader - ITCM-RAM/AXI-SRAM size"]
pub type TCM_AXI_SHARED_CFG_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - I/O high speed / low voltage"]
    #[inline(always)]
    pub fn io_hslv(&self) -> IO_HSLV_R {
        IO_HSLV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:17 - ITCM-RAM/AXI-SRAM size"]
    #[inline(always)]
    pub fn tcm_axi_shared_cfg(&self) -> TCM_AXI_SHARED_CFG_R {
        TCM_AXI_SHARED_CFG_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[doc = "SYSCFG user register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur17::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR17rs;
impl crate::RegisterSpec for UR17rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur17::R`](R) reader structure"]
impl crate::Readable for UR17rs {}
#[doc = "`reset()` method sets UR17 to value 0"]
impl crate::Resettable for UR17rs {
    const RESET_VALUE: u32 = 0;
}
