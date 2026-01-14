///Register `SER` reader
pub type R = crate::R<SERrs>;
///Field `CODERR` reader - Protocol error code/type
pub type CODERR_R = crate::FieldReader;
///Field `PERR` reader - Protocol error
pub type PERR_R = crate::BitReader;
///Field `STALL` reader - SCL stall error (when the I3C acts as target)
pub type STALL_R = crate::BitReader;
///Field `DOVR` reader - RX-FIFO overrun or TX-FIFO underrun
pub type DOVR_R = crate::BitReader;
///Field `COVR` reader - C-FIFO underrun or S-FIFO overrun (when the I3C acts as controller)
pub type COVR_R = crate::BitReader;
///Field `ANACK` reader - Address not acknowledged (when the I3C is configured as controller)
pub type ANACK_R = crate::BitReader;
///Field `DNACK` reader - Data not acknowledged (when the I3C acts as controller)
pub type DNACK_R = crate::BitReader;
///Field `DERR` reader - Data error (when the I3C acts as controller)
pub type DERR_R = crate::BitReader;
impl R {
    ///Bits 0:3 - Protocol error code/type
    #[inline(always)]
    pub fn coderr(&self) -> CODERR_R {
        CODERR_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Protocol error
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SCL stall error (when the I3C acts as target)
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RX-FIFO overrun or TX-FIFO underrun
    #[inline(always)]
    pub fn dovr(&self) -> DOVR_R {
        DOVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - C-FIFO underrun or S-FIFO overrun (when the I3C acts as controller)
    #[inline(always)]
    pub fn covr(&self) -> COVR_R {
        COVR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Address not acknowledged (when the I3C is configured as controller)
    #[inline(always)]
    pub fn anack(&self) -> ANACK_R {
        ANACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Data not acknowledged (when the I3C acts as controller)
    #[inline(always)]
    pub fn dnack(&self) -> DNACK_R {
        DNACK_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Data error (when the I3C acts as controller)
    #[inline(always)]
    pub fn derr(&self) -> DERR_R {
        DERR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SER")
            .field("coderr", &self.coderr())
            .field("perr", &self.perr())
            .field("stall", &self.stall())
            .field("dovr", &self.dovr())
            .field("covr", &self.covr())
            .field("anack", &self.anack())
            .field("dnack", &self.dnack())
            .field("derr", &self.derr())
            .finish()
    }
}
/**I3C status error register

You can [`read`](crate::Reg::read) this register and get [`ser::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#I3C1:SER)*/
pub struct SERrs;
impl crate::RegisterSpec for SERrs {
    type Ux = u32;
}
///`read()` method returns [`ser::R`](R) reader structure
impl crate::Readable for SERrs {}
///`reset()` method sets SER to value 0
impl crate::Resettable for SERrs {}
