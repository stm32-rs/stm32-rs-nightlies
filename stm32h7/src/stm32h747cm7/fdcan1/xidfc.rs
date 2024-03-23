#[doc = "Register `XIDFC` reader"]
pub type R = crate::R<XIDFCrs>;
#[doc = "Register `XIDFC` writer"]
pub type W = crate::W<XIDFCrs>;
#[doc = "Field `FLESA` reader - Filter List Standard Start Address"]
pub type FLESA_R = crate::FieldReader<u16>;
#[doc = "Field `FLESA` writer - Filter List Standard Start Address"]
pub type FLESA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `LSE` reader - List Size Extended"]
pub type LSE_R = crate::FieldReader;
#[doc = "Field `LSE` writer - List Size Extended"]
pub type LSE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 2:15 - Filter List Standard Start Address"]
    #[inline(always)]
    pub fn flesa(&self) -> FLESA_R {
        FLESA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23 - List Size Extended"]
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Filter List Standard Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn flesa(&mut self) -> FLESA_W<XIDFCrs> {
        FLESA_W::new(self, 2)
    }
    #[doc = "Bits 16:23 - List Size Extended"]
    #[inline(always)]
    #[must_use]
    pub fn lse(&mut self) -> LSE_W<XIDFCrs> {
        LSE_W::new(self, 16)
    }
}
#[doc = "FDCAN Extended ID Filter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xidfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xidfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XIDFCrs;
impl crate::RegisterSpec for XIDFCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xidfc::R`](R) reader structure"]
impl crate::Readable for XIDFCrs {}
#[doc = "`write(|w| ..)` method takes [`xidfc::W`](W) writer structure"]
impl crate::Writable for XIDFCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XIDFC to value 0"]
impl crate::Resettable for XIDFCrs {
    const RESET_VALUE: u32 = 0;
}
