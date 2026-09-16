///Register `COMP_CTN` reader
pub type R = crate::R<COMP_CTNrs>;
///Field `CMP_LCA_CNT` reader - LCA Comparator last damping count
pub type CMP_LCA_CNT_R = crate::FieldReader;
///Field `CMP_LCB_CNT` reader - LCB Comparator last damping count
pub type CMP_LCB_CNT_R = crate::FieldReader;
///Field `CMP_LCT_CNT` reader - LCT Comparator last damping count
pub type CMP_LCT_CNT_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - LCA Comparator last damping count
    #[inline(always)]
    pub fn cmp_lca_cnt(&self) -> CMP_LCA_CNT_R {
        CMP_LCA_CNT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 10:17 - LCB Comparator last damping count
    #[inline(always)]
    pub fn cmp_lcb_cnt(&self) -> CMP_LCB_CNT_R {
        CMP_LCB_CNT_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    ///Bits 20:27 - LCT Comparator last damping count
    #[inline(always)]
    pub fn cmp_lct_cnt(&self) -> CMP_LCT_CNT_R {
        CMP_LCT_CNT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP_CTN")
            .field("cmp_lca_cnt", &self.cmp_lca_cnt())
            .field("cmp_lcb_cnt", &self.cmp_lcb_cnt())
            .field("cmp_lct_cnt", &self.cmp_lct_cnt())
            .finish()
    }
}
/**LCSC_COMP_CTN register

You can [`read`](crate::Reg::read) this register and get [`comp_ctn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:COMP_CTN)*/
pub struct COMP_CTNrs;
impl crate::RegisterSpec for COMP_CTNrs {
    type Ux = u32;
}
///`read()` method returns [`comp_ctn::R`](R) reader structure
impl crate::Readable for COMP_CTNrs {}
///`reset()` method sets COMP_CTN to value 0
impl crate::Resettable for COMP_CTNrs {}
