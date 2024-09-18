///Register `TIMBCR` reader
pub type R = crate::R<TIMBCRrs>;
///Register `TIMBCR` writer
pub type W = crate::W<TIMBCRrs>;
///Field `CK_PSCx` reader - HRTIM Timer x Clock prescaler
pub type CK_PSCX_R = crate::FieldReader;
///Field `CK_PSCx` writer - HRTIM Timer x Clock prescaler
pub type CK_PSCX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CONT` reader - Continuous mode
pub type CONT_R = crate::BitReader;
///Field `CONT` writer - Continuous mode
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RETRIG` reader - Re-triggerable mode
pub type RETRIG_R = crate::BitReader;
///Field `RETRIG` writer - Re-triggerable mode
pub type RETRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HALF` reader - Half mode enable
pub type HALF_R = crate::BitReader;
///Field `HALF` writer - Half mode enable
pub type HALF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSHPLL` reader - Push-Pull mode enable
pub type PSHPLL_R = crate::BitReader;
///Field `PSHPLL` writer - Push-Pull mode enable
pub type PSHPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNCRSTx` reader - Synchronization Resets Timer x
pub type SYNCRSTX_R = crate::BitReader;
///Field `SYNCRSTx` writer - Synchronization Resets Timer x
pub type SYNCRSTX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNCSTRTx` reader - Synchronization Starts Timer x
pub type SYNCSTRTX_R = crate::BitReader;
///Field `SYNCSTRTx` writer - Synchronization Starts Timer x
pub type SYNCSTRTX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DELCMP2` reader - Delayed CMP2 mode
pub type DELCMP2_R = crate::FieldReader;
///Field `DELCMP2` writer - Delayed CMP2 mode
pub type DELCMP2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DELCMP4` reader - Delayed CMP4 mode
pub type DELCMP4_R = crate::FieldReader;
///Field `DELCMP4` writer - Delayed CMP4 mode
pub type DELCMP4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TxREPU` reader - Timer x Repetition update
pub type TX_REPU_R = crate::BitReader;
///Field `TxREPU` writer - Timer x Repetition update
pub type TX_REPU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TxRSTU` reader - Timerx reset update
pub type TX_RSTU_R = crate::BitReader;
///Field `TxRSTU` writer - Timerx reset update
pub type TX_RSTU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TBU` reader - TBU
pub type TBU_R = crate::BitReader;
///Field `TBU` writer - TBU
pub type TBU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCU` reader - TCU
pub type TCU_R = crate::BitReader;
///Field `TCU` writer - TCU
pub type TCU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TDU` reader - TDU
pub type TDU_R = crate::BitReader;
///Field `TDU` writer - TDU
pub type TDU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEU` reader - TEU
pub type TEU_R = crate::BitReader;
///Field `TEU` writer - TEU
pub type TEU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSTU` reader - Master Timer update
pub type MSTU_R = crate::BitReader;
///Field `MSTU` writer - Master Timer update
pub type MSTU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DACSYNC` reader - AC Synchronization
pub type DACSYNC_R = crate::FieldReader;
///Field `DACSYNC` writer - AC Synchronization
pub type DACSYNC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PREEN` reader - Preload enable
pub type PREEN_R = crate::BitReader;
///Field `PREEN` writer - Preload enable
pub type PREEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UPDGAT` reader - Update Gating
pub type UPDGAT_R = crate::FieldReader;
///Field `UPDGAT` writer - Update Gating
pub type UPDGAT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:2 - HRTIM Timer x Clock prescaler
    #[inline(always)]
    pub fn ck_pscx(&self) -> CK_PSCX_R {
        CK_PSCX_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Continuous mode
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Re-triggerable mode
    #[inline(always)]
    pub fn retrig(&self) -> RETRIG_R {
        RETRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Half mode enable
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Push-Pull mode enable
    #[inline(always)]
    pub fn pshpll(&self) -> PSHPLL_R {
        PSHPLL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 10 - Synchronization Resets Timer x
    #[inline(always)]
    pub fn syncrstx(&self) -> SYNCRSTX_R {
        SYNCRSTX_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Synchronization Starts Timer x
    #[inline(always)]
    pub fn syncstrtx(&self) -> SYNCSTRTX_R {
        SYNCSTRTX_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Delayed CMP2 mode
    #[inline(always)]
    pub fn delcmp2(&self) -> DELCMP2_R {
        DELCMP2_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Delayed CMP4 mode
    #[inline(always)]
    pub fn delcmp4(&self) -> DELCMP4_R {
        DELCMP4_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 17 - Timer x Repetition update
    #[inline(always)]
    pub fn tx_repu(&self) -> TX_REPU_R {
        TX_REPU_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timerx reset update
    #[inline(always)]
    pub fn tx_rstu(&self) -> TX_RSTU_R {
        TX_RSTU_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - TBU
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TCU
    #[inline(always)]
    pub fn tcu(&self) -> TCU_R {
        TCU_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - TDU
    #[inline(always)]
    pub fn tdu(&self) -> TDU_R {
        TDU_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TEU
    #[inline(always)]
    pub fn teu(&self) -> TEU_R {
        TEU_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Master Timer update
    #[inline(always)]
    pub fn mstu(&self) -> MSTU_R {
        MSTU_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - AC Synchronization
    #[inline(always)]
    pub fn dacsync(&self) -> DACSYNC_R {
        DACSYNC_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bit 27 - Preload enable
    #[inline(always)]
    pub fn preen(&self) -> PREEN_R {
        PREEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:31 - Update Gating
    #[inline(always)]
    pub fn updgat(&self) -> UPDGAT_R {
        UPDGAT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMBCR")
            .field("updgat", &self.updgat())
            .field("preen", &self.preen())
            .field("dacsync", &self.dacsync())
            .field("mstu", &self.mstu())
            .field("teu", &self.teu())
            .field("tdu", &self.tdu())
            .field("tcu", &self.tcu())
            .field("tbu", &self.tbu())
            .field("tx_rstu", &self.tx_rstu())
            .field("tx_repu", &self.tx_repu())
            .field("delcmp4", &self.delcmp4())
            .field("delcmp2", &self.delcmp2())
            .field("syncstrtx", &self.syncstrtx())
            .field("syncrstx", &self.syncrstx())
            .field("pshpll", &self.pshpll())
            .field("half", &self.half())
            .field("retrig", &self.retrig())
            .field("cont", &self.cont())
            .field("ck_pscx", &self.ck_pscx())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - HRTIM Timer x Clock prescaler
    #[inline(always)]
    #[must_use]
    pub fn ck_pscx(&mut self) -> CK_PSCX_W<TIMBCRrs> {
        CK_PSCX_W::new(self, 0)
    }
    ///Bit 3 - Continuous mode
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<TIMBCRrs> {
        CONT_W::new(self, 3)
    }
    ///Bit 4 - Re-triggerable mode
    #[inline(always)]
    #[must_use]
    pub fn retrig(&mut self) -> RETRIG_W<TIMBCRrs> {
        RETRIG_W::new(self, 4)
    }
    ///Bit 5 - Half mode enable
    #[inline(always)]
    #[must_use]
    pub fn half(&mut self) -> HALF_W<TIMBCRrs> {
        HALF_W::new(self, 5)
    }
    ///Bit 6 - Push-Pull mode enable
    #[inline(always)]
    #[must_use]
    pub fn pshpll(&mut self) -> PSHPLL_W<TIMBCRrs> {
        PSHPLL_W::new(self, 6)
    }
    ///Bit 10 - Synchronization Resets Timer x
    #[inline(always)]
    #[must_use]
    pub fn syncrstx(&mut self) -> SYNCRSTX_W<TIMBCRrs> {
        SYNCRSTX_W::new(self, 10)
    }
    ///Bit 11 - Synchronization Starts Timer x
    #[inline(always)]
    #[must_use]
    pub fn syncstrtx(&mut self) -> SYNCSTRTX_W<TIMBCRrs> {
        SYNCSTRTX_W::new(self, 11)
    }
    ///Bits 12:13 - Delayed CMP2 mode
    #[inline(always)]
    #[must_use]
    pub fn delcmp2(&mut self) -> DELCMP2_W<TIMBCRrs> {
        DELCMP2_W::new(self, 12)
    }
    ///Bits 14:15 - Delayed CMP4 mode
    #[inline(always)]
    #[must_use]
    pub fn delcmp4(&mut self) -> DELCMP4_W<TIMBCRrs> {
        DELCMP4_W::new(self, 14)
    }
    ///Bit 17 - Timer x Repetition update
    #[inline(always)]
    #[must_use]
    pub fn tx_repu(&mut self) -> TX_REPU_W<TIMBCRrs> {
        TX_REPU_W::new(self, 17)
    }
    ///Bit 18 - Timerx reset update
    #[inline(always)]
    #[must_use]
    pub fn tx_rstu(&mut self) -> TX_RSTU_W<TIMBCRrs> {
        TX_RSTU_W::new(self, 18)
    }
    ///Bit 20 - TBU
    #[inline(always)]
    #[must_use]
    pub fn tbu(&mut self) -> TBU_W<TIMBCRrs> {
        TBU_W::new(self, 20)
    }
    ///Bit 21 - TCU
    #[inline(always)]
    #[must_use]
    pub fn tcu(&mut self) -> TCU_W<TIMBCRrs> {
        TCU_W::new(self, 21)
    }
    ///Bit 22 - TDU
    #[inline(always)]
    #[must_use]
    pub fn tdu(&mut self) -> TDU_W<TIMBCRrs> {
        TDU_W::new(self, 22)
    }
    ///Bit 23 - TEU
    #[inline(always)]
    #[must_use]
    pub fn teu(&mut self) -> TEU_W<TIMBCRrs> {
        TEU_W::new(self, 23)
    }
    ///Bit 24 - Master Timer update
    #[inline(always)]
    #[must_use]
    pub fn mstu(&mut self) -> MSTU_W<TIMBCRrs> {
        MSTU_W::new(self, 24)
    }
    ///Bits 25:26 - AC Synchronization
    #[inline(always)]
    #[must_use]
    pub fn dacsync(&mut self) -> DACSYNC_W<TIMBCRrs> {
        DACSYNC_W::new(self, 25)
    }
    ///Bit 27 - Preload enable
    #[inline(always)]
    #[must_use]
    pub fn preen(&mut self) -> PREEN_W<TIMBCRrs> {
        PREEN_W::new(self, 27)
    }
    ///Bits 28:31 - Update Gating
    #[inline(always)]
    #[must_use]
    pub fn updgat(&mut self) -> UPDGAT_W<TIMBCRrs> {
        UPDGAT_W::new(self, 28)
    }
}
/**Timerx Control Register

You can [`read`](crate::Reg::read) this register and get [`timbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#HRTIM_TIMB:TIMBCR)*/
pub struct TIMBCRrs;
impl crate::RegisterSpec for TIMBCRrs {
    type Ux = u32;
}
///`read()` method returns [`timbcr::R`](R) reader structure
impl crate::Readable for TIMBCRrs {}
///`write(|w| ..)` method takes [`timbcr::W`](W) writer structure
impl crate::Writable for TIMBCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIMBCR to value 0
impl crate::Resettable for TIMBCRrs {
    const RESET_VALUE: u32 = 0;
}
