///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `CAFCOF` writer - clear absolute frame counter overflow flag This bit clears AFCOF in GXTIM_ISR.
pub type CAFCOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALCOF` writer - clear absolute line counter overflow flag This bit clears ALCOF in GXTIM_ISR.
pub type CALCOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEF` writer - clear tearing-effect flag This bit clears TEF in GXTIM_ISR.
pub type CTEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAFCC1F` writer - clear absolute frame counter compare 1 flag This bit clears AFCC1F in GXTIM_ISR.
pub type CAFCC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALCC1F` writer - clear absolute line counter compare 1 flag This bit clears ALCC1F in GXTIM_ISR.
pub type CALCC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALCC2F` writer - clear absolute line counter compare 2 flag This bit clears ALCC2F in GXTIM_ISR.
pub type CALCC2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRFC1RF` writer - clear relative frame counter 1 reload flag This bit clears RFC1RF in GXTIM_ISR.
pub type CRFC1RF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRFC2RF` writer - clear relative frame counter 2 reload flag This bit clears RFC2RF in GXFXTIM_ISR.
pub type CRFC2RF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEV1F` writer - clear event 1 flag This bit EV1F in GXFXTIM_ISR.
pub type CEV1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEV2F` writer - clear event 2 flag This bit clears EV2F in GXFXTIM_ISR.
pub type CEV2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEV3F` writer - clear event 3 flag This bit clears EV3F in GXFXTIM_ISR.
pub type CEV3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEV4F` writer - clear event 4 flag This bit clears EV4F in GXFXTIM_ISR.
pub type CEV4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWDGAF` writer - clear watchdog alarm flag This bit clears WDGAF in GXFXTIM_ISR.
pub type CWDGAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWDGPF` writer - clear watchdog pre-alarm flag This bit clears WDGPF in GXFXTIM_ISR.
pub type CWDGPF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clear absolute frame counter overflow flag This bit clears AFCOF in GXTIM_ISR.
    #[inline(always)]
    pub fn cafcof(&mut self) -> CAFCOF_W<ICRrs> {
        CAFCOF_W::new(self, 0)
    }
    ///Bit 1 - clear absolute line counter overflow flag This bit clears ALCOF in GXTIM_ISR.
    #[inline(always)]
    pub fn calcof(&mut self) -> CALCOF_W<ICRrs> {
        CALCOF_W::new(self, 1)
    }
    ///Bit 2 - clear tearing-effect flag This bit clears TEF in GXTIM_ISR.
    #[inline(always)]
    pub fn ctef(&mut self) -> CTEF_W<ICRrs> {
        CTEF_W::new(self, 2)
    }
    ///Bit 4 - clear absolute frame counter compare 1 flag This bit clears AFCC1F in GXTIM_ISR.
    #[inline(always)]
    pub fn cafcc1f(&mut self) -> CAFCC1F_W<ICRrs> {
        CAFCC1F_W::new(self, 4)
    }
    ///Bit 8 - clear absolute line counter compare 1 flag This bit clears ALCC1F in GXTIM_ISR.
    #[inline(always)]
    pub fn calcc1f(&mut self) -> CALCC1F_W<ICRrs> {
        CALCC1F_W::new(self, 8)
    }
    ///Bit 9 - clear absolute line counter compare 2 flag This bit clears ALCC2F in GXTIM_ISR.
    #[inline(always)]
    pub fn calcc2f(&mut self) -> CALCC2F_W<ICRrs> {
        CALCC2F_W::new(self, 9)
    }
    ///Bit 12 - clear relative frame counter 1 reload flag This bit clears RFC1RF in GXTIM_ISR.
    #[inline(always)]
    pub fn crfc1rf(&mut self) -> CRFC1RF_W<ICRrs> {
        CRFC1RF_W::new(self, 12)
    }
    ///Bit 13 - clear relative frame counter 2 reload flag This bit clears RFC2RF in GXFXTIM_ISR.
    #[inline(always)]
    pub fn crfc2rf(&mut self) -> CRFC2RF_W<ICRrs> {
        CRFC2RF_W::new(self, 13)
    }
    ///Bit 16 - clear event 1 flag This bit EV1F in GXFXTIM_ISR.
    #[inline(always)]
    pub fn cev1f(&mut self) -> CEV1F_W<ICRrs> {
        CEV1F_W::new(self, 16)
    }
    ///Bit 17 - clear event 2 flag This bit clears EV2F in GXFXTIM_ISR.
    #[inline(always)]
    pub fn cev2f(&mut self) -> CEV2F_W<ICRrs> {
        CEV2F_W::new(self, 17)
    }
    ///Bit 18 - clear event 3 flag This bit clears EV3F in GXFXTIM_ISR.
    #[inline(always)]
    pub fn cev3f(&mut self) -> CEV3F_W<ICRrs> {
        CEV3F_W::new(self, 18)
    }
    ///Bit 19 - clear event 4 flag This bit clears EV4F in GXFXTIM_ISR.
    #[inline(always)]
    pub fn cev4f(&mut self) -> CEV4F_W<ICRrs> {
        CEV4F_W::new(self, 19)
    }
    ///Bit 24 - clear watchdog alarm flag This bit clears WDGAF in GXFXTIM_ISR.
    #[inline(always)]
    pub fn cwdgaf(&mut self) -> CWDGAF_W<ICRrs> {
        CWDGAF_W::new(self, 24)
    }
    ///Bit 25 - clear watchdog pre-alarm flag This bit clears WDGPF in GXFXTIM_ISR.
    #[inline(always)]
    pub fn cwdgpf(&mut self) -> CWDGPF_W<ICRrs> {
        CWDGPF_W::new(self, 25)
    }
}
/**GFXTIM interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#GFXTIM:ICR)*/
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
