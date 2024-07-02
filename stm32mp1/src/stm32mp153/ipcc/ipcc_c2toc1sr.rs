///Register `IPCC_C2TOC1SR` reader
pub type R = crate::R<IPCC_C2TOC1SRrs>;
///Field `CHxF` reader - CHxF
pub type CHX_F_R = crate::FieldReader;
impl R {
    ///Bits 0:5 - CHxF
    #[inline(always)]
    pub fn chx_f(&self) -> CHX_F_R {
        CHX_F_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPCC_C2TOC1SR")
            .field("chx_f", &self.chx_f())
            .finish()
    }
}
/**IPCC processor 2 to processor 1 status register

You can [`read`](crate::Reg::read) this register and get [`ipcc_c2toc1sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#IPCC:IPCC_C2TOC1SR)*/
pub struct IPCC_C2TOC1SRrs;
impl crate::RegisterSpec for IPCC_C2TOC1SRrs {
    type Ux = u32;
}
///`read()` method returns [`ipcc_c2toc1sr::R`](R) reader structure
impl crate::Readable for IPCC_C2TOC1SRrs {}
///`reset()` method sets IPCC_C2TOC1SR to value 0
impl crate::Resettable for IPCC_C2TOC1SRrs {
    const RESET_VALUE: u32 = 0;
}
