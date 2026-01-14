///Register `DBGR` reader
pub type R = crate::R<DBGRrs>;
///Register `DBGR` writer
pub type W = crate::W<DBGRrs>;
///Field `DEEPSTOP2` reader - DEEPSTOP2: DEEPSTOP2 low power saving emulation enable. 0: normal DEEPSTOP will be applied 1: DEEPSTOP2 (debugger features not lost) will be applied instead of DEEPSTOP.
pub type DEEPSTOP2_R = crate::BitReader;
///Field `DEEPSTOP2` writer - DEEPSTOP2: DEEPSTOP2 low power saving emulation enable. 0: normal DEEPSTOP will be applied 1: DEEPSTOP2 (debugger features not lost) will be applied instead of DEEPSTOP.
pub type DEEPSTOP2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DEEPSTOP2: DEEPSTOP2 low power saving emulation enable. 0: normal DEEPSTOP will be applied 1: DEEPSTOP2 (debugger features not lost) will be applied instead of DEEPSTOP.
    #[inline(always)]
    pub fn deepstop2(&self) -> DEEPSTOP2_R {
        DEEPSTOP2_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGR")
            .field("deepstop2", &self.deepstop2())
            .finish()
    }
}
impl W {
    ///Bit 0 - DEEPSTOP2: DEEPSTOP2 low power saving emulation enable. 0: normal DEEPSTOP will be applied 1: DEEPSTOP2 (debugger features not lost) will be applied instead of DEEPSTOP.
    #[inline(always)]
    pub fn deepstop2(&mut self) -> DEEPSTOP2_W<'_, DBGRrs> {
        DEEPSTOP2_W::new(self, 0)
    }
}
/**DBGR register

You can [`read`](crate::Reg::read) this register and get [`dbgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#PWRC:DBGR)*/
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
