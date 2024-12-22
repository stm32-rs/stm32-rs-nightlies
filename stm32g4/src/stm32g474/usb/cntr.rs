///Register `CNTR` reader
pub type R = crate::R<CNTRrs>;
///Register `CNTR` writer
pub type W = crate::W<CNTRrs>;
///Field `FRES` reader - FRES
pub type FRES_R = crate::BitReader;
///Field `FRES` writer - FRES
pub type FRES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDWN` reader - PDWN
pub type PDWN_R = crate::BitReader;
///Field `PDWN` writer - PDWN
pub type PDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_MODE` reader - LP_MODE
pub type LP_MODE_R = crate::BitReader;
///Field `LP_MODE` writer - LP_MODE
pub type LP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSUSP` reader - FSUSP
pub type FSUSP_R = crate::BitReader;
///Field `FSUSP` writer - FSUSP
pub type FSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESUME` reader - RESUME
pub type RESUME_R = crate::BitReader;
///Field `RESUME` writer - RESUME
pub type RESUME_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1RESUME` reader - L1RESUME
pub type L1RESUME_R = crate::BitReader;
///Field `L1RESUME` writer - L1RESUME
pub type L1RESUME_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1REQM` reader - L1REQM
pub type L1REQM_R = crate::BitReader;
///Field `L1REQM` writer - L1REQM
pub type L1REQM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ESOFM` reader - ESOFM
pub type ESOFM_R = crate::BitReader;
///Field `ESOFM` writer - ESOFM
pub type ESOFM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOFM` reader - SOFM
pub type SOFM_R = crate::BitReader;
///Field `SOFM` writer - SOFM
pub type SOFM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESETM` reader - RESETM
pub type RESETM_R = crate::BitReader;
///Field `RESETM` writer - RESETM
pub type RESETM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUSPM` reader - SUSPM
pub type SUSPM_R = crate::BitReader;
///Field `SUSPM` writer - SUSPM
pub type SUSPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPM` reader - WKUPM
pub type WKUPM_R = crate::BitReader;
///Field `WKUPM` writer - WKUPM
pub type WKUPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRM` reader - ERRM
pub type ERRM_R = crate::BitReader;
///Field `ERRM` writer - ERRM
pub type ERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PMAOVRM` reader - PMAOVRM
pub type PMAOVRM_R = crate::BitReader;
///Field `PMAOVRM` writer - PMAOVRM
pub type PMAOVRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTRM` reader - CTRM
pub type CTRM_R = crate::BitReader;
///Field `CTRM` writer - CTRM
pub type CTRM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - FRES
    #[inline(always)]
    pub fn fres(&self) -> FRES_R {
        FRES_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PDWN
    #[inline(always)]
    pub fn pdwn(&self) -> PDWN_R {
        PDWN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LP_MODE
    #[inline(always)]
    pub fn lp_mode(&self) -> LP_MODE_R {
        LP_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FSUSP
    #[inline(always)]
    pub fn fsusp(&self) -> FSUSP_R {
        FSUSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RESUME
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - L1RESUME
    #[inline(always)]
    pub fn l1resume(&self) -> L1RESUME_R {
        L1RESUME_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - L1REQM
    #[inline(always)]
    pub fn l1reqm(&self) -> L1REQM_R {
        L1REQM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ESOFM
    #[inline(always)]
    pub fn esofm(&self) -> ESOFM_R {
        ESOFM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SOFM
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RESETM
    #[inline(always)]
    pub fn resetm(&self) -> RESETM_R {
        RESETM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SUSPM
    #[inline(always)]
    pub fn suspm(&self) -> SUSPM_R {
        SUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - WKUPM
    #[inline(always)]
    pub fn wkupm(&self) -> WKUPM_R {
        WKUPM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ERRM
    #[inline(always)]
    pub fn errm(&self) -> ERRM_R {
        ERRM_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - PMAOVRM
    #[inline(always)]
    pub fn pmaovrm(&self) -> PMAOVRM_R {
        PMAOVRM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CTRM
    #[inline(always)]
    pub fn ctrm(&self) -> CTRM_R {
        CTRM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTR")
            .field("fres", &self.fres())
            .field("pdwn", &self.pdwn())
            .field("lp_mode", &self.lp_mode())
            .field("fsusp", &self.fsusp())
            .field("resume", &self.resume())
            .field("l1resume", &self.l1resume())
            .field("l1reqm", &self.l1reqm())
            .field("esofm", &self.esofm())
            .field("sofm", &self.sofm())
            .field("resetm", &self.resetm())
            .field("suspm", &self.suspm())
            .field("wkupm", &self.wkupm())
            .field("errm", &self.errm())
            .field("pmaovrm", &self.pmaovrm())
            .field("ctrm", &self.ctrm())
            .finish()
    }
}
impl W {
    ///Bit 0 - FRES
    #[inline(always)]
    pub fn fres(&mut self) -> FRES_W<CNTRrs> {
        FRES_W::new(self, 0)
    }
    ///Bit 1 - PDWN
    #[inline(always)]
    pub fn pdwn(&mut self) -> PDWN_W<CNTRrs> {
        PDWN_W::new(self, 1)
    }
    ///Bit 2 - LP_MODE
    #[inline(always)]
    pub fn lp_mode(&mut self) -> LP_MODE_W<CNTRrs> {
        LP_MODE_W::new(self, 2)
    }
    ///Bit 3 - FSUSP
    #[inline(always)]
    pub fn fsusp(&mut self) -> FSUSP_W<CNTRrs> {
        FSUSP_W::new(self, 3)
    }
    ///Bit 4 - RESUME
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W<CNTRrs> {
        RESUME_W::new(self, 4)
    }
    ///Bit 5 - L1RESUME
    #[inline(always)]
    pub fn l1resume(&mut self) -> L1RESUME_W<CNTRrs> {
        L1RESUME_W::new(self, 5)
    }
    ///Bit 7 - L1REQM
    #[inline(always)]
    pub fn l1reqm(&mut self) -> L1REQM_W<CNTRrs> {
        L1REQM_W::new(self, 7)
    }
    ///Bit 8 - ESOFM
    #[inline(always)]
    pub fn esofm(&mut self) -> ESOFM_W<CNTRrs> {
        ESOFM_W::new(self, 8)
    }
    ///Bit 9 - SOFM
    #[inline(always)]
    pub fn sofm(&mut self) -> SOFM_W<CNTRrs> {
        SOFM_W::new(self, 9)
    }
    ///Bit 10 - RESETM
    #[inline(always)]
    pub fn resetm(&mut self) -> RESETM_W<CNTRrs> {
        RESETM_W::new(self, 10)
    }
    ///Bit 11 - SUSPM
    #[inline(always)]
    pub fn suspm(&mut self) -> SUSPM_W<CNTRrs> {
        SUSPM_W::new(self, 11)
    }
    ///Bit 12 - WKUPM
    #[inline(always)]
    pub fn wkupm(&mut self) -> WKUPM_W<CNTRrs> {
        WKUPM_W::new(self, 12)
    }
    ///Bit 13 - ERRM
    #[inline(always)]
    pub fn errm(&mut self) -> ERRM_W<CNTRrs> {
        ERRM_W::new(self, 13)
    }
    ///Bit 14 - PMAOVRM
    #[inline(always)]
    pub fn pmaovrm(&mut self) -> PMAOVRM_W<CNTRrs> {
        PMAOVRM_W::new(self, 14)
    }
    ///Bit 15 - CTRM
    #[inline(always)]
    pub fn ctrm(&mut self) -> CTRM_W<CNTRrs> {
        CTRM_W::new(self, 15)
    }
}
/**USB control register

You can [`read`](crate::Reg::read) this register and get [`cntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#USB:CNTR)*/
pub struct CNTRrs;
impl crate::RegisterSpec for CNTRrs {
    type Ux = u32;
}
///`read()` method returns [`cntr::R`](R) reader structure
impl crate::Readable for CNTRrs {}
///`write(|w| ..)` method takes [`cntr::W`](W) writer structure
impl crate::Writable for CNTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CNTR to value 0
impl crate::Resettable for CNTRrs {
    const RESET_VALUE: u32 = 0;
}
