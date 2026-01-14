///Register `AHBWP_ERROR_SR` reader
pub type R = crate::R<AHBWP_ERROR_SRrs>;
///Field `PAHB_ERROR_ADDR` reader - Reports address of the first error in P-AHB write-posting buffer
pub type PAHB_ERROR_ADDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Reports address of the first error in P-AHB write-posting buffer
    #[inline(always)]
    pub fn pahb_error_addr(&self) -> PAHB_ERROR_ADDR_R {
        PAHB_ERROR_ADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBWP_ERROR_SR")
            .field("pahb_error_addr", &self.pahb_error_addr())
            .finish()
    }
}
/**SYSCFG AHB write posting address error register

You can [`read`](crate::Reg::read) this register and get [`ahbwp_error_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#SYSCFG:AHBWP_ERROR_SR)*/
pub struct AHBWP_ERROR_SRrs;
impl crate::RegisterSpec for AHBWP_ERROR_SRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbwp_error_sr::R`](R) reader structure
impl crate::Readable for AHBWP_ERROR_SRrs {}
///`reset()` method sets AHBWP_ERROR_SR to value 0
impl crate::Resettable for AHBWP_ERROR_SRrs {}
