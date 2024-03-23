#[doc = "Register `ATCR1` reader"]
pub type R = crate::R<ATCR1rs>;
#[doc = "Register `ATCR1` writer"]
pub type W = crate::W<ATCR1rs>;
#[doc = "Field `TAMP1AM` reader - Tamper 1 active mode"]
pub type TAMP1AM_R = crate::BitReader;
#[doc = "Field `TAMP1AM` writer - Tamper 1 active mode"]
pub type TAMP1AM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2AM` reader - Tamper 2 active mode"]
pub type TAMP2AM_R = crate::BitReader;
#[doc = "Field `TAMP2AM` writer - Tamper 2 active mode"]
pub type TAMP2AM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3AM` reader - Tamper 3 active mode"]
pub type TAMP3AM_R = crate::BitReader;
#[doc = "Field `TAMP3AM` writer - Tamper 3 active mode"]
pub type TAMP3AM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP4AM` reader - Tamper 4 active mode"]
pub type TAMP4AM_R = crate::BitReader;
#[doc = "Field `TAMP4AM` writer - Tamper 4 active mode"]
pub type TAMP4AM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP5AM` reader - Tamper 5 active mode"]
pub type TAMP5AM_R = crate::BitReader;
#[doc = "Field `TAMP5AM` writer - Tamper 5 active mode"]
pub type TAMP5AM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP6AM` reader - Tamper 6 active mode"]
pub type TAMP6AM_R = crate::BitReader;
#[doc = "Field `TAMP6AM` writer - Tamper 6 active mode"]
pub type TAMP6AM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP7AM` reader - Tamper 7 active mode"]
pub type TAMP7AM_R = crate::BitReader;
#[doc = "Field `TAMP7AM` writer - Tamper 7 active mode"]
pub type TAMP7AM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP8AM` reader - Tamper 8 active mode"]
pub type TAMP8AM_R = crate::BitReader;
#[doc = "Field `TAMP8AM` writer - Tamper 8 active mode"]
pub type TAMP8AM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATOSEL1` reader - Active tamper shared output 1 selection The selected output must be available in the package pinout"]
pub type ATOSEL1_R = crate::FieldReader;
#[doc = "Field `ATOSEL1` writer - Active tamper shared output 1 selection The selected output must be available in the package pinout"]
pub type ATOSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ATOSEL2` reader - Active tamper shared output 2 selection The selected output must be available in the package pinout"]
pub type ATOSEL2_R = crate::FieldReader;
#[doc = "Field `ATOSEL2` writer - Active tamper shared output 2 selection The selected output must be available in the package pinout"]
pub type ATOSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ATOSEL3` reader - Active tamper shared output 3 selection The selected output must be available in the package pinout"]
pub type ATOSEL3_R = crate::FieldReader;
#[doc = "Field `ATOSEL3` writer - Active tamper shared output 3 selection The selected output must be available in the package pinout"]
pub type ATOSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ATOSEL4` reader - Active tamper shared output 4 selection The selected output must be available in the package pinout."]
pub type ATOSEL4_R = crate::FieldReader;
#[doc = "Field `ATOSEL4` writer - Active tamper shared output 4 selection The selected output must be available in the package pinout."]
pub type ATOSEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ATCKSEL` reader - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output.The selected clock is CK_ATPRE. fCK_ATPRE = fRTCCLK / 2ATCKSEL when (PREDIV_A+1) = 128. ... These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 ck_atpre cycles after all the active tampers are disable."]
pub type ATCKSEL_R = crate::FieldReader;
#[doc = "Field `ATCKSEL` writer - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output.The selected clock is CK_ATPRE. fCK_ATPRE = fRTCCLK / 2ATCKSEL when (PREDIV_A+1) = 128. ... These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 ck_atpre cycles after all the active tampers are disable."]
pub type ATCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ATPER` reader - Active tamper output change period The tamper output is changed every CK_ATPER = (2ATPER x CK_ATPRE) cycles. Refer to ."]
pub type ATPER_R = crate::FieldReader;
#[doc = "Field `ATPER` writer - Active tamper output change period The tamper output is changed every CK_ATPER = (2ATPER x CK_ATPRE) cycles. Refer to ."]
pub type ATPER_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ATOSHARE` reader - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8"]
pub type ATOSHARE_R = crate::BitReader;
#[doc = "Field `ATOSHARE` writer - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8"]
pub type ATOSHARE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTEN` reader - Active tamper filter enable"]
pub type FLTEN_R = crate::BitReader;
#[doc = "Field `FLTEN` writer - Active tamper filter enable"]
pub type FLTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tamper 1 active mode"]
    #[inline(always)]
    pub fn tamp1am(&self) -> TAMP1AM_R {
        TAMP1AM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper 2 active mode"]
    #[inline(always)]
    pub fn tamp2am(&self) -> TAMP2AM_R {
        TAMP2AM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper 3 active mode"]
    #[inline(always)]
    pub fn tamp3am(&self) -> TAMP3AM_R {
        TAMP3AM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper 4 active mode"]
    #[inline(always)]
    pub fn tamp4am(&self) -> TAMP4AM_R {
        TAMP4AM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tamper 5 active mode"]
    #[inline(always)]
    pub fn tamp5am(&self) -> TAMP5AM_R {
        TAMP5AM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tamper 6 active mode"]
    #[inline(always)]
    pub fn tamp6am(&self) -> TAMP6AM_R {
        TAMP6AM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Tamper 7 active mode"]
    #[inline(always)]
    pub fn tamp7am(&self) -> TAMP7AM_R {
        TAMP7AM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tamper 8 active mode"]
    #[inline(always)]
    pub fn tamp8am(&self) -> TAMP8AM_R {
        TAMP8AM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Active tamper shared output 1 selection The selected output must be available in the package pinout"]
    #[inline(always)]
    pub fn atosel1(&self) -> ATOSEL1_R {
        ATOSEL1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Active tamper shared output 2 selection The selected output must be available in the package pinout"]
    #[inline(always)]
    pub fn atosel2(&self) -> ATOSEL2_R {
        ATOSEL2_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Active tamper shared output 3 selection The selected output must be available in the package pinout"]
    #[inline(always)]
    pub fn atosel3(&self) -> ATOSEL3_R {
        ATOSEL3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Active tamper shared output 4 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    pub fn atosel4(&self) -> ATOSEL4_R {
        ATOSEL4_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output.The selected clock is CK_ATPRE. fCK_ATPRE = fRTCCLK / 2ATCKSEL when (PREDIV_A+1) = 128. ... These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 ck_atpre cycles after all the active tampers are disable."]
    #[inline(always)]
    pub fn atcksel(&self) -> ATCKSEL_R {
        ATCKSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Active tamper output change period The tamper output is changed every CK_ATPER = (2ATPER x CK_ATPRE) cycles. Refer to ."]
    #[inline(always)]
    pub fn atper(&self) -> ATPER_R {
        ATPER_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 30 - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8"]
    #[inline(always)]
    pub fn atoshare(&self) -> ATOSHARE_R {
        ATOSHARE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Active tamper filter enable"]
    #[inline(always)]
    pub fn flten(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 1 active mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1am(&mut self) -> TAMP1AM_W<ATCR1rs> {
        TAMP1AM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper 2 active mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2am(&mut self) -> TAMP2AM_W<ATCR1rs> {
        TAMP2AM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper 3 active mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3am(&mut self) -> TAMP3AM_W<ATCR1rs> {
        TAMP3AM_W::new(self, 2)
    }
    #[doc = "Bit 3 - Tamper 4 active mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp4am(&mut self) -> TAMP4AM_W<ATCR1rs> {
        TAMP4AM_W::new(self, 3)
    }
    #[doc = "Bit 4 - Tamper 5 active mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp5am(&mut self) -> TAMP5AM_W<ATCR1rs> {
        TAMP5AM_W::new(self, 4)
    }
    #[doc = "Bit 5 - Tamper 6 active mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp6am(&mut self) -> TAMP6AM_W<ATCR1rs> {
        TAMP6AM_W::new(self, 5)
    }
    #[doc = "Bit 6 - Tamper 7 active mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp7am(&mut self) -> TAMP7AM_W<ATCR1rs> {
        TAMP7AM_W::new(self, 6)
    }
    #[doc = "Bit 7 - Tamper 8 active mode"]
    #[inline(always)]
    #[must_use]
    pub fn tamp8am(&mut self) -> TAMP8AM_W<ATCR1rs> {
        TAMP8AM_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Active tamper shared output 1 selection The selected output must be available in the package pinout"]
    #[inline(always)]
    #[must_use]
    pub fn atosel1(&mut self) -> ATOSEL1_W<ATCR1rs> {
        ATOSEL1_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Active tamper shared output 2 selection The selected output must be available in the package pinout"]
    #[inline(always)]
    #[must_use]
    pub fn atosel2(&mut self) -> ATOSEL2_W<ATCR1rs> {
        ATOSEL2_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Active tamper shared output 3 selection The selected output must be available in the package pinout"]
    #[inline(always)]
    #[must_use]
    pub fn atosel3(&mut self) -> ATOSEL3_W<ATCR1rs> {
        ATOSEL3_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Active tamper shared output 4 selection The selected output must be available in the package pinout."]
    #[inline(always)]
    #[must_use]
    pub fn atosel4(&mut self) -> ATOSEL4_W<ATCR1rs> {
        ATOSEL4_W::new(self, 14)
    }
    #[doc = "Bits 16:18 - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output.The selected clock is CK_ATPRE. fCK_ATPRE = fRTCCLK / 2ATCKSEL when (PREDIV_A+1) = 128. ... These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 ck_atpre cycles after all the active tampers are disable."]
    #[inline(always)]
    #[must_use]
    pub fn atcksel(&mut self) -> ATCKSEL_W<ATCR1rs> {
        ATCKSEL_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Active tamper output change period The tamper output is changed every CK_ATPER = (2ATPER x CK_ATPRE) cycles. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn atper(&mut self) -> ATPER_W<ATCR1rs> {
        ATPER_W::new(self, 24)
    }
    #[doc = "Bit 30 - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8"]
    #[inline(always)]
    #[must_use]
    pub fn atoshare(&mut self) -> ATOSHARE_W<ATCR1rs> {
        ATOSHARE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Active tamper filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn flten(&mut self) -> FLTEN_W<ATCR1rs> {
        FLTEN_W::new(self, 31)
    }
}
#[doc = "TAMP active tamper control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atcr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atcr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATCR1rs;
impl crate::RegisterSpec for ATCR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atcr1::R`](R) reader structure"]
impl crate::Readable for ATCR1rs {}
#[doc = "`write(|w| ..)` method takes [`atcr1::W`](W) writer structure"]
impl crate::Writable for ATCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATCR1 to value 0x0007_0000"]
impl crate::Resettable for ATCR1rs {
    const RESET_VALUE: u32 = 0x0007_0000;
}
