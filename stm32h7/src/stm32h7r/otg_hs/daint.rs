///Register `DAINT` reader
pub type R = crate::R<DAINTrs>;
///Field `IEPINT` reader - IN endpoint interrupt bits One bit per IN endpoint: Bit 0 for IN endpoint 0, bit 3 for endpoint 3.
pub type IEPINT_R = crate::FieldReader<u16>;
///Field `OEPINT` reader - OUT endpoint interrupt bits One bit per OUT endpoint: Bit 16 for OUT endpoint 0, bit 19 for OUT endpoint 3.
pub type OEPINT_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - IN endpoint interrupt bits One bit per IN endpoint: Bit 0 for IN endpoint 0, bit 3 for endpoint 3.
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - OUT endpoint interrupt bits One bit per OUT endpoint: Bit 16 for OUT endpoint 0, bit 19 for OUT endpoint 3.
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAINT")
            .field("iepint", &self.iepint())
            .field("oepint", &self.oepint())
            .finish()
    }
}
/**OTG device all endpoints interrupt register

You can [`read`](crate::Reg::read) this register and get [`daint::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#OTG_HS:DAINT)*/
pub struct DAINTrs;
impl crate::RegisterSpec for DAINTrs {
    type Ux = u32;
}
///`read()` method returns [`daint::R`](R) reader structure
impl crate::Readable for DAINTrs {}
///`reset()` method sets DAINT to value 0
impl crate::Resettable for DAINTrs {}
