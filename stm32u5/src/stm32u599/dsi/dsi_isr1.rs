#[doc = "Register `DSI_ISR1` reader"]
pub type R = crate::R<DSI_ISR1rs>;
#[doc = "Field `TOHSTX` reader - Timeout high-speed transmission This bit indicates that the high-speed transmission timeout counter reached the end and contention is detected."]
pub type TOHSTX_R = crate::BitReader;
#[doc = "Field `TOLPRX` reader - Timeout low-power reception This bit indicates that the low-power reception timeout counter reached the end and contention is detected."]
pub type TOLPRX_R = crate::BitReader;
#[doc = "Field `ECCSE` reader - ECC single-bit error This bit indicates that the ECC single error is detected and corrected in a received packet."]
pub type ECCSE_R = crate::BitReader;
#[doc = "Field `ECCME` reader - ECC multi-bit error This bit indicates that the ECC multiple error is detected in a received packet."]
pub type ECCME_R = crate::BitReader;
#[doc = "Field `CRCE` reader - CRC error This bit indicates that the CRC error is detected in the received packet payload."]
pub type CRCE_R = crate::BitReader;
#[doc = "Field `PSE` reader - Packet size error This bit indicates that the packet size error is detected during the packet reception."]
pub type PSE_R = crate::BitReader;
#[doc = "Field `EOTPE` reader - EoTp error This bit indicates that the EoTp packet is not received at the end of the incoming peripheral transmission."]
pub type EOTPE_R = crate::BitReader;
#[doc = "Field `LPWRE` reader - LTDC payload write error This bit indicates that during a DPI pixel line storage, the payload FIFO becomes full and the data stored is corrupted."]
pub type LPWRE_R = crate::BitReader;
#[doc = "Field `GCWRE` reader - Generic command write error This bit indicates that the system tried to write a command through the generic interface and the FIFO is full. Therefore, the command is not written."]
pub type GCWRE_R = crate::BitReader;
#[doc = "Field `GPWRE` reader - Generic payload write error This bit indicates that the system tried to write a payload data through the generic interface and the FIFO is full. Therefore, the payload is not written."]
pub type GPWRE_R = crate::BitReader;
#[doc = "Field `GPTXE` reader - Generic payload transmit error This bit indicates that during a generic interface packet build, the payload FIFO becomes empty and corrupt data is sent."]
pub type GPTXE_R = crate::BitReader;
#[doc = "Field `GPRDE` reader - Generic payload read error This bit indicates that during a DCS read data, the payload FIFO becomes empty and the data sent to the interface is corrupted."]
pub type GPRDE_R = crate::BitReader;
#[doc = "Field `GPRXE` reader - Generic payload receive error This bit indicates that during a generic interface packet read back, the payload FIFO becomes full and the received data is corrupted."]
pub type GPRXE_R = crate::BitReader;
#[doc = "Field `PBUE` reader - Payload buffer underflow error This bit indicates that underflow has occurred when reading payload to build DSI packet for video mode."]
pub type PBUE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timeout high-speed transmission This bit indicates that the high-speed transmission timeout counter reached the end and contention is detected."]
    #[inline(always)]
    pub fn tohstx(&self) -> TOHSTX_R {
        TOHSTX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timeout low-power reception This bit indicates that the low-power reception timeout counter reached the end and contention is detected."]
    #[inline(always)]
    pub fn tolprx(&self) -> TOLPRX_R {
        TOLPRX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ECC single-bit error This bit indicates that the ECC single error is detected and corrected in a received packet."]
    #[inline(always)]
    pub fn eccse(&self) -> ECCSE_R {
        ECCSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC multi-bit error This bit indicates that the ECC multiple error is detected in a received packet."]
    #[inline(always)]
    pub fn eccme(&self) -> ECCME_R {
        ECCME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC error This bit indicates that the CRC error is detected in the received packet payload."]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Packet size error This bit indicates that the packet size error is detected during the packet reception."]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EoTp error This bit indicates that the EoTp packet is not received at the end of the incoming peripheral transmission."]
    #[inline(always)]
    pub fn eotpe(&self) -> EOTPE_R {
        EOTPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LTDC payload write error This bit indicates that during a DPI pixel line storage, the payload FIFO becomes full and the data stored is corrupted."]
    #[inline(always)]
    pub fn lpwre(&self) -> LPWRE_R {
        LPWRE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Generic command write error This bit indicates that the system tried to write a command through the generic interface and the FIFO is full. Therefore, the command is not written."]
    #[inline(always)]
    pub fn gcwre(&self) -> GCWRE_R {
        GCWRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generic payload write error This bit indicates that the system tried to write a payload data through the generic interface and the FIFO is full. Therefore, the payload is not written."]
    #[inline(always)]
    pub fn gpwre(&self) -> GPWRE_R {
        GPWRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Generic payload transmit error This bit indicates that during a generic interface packet build, the payload FIFO becomes empty and corrupt data is sent."]
    #[inline(always)]
    pub fn gptxe(&self) -> GPTXE_R {
        GPTXE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Generic payload read error This bit indicates that during a DCS read data, the payload FIFO becomes empty and the data sent to the interface is corrupted."]
    #[inline(always)]
    pub fn gprde(&self) -> GPRDE_R {
        GPRDE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Generic payload receive error This bit indicates that during a generic interface packet read back, the payload FIFO becomes full and the received data is corrupted."]
    #[inline(always)]
    pub fn gprxe(&self) -> GPRXE_R {
        GPRXE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 19 - Payload buffer underflow error This bit indicates that underflow has occurred when reading payload to build DSI packet for video mode."]
    #[inline(always)]
    pub fn pbue(&self) -> PBUE_R {
        PBUE_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "DSI Host interrupt and status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_isr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_ISR1rs;
impl crate::RegisterSpec for DSI_ISR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_isr1::R`](R) reader structure"]
impl crate::Readable for DSI_ISR1rs {}
#[doc = "`reset()` method sets DSI_ISR1 to value 0"]
impl crate::Resettable for DSI_ISR1rs {
    const RESET_VALUE: u32 = 0;
}
