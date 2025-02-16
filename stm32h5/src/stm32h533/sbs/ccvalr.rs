///Register `CCVALR` reader
pub type R = crate::R<CCVALRrs>;
///Field `ANSRC1` reader - compensation value for the NMOS transistor
pub type ANSRC1_R = crate::FieldReader;
///Field `APSRC1` reader - compensation value for the PMOS transistor
pub type APSRC1_R = crate::FieldReader;
///Field `ANSRC2` reader - Compensation value for the NMOS transistor
pub type ANSRC2_R = crate::FieldReader;
///Field `APSRC2` reader - compensation value for the PMOS transistor
pub type APSRC2_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - compensation value for the NMOS transistor
    #[inline(always)]
    pub fn ansrc1(&self) -> ANSRC1_R {
        ANSRC1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - compensation value for the PMOS transistor
    #[inline(always)]
    pub fn apsrc1(&self) -> APSRC1_R {
        APSRC1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Compensation value for the NMOS transistor
    #[inline(always)]
    pub fn ansrc2(&self) -> ANSRC2_R {
        ANSRC2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - compensation value for the PMOS transistor
    #[inline(always)]
    pub fn apsrc2(&self) -> APSRC2_R {
        APSRC2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCVALR")
            .field("ansrc1", &self.ansrc1())
            .field("apsrc1", &self.apsrc1())
            .field("ansrc2", &self.ansrc2())
            .field("apsrc2", &self.apsrc2())
            .finish()
    }
}
/**SBS compensation cell for I/Os value register

You can [`read`](crate::Reg::read) this register and get [`ccvalr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#SBS:CCVALR)*/
pub struct CCVALRrs;
impl crate::RegisterSpec for CCVALRrs {
    type Ux = u32;
}
///`read()` method returns [`ccvalr::R`](R) reader structure
impl crate::Readable for CCVALRrs {}
///`reset()` method sets CCVALR to value 0x88
impl crate::Resettable for CCVALRrs {
    const RESET_VALUE: u32 = 0x88;
}
