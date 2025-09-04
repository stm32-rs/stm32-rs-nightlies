///Register `SR2` reader
pub type R = crate::R<SR2rs>;
///Field `MSL` reader - Master/slave
pub type MSL_R = crate::BitReader;
///Field `BUSY` reader - Bus busy
pub type BUSY_R = crate::BitReader;
///Field `TRA` reader - Transmitter/receiver
pub type TRA_R = crate::BitReader;
///Field `GENCALL` reader - General call address (Slave mode)
pub type GENCALL_R = crate::BitReader;
///Field `SMBDEFAULT` reader - SMBus device default address (Slave mode)
pub type SMBDEFAULT_R = crate::BitReader;
///Field `SMBHOST` reader - SMBus host header (Slave mode)
pub type SMBHOST_R = crate::BitReader;
///Field `DUALF` reader - Dual flag (Slave mode)
pub type DUALF_R = crate::BitReader;
///Field `PEC` reader - acket error checking register
pub type PEC_R = crate::FieldReader;
impl R {
    ///Bit 0 - Master/slave
    #[inline(always)]
    pub fn msl(&self) -> MSL_R {
        MSL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Bus busy
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transmitter/receiver
    #[inline(always)]
    pub fn tra(&self) -> TRA_R {
        TRA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - General call address (Slave mode)
    #[inline(always)]
    pub fn gencall(&self) -> GENCALL_R {
        GENCALL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SMBus device default address (Slave mode)
    #[inline(always)]
    pub fn smbdefault(&self) -> SMBDEFAULT_R {
        SMBDEFAULT_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SMBus host header (Slave mode)
    #[inline(always)]
    pub fn smbhost(&self) -> SMBHOST_R {
        SMBHOST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Dual flag (Slave mode)
    #[inline(always)]
    pub fn dualf(&self) -> DUALF_R {
        DUALF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - acket error checking register
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR2")
            .field("pec", &self.pec())
            .field("dualf", &self.dualf())
            .field("smbhost", &self.smbhost())
            .field("smbdefault", &self.smbdefault())
            .field("gencall", &self.gencall())
            .field("tra", &self.tra())
            .field("busy", &self.busy())
            .field("msl", &self.msl())
            .finish()
    }
}
/**Status register 2

You can [`read`](crate::Reg::read) this register and get [`sr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F407.html#I2C1:SR2)*/
pub struct SR2rs;
impl crate::RegisterSpec for SR2rs {
    type Ux = u16;
}
///`read()` method returns [`sr2::R`](R) reader structure
impl crate::Readable for SR2rs {}
///`reset()` method sets SR2 to value 0
impl crate::Resettable for SR2rs {}
