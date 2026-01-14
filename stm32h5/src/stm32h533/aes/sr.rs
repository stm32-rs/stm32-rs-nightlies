///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `RDERRF` reader - Read error flag
pub type RDERRF_R = crate::BitReader;
///Field `WRERRF` reader - Write error flag
pub type WRERRF_R = crate::BitReader;
///Field `BUSY` reader - Busy
pub type BUSY_R = crate::BitReader;
///Field `KEYVALID` reader - Key valid flag
pub type KEYVALID_R = crate::BitReader;
impl R {
    ///Bit 1 - Read error flag
    #[inline(always)]
    pub fn rderrf(&self) -> RDERRF_R {
        RDERRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Write error flag
    #[inline(always)]
    pub fn wrerrf(&self) -> WRERRF_R {
        WRERRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Busy
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - Key valid flag
    #[inline(always)]
    pub fn keyvalid(&self) -> KEYVALID_R {
        KEYVALID_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("rderrf", &self.rderrf())
            .field("wrerrf", &self.wrerrf())
            .field("busy", &self.busy())
            .field("keyvalid", &self.keyvalid())
            .finish()
    }
}
/**AES status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#AES:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
