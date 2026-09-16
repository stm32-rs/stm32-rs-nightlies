///Register `PIDR` reader
pub type R = crate::R<PIDRrs>;
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
        f.debug_struct("PIDR").field("p_id", &self.p_id()).finish()
    }
}
/**LPTIM peripheral type identification register

You can [`read`](crate::Reg::read) this register and get [`pidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LPTIM1:PIDR)*/
pub struct PIDRrs;
impl crate::RegisterSpec for PIDRrs {
    type Ux = u32;
}
///`read()` method returns [`pidr::R`](R) reader structure
impl crate::Readable for PIDRrs {}
///`reset()` method sets PIDR to value 0x0012_0011
impl crate::Resettable for PIDRrs {
    const RESET_VALUE: u32 = 0x0012_0011;
}
