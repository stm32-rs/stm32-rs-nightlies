///Register `SCHED` reader
pub type R = crate::R<SCHEDrs>;
///Register `SCHED` writer
pub type W = crate::W<SCHEDrs>;
///Field `FORCE_LOW_PRI_N` reader - FORCE_LOW_PRI_N
pub type FORCE_LOW_PRI_N_R = crate::BitReader;
///Field `FORCE_LOW_PRI_N` writer - FORCE_LOW_PRI_N
pub type FORCE_LOW_PRI_N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PREFER_WRITE` reader - PREFER_WRITE
pub type PREFER_WRITE_R = crate::BitReader;
///Field `PREFER_WRITE` writer - PREFER_WRITE
pub type PREFER_WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PAGECLOSE` reader - PAGECLOSE
pub type PAGECLOSE_R = crate::BitReader;
///Field `PAGECLOSE` writer - PAGECLOSE
pub type PAGECLOSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPR_NUM_ENTRIES` reader - LPR_NUM_ENTRIES
pub type LPR_NUM_ENTRIES_R = crate::FieldReader;
///Field `LPR_NUM_ENTRIES` writer - LPR_NUM_ENTRIES
pub type LPR_NUM_ENTRIES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `GO2CRITICAL_HYSTERESIS` reader - GO2CRITICAL_HYSTERESIS
pub type GO2CRITICAL_HYSTERESIS_R = crate::FieldReader;
///Field `GO2CRITICAL_HYSTERESIS` writer - GO2CRITICAL_HYSTERESIS
pub type GO2CRITICAL_HYSTERESIS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RDWR_IDLE_GAP` reader - RDWR_IDLE_GAP
pub type RDWR_IDLE_GAP_R = crate::FieldReader;
///Field `RDWR_IDLE_GAP` writer - RDWR_IDLE_GAP
pub type RDWR_IDLE_GAP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bit 0 - FORCE_LOW_PRI_N
    #[inline(always)]
    pub fn force_low_pri_n(&self) -> FORCE_LOW_PRI_N_R {
        FORCE_LOW_PRI_N_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PREFER_WRITE
    #[inline(always)]
    pub fn prefer_write(&self) -> PREFER_WRITE_R {
        PREFER_WRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PAGECLOSE
    #[inline(always)]
    pub fn pageclose(&self) -> PAGECLOSE_R {
        PAGECLOSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 8:11 - LPR_NUM_ENTRIES
    #[inline(always)]
    pub fn lpr_num_entries(&self) -> LPR_NUM_ENTRIES_R {
        LPR_NUM_ENTRIES_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - GO2CRITICAL_HYSTERESIS
    #[inline(always)]
    pub fn go2critical_hysteresis(&self) -> GO2CRITICAL_HYSTERESIS_R {
        GO2CRITICAL_HYSTERESIS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:30 - RDWR_IDLE_GAP
    #[inline(always)]
    pub fn rdwr_idle_gap(&self) -> RDWR_IDLE_GAP_R {
        RDWR_IDLE_GAP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCHED")
            .field("force_low_pri_n", &self.force_low_pri_n())
            .field("prefer_write", &self.prefer_write())
            .field("pageclose", &self.pageclose())
            .field("lpr_num_entries", &self.lpr_num_entries())
            .field("go2critical_hysteresis", &self.go2critical_hysteresis())
            .field("rdwr_idle_gap", &self.rdwr_idle_gap())
            .finish()
    }
}
impl W {
    ///Bit 0 - FORCE_LOW_PRI_N
    #[inline(always)]
    pub fn force_low_pri_n(&mut self) -> FORCE_LOW_PRI_N_W<'_, SCHEDrs> {
        FORCE_LOW_PRI_N_W::new(self, 0)
    }
    ///Bit 1 - PREFER_WRITE
    #[inline(always)]
    pub fn prefer_write(&mut self) -> PREFER_WRITE_W<'_, SCHEDrs> {
        PREFER_WRITE_W::new(self, 1)
    }
    ///Bit 2 - PAGECLOSE
    #[inline(always)]
    pub fn pageclose(&mut self) -> PAGECLOSE_W<'_, SCHEDrs> {
        PAGECLOSE_W::new(self, 2)
    }
    ///Bits 8:11 - LPR_NUM_ENTRIES
    #[inline(always)]
    pub fn lpr_num_entries(&mut self) -> LPR_NUM_ENTRIES_W<'_, SCHEDrs> {
        LPR_NUM_ENTRIES_W::new(self, 8)
    }
    ///Bits 16:23 - GO2CRITICAL_HYSTERESIS
    #[inline(always)]
    pub fn go2critical_hysteresis(&mut self) -> GO2CRITICAL_HYSTERESIS_W<'_, SCHEDrs> {
        GO2CRITICAL_HYSTERESIS_W::new(self, 16)
    }
    ///Bits 24:30 - RDWR_IDLE_GAP
    #[inline(always)]
    pub fn rdwr_idle_gap(&mut self) -> RDWR_IDLE_GAP_W<'_, SCHEDrs> {
        RDWR_IDLE_GAP_W::new(self, 24)
    }
}
/**DDRCTRL scheduler control register

You can [`read`](crate::Reg::read) this register and get [`sched::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sched::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:SCHED)*/
pub struct SCHEDrs;
impl crate::RegisterSpec for SCHEDrs {
    type Ux = u32;
}
///`read()` method returns [`sched::R`](R) reader structure
impl crate::Readable for SCHEDrs {}
///`write(|w| ..)` method takes [`sched::W`](W) writer structure
impl crate::Writable for SCHEDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCHED to value 0x0805
impl crate::Resettable for SCHEDrs {
    const RESET_VALUE: u32 = 0x0805;
}
