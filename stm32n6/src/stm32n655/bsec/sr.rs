///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `HVALID` reader - Hardware key valid
pub type HVALID_R = crate::BitReader;
///Field `DBGREQ` reader - debug request
pub type DBGREQ_R = crate::BitReader;
///Field `NVSTATE` reader - Non-volatile state
pub type NVSTATE_R = crate::FieldReader;
impl R {
    ///Bit 1 - Hardware key valid
    #[inline(always)]
    pub fn hvalid(&self) -> HVALID_R {
        HVALID_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - debug request
    #[inline(always)]
    pub fn dbgreq(&self) -> DBGREQ_R {
        DBGREQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 26:31 - Non-volatile state
    #[inline(always)]
    pub fn nvstate(&self) -> NVSTATE_R {
        NVSTATE_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("hvalid", &self.hvalid())
            .field("dbgreq", &self.dbgreq())
            .field("nvstate", &self.nvstate())
            .finish()
    }
}
/**BSEC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#BSEC:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
