///Register `ATCR1` reader
pub type R = crate::R<ATCR1rs>;
///Register `ATCR1` writer
pub type W = crate::W<ATCR1rs>;
///Field `TAMP1AM` reader - Tamper 1 active mode
pub type TAMP1AM_R = crate::BitReader;
///Field `TAMP1AM` writer - Tamper 1 active mode
pub type TAMP1AM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2AM` reader - Tamper 2 active mode
pub type TAMP2AM_R = crate::BitReader;
///Field `TAMP2AM` writer - Tamper 2 active mode
pub type TAMP2AM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3AM` reader - Tamper 3 active mode
pub type TAMP3AM_R = crate::BitReader;
///Field `TAMP3AM` writer - Tamper 3 active mode
pub type TAMP3AM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP4AM` reader - Tamper 4 active mode
pub type TAMP4AM_R = crate::BitReader;
///Field `TAMP4AM` writer - Tamper 4 active mode
pub type TAMP4AM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP5AM` reader - Tamper 5 active mode
pub type TAMP5AM_R = crate::BitReader;
///Field `TAMP5AM` writer - Tamper 5 active mode
pub type TAMP5AM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP6AM` reader - Tamper 6 active mode
pub type TAMP6AM_R = crate::BitReader;
///Field `TAMP6AM` writer - Tamper 6 active mode
pub type TAMP6AM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ATOSEL1` reader - Active tamper shared output 1 selection The selected output must be available in the package pinout
pub type ATOSEL1_R = crate::FieldReader;
///Field `ATOSEL1` writer - Active tamper shared output 1 selection The selected output must be available in the package pinout
pub type ATOSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ATOSEL2` reader - Active tamper shared output 2 selection The selected output must be available in the package pinout
pub type ATOSEL2_R = crate::FieldReader;
///Field `ATOSEL2` writer - Active tamper shared output 2 selection The selected output must be available in the package pinout
pub type ATOSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ATOSEL3` reader - Active tamper shared output 3 selection The selected output must be available in the package pinout
pub type ATOSEL3_R = crate::FieldReader;
///Field `ATOSEL3` writer - Active tamper shared output 3 selection The selected output must be available in the package pinout
pub type ATOSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ATOSEL4` reader - Active tamper shared output 4 selection The selected output must be available in the package pinout.
pub type ATOSEL4_R = crate::FieldReader;
///Field `ATOSEL4` writer - Active tamper shared output 4 selection The selected output must be available in the package pinout.
pub type ATOSEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ATCKSEL` reader - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output. The selected clock is CK_ATPRE. ... Note: These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 CK_ATPRE cycles after all the active tampers are disable.
pub type ATCKSEL_R = crate::FieldReader;
///Field `ATCKSEL` writer - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output. The selected clock is CK_ATPRE. ... Note: These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 CK_ATPRE cycles after all the active tampers are disable.
pub type ATCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ATPER` reader - Active tamper output change period The tamper output is changed every CK_ATPER = (2supATPER /supx CK_ATPRE) cycles. Refer to Table713: Minimum ATPER value.
pub type ATPER_R = crate::FieldReader;
///Field `ATPER` writer - Active tamper output change period The tamper output is changed every CK_ATPER = (2supATPER /supx CK_ATPRE) cycles. Refer to Table713: Minimum ATPER value.
pub type ATPER_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ATOSHARE` reader - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6
pub type ATOSHARE_R = crate::BitReader;
///Field `ATOSHARE` writer - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6
pub type ATOSHARE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLTEN` reader - Active tamper filter enable
pub type FLTEN_R = crate::BitReader;
///Field `FLTEN` writer - Active tamper filter enable
pub type FLTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Tamper 1 active mode
    #[inline(always)]
    pub fn tamp1am(&self) -> TAMP1AM_R {
        TAMP1AM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tamper 2 active mode
    #[inline(always)]
    pub fn tamp2am(&self) -> TAMP2AM_R {
        TAMP2AM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Tamper 3 active mode
    #[inline(always)]
    pub fn tamp3am(&self) -> TAMP3AM_R {
        TAMP3AM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Tamper 4 active mode
    #[inline(always)]
    pub fn tamp4am(&self) -> TAMP4AM_R {
        TAMP4AM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Tamper 5 active mode
    #[inline(always)]
    pub fn tamp5am(&self) -> TAMP5AM_R {
        TAMP5AM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Tamper 6 active mode
    #[inline(always)]
    pub fn tamp6am(&self) -> TAMP6AM_R {
        TAMP6AM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:9 - Active tamper shared output 1 selection The selected output must be available in the package pinout
    #[inline(always)]
    pub fn atosel1(&self) -> ATOSEL1_R {
        ATOSEL1_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Active tamper shared output 2 selection The selected output must be available in the package pinout
    #[inline(always)]
    pub fn atosel2(&self) -> ATOSEL2_R {
        ATOSEL2_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Active tamper shared output 3 selection The selected output must be available in the package pinout
    #[inline(always)]
    pub fn atosel3(&self) -> ATOSEL3_R {
        ATOSEL3_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Active tamper shared output 4 selection The selected output must be available in the package pinout.
    #[inline(always)]
    pub fn atosel4(&self) -> ATOSEL4_R {
        ATOSEL4_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:18 - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output. The selected clock is CK_ATPRE. ... Note: These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 CK_ATPRE cycles after all the active tampers are disable.
    #[inline(always)]
    pub fn atcksel(&self) -> ATCKSEL_R {
        ATCKSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 24:26 - Active tamper output change period The tamper output is changed every CK_ATPER = (2supATPER /supx CK_ATPRE) cycles. Refer to Table713: Minimum ATPER value.
    #[inline(always)]
    pub fn atper(&self) -> ATPER_R {
        ATPER_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 30 - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6
    #[inline(always)]
    pub fn atoshare(&self) -> ATOSHARE_R {
        ATOSHARE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Active tamper filter enable
    #[inline(always)]
    pub fn flten(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ATCR1")
            .field("tamp1am", &self.tamp1am())
            .field("tamp2am", &self.tamp2am())
            .field("tamp3am", &self.tamp3am())
            .field("tamp4am", &self.tamp4am())
            .field("tamp5am", &self.tamp5am())
            .field("tamp6am", &self.tamp6am())
            .field("atosel1", &self.atosel1())
            .field("atosel2", &self.atosel2())
            .field("atosel3", &self.atosel3())
            .field("atosel4", &self.atosel4())
            .field("atcksel", &self.atcksel())
            .field("atper", &self.atper())
            .field("atoshare", &self.atoshare())
            .field("flten", &self.flten())
            .finish()
    }
}
impl W {
    ///Bit 0 - Tamper 1 active mode
    #[inline(always)]
    pub fn tamp1am(&mut self) -> TAMP1AM_W<ATCR1rs> {
        TAMP1AM_W::new(self, 0)
    }
    ///Bit 1 - Tamper 2 active mode
    #[inline(always)]
    pub fn tamp2am(&mut self) -> TAMP2AM_W<ATCR1rs> {
        TAMP2AM_W::new(self, 1)
    }
    ///Bit 2 - Tamper 3 active mode
    #[inline(always)]
    pub fn tamp3am(&mut self) -> TAMP3AM_W<ATCR1rs> {
        TAMP3AM_W::new(self, 2)
    }
    ///Bit 3 - Tamper 4 active mode
    #[inline(always)]
    pub fn tamp4am(&mut self) -> TAMP4AM_W<ATCR1rs> {
        TAMP4AM_W::new(self, 3)
    }
    ///Bit 4 - Tamper 5 active mode
    #[inline(always)]
    pub fn tamp5am(&mut self) -> TAMP5AM_W<ATCR1rs> {
        TAMP5AM_W::new(self, 4)
    }
    ///Bit 5 - Tamper 6 active mode
    #[inline(always)]
    pub fn tamp6am(&mut self) -> TAMP6AM_W<ATCR1rs> {
        TAMP6AM_W::new(self, 5)
    }
    ///Bits 8:9 - Active tamper shared output 1 selection The selected output must be available in the package pinout
    #[inline(always)]
    pub fn atosel1(&mut self) -> ATOSEL1_W<ATCR1rs> {
        ATOSEL1_W::new(self, 8)
    }
    ///Bits 10:11 - Active tamper shared output 2 selection The selected output must be available in the package pinout
    #[inline(always)]
    pub fn atosel2(&mut self) -> ATOSEL2_W<ATCR1rs> {
        ATOSEL2_W::new(self, 10)
    }
    ///Bits 12:13 - Active tamper shared output 3 selection The selected output must be available in the package pinout
    #[inline(always)]
    pub fn atosel3(&mut self) -> ATOSEL3_W<ATCR1rs> {
        ATOSEL3_W::new(self, 12)
    }
    ///Bits 14:15 - Active tamper shared output 4 selection The selected output must be available in the package pinout.
    #[inline(always)]
    pub fn atosel4(&mut self) -> ATOSEL4_W<ATCR1rs> {
        ATOSEL4_W::new(self, 14)
    }
    ///Bits 16:18 - Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output. The selected clock is CK_ATPRE. ... Note: These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 CK_ATPRE cycles after all the active tampers are disable.
    #[inline(always)]
    pub fn atcksel(&mut self) -> ATCKSEL_W<ATCR1rs> {
        ATCKSEL_W::new(self, 16)
    }
    ///Bits 24:26 - Active tamper output change period The tamper output is changed every CK_ATPER = (2supATPER /supx CK_ATPRE) cycles. Refer to Table713: Minimum ATPER value.
    #[inline(always)]
    pub fn atper(&mut self) -> ATPER_W<ATCR1rs> {
        ATPER_W::new(self, 24)
    }
    ///Bit 30 - Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6
    #[inline(always)]
    pub fn atoshare(&mut self) -> ATOSHARE_W<ATCR1rs> {
        ATOSHARE_W::new(self, 30)
    }
    ///Bit 31 - Active tamper filter enable
    #[inline(always)]
    pub fn flten(&mut self) -> FLTEN_W<ATCR1rs> {
        FLTEN_W::new(self, 31)
    }
}
/**TAMP active tamper control register 1

You can [`read`](crate::Reg::read) this register and get [`atcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#TAMP:ATCR1)*/
pub struct ATCR1rs;
impl crate::RegisterSpec for ATCR1rs {
    type Ux = u32;
}
///`read()` method returns [`atcr1::R`](R) reader structure
impl crate::Readable for ATCR1rs {}
///`write(|w| ..)` method takes [`atcr1::W`](W) writer structure
impl crate::Writable for ATCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ATCR1 to value 0x0007_0000
impl crate::Resettable for ATCR1rs {
    const RESET_VALUE: u32 = 0x0007_0000;
}
