#[doc = "Register `FEEFR3` reader"]
pub type R = crate::R<FEEFR3rs>;
#[doc = "Register `FEEFR3` writer"]
pub type W = crate::W<FEEFR3rs>;
#[doc = "Field `EEVACE` reader - External Event A Counter Enable"]
pub type EEVACE_R = crate::BitReader;
#[doc = "Field `EEVACE` writer - External Event A Counter Enable"]
pub type EEVACE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEVACRES` reader - External Event A Counter Reset"]
pub type EEVACRES_R = crate::BitReader;
#[doc = "Field `EEVACRES` writer - External Event A Counter Reset"]
pub type EEVACRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEVARSTM` reader - External Event A Reset Mode"]
pub type EEVARSTM_R = crate::BitReader;
#[doc = "Field `EEVARSTM` writer - External Event A Reset Mode"]
pub type EEVARSTM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEVASEL` reader - External Event A Selection"]
pub type EEVASEL_R = crate::FieldReader;
#[doc = "Field `EEVASEL` writer - External Event A Selection"]
pub type EEVASEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EEVACNT` reader - External Event A counter"]
pub type EEVACNT_R = crate::FieldReader;
#[doc = "Field `EEVACNT` writer - External Event A counter"]
pub type EEVACNT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - External Event A Counter Enable"]
    #[inline(always)]
    pub fn eevace(&self) -> EEVACE_R {
        EEVACE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Event A Counter Reset"]
    #[inline(always)]
    pub fn eevacres(&self) -> EEVACRES_R {
        EEVACRES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Event A Reset Mode"]
    #[inline(always)]
    pub fn eevarstm(&self) -> EEVARSTM_R {
        EEVARSTM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - External Event A Selection"]
    #[inline(always)]
    pub fn eevasel(&self) -> EEVASEL_R {
        EEVASEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - External Event A counter"]
    #[inline(always)]
    pub fn eevacnt(&self) -> EEVACNT_R {
        EEVACNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Event A Counter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eevace(&mut self) -> EEVACE_W<FEEFR3rs> {
        EEVACE_W::new(self, 0)
    }
    #[doc = "Bit 1 - External Event A Counter Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eevacres(&mut self) -> EEVACRES_W<FEEFR3rs> {
        EEVACRES_W::new(self, 1)
    }
    #[doc = "Bit 2 - External Event A Reset Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eevarstm(&mut self) -> EEVARSTM_W<FEEFR3rs> {
        EEVARSTM_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - External Event A Selection"]
    #[inline(always)]
    #[must_use]
    pub fn eevasel(&mut self) -> EEVASEL_W<FEEFR3rs> {
        EEVASEL_W::new(self, 4)
    }
    #[doc = "Bits 8:13 - External Event A counter"]
    #[inline(always)]
    #[must_use]
    pub fn eevacnt(&mut self) -> EEVACNT_W<FEEFR3rs> {
        EEVACNT_W::new(self, 8)
    }
}
#[doc = "HRTIM Timerx External Event Filtering Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`feefr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`feefr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FEEFR3rs;
impl crate::RegisterSpec for FEEFR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`feefr3::R`](R) reader structure"]
impl crate::Readable for FEEFR3rs {}
#[doc = "`write(|w| ..)` method takes [`feefr3::W`](W) writer structure"]
impl crate::Writable for FEEFR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEEFR3 to value 0"]
impl crate::Resettable for FEEFR3rs {
    const RESET_VALUE: u32 = 0;
}
