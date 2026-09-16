///Register `OTG_HS_DAINT` reader
pub type R = crate::R<OTG_HS_DAINTrs>;
///Field `IEPINT` reader - IN endpoint interrupt bits
pub type IEPINT_R = crate::FieldReader<u16>;
///Field `OEPINT` reader - OUT endpoint interrupt bits
pub type OEPINT_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - IN endpoint interrupt bits
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - OUT endpoint interrupt bits
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_DAINT")
            .field("iepint", &self.iepint())
            .field("oepint", &self.oepint())
            .finish()
    }
}
/**OTG_HS device all endpoints interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_daint::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_HS_DEVICE:OTG_HS_DAINT)*/
pub struct OTG_HS_DAINTrs;
impl crate::RegisterSpec for OTG_HS_DAINTrs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_daint::R`](R) reader structure
impl crate::Readable for OTG_HS_DAINTrs {}
///`reset()` method sets OTG_HS_DAINT to value 0
impl crate::Resettable for OTG_HS_DAINTrs {}
