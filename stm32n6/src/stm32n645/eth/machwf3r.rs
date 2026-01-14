///Register `MACHWF3R` reader
pub type R = crate::R<MACHWF3Rrs>;
///Field `NRVF` reader - Number of Extended VLAN Tag Filters Enabled
pub type NRVF_R = crate::FieldReader;
///Field `CBTISEL` reader - Queue/Channel based VLAN tag insertion on Tx enable
pub type CBTISEL_R = crate::BitReader;
///Field `DVLAN` reader - Double VLAN processing enable
pub type DVLAN_R = crate::BitReader;
///Field `PDUPSEL` reader - Broadcast/Multicast Packet Duplication
pub type PDUPSEL_R = crate::BitReader;
///Field `FRPSEL` reader - Flexible Receive Parser Selected
pub type FRPSEL_R = crate::BitReader;
///Field `FRPBS` reader - Flexible Receive Parser Buffer size
pub type FRPBS_R = crate::FieldReader;
///Field `FRPES` reader - Flexible Receive Parser Table Entries size
pub type FRPES_R = crate::FieldReader;
///Field `ESTSEL` reader - Enhancements to Scheduled Traffic Enable
pub type ESTSEL_R = crate::BitReader;
///Field `ESTDEP` reader - Depth of the Gate Control List
pub type ESTDEP_R = crate::FieldReader;
///Field `ESTWID` reader - Width of the Time Interval field in the Gate Control List
pub type ESTWID_R = crate::FieldReader;
///Field `FPESEL` reader - Frame Preemption Enable
pub type FPESEL_R = crate::BitReader;
///Field `TBSSEL` reader - Time-based scheduling Enable
pub type TBSSEL_R = crate::BitReader;
///Field `ASP` reader - Automotive Safety Package
pub type ASP_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - Number of Extended VLAN Tag Filters Enabled
    #[inline(always)]
    pub fn nrvf(&self) -> NRVF_R {
        NRVF_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - Queue/Channel based VLAN tag insertion on Tx enable
    #[inline(always)]
    pub fn cbtisel(&self) -> CBTISEL_R {
        CBTISEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Double VLAN processing enable
    #[inline(always)]
    pub fn dvlan(&self) -> DVLAN_R {
        DVLAN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - Broadcast/Multicast Packet Duplication
    #[inline(always)]
    pub fn pdupsel(&self) -> PDUPSEL_R {
        PDUPSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Flexible Receive Parser Selected
    #[inline(always)]
    pub fn frpsel(&self) -> FRPSEL_R {
        FRPSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:12 - Flexible Receive Parser Buffer size
    #[inline(always)]
    pub fn frpbs(&self) -> FRPBS_R {
        FRPBS_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bits 13:14 - Flexible Receive Parser Table Entries size
    #[inline(always)]
    pub fn frpes(&self) -> FRPES_R {
        FRPES_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 16 - Enhancements to Scheduled Traffic Enable
    #[inline(always)]
    pub fn estsel(&self) -> ESTSEL_R {
        ESTSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - Depth of the Gate Control List
    #[inline(always)]
    pub fn estdep(&self) -> ESTDEP_R {
        ESTDEP_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bits 20:21 - Width of the Time Interval field in the Gate Control List
    #[inline(always)]
    pub fn estwid(&self) -> ESTWID_R {
        ESTWID_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 26 - Frame Preemption Enable
    #[inline(always)]
    pub fn fpesel(&self) -> FPESEL_R {
        FPESEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Time-based scheduling Enable
    #[inline(always)]
    pub fn tbssel(&self) -> TBSSEL_R {
        TBSSEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:29 - Automotive Safety Package
    #[inline(always)]
    pub fn asp(&self) -> ASP_R {
        ASP_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACHWF3R")
            .field("nrvf", &self.nrvf())
            .field("cbtisel", &self.cbtisel())
            .field("dvlan", &self.dvlan())
            .field("pdupsel", &self.pdupsel())
            .field("frpsel", &self.frpsel())
            .field("frpbs", &self.frpbs())
            .field("frpes", &self.frpes())
            .field("estsel", &self.estsel())
            .field("estdep", &self.estdep())
            .field("estwid", &self.estwid())
            .field("fpesel", &self.fpesel())
            .field("tbssel", &self.tbssel())
            .field("asp", &self.asp())
            .finish()
    }
}
/**HW feature 3 register

You can [`read`](crate::Reg::read) this register and get [`machwf3r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MACHWF3R)*/
pub struct MACHWF3Rrs;
impl crate::RegisterSpec for MACHWF3Rrs {
    type Ux = u32;
}
///`read()` method returns [`machwf3r::R`](R) reader structure
impl crate::Readable for MACHWF3Rrs {}
///`reset()` method sets MACHWF3R to value 0x0c33_0031
impl crate::Resettable for MACHWF3Rrs {
    const RESET_VALUE: u32 = 0x0c33_0031;
}
