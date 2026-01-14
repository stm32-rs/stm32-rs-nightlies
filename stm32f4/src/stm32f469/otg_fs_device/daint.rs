///Register `DAINT` reader
pub type R = crate::R<DAINTrs>;
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
        f.debug_struct("DAINT")
            .field("iepint", &self.iepint())
            .field("oepint", &self.oepint())
            .finish()
    }
}
/**OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)

You can [`read`](crate::Reg::read) this register and get [`daint::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#OTG_FS_DEVICE:DAINT)*/
pub struct DAINTrs;
impl crate::RegisterSpec for DAINTrs {
    type Ux = u32;
}
///`read()` method returns [`daint::R`](R) reader structure
impl crate::Readable for DAINTrs {}
///`reset()` method sets DAINT to value 0
impl crate::Resettable for DAINTrs {}
