#[doc = "Register `SDTR2` reader"]
pub type R = crate::R<SDTR2rs>;
#[doc = "Register `SDTR2` writer"]
pub type W = crate::W<SDTR2rs>;
#[doc = "Field `TMRD` reader - Load Mode Register to Active"]
pub type TMRD_R = crate::FieldReader;
#[doc = "Field `TMRD` writer - Load Mode Register to Active"]
pub type TMRD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `TXSR` reader - Exit self-refresh delay"]
pub type TXSR_R = crate::FieldReader;
#[doc = "Field `TXSR` writer - Exit self-refresh delay"]
pub type TXSR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `TRAS` reader - Self refresh time"]
pub type TRAS_R = crate::FieldReader;
#[doc = "Field `TRAS` writer - Self refresh time"]
pub type TRAS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `TRC` reader - Row cycle delay"]
pub type TRC_R = crate::FieldReader;
#[doc = "Field `TRC` writer - Row cycle delay"]
pub type TRC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `TWR` reader - Recovery delay"]
pub type TWR_R = crate::FieldReader;
#[doc = "Field `TWR` writer - Recovery delay"]
pub type TWR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `TRP` reader - Row precharge delay"]
pub type TRP_R = crate::FieldReader;
#[doc = "Field `TRP` writer - Row precharge delay"]
pub type TRP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `TRCD` reader - Row to column delay"]
pub type TRCD_R = crate::FieldReader;
#[doc = "Field `TRCD` writer - Row to column delay"]
pub type TRCD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Load Mode Register to Active"]
    #[inline(always)]
    pub fn tmrd(&self) -> TMRD_R {
        TMRD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Exit self-refresh delay"]
    #[inline(always)]
    pub fn txsr(&self) -> TXSR_R {
        TXSR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Self refresh time"]
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Row cycle delay"]
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Recovery delay"]
    #[inline(always)]
    pub fn twr(&self) -> TWR_R {
        TWR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Row precharge delay"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Row to column delay"]
    #[inline(always)]
    pub fn trcd(&self) -> TRCD_R {
        TRCD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Load Mode Register to Active"]
    #[inline(always)]
    #[must_use]
    pub fn tmrd(&mut self) -> TMRD_W<SDTR2rs> {
        TMRD_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Exit self-refresh delay"]
    #[inline(always)]
    #[must_use]
    pub fn txsr(&mut self) -> TXSR_W<SDTR2rs> {
        TXSR_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Self refresh time"]
    #[inline(always)]
    #[must_use]
    pub fn tras(&mut self) -> TRAS_W<SDTR2rs> {
        TRAS_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Row cycle delay"]
    #[inline(always)]
    #[must_use]
    pub fn trc(&mut self) -> TRC_W<SDTR2rs> {
        TRC_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Recovery delay"]
    #[inline(always)]
    #[must_use]
    pub fn twr(&mut self) -> TWR_W<SDTR2rs> {
        TWR_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Row precharge delay"]
    #[inline(always)]
    #[must_use]
    pub fn trp(&mut self) -> TRP_W<SDTR2rs> {
        TRP_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Row to column delay"]
    #[inline(always)]
    #[must_use]
    pub fn trcd(&mut self) -> TRCD_W<SDTR2rs> {
        TRCD_W::new(self, 24)
    }
}
#[doc = "SDRAM Timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdtr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdtr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDTR2rs;
impl crate::RegisterSpec for SDTR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdtr2::R`](R) reader structure"]
impl crate::Readable for SDTR2rs {}
#[doc = "`write(|w| ..)` method takes [`sdtr2::W`](W) writer structure"]
impl crate::Writable for SDTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDTR2 to value 0x0fff_ffff"]
impl crate::Resettable for SDTR2rs {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
