#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `TXIS` reader - Transmit interrupt status The flag indicates that the UCPD_TXDR register is empty and new data write is required (as the amount of data sent has not reached the payload size defined in the TXPAYSZ bitfield). The flag is cleared with the data write into the UCPD_TXDR register."]
pub type TXIS_R = crate::BitReader;
#[doc = "Field `TXMSGDISC` reader - Message transmission discarded The flag indicates that a message transmission was dropped. The flag is cleared by setting the TXMSGDISCCF bit. Transmission of a message can be dropped if there is a concurrent receive in progress or at excessive noise on the line. After a Tx message is discarded, the flag is only raised when the CC line becomes idle."]
pub type TXMSGDISC_R = crate::BitReader;
#[doc = "Field `TXMSGSENT` reader - Message transmission completed The flag indicates the completion of packet transmission. It is cleared by setting the TXMSGSENTCF bit. In the event of a message transmission interrupted by a Hard Reset, the flag is not raised."]
pub type TXMSGSENT_R = crate::BitReader;
#[doc = "Field `TXMSGABT` reader - Transmit message abort The flag indicates that a Tx message is aborted due to a subsequent Hard Reset message send request taking priority during transmit. It is cleared by setting the TXMSGABTCF bit."]
pub type TXMSGABT_R = crate::BitReader;
#[doc = "Field `HRSTDISC` reader - Hard Reset discarded The flag indicates that the Hard Reset message is discarded. The flag is cleared by setting the HRSTDISCCF bit."]
pub type HRSTDISC_R = crate::BitReader;
#[doc = "Field `HRSTSENT` reader - Hard Reset message sent The flag indicates that the Hard Reset message is sent. The flag is cleared by setting the HRSTSENTCF bit."]
pub type HRSTSENT_R = crate::BitReader;
#[doc = "Field `TXUND` reader - Tx data underrun detection The flag indicates that the Tx data register (UCPD_TXDR) was not written in time for a transmit message to execute normally. It is cleared by setting the TXUNDCF bit."]
pub type TXUND_R = crate::BitReader;
#[doc = "Field `RXNE` reader - Receive data register not empty detection The flag indicates that the UCPD_RXDR register is not empty. It is automatically cleared upon reading UCPD_RXDR."]
pub type RXNE_R = crate::BitReader;
#[doc = "Field `RXORDDET` reader - Rx ordered set (4 K-codes) detection The flag indicates the detection of an ordered set. The relevant information is stored in the RXORDSET\\[2:0\\]
bitfield of the UCPD_RX_ORDSET register. It is cleared by setting the RXORDDETCF bit."]
pub type RXORDDET_R = crate::BitReader;
#[doc = "Field `RXHRSTDET` reader - Rx Hard Reset receipt detection The flag indicates the receipt of valid Hard Reset message. It is cleared by setting the RXHRSTDETCF bit."]
pub type RXHRSTDET_R = crate::BitReader;
#[doc = "Field `RXOVR` reader - Rx data overflow detection The flag indicates Rx data buffer overflow. It is cleared by setting the RXOVRCF bit. The buffer overflow can occur if the received data are not read fast enough."]
pub type RXOVR_R = crate::BitReader;
#[doc = "Field `RXMSGEND` reader - Rx message received The flag indicates whether a message (except Hard Reset message) has been received, regardless the CRC value. The flag is cleared by setting the RXMSGENDCF bit. The RXERR flag set when the RXMSGEND flag goes high indicates errors in the last-received message."]
pub type RXMSGEND_R = crate::BitReader;
#[doc = "Field `RXERR` reader - Receive message error The flag indicates errors of the last Rx message declared (via RXMSGEND), such as incorrect CRC or truncated message (a line becoming static before EOP is met). It is asserted whenever the RXMSGEND flag is set."]
pub type RXERR_R = crate::BitReader;
#[doc = "Field `TYPECEVT1` reader - Type-C voltage level event on CC1 line The flag indicates a change of the TYPEC_VSTATE_CC1\\[1:0\\]
bitfield value, which corresponds to a new Type-C event. It is cleared by setting the TYPECEVT2CF bit."]
pub type TYPECEVT1_R = crate::BitReader;
#[doc = "Field `TYPECEVT2` reader - Type-C voltage level event on CC2 line The flag indicates a change of the TYPEC_VSTATE_CC2\\[1:0\\]
bitfield value, which corresponds to a new Type-C event. It is cleared by setting the TYPECEVT2CF bit."]
pub type TYPECEVT2_R = crate::BitReader;
#[doc = "Field `TYPEC_VSTATE_CC1` reader - The status bitfield indicates the voltage level on the CC1 line in its steady state. The voltage variation on the CC1 line during USB PD messages due to the BMC PHY modulation does not impact the bitfield value."]
pub type TYPEC_VSTATE_CC1_R = crate::FieldReader;
#[doc = "Field `TYPEC_VSTATE_CC2` reader - CC2 line voltage level The status bitfield indicates the voltage level on the CC2 line in its steady state. The voltage variation on the CC2 line during USB PD messages due to the BMC PHY modulation does not impact the bitfield value."]
pub type TYPEC_VSTATE_CC2_R = crate::FieldReader;
#[doc = "Field `FRSEVT` reader - FRS detection event The flag is cleared by setting the FRSEVTCF bit."]
pub type FRSEVT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit interrupt status The flag indicates that the UCPD_TXDR register is empty and new data write is required (as the amount of data sent has not reached the payload size defined in the TXPAYSZ bitfield). The flag is cleared with the data write into the UCPD_TXDR register."]
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Message transmission discarded The flag indicates that a message transmission was dropped. The flag is cleared by setting the TXMSGDISCCF bit. Transmission of a message can be dropped if there is a concurrent receive in progress or at excessive noise on the line. After a Tx message is discarded, the flag is only raised when the CC line becomes idle."]
    #[inline(always)]
    pub fn txmsgdisc(&self) -> TXMSGDISC_R {
        TXMSGDISC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Message transmission completed The flag indicates the completion of packet transmission. It is cleared by setting the TXMSGSENTCF bit. In the event of a message transmission interrupted by a Hard Reset, the flag is not raised."]
    #[inline(always)]
    pub fn txmsgsent(&self) -> TXMSGSENT_R {
        TXMSGSENT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit message abort The flag indicates that a Tx message is aborted due to a subsequent Hard Reset message send request taking priority during transmit. It is cleared by setting the TXMSGABTCF bit."]
    #[inline(always)]
    pub fn txmsgabt(&self) -> TXMSGABT_R {
        TXMSGABT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hard Reset discarded The flag indicates that the Hard Reset message is discarded. The flag is cleared by setting the HRSTDISCCF bit."]
    #[inline(always)]
    pub fn hrstdisc(&self) -> HRSTDISC_R {
        HRSTDISC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hard Reset message sent The flag indicates that the Hard Reset message is sent. The flag is cleared by setting the HRSTSENTCF bit."]
    #[inline(always)]
    pub fn hrstsent(&self) -> HRSTSENT_R {
        HRSTSENT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Tx data underrun detection The flag indicates that the Tx data register (UCPD_TXDR) was not written in time for a transmit message to execute normally. It is cleared by setting the TXUNDCF bit."]
    #[inline(always)]
    pub fn txund(&self) -> TXUND_R {
        TXUND_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive data register not empty detection The flag indicates that the UCPD_RXDR register is not empty. It is automatically cleared upon reading UCPD_RXDR."]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rx ordered set (4 K-codes) detection The flag indicates the detection of an ordered set. The relevant information is stored in the RXORDSET\\[2:0\\]
bitfield of the UCPD_RX_ORDSET register. It is cleared by setting the RXORDDETCF bit."]
    #[inline(always)]
    pub fn rxorddet(&self) -> RXORDDET_R {
        RXORDDET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rx Hard Reset receipt detection The flag indicates the receipt of valid Hard Reset message. It is cleared by setting the RXHRSTDETCF bit."]
    #[inline(always)]
    pub fn rxhrstdet(&self) -> RXHRSTDET_R {
        RXHRSTDET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rx data overflow detection The flag indicates Rx data buffer overflow. It is cleared by setting the RXOVRCF bit. The buffer overflow can occur if the received data are not read fast enough."]
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rx message received The flag indicates whether a message (except Hard Reset message) has been received, regardless the CRC value. The flag is cleared by setting the RXMSGENDCF bit. The RXERR flag set when the RXMSGEND flag goes high indicates errors in the last-received message."]
    #[inline(always)]
    pub fn rxmsgend(&self) -> RXMSGEND_R {
        RXMSGEND_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive message error The flag indicates errors of the last Rx message declared (via RXMSGEND), such as incorrect CRC or truncated message (a line becoming static before EOP is met). It is asserted whenever the RXMSGEND flag is set."]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Type-C voltage level event on CC1 line The flag indicates a change of the TYPEC_VSTATE_CC1\\[1:0\\]
bitfield value, which corresponds to a new Type-C event. It is cleared by setting the TYPECEVT2CF bit."]
    #[inline(always)]
    pub fn typecevt1(&self) -> TYPECEVT1_R {
        TYPECEVT1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Type-C voltage level event on CC2 line The flag indicates a change of the TYPEC_VSTATE_CC2\\[1:0\\]
bitfield value, which corresponds to a new Type-C event. It is cleared by setting the TYPECEVT2CF bit."]
    #[inline(always)]
    pub fn typecevt2(&self) -> TYPECEVT2_R {
        TYPECEVT2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - The status bitfield indicates the voltage level on the CC1 line in its steady state. The voltage variation on the CC1 line during USB PD messages due to the BMC PHY modulation does not impact the bitfield value."]
    #[inline(always)]
    pub fn typec_vstate_cc1(&self) -> TYPEC_VSTATE_CC1_R {
        TYPEC_VSTATE_CC1_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - CC2 line voltage level The status bitfield indicates the voltage level on the CC2 line in its steady state. The voltage variation on the CC2 line during USB PD messages due to the BMC PHY modulation does not impact the bitfield value."]
    #[inline(always)]
    pub fn typec_vstate_cc2(&self) -> TYPEC_VSTATE_CC2_R {
        TYPEC_VSTATE_CC2_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - FRS detection event The flag is cleared by setting the FRSEVTCF bit."]
    #[inline(always)]
    pub fn frsevt(&self) -> FRSEVT_R {
        FRSEVT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "UCPD status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
