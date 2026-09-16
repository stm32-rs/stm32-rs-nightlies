///Register `CCVR` reader
pub type R = crate::R<CCVRrs>;
///Field `NCV` reader - NMOS compensation value
pub type NCV_R = crate::FieldReader;
///Field `PCV` reader - PMOS compensation value
pub type PCV_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - NMOS compensation value
    #[inline(always)]
    pub fn ncv(&self) -> NCV_R {
        NCV_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - PMOS compensation value
    #[inline(always)]
    pub fn pcv(&self) -> PCV_R {
        PCV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCVR")
            .field("ncv", &self.ncv())
            .field("pcv", &self.pcv())
            .finish()
    }
}
/**SYSCFG compensation cell value register

You can [`read`](crate::Reg::read) this register and get [`ccvr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#SYSCFG:CCVR)*/
pub struct CCVRrs;
impl crate::RegisterSpec for CCVRrs {
    type Ux = u32;
}
///`read()` method returns [`ccvr::R`](R) reader structure
impl crate::Readable for CCVRrs {}
///`reset()` method sets CCVR to value 0
impl crate::Resettable for CCVRrs {}
