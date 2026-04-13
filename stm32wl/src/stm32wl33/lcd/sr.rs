///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `ENS` reader - LCD enabled status
pub type ENS_R = crate::BitReader;
///Field `SOF` reader - Start of frame flag
pub type SOF_R = crate::BitReader;
///Field `UDR` reader - Update display request
pub type UDR_R = crate::BitReader;
///Field `UDD` reader - Update Display Done
pub type UDD_R = crate::BitReader;
///Field `RDY` reader - Ready flag
pub type RDY_R = crate::BitReader;
///Field `FCRSF` reader - LCD Frame Control Register Synchronization flag
pub type FCRSF_R = crate::BitReader;
impl R {
    ///Bit 0 - LCD enabled status
    #[inline(always)]
    pub fn ens(&self) -> ENS_R {
        ENS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Start of frame flag
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Update display request
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Update Display Done
    #[inline(always)]
    pub fn udd(&self) -> UDD_R {
        UDD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Ready flag
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LCD Frame Control Register Synchronization flag
    #[inline(always)]
    pub fn fcrsf(&self) -> FCRSF_R {
        FCRSF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("ens", &self.ens())
            .field("sof", &self.sof())
            .field("udr", &self.udr())
            .field("udd", &self.udd())
            .field("rdy", &self.rdy())
            .field("fcrsf", &self.fcrsf())
            .finish()
    }
}
/**LCD_SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCD:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0x20
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x20;
}
