///Register `ERR2` reader
pub type R = crate::R<ERR2rs>;
///Field `SPKTDTERR` reader - Data type having a short packet error
pub type SPKTDTERR_R = crate::FieldReader;
///Field `SPKTVCERR` reader - Virtual channel having a short packet error
pub type SPKTVCERR_R = crate::FieldReader;
///Field `WDVCERR` reader - Virtual channel having a watchdog error
pub type WDVCERR_R = crate::FieldReader;
///Field `SYNCVCERR` reader - Virtual channel having synchronization error
pub type SYNCVCERR_R = crate::FieldReader;
impl R {
    ///Bits 0:5 - Data type having a short packet error
    #[inline(always)]
    pub fn spktdterr(&self) -> SPKTDTERR_R {
        SPKTDTERR_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7 - Virtual channel having a short packet error
    #[inline(always)]
    pub fn spktvcerr(&self) -> SPKTVCERR_R {
        SPKTVCERR_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 16:17 - Virtual channel having a watchdog error
    #[inline(always)]
    pub fn wdvcerr(&self) -> WDVCERR_R {
        WDVCERR_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Virtual channel having synchronization error
    #[inline(always)]
    pub fn syncvcerr(&self) -> SYNCVCERR_R {
        SYNCVCERR_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR2")
            .field("spktdterr", &self.spktdterr())
            .field("spktvcerr", &self.spktvcerr())
            .field("wdvcerr", &self.wdvcerr())
            .field("syncvcerr", &self.syncvcerr())
            .finish()
    }
}
/**CSI-2 Host error register 2

You can [`read`](crate::Reg::read) this register and get [`err2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#CSI:ERR2)*/
pub struct ERR2rs;
impl crate::RegisterSpec for ERR2rs {
    type Ux = u32;
}
///`read()` method returns [`err2::R`](R) reader structure
impl crate::Readable for ERR2rs {}
///`reset()` method sets ERR2 to value 0
impl crate::Resettable for ERR2rs {}
