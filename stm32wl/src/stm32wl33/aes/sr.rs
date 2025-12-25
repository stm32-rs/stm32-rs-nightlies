///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `CCF` reader - CCF: Computation complete flag
pub type CCF_R = crate::BitReader;
///Field `RDERR` reader - RDERR: Read error flag
pub type RDERR_R = crate::BitReader;
///Field `WRERR` reader - WRERR: Write error flag
pub type WRERR_R = crate::BitReader;
///Field `BUSY` reader - BUSY: Busy flag
pub type BUSY_R = crate::BitReader;
impl R {
    ///Bit 0 - CCF: Computation complete flag
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RDERR: Read error flag
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WRERR: Write error flag
    #[inline(always)]
    pub fn wrerr(&self) -> WRERR_R {
        WRERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BUSY: Busy flag
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("ccf", &self.ccf())
            .field("rderr", &self.rderr())
            .field("wrerr", &self.wrerr())
            .field("busy", &self.busy())
            .finish()
    }
}
/**AES_SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#AES:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
