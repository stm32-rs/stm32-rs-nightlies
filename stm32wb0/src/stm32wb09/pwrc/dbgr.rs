///Register `DBGR` reader
pub type R = crate::R<DBGRrs>;
///Register `DBGR` writer
pub type W = crate::W<DBGRrs>;
///Field `DEEPSTOP2` reader - DEEPSTOP2: DEEPSTOP2 low power saving emulation enable. 0: normal DEEPSTOP will be applied 1: DEEPSTOP2 (debugger features not lost) will be applied instead of DEEPSTOP.
pub type DEEPSTOP2_R = crate::BitReader;
///Field `DEEPSTOP2` writer - DEEPSTOP2: DEEPSTOP2 low power saving emulation enable. 0: normal DEEPSTOP will be applied 1: DEEPSTOP2 (debugger features not lost) will be applied instead of DEEPSTOP.
pub type DEEPSTOP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIS_PRECH` reader - DIS_PRECH\[2:0\]: disable precharge during deepstop (debug) 111: precharge and SMPS monitoring are disabled (whatever CR5.SMPSLPOPEN) 101: precharge are activated only at deepstop exit (to be used only with CR5.SMPSLPOPEN=1) else: No effect (default 0x0)
pub type DIS_PRECH_R = crate::FieldReader;
///Field `DIS_PRECH` writer - DIS_PRECH\[2:0\]: disable precharge during deepstop (debug) 111: precharge and SMPS monitoring are disabled (whatever CR5.SMPSLPOPEN) 101: precharge are activated only at deepstop exit (to be used only with CR5.SMPSLPOPEN=1) else: No effect (default 0x0)
pub type DIS_PRECH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - DEEPSTOP2: DEEPSTOP2 low power saving emulation enable. 0: normal DEEPSTOP will be applied 1: DEEPSTOP2 (debugger features not lost) will be applied instead of DEEPSTOP.
    #[inline(always)]
    pub fn deepstop2(&self) -> DEEPSTOP2_R {
        DEEPSTOP2_R::new((self.bits & 1) != 0)
    }
    ///Bits 13:15 - DIS_PRECH\[2:0\]: disable precharge during deepstop (debug) 111: precharge and SMPS monitoring are disabled (whatever CR5.SMPSLPOPEN) 101: precharge are activated only at deepstop exit (to be used only with CR5.SMPSLPOPEN=1) else: No effect (default 0x0)
    #[inline(always)]
    pub fn dis_prech(&self) -> DIS_PRECH_R {
        DIS_PRECH_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGR")
            .field("deepstop2", &self.deepstop2())
            .field("dis_prech", &self.dis_prech())
            .finish()
    }
}
impl W {
    ///Bit 0 - DEEPSTOP2: DEEPSTOP2 low power saving emulation enable. 0: normal DEEPSTOP will be applied 1: DEEPSTOP2 (debugger features not lost) will be applied instead of DEEPSTOP.
    #[inline(always)]
    pub fn deepstop2(&mut self) -> DEEPSTOP2_W<'_, DBGRrs> {
        DEEPSTOP2_W::new(self, 0)
    }
    ///Bits 13:15 - DIS_PRECH\[2:0\]: disable precharge during deepstop (debug) 111: precharge and SMPS monitoring are disabled (whatever CR5.SMPSLPOPEN) 101: precharge are activated only at deepstop exit (to be used only with CR5.SMPSLPOPEN=1) else: No effect (default 0x0)
    #[inline(always)]
    pub fn dis_prech(&mut self) -> DIS_PRECH_W<'_, DBGRrs> {
        DIS_PRECH_W::new(self, 13)
    }
}
/**DBGR register

You can [`read`](crate::Reg::read) this register and get [`dbgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:DBGR)*/
pub struct DBGRrs;
impl crate::RegisterSpec for DBGRrs {
    type Ux = u32;
}
///`read()` method returns [`dbgr::R`](R) reader structure
impl crate::Readable for DBGRrs {}
///`write(|w| ..)` method takes [`dbgr::W`](W) writer structure
impl crate::Writable for DBGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBGR to value 0
impl crate::Resettable for DBGRrs {}
