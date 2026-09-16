///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `MKVALID` reader - Master key valid
pub type MKVALID_R = crate::BitReader;
///Field `FMKVALID` reader - Fast master key valid
pub type FMKVALID_R = crate::BitReader;
///Field `ENCDIS` reader - encryption disabled
pub type ENCDIS_R = crate::BitReader;
impl R {
    ///Bit 0 - Master key valid
    #[inline(always)]
    pub fn mkvalid(&self) -> MKVALID_R {
        MKVALID_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Fast master key valid
    #[inline(always)]
    pub fn fmkvalid(&self) -> FMKVALID_R {
        FMKVALID_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - encryption disabled
    #[inline(always)]
    pub fn encdis(&self) -> ENCDIS_R {
        ENCDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("mkvalid", &self.mkvalid())
            .field("fmkvalid", &self.fmkvalid())
            .field("encdis", &self.encdis())
            .finish()
    }
}
/**MCE status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MCE1:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
