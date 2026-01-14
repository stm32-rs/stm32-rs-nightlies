///Register `RDTR` reader
pub type R = crate::R<RDTRrs>;
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
        f.debug_struct("RDTR")
            .field("time", &self.time())
            .field("fmi", &self.fmi())
            .field("dlc", &self.dlc())
            .finish()
    }
}
/**CAN_RDT0R

You can [`read`](crate::Reg::read) this register and get [`rdtr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RDTRrs;
impl crate::RegisterSpec for RDTRrs {
    type Ux = u32;
}
///`read()` method returns [`rdtr::R`](R) reader structure
impl crate::Readable for RDTRrs {}
///`reset()` method sets RDTR to value 0
impl crate::Resettable for RDTRrs {}
