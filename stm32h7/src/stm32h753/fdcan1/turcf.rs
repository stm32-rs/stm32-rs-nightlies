#[doc = "Register `TURCF` reader"]
pub type R = crate::R<TURCFrs>;
#[doc = "Register `TURCF` writer"]
pub type W = crate::W<TURCFrs>;
#[doc = "Field `NCL` reader - Numerator Configuration Low."]
pub type NCL_R = crate::FieldReader<u16>;
#[doc = "Field `NCL` writer - Numerator Configuration Low."]
pub type NCL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DC` reader - Denominator Configuration."]
pub type DC_R = crate::FieldReader<u16>;
#[doc = "Field `DC` writer - Denominator Configuration."]
pub type DC_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `ELT` reader - Enable Local Time"]
pub type ELT_R = crate::BitReader;
#[doc = "Field `ELT` writer - Enable Local Time"]
pub type ELT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Numerator Configuration Low."]
    #[inline(always)]
    pub fn ncl(&self) -> NCL_R {
        NCL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:29 - Denominator Configuration."]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Enable Local Time"]
    #[inline(always)]
    pub fn elt(&self) -> ELT_R {
        ELT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Numerator Configuration Low."]
    #[inline(always)]
    #[must_use]
    pub fn ncl(&mut self) -> NCL_W<TURCFrs> {
        NCL_W::new(self, 0)
    }
    #[doc = "Bits 16:29 - Denominator Configuration."]
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<TURCFrs> {
        DC_W::new(self, 16)
    }
    #[doc = "Bit 31 - Enable Local Time"]
    #[inline(always)]
    #[must_use]
    pub fn elt(&mut self) -> ELT_W<TURCFrs> {
        ELT_W::new(self, 31)
    }
}
#[doc = "FDCAN TUR Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`turcf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`turcf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TURCFrs;
impl crate::RegisterSpec for TURCFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`turcf::R`](R) reader structure"]
impl crate::Readable for TURCFrs {}
#[doc = "`write(|w| ..)` method takes [`turcf::W`](W) writer structure"]
impl crate::Writable for TURCFrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TURCF to value 0"]
impl crate::Resettable for TURCFrs {
    const RESET_VALUE: u32 = 0;
}
