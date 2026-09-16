///Register `MACTSSR` reader
pub type R = crate::R<MACTSSRrs>;
///Field `TSSOVF` reader - Timestamp Seconds Overflow
pub type TSSOVF_R = crate::BitReader;
///Field `TSTARGT0` reader - Timestamp Target Time Reached
pub type TSTARGT0_R = crate::BitReader;
///Field `AUXTSTRIG` reader - Auxiliary Timestamp Trigger Snapshot
pub type AUXTSTRIG_R = crate::BitReader;
///Field `TSTRGTERR0` reader - Timestamp Target Time Error
pub type TSTRGTERR0_R = crate::BitReader;
///Field `TXTSSIS` reader - Tx Timestamp Status Interrupt Status
pub type TXTSSIS_R = crate::BitReader;
///Field `ATSSTN` reader - Auxiliary Timestamp Snapshot Trigger Identifier
pub type ATSSTN_R = crate::FieldReader;
///Field `ATSSTM` reader - Auxiliary Timestamp Snapshot Trigger Missed
pub type ATSSTM_R = crate::BitReader;
///Field `ATSNS` reader - Number of Auxiliary Timestamp Snapshots
pub type ATSNS_R = crate::FieldReader;
impl R {
    ///Bit 0 - Timestamp Seconds Overflow
    #[inline(always)]
    pub fn tssovf(&self) -> TSSOVF_R {
        TSSOVF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timestamp Target Time Reached
    #[inline(always)]
    pub fn tstargt0(&self) -> TSTARGT0_R {
        TSTARGT0_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Auxiliary Timestamp Trigger Snapshot
    #[inline(always)]
    pub fn auxtstrig(&self) -> AUXTSTRIG_R {
        AUXTSTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timestamp Target Time Error
    #[inline(always)]
    pub fn tstrgterr0(&self) -> TSTRGTERR0_R {
        TSTRGTERR0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 15 - Tx Timestamp Status Interrupt Status
    #[inline(always)]
    pub fn txtssis(&self) -> TXTSSIS_R {
        TXTSSIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - Auxiliary Timestamp Snapshot Trigger Identifier
    #[inline(always)]
    pub fn atsstn(&self) -> ATSSTN_R {
        ATSSTN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 24 - Auxiliary Timestamp Snapshot Trigger Missed
    #[inline(always)]
    pub fn atsstm(&self) -> ATSSTM_R {
        ATSSTM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:29 - Number of Auxiliary Timestamp Snapshots
    #[inline(always)]
    pub fn atsns(&self) -> ATSNS_R {
        ATSNS_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACTSSR")
            .field("tssovf", &self.tssovf())
            .field("tstargt0", &self.tstargt0())
            .field("auxtstrig", &self.auxtstrig())
            .field("tstrgterr0", &self.tstrgterr0())
            .field("txtssis", &self.txtssis())
            .field("atsstn", &self.atsstn())
            .field("atsstm", &self.atsstm())
            .field("atsns", &self.atsns())
            .finish()
    }
}
/**Timestamp status register

You can [`read`](crate::Reg::read) this register and get [`mactssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#Ethernet_MAC:MACTSSR)*/
pub struct MACTSSRrs;
impl crate::RegisterSpec for MACTSSRrs {
    type Ux = u32;
}
///`read()` method returns [`mactssr::R`](R) reader structure
impl crate::Readable for MACTSSRrs {}
///`reset()` method sets MACTSSR to value 0
impl crate::Resettable for MACTSSRrs {}
