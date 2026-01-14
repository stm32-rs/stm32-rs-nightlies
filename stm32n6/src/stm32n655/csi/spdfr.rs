///Register `SPDFR` reader
pub type R = crate::R<SPDFRrs>;
///Field `DATAFIELD` reader - Data field
pub type DATAFIELD_R = crate::FieldReader<u16>;
///Field `DATATYPE` reader - Data type class
pub type DATATYPE_R = crate::FieldReader;
///Field `VCHANNEL` reader - Virtual channel
pub type VCHANNEL_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - Data field
    #[inline(always)]
    pub fn datafield(&self) -> DATAFIELD_R {
        DATAFIELD_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:21 - Data type class
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 22:23 - Virtual channel
    #[inline(always)]
    pub fn vchannel(&self) -> VCHANNEL_R {
        VCHANNEL_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPDFR")
            .field("datafield", &self.datafield())
            .field("datatype", &self.datatype())
            .field("vchannel", &self.vchannel())
            .finish()
    }
}
/**CSI-2 Host short packet data field register

You can [`read`](crate::Reg::read) this register and get [`spdfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:SPDFR)*/
pub struct SPDFRrs;
impl crate::RegisterSpec for SPDFRrs {
    type Ux = u32;
}
///`read()` method returns [`spdfr::R`](R) reader structure
impl crate::Readable for SPDFRrs {}
///`reset()` method sets SPDFR to value 0
impl crate::Resettable for SPDFRrs {}
