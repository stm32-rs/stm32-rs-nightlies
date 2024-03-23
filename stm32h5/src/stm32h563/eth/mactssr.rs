#[doc = "Register `MACTSSR` reader"]
pub type R = crate::R<MACTSSRrs>;
#[doc = "Register `MACTSSR` writer"]
pub type W = crate::W<MACTSSRrs>;
#[doc = "Field `TSSOVF` reader - Timestamp Seconds Overflow When this bit is set, it indicates that the seconds value of the timestamp (when supporting version 2 format) has overflowed beyond 32'hFFFF_FFFF. This bit is cleared when the application reads this bit (or writes it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type TSSOVF_R = crate::BitReader;
#[doc = "Field `TSSOVF` writer - Timestamp Seconds Overflow When this bit is set, it indicates that the seconds value of the timestamp (when supporting version 2 format) has overflowed beyond 32'hFFFF_FFFF. This bit is cleared when the application reads this bit (or writes it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)"]
pub type TSSOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTARGT0` reader - Timestamp Target Time Reached When set, this bit indicates that the value of system time is greater than or equal to the value specified in the ETH_MACPPSTTSR and ETH_MACPPSTTSNR registers (see PPS target time seconds register (ETH_MACPPSTTSR) and PPS target time nanoseconds register (ETH_MACPPSTTNR)). This bit is cleared when the application reads this bit (or writes of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type TSTARGT0_R = crate::BitReader;
#[doc = "Field `TSTARGT0` writer - Timestamp Target Time Reached When set, this bit indicates that the value of system time is greater than or equal to the value specified in the ETH_MACPPSTTSR and ETH_MACPPSTTSNR registers (see PPS target time seconds register (ETH_MACPPSTTSR) and PPS target time nanoseconds register (ETH_MACPPSTTNR)). This bit is cleared when the application reads this bit (or writes of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)"]
pub type TSTARGT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXTSTRIG` reader - Auxiliary Timestamp Trigger Snapshot This bit is set high when the auxiliary snapshot is written to the FIFO. This bit is cleared when the application reads this bit (or writes it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type AUXTSTRIG_R = crate::BitReader;
#[doc = "Field `AUXTSTRIG` writer - Auxiliary Timestamp Trigger Snapshot This bit is set high when the auxiliary snapshot is written to the FIFO. This bit is cleared when the application reads this bit (or writes it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)."]
pub type AUXTSTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTRGTERR0` reader - Timestamp Target Time Error This bit is set when the latest target time programmed in the ETH_MACPPSTTSR and ETH_MACPPSTTSNR elapses (see PPS target time seconds register (ETH_MACPPSTTSR) and PPS target time nanoseconds register (ETH_MACPPSTTNR)). This bit is cleared when the application reads this bit (or writes it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type TSTRGTERR0_R = crate::BitReader;
#[doc = "Field `TSTRGTERR0` writer - Timestamp Target Time Error This bit is set when the latest target time programmed in the ETH_MACPPSTTSR and ETH_MACPPSTTSNR elapses (see PPS target time seconds register (ETH_MACPPSTTSR) and PPS target time nanoseconds register (ETH_MACPPSTTNR)). This bit is cleared when the application reads this bit (or writes it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)."]
pub type TSTRGTERR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTSSIS` reader - Tx Timestamp Status Interrupt Status When drop transmit status is enabled in MTL, this bit is set when the captured transmit timestamp is updated in the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) and Tx timestamp status seconds register (ETH_MACTXTSSSR). When PTP offload feature is enabled, this bit is set when the captured transmit timestamp is updated in the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) and Tx timestamp status seconds register (ETH_MACTXTSSSR), for PTO generated Delay Request and Pdelay request packets. This bit is cleared when the Tx timestamp status seconds register (ETH_MACTXTSSSR) is read (or written to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type TXTSSIS_R = crate::BitReader;
#[doc = "Field `TXTSSIS` writer - Tx Timestamp Status Interrupt Status When drop transmit status is enabled in MTL, this bit is set when the captured transmit timestamp is updated in the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) and Tx timestamp status seconds register (ETH_MACTXTSSSR). When PTP offload feature is enabled, this bit is set when the captured transmit timestamp is updated in the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) and Tx timestamp status seconds register (ETH_MACTXTSSSR), for PTO generated Delay Request and Pdelay request packets. This bit is cleared when the Tx timestamp status seconds register (ETH_MACTXTSSSR) is read (or written to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)."]
pub type TXTSSIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSSTN` reader - Auxiliary Timestamp Snapshot Trigger Identifier These bits identify the Auxiliary trigger inputs for which the timestamp available in the Auxiliary Snapshot Register is applicable. When more than one bit is set at the same time, it means that corresponding auxiliary triggers were sampled at the same clock. These bits are applicable only if the number of Auxiliary snapshots is more than one. One bit is assigned for each trigger as shown in the following list: Bit 16: Auxiliary trigger 0 Bit 17: Auxiliary trigger 1 Bit 18: Auxiliary trigger 2 Bit 19: Auxiliary trigger 3 The software can read this register to find the triggers that are set when the timestamp is taken.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type ATSSTN_R = crate::FieldReader;
#[doc = "Field `ATSSTN` writer - Auxiliary Timestamp Snapshot Trigger Identifier These bits identify the Auxiliary trigger inputs for which the timestamp available in the Auxiliary Snapshot Register is applicable. When more than one bit is set at the same time, it means that corresponding auxiliary triggers were sampled at the same clock. These bits are applicable only if the number of Auxiliary snapshots is more than one. One bit is assigned for each trigger as shown in the following list: Bit 16: Auxiliary trigger 0 Bit 17: Auxiliary trigger 1 Bit 18: Auxiliary trigger 2 Bit 19: Auxiliary trigger 3 The software can read this register to find the triggers that are set when the timestamp is taken."]
pub type ATSSTN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ATSSTM` reader - Auxiliary Timestamp Snapshot Trigger Missed This bit is set when the Auxiliary timestamp snapshot FIFO is full and external trigger was set. This indicates that the latest snapshot is not stored in the FIFO.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type ATSSTM_R = crate::BitReader;
#[doc = "Field `ATSSTM` writer - Auxiliary Timestamp Snapshot Trigger Missed This bit is set when the Auxiliary timestamp snapshot FIFO is full and external trigger was set. This indicates that the latest snapshot is not stored in the FIFO."]
pub type ATSSTM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSNS` reader - Number of Auxiliary Timestamp Snapshots This field indicates the number of Snapshots available in the FIFO. A value equal to the depth of FIFO (4) indicates that the Auxiliary Snapshot FIFO is full. These bits are cleared (to 00000) when the Auxiliary snapshot FIFO clear bit is set."]
pub type ATSNS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Timestamp Seconds Overflow When this bit is set, it indicates that the seconds value of the timestamp (when supporting version 2 format) has overflowed beyond 32'hFFFF_FFFF. This bit is cleared when the application reads this bit (or writes it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)"]
    #[inline(always)]
    pub fn tssovf(&self) -> TSSOVF_R {
        TSSOVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timestamp Target Time Reached When set, this bit indicates that the value of system time is greater than or equal to the value specified in the ETH_MACPPSTTSR and ETH_MACPPSTTSNR registers (see PPS target time seconds register (ETH_MACPPSTTSR) and PPS target time nanoseconds register (ETH_MACPPSTTNR)). This bit is cleared when the application reads this bit (or writes of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)"]
    #[inline(always)]
    pub fn tstargt0(&self) -> TSTARGT0_R {
        TSTARGT0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auxiliary Timestamp Trigger Snapshot This bit is set high when the auxiliary snapshot is written to the FIFO. This bit is cleared when the application reads this bit (or writes it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)."]
    #[inline(always)]
    pub fn auxtstrig(&self) -> AUXTSTRIG_R {
        AUXTSTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp Target Time Error This bit is set when the latest target time programmed in the ETH_MACPPSTTSR and ETH_MACPPSTTSNR elapses (see PPS target time seconds register (ETH_MACPPSTTSR) and PPS target time nanoseconds register (ETH_MACPPSTTNR)). This bit is cleared when the application reads this bit (or writes it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)."]
    #[inline(always)]
    pub fn tstrgterr0(&self) -> TSTRGTERR0_R {
        TSTRGTERR0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Timestamp Status Interrupt Status When drop transmit status is enabled in MTL, this bit is set when the captured transmit timestamp is updated in the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) and Tx timestamp status seconds register (ETH_MACTXTSSSR). When PTP offload feature is enabled, this bit is set when the captured transmit timestamp is updated in the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) and Tx timestamp status seconds register (ETH_MACTXTSSSR), for PTO generated Delay Request and Pdelay request packets. This bit is cleared when the Tx timestamp status seconds register (ETH_MACTXTSSSR) is read (or written to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)."]
    #[inline(always)]
    pub fn txtssis(&self) -> TXTSSIS_R {
        TXTSSIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Auxiliary Timestamp Snapshot Trigger Identifier These bits identify the Auxiliary trigger inputs for which the timestamp available in the Auxiliary Snapshot Register is applicable. When more than one bit is set at the same time, it means that corresponding auxiliary triggers were sampled at the same clock. These bits are applicable only if the number of Auxiliary snapshots is more than one. One bit is assigned for each trigger as shown in the following list: Bit 16: Auxiliary trigger 0 Bit 17: Auxiliary trigger 1 Bit 18: Auxiliary trigger 2 Bit 19: Auxiliary trigger 3 The software can read this register to find the triggers that are set when the timestamp is taken."]
    #[inline(always)]
    pub fn atsstn(&self) -> ATSSTN_R {
        ATSSTN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Auxiliary Timestamp Snapshot Trigger Missed This bit is set when the Auxiliary timestamp snapshot FIFO is full and external trigger was set. This indicates that the latest snapshot is not stored in the FIFO."]
    #[inline(always)]
    pub fn atsstm(&self) -> ATSSTM_R {
        ATSSTM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - Number of Auxiliary Timestamp Snapshots This field indicates the number of Snapshots available in the FIFO. A value equal to the depth of FIFO (4) indicates that the Auxiliary Snapshot FIFO is full. These bits are cleared (to 00000) when the Auxiliary snapshot FIFO clear bit is set."]
    #[inline(always)]
    pub fn atsns(&self) -> ATSNS_R {
        ATSNS_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Timestamp Seconds Overflow When this bit is set, it indicates that the seconds value of the timestamp (when supporting version 2 format) has overflowed beyond 32'hFFFF_FFFF. This bit is cleared when the application reads this bit (or writes it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)"]
    #[inline(always)]
    #[must_use]
    pub fn tssovf(&mut self) -> TSSOVF_W<MACTSSRrs> {
        TSSOVF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timestamp Target Time Reached When set, this bit indicates that the value of system time is greater than or equal to the value specified in the ETH_MACPPSTTSR and ETH_MACPPSTTSNR registers (see PPS target time seconds register (ETH_MACPPSTTSR) and PPS target time nanoseconds register (ETH_MACPPSTTNR)). This bit is cleared when the application reads this bit (or writes of 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)"]
    #[inline(always)]
    #[must_use]
    pub fn tstargt0(&mut self) -> TSTARGT0_W<MACTSSRrs> {
        TSTARGT0_W::new(self, 1)
    }
    #[doc = "Bit 2 - Auxiliary Timestamp Trigger Snapshot This bit is set high when the auxiliary snapshot is written to the FIFO. This bit is cleared when the application reads this bit (or writes it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)."]
    #[inline(always)]
    #[must_use]
    pub fn auxtstrig(&mut self) -> AUXTSTRIG_W<MACTSSRrs> {
        AUXTSTRIG_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timestamp Target Time Error This bit is set when the latest target time programmed in the ETH_MACPPSTTSR and ETH_MACPPSTTSNR elapses (see PPS target time seconds register (ETH_MACPPSTTSR) and PPS target time nanoseconds register (ETH_MACPPSTTNR)). This bit is cleared when the application reads this bit (or writes it to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)."]
    #[inline(always)]
    #[must_use]
    pub fn tstrgterr0(&mut self) -> TSTRGTERR0_W<MACTSSRrs> {
        TSTRGTERR0_W::new(self, 3)
    }
    #[doc = "Bit 15 - Tx Timestamp Status Interrupt Status When drop transmit status is enabled in MTL, this bit is set when the captured transmit timestamp is updated in the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) and Tx timestamp status seconds register (ETH_MACTXTSSSR). When PTP offload feature is enabled, this bit is set when the captured transmit timestamp is updated in the Tx timestamp status nanoseconds register (ETH_MACTXTSSNR) and Tx timestamp status seconds register (ETH_MACTXTSSSR), for PTO generated Delay Request and Pdelay request packets. This bit is cleared when the Tx timestamp status seconds register (ETH_MACTXTSSSR) is read (or written to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set)."]
    #[inline(always)]
    #[must_use]
    pub fn txtssis(&mut self) -> TXTSSIS_W<MACTSSRrs> {
        TXTSSIS_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Auxiliary Timestamp Snapshot Trigger Identifier These bits identify the Auxiliary trigger inputs for which the timestamp available in the Auxiliary Snapshot Register is applicable. When more than one bit is set at the same time, it means that corresponding auxiliary triggers were sampled at the same clock. These bits are applicable only if the number of Auxiliary snapshots is more than one. One bit is assigned for each trigger as shown in the following list: Bit 16: Auxiliary trigger 0 Bit 17: Auxiliary trigger 1 Bit 18: Auxiliary trigger 2 Bit 19: Auxiliary trigger 3 The software can read this register to find the triggers that are set when the timestamp is taken."]
    #[inline(always)]
    #[must_use]
    pub fn atsstn(&mut self) -> ATSSTN_W<MACTSSRrs> {
        ATSSTN_W::new(self, 16)
    }
    #[doc = "Bit 24 - Auxiliary Timestamp Snapshot Trigger Missed This bit is set when the Auxiliary timestamp snapshot FIFO is full and external trigger was set. This indicates that the latest snapshot is not stored in the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn atsstm(&mut self) -> ATSSTM_W<MACTSSRrs> {
        ATSSTM_W::new(self, 24)
    }
}
#[doc = "Timestamp status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactssr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactssr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTSSRrs;
impl crate::RegisterSpec for MACTSSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactssr::R`](R) reader structure"]
impl crate::Readable for MACTSSRrs {}
#[doc = "`write(|w| ..)` method takes [`mactssr::W`](W) writer structure"]
impl crate::Writable for MACTSSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACTSSR to value 0"]
impl crate::Resettable for MACTSSRrs {
    const RESET_VALUE: u32 = 0;
}
