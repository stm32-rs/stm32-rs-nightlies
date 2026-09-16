///Register `CCVR` reader
pub type R = crate::R<CCVRrs>;
///Field `NCV1` reader - NMOS compensation value of the I/Os supplied by VsubDD/sub This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the SYSCFG_CCCSR is reset.
pub type NCV1_R = crate::FieldReader;
///Field `PCV1` reader - PMOS compensation value of the I/Os supplied by VsubDD/sub This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the SYSCFG_CCCSR is reset.
pub type PCV1_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - NMOS compensation value of the I/Os supplied by VsubDD/sub This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the SYSCFG_CCCSR is reset.
    #[inline(always)]
    pub fn ncv1(&self) -> NCV1_R {
        NCV1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - PMOS compensation value of the I/Os supplied by VsubDD/sub This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the SYSCFG_CCCSR is reset.
    #[inline(always)]
    pub fn pcv1(&self) -> PCV1_R {
        PCV1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCVR")
            .field("ncv1", &self.ncv1())
            .field("pcv1", &self.pcv1())
            .finish()
    }
}
/**SYSCFG compensation cell value register

You can [`read`](crate::Reg::read) this register and get [`ccvr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#SYSCFG:CCVR)*/
pub struct CCVRrs;
impl crate::RegisterSpec for CCVRrs {
    type Ux = u32;
}
///`read()` method returns [`ccvr::R`](R) reader structure
impl crate::Readable for CCVRrs {}
///`reset()` method sets CCVR to value 0
impl crate::Resettable for CCVRrs {}
