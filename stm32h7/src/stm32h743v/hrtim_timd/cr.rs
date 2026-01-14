///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `CKPSC` reader - HRTIM Timer x Clock prescaler
pub type CKPSC_R = crate::FieldReader;
///Field `CKPSC` writer - HRTIM Timer x Clock prescaler
pub type CKPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
///Continuous mode
pub use crate::stm32h743v::hrtim_master::cr::CONT;
///Field `CONT` reader - Continuous mode
pub use crate::stm32h743v::hrtim_master::cr::CONT_R;
///Field `CONT` writer - Continuous mode
pub use crate::stm32h743v::hrtim_master::cr::CONT_W;
///Half mode enable
pub use crate::stm32h743v::hrtim_master::cr::HALF;
///Field `HALF` reader - Half mode enable
pub use crate::stm32h743v::hrtim_master::cr::HALF_R;
///Field `HALF` writer - Half mode enable
pub use crate::stm32h743v::hrtim_master::cr::HALF_W;
///Re-triggerable mode
pub use crate::stm32h743v::hrtim_master::cr::RETRIG;
///Field `RETRIG` reader - Re-triggerable mode
pub use crate::stm32h743v::hrtim_master::cr::RETRIG_R;
///Field `RETRIG` writer - Re-triggerable mode
pub use crate::stm32h743v::hrtim_master::cr::RETRIG_W;
///Synchronization Resets Timer x
pub use crate::stm32h743v::hrtim_master::cr::SYNCRST;
///Field `SYNCRST` reader - Synchronization Resets Timer x
pub use crate::stm32h743v::hrtim_master::cr::SYNCRST_R;
///Field `SYNCRST` writer - Synchronization Resets Timer x
pub use crate::stm32h743v::hrtim_master::cr::SYNCRST_W;
///Synchronization Starts Timer x
pub use crate::stm32h743v::hrtim_master::cr::SYNCSTRT;
///Field `SYNCSTRT` reader - Synchronization Starts Timer x
pub use crate::stm32h743v::hrtim_master::cr::SYNCSTRT_R;
///Field `SYNCSTRT` writer - Synchronization Starts Timer x
pub use crate::stm32h743v::hrtim_master::cr::SYNCSTRT_W;
///Delayed CMP2 mode
pub use crate::stm32h743v::hrtim_tima::cr::DELCMP2;
///Field `DELCMP2` reader - Delayed CMP2 mode
pub use crate::stm32h743v::hrtim_tima::cr::DELCMP2_R;
///Field `DELCMP2` writer - Delayed CMP2 mode
pub use crate::stm32h743v::hrtim_tima::cr::DELCMP2_W;
///Delayed CMP4 mode
pub use crate::stm32h743v::hrtim_tima::cr::DELCMP4;
///Field `DELCMP4` reader - Delayed CMP4 mode
pub use crate::stm32h743v::hrtim_tima::cr::DELCMP4_R;
///Field `DELCMP4` writer - Delayed CMP4 mode
pub use crate::stm32h743v::hrtim_tima::cr::DELCMP4_W;
///Push-Pull mode enable
pub use crate::stm32h743v::hrtim_tima::cr::PSHPLL;
///Field `PSHPLL` reader - Push-Pull mode enable
pub use crate::stm32h743v::hrtim_tima::cr::PSHPLL_R;
///Field `PSHPLL` writer - Push-Pull mode enable
pub use crate::stm32h743v::hrtim_tima::cr::PSHPLL_W;
///Timer x Repetition update
pub use crate::stm32h743v::hrtim_tima::cr::TREPU;
///Field `TREPU` reader - Timer x Repetition update
pub use crate::stm32h743v::hrtim_tima::cr::TREPU_R;
///Field `TREPU` writer - Timer x Repetition update
pub use crate::stm32h743v::hrtim_tima::cr::TREPU_W;
///Timerx reset update
pub use crate::stm32h743v::hrtim_tima::cr::TRSTU;
///Field `TRSTU` reader - Timerx reset update
pub use crate::stm32h743v::hrtim_tima::cr::TRSTU_R;
///Field `TRSTU` writer - Timerx reset update
pub use crate::stm32h743v::hrtim_tima::cr::TRSTU_W;
/**TBU

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBU {
    ///0: Update by timer x disabled
    Disabled = 0,
    ///1: Update by timer x enabled
    Enabled = 1,
}
impl From<TBU> for bool {
    #[inline(always)]
    fn from(variant: TBU) -> Self {
        variant as u8 != 0
    }
}
///Field `TBU` reader - TBU
pub type TBU_R = crate::BitReader<TBU>;
impl TBU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TBU {
        match self.bits {
            false => TBU::Disabled,
            true => TBU::Enabled,
        }
    }
    ///Update by timer x disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TBU::Disabled
    }
    ///Update by timer x enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TBU::Enabled
    }
}
///Field `TBU` writer - TBU
pub type TBU_W<'a, REG> = crate::BitWriter<'a, REG, TBU>;
impl<'a, REG> TBU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update by timer x disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TBU::Disabled)
    }
    ///Update by timer x enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TBU::Enabled)
    }
}
///AC Synchronization
pub use crate::stm32h743v::hrtim_master::cr::DACSYNC;
///Field `DACSYNC` reader - AC Synchronization
pub use crate::stm32h743v::hrtim_master::cr::DACSYNC_R;
///Field `DACSYNC` writer - AC Synchronization
pub use crate::stm32h743v::hrtim_master::cr::DACSYNC_W;
///Preload enable
pub use crate::stm32h743v::hrtim_master::cr::PREEN;
///Field `PREEN` reader - Preload enable
pub use crate::stm32h743v::hrtim_master::cr::PREEN_R;
///Field `PREEN` writer - Preload enable
pub use crate::stm32h743v::hrtim_master::cr::PREEN_W;
///Master Timer update
pub use crate::stm32h743v::hrtim_tima::cr::MSTU;
///Field `MSTU` reader - Master Timer update
pub use crate::stm32h743v::hrtim_tima::cr::MSTU_R;
///Field `MSTU` writer - Master Timer update
pub use crate::stm32h743v::hrtim_tima::cr::MSTU_W;
///Update Gating
pub use crate::stm32h743v::hrtim_tima::cr::UPDGAT;
///Field `UPDGAT` reader - Update Gating
pub use crate::stm32h743v::hrtim_tima::cr::UPDGAT_R;
///Field `UPDGAT` writer - Update Gating
pub use crate::stm32h743v::hrtim_tima::cr::UPDGAT_W;
///Field `TCU` reader - TCU
pub use TBU_R as TCU_R;
///Field `TDU` reader - TDU
pub use TBU_R as TDU_R;
///Field `TEU` reader - TEU
pub use TBU_R as TEU_R;
///Field `TCU` writer - TCU
pub use TBU_W as TCU_W;
///Field `TDU` writer - TDU
pub use TBU_W as TDU_W;
///Field `TEU` writer - TEU
pub use TBU_W as TEU_W;
impl R {
    ///Bits 0:2 - HRTIM Timer x Clock prescaler
    #[inline(always)]
    pub fn ckpsc(&self) -> CKPSC_R {
        CKPSC_R::new((self.bits & 7) as u8)
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
    pub fn syncrst(&self) -> SYNCRST_R {
        SYNCRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Synchronization Starts Timer x
    #[inline(always)]
    pub fn syncstrt(&self) -> SYNCSTRT_R {
        SYNCSTRT_R::new(((self.bits >> 11) & 1) != 0)
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
    pub fn trepu(&self) -> TREPU_R {
        TREPU_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timerx reset update
    #[inline(always)]
    pub fn trstu(&self) -> TRSTU_R {
        TRSTU_R::new(((self.bits >> 18) & 1) != 0)
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
        f.debug_struct("CR")
            .field("updgat", &self.updgat())
            .field("preen", &self.preen())
            .field("dacsync", &self.dacsync())
            .field("mstu", &self.mstu())
            .field("tbu", &self.tbu())
            .field("teu", &self.teu())
            .field("tdu", &self.tdu())
            .field("tcu", &self.tcu())
            .field("trstu", &self.trstu())
            .field("trepu", &self.trepu())
            .field("delcmp4", &self.delcmp4())
            .field("delcmp2", &self.delcmp2())
            .field("syncstrt", &self.syncstrt())
            .field("syncrst", &self.syncrst())
            .field("pshpll", &self.pshpll())
            .field("half", &self.half())
            .field("retrig", &self.retrig())
            .field("cont", &self.cont())
            .field("ckpsc", &self.ckpsc())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - HRTIM Timer x Clock prescaler
    #[inline(always)]
    pub fn ckpsc(&mut self) -> CKPSC_W<'_, CRrs> {
        CKPSC_W::new(self, 0)
    }
    ///Bit 3 - Continuous mode
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<'_, CRrs> {
        CONT_W::new(self, 3)
    }
    ///Bit 4 - Re-triggerable mode
    #[inline(always)]
    pub fn retrig(&mut self) -> RETRIG_W<'_, CRrs> {
        RETRIG_W::new(self, 4)
    }
    ///Bit 5 - Half mode enable
    #[inline(always)]
    pub fn half(&mut self) -> HALF_W<'_, CRrs> {
        HALF_W::new(self, 5)
    }
    ///Bit 6 - Push-Pull mode enable
    #[inline(always)]
    pub fn pshpll(&mut self) -> PSHPLL_W<'_, CRrs> {
        PSHPLL_W::new(self, 6)
    }
    ///Bit 10 - Synchronization Resets Timer x
    #[inline(always)]
    pub fn syncrst(&mut self) -> SYNCRST_W<'_, CRrs> {
        SYNCRST_W::new(self, 10)
    }
    ///Bit 11 - Synchronization Starts Timer x
    #[inline(always)]
    pub fn syncstrt(&mut self) -> SYNCSTRT_W<'_, CRrs> {
        SYNCSTRT_W::new(self, 11)
    }
    ///Bits 12:13 - Delayed CMP2 mode
    #[inline(always)]
    pub fn delcmp2(&mut self) -> DELCMP2_W<'_, CRrs> {
        DELCMP2_W::new(self, 12)
    }
    ///Bits 14:15 - Delayed CMP4 mode
    #[inline(always)]
    pub fn delcmp4(&mut self) -> DELCMP4_W<'_, CRrs> {
        DELCMP4_W::new(self, 14)
    }
    ///Bit 17 - Timer x Repetition update
    #[inline(always)]
    pub fn trepu(&mut self) -> TREPU_W<'_, CRrs> {
        TREPU_W::new(self, 17)
    }
    ///Bit 18 - Timerx reset update
    #[inline(always)]
    pub fn trstu(&mut self) -> TRSTU_W<'_, CRrs> {
        TRSTU_W::new(self, 18)
    }
    ///Bit 20 - TBU
    #[inline(always)]
    pub fn tbu(&mut self) -> TBU_W<'_, CRrs> {
        TBU_W::new(self, 20)
    }
    ///Bit 21 - TCU
    #[inline(always)]
    pub fn tcu(&mut self) -> TCU_W<'_, CRrs> {
        TCU_W::new(self, 21)
    }
    ///Bit 22 - TDU
    #[inline(always)]
    pub fn tdu(&mut self) -> TDU_W<'_, CRrs> {
        TDU_W::new(self, 22)
    }
    ///Bit 23 - TEU
    #[inline(always)]
    pub fn teu(&mut self) -> TEU_W<'_, CRrs> {
        TEU_W::new(self, 23)
    }
    ///Bit 24 - Master Timer update
    #[inline(always)]
    pub fn mstu(&mut self) -> MSTU_W<'_, CRrs> {
        MSTU_W::new(self, 24)
    }
    ///Bits 25:26 - AC Synchronization
    #[inline(always)]
    pub fn dacsync(&mut self) -> DACSYNC_W<'_, CRrs> {
        DACSYNC_W::new(self, 25)
    }
    ///Bit 27 - Preload enable
    #[inline(always)]
    pub fn preen(&mut self) -> PREEN_W<'_, CRrs> {
        PREEN_W::new(self, 27)
    }
    ///Bits 28:31 - Update Gating
    #[inline(always)]
    pub fn updgat(&mut self) -> UPDGAT_W<'_, CRrs> {
        UPDGAT_W::new(self, 28)
    }
}
/**Timerx Control Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#HRTIM_TIMD:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
