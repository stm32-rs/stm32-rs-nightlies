///Register `TURNA` reader
pub type R = crate::R<TURNArs>;
///Field `NAV` reader - Numerator Actual Value
pub type NAV_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:17 - Numerator Actual Value
    #[inline(always)]
    pub fn nav(&self) -> NAV_R {
        NAV_R::new(self.bits & 0x0003_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TURNA").field("nav", &self.nav()).finish()
    }
}
/**FDCAN TUR Numerator Actual Register

You can [`read`](crate::Reg::read) this register and get [`turna::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#FDCAN1:TURNA)*/
pub struct TURNArs;
impl crate::RegisterSpec for TURNArs {
    type Ux = u32;
}
///`read()` method returns [`turna::R`](R) reader structure
impl crate::Readable for TURNArs {}
///`reset()` method sets TURNA to value 0
impl crate::Resettable for TURNArs {}
