#[doc = "Register `CNTR` reader"]
pub type R = crate::R<CNTRrs>;
#[doc = "Register `CNTR` writer"]
pub type W = crate::W<CNTRrs>;
#[doc = "Field `FRES` reader - Force USB Reset"]
pub type FRES_R = crate::BitReader;
#[doc = "Field `FRES` writer - Force USB Reset"]
pub type FRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDWN` reader - Power down"]
pub type PDWN_R = crate::BitReader;
#[doc = "Field `PDWN` writer - Power down"]
pub type PDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMODE` reader - Low-power mode"]
pub type LPMODE_R = crate::BitReader;
#[doc = "Field `LPMODE` writer - Low-power mode"]
pub type LPMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSUSP` reader - Force suspend"]
pub type FSUSP_R = crate::BitReader;
#[doc = "Field `FSUSP` writer - Force suspend"]
pub type FSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME` reader - Resume request"]
pub type RESUME_R = crate::BitReader;
#[doc = "Field `RESUME` writer - Resume request"]
pub type RESUME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1RESUME` reader - LPM L1 Resume request"]
pub type L1RESUME_R = crate::BitReader;
#[doc = "Field `L1RESUME` writer - LPM L1 Resume request"]
pub type L1RESUME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1REQM` reader - LPM L1 state request interrupt mask"]
pub type L1REQM_R = crate::BitReader;
#[doc = "Field `L1REQM` writer - LPM L1 state request interrupt mask"]
pub type L1REQM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESOFM` reader - Expected start of frame interrupt mask"]
pub type ESOFM_R = crate::BitReader;
#[doc = "Field `ESOFM` writer - Expected start of frame interrupt mask"]
pub type ESOFM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFM` reader - Start of frame interrupt mask"]
pub type SOFM_R = crate::BitReader;
#[doc = "Field `SOFM` writer - Start of frame interrupt mask"]
pub type SOFM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETM` reader - USB reset interrupt mask"]
pub type RESETM_R = crate::BitReader;
#[doc = "Field `RESETM` writer - USB reset interrupt mask"]
pub type RESETM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPM` reader - Suspend mode interrupt mask"]
pub type SUSPM_R = crate::BitReader;
#[doc = "Field `SUSPM` writer - Suspend mode interrupt mask"]
pub type SUSPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPM` reader - Wakeup interrupt mask"]
pub type WKUPM_R = crate::BitReader;
#[doc = "Field `WKUPM` writer - Wakeup interrupt mask"]
pub type WKUPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRM` reader - Error interrupt mask"]
pub type ERRM_R = crate::BitReader;
#[doc = "Field `ERRM` writer - Error interrupt mask"]
pub type ERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMAOVRM` reader - Packet memory area over / underrun interrupt mask"]
pub type PMAOVRM_R = crate::BitReader;
#[doc = "Field `PMAOVRM` writer - Packet memory area over / underrun interrupt mask"]
pub type PMAOVRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRM` reader - Correct transfer interrupt mask"]
pub type CTRM_R = crate::BitReader;
#[doc = "Field `CTRM` writer - Correct transfer interrupt mask"]
pub type CTRM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Force USB Reset"]
    #[inline(always)]
    pub fn fres(&self) -> FRES_R {
        FRES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down"]
    #[inline(always)]
    pub fn pdwn(&self) -> PDWN_R {
        PDWN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low-power mode"]
    #[inline(always)]
    pub fn lpmode(&self) -> LPMODE_R {
        LPMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force suspend"]
    #[inline(always)]
    pub fn fsusp(&self) -> FSUSP_R {
        FSUSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Resume request"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPM L1 Resume request"]
    #[inline(always)]
    pub fn l1resume(&self) -> L1RESUME_R {
        L1RESUME_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - LPM L1 state request interrupt mask"]
    #[inline(always)]
    pub fn l1reqm(&self) -> L1REQM_R {
        L1REQM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline(always)]
    pub fn esofm(&self) -> ESOFM_R {
        ESOFM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB reset interrupt mask"]
    #[inline(always)]
    pub fn resetm(&self) -> RESETM_R {
        RESETM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    pub fn suspm(&self) -> SUSPM_R {
        SUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    pub fn wkupm(&self) -> WKUPM_R {
        WKUPM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    pub fn errm(&self) -> ERRM_R {
        ERRM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline(always)]
    pub fn pmaovrm(&self) -> PMAOVRM_R {
        PMAOVRM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline(always)]
    pub fn ctrm(&self) -> CTRM_R {
        CTRM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force USB Reset"]
    #[inline(always)]
    #[must_use]
    pub fn fres(&mut self) -> FRES_W<CNTRrs> {
        FRES_W::new(self, 0)
    }
    #[doc = "Bit 1 - Power down"]
    #[inline(always)]
    #[must_use]
    pub fn pdwn(&mut self) -> PDWN_W<CNTRrs> {
        PDWN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Low-power mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpmode(&mut self) -> LPMODE_W<CNTRrs> {
        LPMODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Force suspend"]
    #[inline(always)]
    #[must_use]
    pub fn fsusp(&mut self) -> FSUSP_W<CNTRrs> {
        FSUSP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Resume request"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<CNTRrs> {
        RESUME_W::new(self, 4)
    }
    #[doc = "Bit 5 - LPM L1 Resume request"]
    #[inline(always)]
    #[must_use]
    pub fn l1resume(&mut self) -> L1RESUME_W<CNTRrs> {
        L1RESUME_W::new(self, 5)
    }
    #[doc = "Bit 7 - LPM L1 state request interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn l1reqm(&mut self) -> L1REQM_W<CNTRrs> {
        L1REQM_W::new(self, 7)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn esofm(&mut self) -> ESOFM_W<CNTRrs> {
        ESOFM_W::new(self, 8)
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn sofm(&mut self) -> SOFM_W<CNTRrs> {
        SOFM_W::new(self, 9)
    }
    #[doc = "Bit 10 - USB reset interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn resetm(&mut self) -> RESETM_W<CNTRrs> {
        RESETM_W::new(self, 10)
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn suspm(&mut self) -> SUSPM_W<CNTRrs> {
        SUSPM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn wkupm(&mut self) -> WKUPM_W<CNTRrs> {
        WKUPM_W::new(self, 12)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn errm(&mut self) -> ERRM_W<CNTRrs> {
        ERRM_W::new(self, 13)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn pmaovrm(&mut self) -> PMAOVRM_W<CNTRrs> {
        PMAOVRM_W::new(self, 14)
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ctrm(&mut self) -> CTRM_W<CNTRrs> {
        CTRM_W::new(self, 15)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTRrs;
impl crate::RegisterSpec for CNTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntr::R`](R) reader structure"]
impl crate::Readable for CNTRrs {}
#[doc = "`write(|w| ..)` method takes [`cntr::W`](W) writer structure"]
impl crate::Writable for CNTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTR to value 0x03"]
impl crate::Resettable for CNTRrs {
    const RESET_VALUE: u32 = 0x03;
}
