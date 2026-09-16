///Register `RDT1R` reader
pub type R = crate::R<RDT1Rrs>;
///Field `DLC` reader - DLC
pub type DLC_R = crate::FieldReader;
///Field `FMI` reader - FMI
pub type FMI_R = crate::FieldReader;
///Field `TIME` reader - TIME
pub type TIME_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:3 - DLC
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:15 - FMI
    #[inline(always)]
    pub fn fmi(&self) -> FMI_R {
        FMI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:31 - TIME
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDT1R")
            .field("time", &self.time())
            .field("fmi", &self.fmi())
            .field("dlc", &self.dlc())
            .finish()
    }
}
/**mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`rdt1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:RDT1R)*/
pub struct RDT1Rrs;
impl crate::RegisterSpec for RDT1Rrs {
    type Ux = u32;
}
///`read()` method returns [`rdt1r::R`](R) reader structure
impl crate::Readable for RDT1Rrs {}
///`reset()` method sets RDT1R to value 0
impl crate::Resettable for RDT1Rrs {}
