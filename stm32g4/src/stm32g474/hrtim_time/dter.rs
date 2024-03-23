#[doc = "Register `DTER` reader"]
pub type R = crate::R<DTERrs>;
#[doc = "Register `DTER` writer"]
pub type W = crate::W<DTERrs>;
#[doc = "Field `DTRx` reader - Deadtime Rising value"]
pub type DTRX_R = crate::FieldReader<u16>;
#[doc = "Field `DTRx` writer - Deadtime Rising value"]
pub type DTRX_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SDTRx` reader - Sign Deadtime Rising value"]
pub type SDTRX_R = crate::BitReader;
#[doc = "Field `SDTRx` writer - Sign Deadtime Rising value"]
pub type SDTRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTPRSC` reader - Deadtime Prescaler"]
pub type DTPRSC_R = crate::FieldReader;
#[doc = "Field `DTPRSC` writer - Deadtime Prescaler"]
pub type DTPRSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DTRSLKx` reader - Deadtime Rising Sign Lock"]
pub type DTRSLKX_R = crate::BitReader;
#[doc = "Field `DTRSLKx` writer - Deadtime Rising Sign Lock"]
pub type DTRSLKX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTRLKx` reader - Deadtime Rising Lock"]
pub type DTRLKX_R = crate::BitReader;
#[doc = "Field `DTRLKx` writer - Deadtime Rising Lock"]
pub type DTRLKX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFx` reader - Deadtime Falling value"]
pub type DTFX_R = crate::FieldReader<u16>;
#[doc = "Field `DTFx` writer - Deadtime Falling value"]
pub type DTFX_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SDTFx` reader - Sign Deadtime Falling value"]
pub type SDTFX_R = crate::BitReader;
#[doc = "Field `SDTFx` writer - Sign Deadtime Falling value"]
pub type SDTFX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFSLKx` reader - Deadtime Falling Sign Lock"]
pub type DTFSLKX_R = crate::BitReader;
#[doc = "Field `DTFSLKx` writer - Deadtime Falling Sign Lock"]
pub type DTFSLKX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFLKx` reader - Deadtime Falling Lock"]
pub type DTFLKX_R = crate::BitReader;
#[doc = "Field `DTFLKx` writer - Deadtime Falling Lock"]
pub type DTFLKX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Deadtime Rising value"]
    #[inline(always)]
    pub fn dtrx(&self) -> DTRX_R {
        DTRX_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Sign Deadtime Rising value"]
    #[inline(always)]
    pub fn sdtrx(&self) -> SDTRX_R {
        SDTRX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Deadtime Prescaler"]
    #[inline(always)]
    pub fn dtprsc(&self) -> DTPRSC_R {
        DTPRSC_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 14 - Deadtime Rising Sign Lock"]
    #[inline(always)]
    pub fn dtrslkx(&self) -> DTRSLKX_R {
        DTRSLKX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Deadtime Rising Lock"]
    #[inline(always)]
    pub fn dtrlkx(&self) -> DTRLKX_R {
        DTRLKX_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:24 - Deadtime Falling value"]
    #[inline(always)]
    pub fn dtfx(&self) -> DTFX_R {
        DTFX_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 25 - Sign Deadtime Falling value"]
    #[inline(always)]
    pub fn sdtfx(&self) -> SDTFX_R {
        SDTFX_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - Deadtime Falling Sign Lock"]
    #[inline(always)]
    pub fn dtfslkx(&self) -> DTFSLKX_R {
        DTFSLKX_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Deadtime Falling Lock"]
    #[inline(always)]
    pub fn dtflkx(&self) -> DTFLKX_R {
        DTFLKX_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Deadtime Rising value"]
    #[inline(always)]
    #[must_use]
    pub fn dtrx(&mut self) -> DTRX_W<DTERrs> {
        DTRX_W::new(self, 0)
    }
    #[doc = "Bit 9 - Sign Deadtime Rising value"]
    #[inline(always)]
    #[must_use]
    pub fn sdtrx(&mut self) -> SDTRX_W<DTERrs> {
        SDTRX_W::new(self, 9)
    }
    #[doc = "Bits 10:12 - Deadtime Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn dtprsc(&mut self) -> DTPRSC_W<DTERrs> {
        DTPRSC_W::new(self, 10)
    }
    #[doc = "Bit 14 - Deadtime Rising Sign Lock"]
    #[inline(always)]
    #[must_use]
    pub fn dtrslkx(&mut self) -> DTRSLKX_W<DTERrs> {
        DTRSLKX_W::new(self, 14)
    }
    #[doc = "Bit 15 - Deadtime Rising Lock"]
    #[inline(always)]
    #[must_use]
    pub fn dtrlkx(&mut self) -> DTRLKX_W<DTERrs> {
        DTRLKX_W::new(self, 15)
    }
    #[doc = "Bits 16:24 - Deadtime Falling value"]
    #[inline(always)]
    #[must_use]
    pub fn dtfx(&mut self) -> DTFX_W<DTERrs> {
        DTFX_W::new(self, 16)
    }
    #[doc = "Bit 25 - Sign Deadtime Falling value"]
    #[inline(always)]
    #[must_use]
    pub fn sdtfx(&mut self) -> SDTFX_W<DTERrs> {
        SDTFX_W::new(self, 25)
    }
    #[doc = "Bit 30 - Deadtime Falling Sign Lock"]
    #[inline(always)]
    #[must_use]
    pub fn dtfslkx(&mut self) -> DTFSLKX_W<DTERrs> {
        DTFSLKX_W::new(self, 30)
    }
    #[doc = "Bit 31 - Deadtime Falling Lock"]
    #[inline(always)]
    #[must_use]
    pub fn dtflkx(&mut self) -> DTFLKX_W<DTERrs> {
        DTFLKX_W::new(self, 31)
    }
}
#[doc = "Timerx Deadtime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTERrs;
impl crate::RegisterSpec for DTERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dter::R`](R) reader structure"]
impl crate::Readable for DTERrs {}
#[doc = "`write(|w| ..)` method takes [`dter::W`](W) writer structure"]
impl crate::Writable for DTERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTER to value 0"]
impl crate::Resettable for DTERrs {
    const RESET_VALUE: u32 = 0;
}
