#[doc = "Register `TIMACR` reader"]
pub type R = crate::R<TIMACRrs>;
#[doc = "Register `TIMACR` writer"]
pub type W = crate::W<TIMACRrs>;
#[doc = "Field `CK_PSCx` reader - HRTIM Timer x Clock prescaler"]
pub type CK_PSCX_R = crate::FieldReader;
#[doc = "Field `CK_PSCx` writer - HRTIM Timer x Clock prescaler"]
pub type CK_PSCX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CONT` reader - Continuous mode"]
pub type CONT_R = crate::BitReader;
#[doc = "Field `CONT` writer - Continuous mode"]
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRIG` reader - Re-triggerable mode"]
pub type RETRIG_R = crate::BitReader;
#[doc = "Field `RETRIG` writer - Re-triggerable mode"]
pub type RETRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALF` reader - Half mode enable"]
pub type HALF_R = crate::BitReader;
#[doc = "Field `HALF` writer - Half mode enable"]
pub type HALF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSHPLL` reader - Push-Pull mode enable"]
pub type PSHPLL_R = crate::BitReader;
#[doc = "Field `PSHPLL` writer - Push-Pull mode enable"]
pub type PSHPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTLVD` reader - Interleaved mode"]
pub type INTLVD_R = crate::FieldReader;
#[doc = "Field `INTLVD` writer - Interleaved mode"]
pub type INTLVD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSYNCU` reader - Re-Synchronized Update"]
pub type RSYNCU_R = crate::BitReader;
#[doc = "Field `RSYNCU` writer - Re-Synchronized Update"]
pub type RSYNCU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCRSTx` reader - Synchronization Resets Timer x"]
pub type SYNCRSTX_R = crate::BitReader;
#[doc = "Field `SYNCRSTx` writer - Synchronization Resets Timer x"]
pub type SYNCRSTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCSTRTx` reader - Synchronization Starts Timer x"]
pub type SYNCSTRTX_R = crate::BitReader;
#[doc = "Field `SYNCSTRTx` writer - Synchronization Starts Timer x"]
pub type SYNCSTRTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DELCMP2` reader - Delayed CMP2 mode"]
pub type DELCMP2_R = crate::FieldReader;
#[doc = "Field `DELCMP2` writer - Delayed CMP2 mode"]
pub type DELCMP2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DELCMP4` reader - Delayed CMP4 mode"]
pub type DELCMP4_R = crate::FieldReader;
#[doc = "Field `DELCMP4` writer - Delayed CMP4 mode"]
pub type DELCMP4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TFU` reader - TFU"]
pub type TFU_R = crate::BitReader;
#[doc = "Field `TFU` writer - TFU"]
pub type TFU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxREPU` reader - Timer x Repetition update"]
pub type TX_REPU_R = crate::BitReader;
#[doc = "Field `TxREPU` writer - Timer x Repetition update"]
pub type TX_REPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxRSTU` reader - Timerx reset update"]
pub type TX_RSTU_R = crate::BitReader;
#[doc = "Field `TxRSTU` writer - Timerx reset update"]
pub type TX_RSTU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBU` reader - TBU"]
pub type TBU_R = crate::BitReader;
#[doc = "Field `TBU` writer - TBU"]
pub type TBU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCU` reader - TCU"]
pub type TCU_R = crate::BitReader;
#[doc = "Field `TCU` writer - TCU"]
pub type TCU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDU` reader - TDU"]
pub type TDU_R = crate::BitReader;
#[doc = "Field `TDU` writer - TDU"]
pub type TDU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEU` reader - TEU"]
pub type TEU_R = crate::BitReader;
#[doc = "Field `TEU` writer - TEU"]
pub type TEU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTU` reader - Master Timer update"]
pub type MSTU_R = crate::BitReader;
#[doc = "Field `MSTU` writer - Master Timer update"]
pub type MSTU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACSYNC` reader - AC Synchronization"]
pub type DACSYNC_R = crate::FieldReader;
#[doc = "Field `DACSYNC` writer - AC Synchronization"]
pub type DACSYNC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PREEN` reader - Preload enable"]
pub type PREEN_R = crate::BitReader;
#[doc = "Field `PREEN` writer - Preload enable"]
pub type PREEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDGAT` reader - Update Gating"]
pub type UPDGAT_R = crate::FieldReader;
#[doc = "Field `UPDGAT` writer - Update Gating"]
pub type UPDGAT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - HRTIM Timer x Clock prescaler"]
    #[inline(always)]
    pub fn ck_pscx(&self) -> CK_PSCX_R {
        CK_PSCX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Re-triggerable mode"]
    #[inline(always)]
    pub fn retrig(&self) -> RETRIG_R {
        RETRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Push-Pull mode enable"]
    #[inline(always)]
    pub fn pshpll(&self) -> PSHPLL_R {
        PSHPLL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Interleaved mode"]
    #[inline(always)]
    pub fn intlvd(&self) -> INTLVD_R {
        INTLVD_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Re-Synchronized Update"]
    #[inline(always)]
    pub fn rsyncu(&self) -> RSYNCU_R {
        RSYNCU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronization Resets Timer x"]
    #[inline(always)]
    pub fn syncrstx(&self) -> SYNCRSTX_R {
        SYNCRSTX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronization Starts Timer x"]
    #[inline(always)]
    pub fn syncstrtx(&self) -> SYNCSTRTX_R {
        SYNCSTRTX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Delayed CMP2 mode"]
    #[inline(always)]
    pub fn delcmp2(&self) -> DELCMP2_R {
        DELCMP2_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Delayed CMP4 mode"]
    #[inline(always)]
    pub fn delcmp4(&self) -> DELCMP4_R {
        DELCMP4_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - TFU"]
    #[inline(always)]
    pub fn tfu(&self) -> TFU_R {
        TFU_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer x Repetition update"]
    #[inline(always)]
    pub fn tx_repu(&self) -> TX_REPU_R {
        TX_REPU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timerx reset update"]
    #[inline(always)]
    pub fn tx_rstu(&self) -> TX_RSTU_R {
        TX_RSTU_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - TBU"]
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TCU"]
    #[inline(always)]
    pub fn tcu(&self) -> TCU_R {
        TCU_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TDU"]
    #[inline(always)]
    pub fn tdu(&self) -> TDU_R {
        TDU_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TEU"]
    #[inline(always)]
    pub fn teu(&self) -> TEU_R {
        TEU_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Master Timer update"]
    #[inline(always)]
    pub fn mstu(&self) -> MSTU_R {
        MSTU_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    pub fn dacsync(&self) -> DACSYNC_R {
        DACSYNC_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    pub fn preen(&self) -> PREEN_R {
        PREEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Update Gating"]
    #[inline(always)]
    pub fn updgat(&self) -> UPDGAT_R {
        UPDGAT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HRTIM Timer x Clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ck_pscx(&mut self) -> CK_PSCX_W<TIMACRrs> {
        CK_PSCX_W::new(self, 0)
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<TIMACRrs> {
        CONT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Re-triggerable mode"]
    #[inline(always)]
    #[must_use]
    pub fn retrig(&mut self) -> RETRIG_W<TIMACRrs> {
        RETRIG_W::new(self, 4)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn half(&mut self) -> HALF_W<TIMACRrs> {
        HALF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Push-Pull mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn pshpll(&mut self) -> PSHPLL_W<TIMACRrs> {
        PSHPLL_W::new(self, 6)
    }
    #[doc = "Bits 7:8 - Interleaved mode"]
    #[inline(always)]
    #[must_use]
    pub fn intlvd(&mut self) -> INTLVD_W<TIMACRrs> {
        INTLVD_W::new(self, 7)
    }
    #[doc = "Bit 9 - Re-Synchronized Update"]
    #[inline(always)]
    #[must_use]
    pub fn rsyncu(&mut self) -> RSYNCU_W<TIMACRrs> {
        RSYNCU_W::new(self, 9)
    }
    #[doc = "Bit 10 - Synchronization Resets Timer x"]
    #[inline(always)]
    #[must_use]
    pub fn syncrstx(&mut self) -> SYNCRSTX_W<TIMACRrs> {
        SYNCRSTX_W::new(self, 10)
    }
    #[doc = "Bit 11 - Synchronization Starts Timer x"]
    #[inline(always)]
    #[must_use]
    pub fn syncstrtx(&mut self) -> SYNCSTRTX_W<TIMACRrs> {
        SYNCSTRTX_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Delayed CMP2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn delcmp2(&mut self) -> DELCMP2_W<TIMACRrs> {
        DELCMP2_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Delayed CMP4 mode"]
    #[inline(always)]
    #[must_use]
    pub fn delcmp4(&mut self) -> DELCMP4_W<TIMACRrs> {
        DELCMP4_W::new(self, 14)
    }
    #[doc = "Bit 16 - TFU"]
    #[inline(always)]
    #[must_use]
    pub fn tfu(&mut self) -> TFU_W<TIMACRrs> {
        TFU_W::new(self, 16)
    }
    #[doc = "Bit 17 - Timer x Repetition update"]
    #[inline(always)]
    #[must_use]
    pub fn tx_repu(&mut self) -> TX_REPU_W<TIMACRrs> {
        TX_REPU_W::new(self, 17)
    }
    #[doc = "Bit 18 - Timerx reset update"]
    #[inline(always)]
    #[must_use]
    pub fn tx_rstu(&mut self) -> TX_RSTU_W<TIMACRrs> {
        TX_RSTU_W::new(self, 18)
    }
    #[doc = "Bit 20 - TBU"]
    #[inline(always)]
    #[must_use]
    pub fn tbu(&mut self) -> TBU_W<TIMACRrs> {
        TBU_W::new(self, 20)
    }
    #[doc = "Bit 21 - TCU"]
    #[inline(always)]
    #[must_use]
    pub fn tcu(&mut self) -> TCU_W<TIMACRrs> {
        TCU_W::new(self, 21)
    }
    #[doc = "Bit 22 - TDU"]
    #[inline(always)]
    #[must_use]
    pub fn tdu(&mut self) -> TDU_W<TIMACRrs> {
        TDU_W::new(self, 22)
    }
    #[doc = "Bit 23 - TEU"]
    #[inline(always)]
    #[must_use]
    pub fn teu(&mut self) -> TEU_W<TIMACRrs> {
        TEU_W::new(self, 23)
    }
    #[doc = "Bit 24 - Master Timer update"]
    #[inline(always)]
    #[must_use]
    pub fn mstu(&mut self) -> MSTU_W<TIMACRrs> {
        MSTU_W::new(self, 24)
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn dacsync(&mut self) -> DACSYNC_W<TIMACRrs> {
        DACSYNC_W::new(self, 25)
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn preen(&mut self) -> PREEN_W<TIMACRrs> {
        PREEN_W::new(self, 27)
    }
    #[doc = "Bits 28:31 - Update Gating"]
    #[inline(always)]
    #[must_use]
    pub fn updgat(&mut self) -> UPDGAT_W<TIMACRrs> {
        UPDGAT_W::new(self, 28)
    }
}
#[doc = "Timerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMACRrs;
impl crate::RegisterSpec for TIMACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timacr::R`](R) reader structure"]
impl crate::Readable for TIMACRrs {}
#[doc = "`write(|w| ..)` method takes [`timacr::W`](W) writer structure"]
impl crate::Writable for TIMACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMACR to value 0"]
impl crate::Resettable for TIMACRrs {
    const RESET_VALUE: u32 = 0;
}
