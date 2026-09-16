///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `TXIS` reader - Transmit interrupt status
pub type TXIS_R = crate::BitReader;
///Field `TXMSGDISC` reader - Message transmission discarded
pub type TXMSGDISC_R = crate::BitReader;
///Field `TXMSGSENT` reader - Message transmission completed
pub type TXMSGSENT_R = crate::BitReader;
///Field `TXMSGABT` reader - Transmit message abort
pub type TXMSGABT_R = crate::BitReader;
///Field `HRSTDISC` reader - Hard Reset discarded
pub type HRSTDISC_R = crate::BitReader;
///Field `HRSTSENT` reader - Hard Reset message sent
pub type HRSTSENT_R = crate::BitReader;
///Field `TXUND` reader - Tx data underrun detection
pub type TXUND_R = crate::BitReader;
///Field `RXNE` reader - Receive data register not empty detection
pub type RXNE_R = crate::BitReader;
///Field `RXORDDET` reader - Rx ordered set (4 K-codes) detection
pub type RXORDDET_R = crate::BitReader;
///Field `RXHRSTDET` reader - Rx Hard Reset receipt detection
pub type RXHRSTDET_R = crate::BitReader;
///Field `RXOVR` reader - Rx data overflow detection
pub type RXOVR_R = crate::BitReader;
///Field `RXMSGEND` reader - Rx message received
pub type RXMSGEND_R = crate::BitReader;
///Field `RXERR` reader - Receive message error
pub type RXERR_R = crate::BitReader;
///Field `TYPECEVT1` reader - Type-C voltage level event on CC1 line
pub type TYPECEVT1_R = crate::BitReader;
///Field `TYPECEVT2` reader - Type-C voltage level event on CC2 line
pub type TYPECEVT2_R = crate::BitReader;
///Field `TYPEC_VSTATE_CC1` reader - The status bitfield indicates the voltage level on the CC1 line in its steady state.
pub type TYPEC_VSTATE_CC1_R = crate::FieldReader;
///Field `TYPEC_VSTATE_CC2` reader - CC2 line voltage level
pub type TYPEC_VSTATE_CC2_R = crate::FieldReader;
///Field `FRSEVT` reader - FRS detection event
pub type FRSEVT_R = crate::BitReader;
impl R {
    ///Bit 0 - Transmit interrupt status
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Message transmission discarded
    #[inline(always)]
    pub fn txmsgdisc(&self) -> TXMSGDISC_R {
        TXMSGDISC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Message transmission completed
    #[inline(always)]
    pub fn txmsgsent(&self) -> TXMSGSENT_R {
        TXMSGSENT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmit message abort
    #[inline(always)]
    pub fn txmsgabt(&self) -> TXMSGABT_R {
        TXMSGABT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Hard Reset discarded
    #[inline(always)]
    pub fn hrstdisc(&self) -> HRSTDISC_R {
        HRSTDISC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Hard Reset message sent
    #[inline(always)]
    pub fn hrstsent(&self) -> HRSTSENT_R {
        HRSTSENT_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Tx data underrun detection
    #[inline(always)]
    pub fn txund(&self) -> TXUND_R {
        TXUND_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Receive data register not empty detection
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Rx ordered set (4 K-codes) detection
    #[inline(always)]
    pub fn rxorddet(&self) -> RXORDDET_R {
        RXORDDET_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Rx Hard Reset receipt detection
    #[inline(always)]
    pub fn rxhrstdet(&self) -> RXHRSTDET_R {
        RXHRSTDET_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Rx data overflow detection
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Rx message received
    #[inline(always)]
    pub fn rxmsgend(&self) -> RXMSGEND_R {
        RXMSGEND_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Receive message error
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Type-C voltage level event on CC1 line
    #[inline(always)]
    pub fn typecevt1(&self) -> TYPECEVT1_R {
        TYPECEVT1_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Type-C voltage level event on CC2 line
    #[inline(always)]
    pub fn typecevt2(&self) -> TYPECEVT2_R {
        TYPECEVT2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - The status bitfield indicates the voltage level on the CC1 line in its steady state.
    #[inline(always)]
    pub fn typec_vstate_cc1(&self) -> TYPEC_VSTATE_CC1_R {
        TYPEC_VSTATE_CC1_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - CC2 line voltage level
    #[inline(always)]
    pub fn typec_vstate_cc2(&self) -> TYPEC_VSTATE_CC2_R {
        TYPEC_VSTATE_CC2_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 20 - FRS detection event
    #[inline(always)]
    pub fn frsevt(&self) -> FRSEVT_R {
        FRSEVT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("txis", &self.txis())
            .field("txmsgdisc", &self.txmsgdisc())
            .field("txmsgsent", &self.txmsgsent())
            .field("txmsgabt", &self.txmsgabt())
            .field("hrstdisc", &self.hrstdisc())
            .field("hrstsent", &self.hrstsent())
            .field("txund", &self.txund())
            .field("rxne", &self.rxne())
            .field("rxorddet", &self.rxorddet())
            .field("rxhrstdet", &self.rxhrstdet())
            .field("rxovr", &self.rxovr())
            .field("rxmsgend", &self.rxmsgend())
            .field("rxerr", &self.rxerr())
            .field("typecevt1", &self.typecevt1())
            .field("typecevt2", &self.typecevt2())
            .field("typec_vstate_cc1", &self.typec_vstate_cc1())
            .field("typec_vstate_cc2", &self.typec_vstate_cc2())
            .field("frsevt", &self.frsevt())
            .finish()
    }
}
/**UCPD status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#UCPD:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
