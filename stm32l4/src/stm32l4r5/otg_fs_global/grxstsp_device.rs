///Register `GRXSTSP_Device` reader
pub type R = crate::R<GRXSTSP_DEVICErs>;
///Field `EPNUM` reader - Endpoint number
pub type EPNUM_R = crate::FieldReader;
///Field `BCNT` reader - Byte count
pub type BCNT_R = crate::FieldReader<u16>;
///Field `DPID` reader - Data PID
pub type DPID_R = crate::FieldReader;
///Field `PKTSTS` reader - Packet status
pub type PKTSTS_R = crate::FieldReader;
///Field `FRMNUM` reader - Frame number
pub type FRMNUM_R = crate::FieldReader;
///Field `STSPHST` reader - ??
pub type STSPHST_R = crate::BitReader;
impl R {
    ///Bits 0:3 - Endpoint number
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:14 - Byte count
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    ///Bits 15:16 - Data PID
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bits 17:20 - Packet status
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    ///Bits 21:24 - Frame number
    #[inline(always)]
    pub fn frmnum(&self) -> FRMNUM_R {
        FRMNUM_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    ///Bit 27 - ??
    #[inline(always)]
    pub fn stsphst(&self) -> STSPHST_R {
        STSPHST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRXSTSP_Device")
            .field("frmnum", &self.frmnum())
            .field("pktsts", &self.pktsts())
            .field("dpid", &self.dpid())
            .field("bcnt", &self.bcnt())
            .field("epnum", &self.epnum())
            .field("stsphst", &self.stsphst())
            .finish()
    }
}
/**OTG status read and pop (device mode)

You can [`read`](crate::Reg::read) this register and get [`grxstsp_device::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#OTG_FS_GLOBAL:GRXSTSP_Device)*/
pub struct GRXSTSP_DEVICErs;
impl crate::RegisterSpec for GRXSTSP_DEVICErs {
    type Ux = u32;
}
///`read()` method returns [`grxstsp_device::R`](R) reader structure
impl crate::Readable for GRXSTSP_DEVICErs {}
///`reset()` method sets GRXSTSP_Device to value 0
impl crate::Resettable for GRXSTSP_DEVICErs {}
