///Register `GRXSTSR_DEVICE` reader
pub type R = crate::R<GRXSTSR_DEVICErs>;
///Field `EPNUM` reader - Endpoint number Indicates the endpoint number to which the current received packet belongs.
pub type EPNUM_R = crate::FieldReader;
///Field `BCNT` reader - Byte count Indicates the byte count of the received data packet.
pub type BCNT_R = crate::FieldReader<u16>;
///Field `DPID` reader - Data PID Indicates the data PID of the received OUT data packet
pub type DPID_R = crate::FieldReader;
///Field `PKTSTS` reader - Packet status Indicates the status of the received packet Others: Reserved
pub type PKTSTS_R = crate::FieldReader;
///Field `FRMNUM` reader - Frame number This is the least significant 4 bits of the frame number in which the packet is received on the USB. This field is supported only when isochronous OUT endpoints are supported.
pub type FRMNUM_R = crate::FieldReader;
///Field `STSPHST` reader - Status phase start Indicates the start of the status phase for a control write transfer. This bit is set along with the OUT transfer completed PKTSTS pattern.
pub type STSPHST_R = crate::BitReader;
impl R {
    ///Bits 0:3 - Endpoint number Indicates the endpoint number to which the current received packet belongs.
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:14 - Byte count Indicates the byte count of the received data packet.
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    ///Bits 15:16 - Data PID Indicates the data PID of the received OUT data packet
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bits 17:20 - Packet status Indicates the status of the received packet Others: Reserved
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    ///Bits 21:24 - Frame number This is the least significant 4 bits of the frame number in which the packet is received on the USB. This field is supported only when isochronous OUT endpoints are supported.
    #[inline(always)]
    pub fn frmnum(&self) -> FRMNUM_R {
        FRMNUM_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    ///Bit 27 - Status phase start Indicates the start of the status phase for a control write transfer. This bit is set along with the OUT transfer completed PKTSTS pattern.
    #[inline(always)]
    pub fn stsphst(&self) -> STSPHST_R {
        STSPHST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRXSTSR_DEVICE")
            .field("epnum", &self.epnum())
            .field("bcnt", &self.bcnt())
            .field("dpid", &self.dpid())
            .field("pktsts", &self.pktsts())
            .field("frmnum", &self.frmnum())
            .field("stsphst", &self.stsphst())
            .finish()
    }
}
/**OTG receive status debug read register

You can [`read`](crate::Reg::read) this register and get [`grxstsr_device::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#OTG_HS:GRXSTSR_DEVICE)*/
pub struct GRXSTSR_DEVICErs;
impl crate::RegisterSpec for GRXSTSR_DEVICErs {
    type Ux = u32;
}
///`read()` method returns [`grxstsr_device::R`](R) reader structure
impl crate::Readable for GRXSTSR_DEVICErs {}
///`reset()` method sets GRXSTSR_DEVICE to value 0
impl crate::Resettable for GRXSTSR_DEVICErs {}
