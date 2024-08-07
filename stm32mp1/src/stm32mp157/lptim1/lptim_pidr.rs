///Register `LPTIM_PIDR` reader
pub type R = crate::R<LPTIM_PIDRrs>;
///Field `P_ID` reader - P_ID
pub type P_ID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - P_ID
    #[inline(always)]
    pub fn p_id(&self) -> P_ID_R {
        P_ID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_PIDR")
            .field("p_id", &self.p_id())
            .finish()
    }
}
/**LPTIM peripheral type identification register

You can [`read`](crate::Reg::read) this register and get [`lptim_pidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:LPTIM_PIDR)*/
pub struct LPTIM_PIDRrs;
impl crate::RegisterSpec for LPTIM_PIDRrs {
    type Ux = u32;
}
///`read()` method returns [`lptim_pidr::R`](R) reader structure
impl crate::Readable for LPTIM_PIDRrs {}
///`reset()` method sets LPTIM_PIDR to value 0x0012_0011
impl crate::Resettable for LPTIM_PIDRrs {
    const RESET_VALUE: u32 = 0x0012_0011;
}
