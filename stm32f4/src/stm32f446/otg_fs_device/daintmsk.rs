#[doc = "Register `DAINTMSK` reader"]
pub type R = crate::R<DAINTMSKrs>;
#[doc = "Register `DAINTMSK` writer"]
pub type W = crate::W<DAINTMSKrs>;
#[doc = "Field `IEPM` reader - IN EP interrupt mask bits"]
pub type IEPM_R = crate::FieldReader<u16>;
#[doc = "Field `IEPM` writer - IN EP interrupt mask bits"]
pub type IEPM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OEPM` reader - OUT EP interrupt mask bits"]
pub type OEPM_R = crate::FieldReader<u16>;
#[doc = "Field `OEPM` writer - OUT EP interrupt mask bits"]
pub type OEPM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN EP interrupt mask bits"]
    #[inline(always)]
    pub fn iepm(&self) -> IEPM_R {
        IEPM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT EP interrupt mask bits"]
    #[inline(always)]
    pub fn oepm(&self) -> OEPM_R {
        OEPM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP interrupt mask bits"]
    #[inline(always)]
    #[must_use]
    pub fn iepm(&mut self) -> IEPM_W<DAINTMSKrs> {
        IEPM_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - OUT EP interrupt mask bits"]
    #[inline(always)]
    #[must_use]
    pub fn oepm(&mut self) -> OEPM_W<DAINTMSKrs> {
        OEPM_W::new(self, 16)
    }
}
#[doc = "OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAINTMSKrs;
impl crate::RegisterSpec for DAINTMSKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daintmsk::R`](R) reader structure"]
impl crate::Readable for DAINTMSKrs {}
#[doc = "`write(|w| ..)` method takes [`daintmsk::W`](W) writer structure"]
impl crate::Writable for DAINTMSKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAINTMSK to value 0"]
impl crate::Resettable for DAINTMSKrs {
    const RESET_VALUE: u32 = 0;
}
