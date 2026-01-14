///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `CAFCOF` writer - clear absolute frame counter overflow flag
pub type CAFCOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALCOF` writer - clear absolute line counter overflow flag
pub type CALCOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEF` writer - clear tearing-effect flag
pub type CTEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAFCC1F` writer - clear absolute frame counter compare 1 flag
pub type CAFCC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALCC1F` writer - clear absolute line counter compare 1 flag
pub type CALCC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALCC2F` writer - clear absolute line counter compare 2 flag
pub type CALCC2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRFC1RF` writer - clear relative frame counter 1 reload flag
pub type CRFC1RF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRFC2RF` writer - clear relative frame counter 2 reload flag
pub type CRFC2RF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEV1F` writer - clear event 1 flag
pub type CEV1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEV2F` writer - clear event 2 flag
pub type CEV2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEV3F` writer - clear event 3 flag
pub type CEV3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEV4F` writer - clear event 4 flag
pub type CEV4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWDGAF` writer - clear watchdog alarm flag
pub type CWDGAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWDGPF` writer - clear watchdog pre-alarm flag
pub type CWDGPF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clear absolute frame counter overflow flag
    #[inline(always)]
    pub fn cafcof(&mut self) -> CAFCOF_W<'_, ICRrs> {
        CAFCOF_W::new(self, 0)
    }
    ///Bit 1 - clear absolute line counter overflow flag
    #[inline(always)]
    pub fn calcof(&mut self) -> CALCOF_W<'_, ICRrs> {
        CALCOF_W::new(self, 1)
    }
    ///Bit 2 - clear tearing-effect flag
    #[inline(always)]
    pub fn ctef(&mut self) -> CTEF_W<'_, ICRrs> {
        CTEF_W::new(self, 2)
    }
    ///Bit 4 - clear absolute frame counter compare 1 flag
    #[inline(always)]
    pub fn cafcc1f(&mut self) -> CAFCC1F_W<'_, ICRrs> {
        CAFCC1F_W::new(self, 4)
    }
    ///Bit 8 - clear absolute line counter compare 1 flag
    #[inline(always)]
    pub fn calcc1f(&mut self) -> CALCC1F_W<'_, ICRrs> {
        CALCC1F_W::new(self, 8)
    }
    ///Bit 9 - clear absolute line counter compare 2 flag
    #[inline(always)]
    pub fn calcc2f(&mut self) -> CALCC2F_W<'_, ICRrs> {
        CALCC2F_W::new(self, 9)
    }
    ///Bit 12 - clear relative frame counter 1 reload flag
    #[inline(always)]
    pub fn crfc1rf(&mut self) -> CRFC1RF_W<'_, ICRrs> {
        CRFC1RF_W::new(self, 12)
    }
    ///Bit 13 - clear relative frame counter 2 reload flag
    #[inline(always)]
    pub fn crfc2rf(&mut self) -> CRFC2RF_W<'_, ICRrs> {
        CRFC2RF_W::new(self, 13)
    }
    ///Bit 16 - clear event 1 flag
    #[inline(always)]
    pub fn cev1f(&mut self) -> CEV1F_W<'_, ICRrs> {
        CEV1F_W::new(self, 16)
    }
    ///Bit 17 - clear event 2 flag
    #[inline(always)]
    pub fn cev2f(&mut self) -> CEV2F_W<'_, ICRrs> {
        CEV2F_W::new(self, 17)
    }
    ///Bit 18 - clear event 3 flag
    #[inline(always)]
    pub fn cev3f(&mut self) -> CEV3F_W<'_, ICRrs> {
        CEV3F_W::new(self, 18)
    }
    ///Bit 19 - clear event 4 flag
    #[inline(always)]
    pub fn cev4f(&mut self) -> CEV4F_W<'_, ICRrs> {
        CEV4F_W::new(self, 19)
    }
    ///Bit 24 - clear watchdog alarm flag
    #[inline(always)]
    pub fn cwdgaf(&mut self) -> CWDGAF_W<'_, ICRrs> {
        CWDGAF_W::new(self, 24)
    }
    ///Bit 25 - clear watchdog pre-alarm flag
    #[inline(always)]
    pub fn cwdgpf(&mut self) -> CWDGPF_W<'_, ICRrs> {
        CWDGPF_W::new(self, 25)
    }
}
/**GFXTIM interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#GFXTIM:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
