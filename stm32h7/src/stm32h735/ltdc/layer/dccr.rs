#[doc = "Register `DCCR` reader"]
pub type R = crate::R<DCCRrs>;
#[doc = "Register `DCCR` writer"]
pub type W = crate::W<DCCRrs>;
#[doc = "Field `DCBLUE` reader - Default Color Blue"]
pub type DCBLUE_R = crate::FieldReader;
#[doc = "Field `DCBLUE` writer - Default Color Blue"]
pub type DCBLUE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Field `DCGREEN` reader - Default Color Green"]
pub type DCGREEN_R = crate::FieldReader;
#[doc = "Field `DCGREEN` writer - Default Color Green"]
pub type DCGREEN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Field `DCRED` reader - Default Color Red"]
pub type DCRED_R = crate::FieldReader;
#[doc = "Field `DCRED` writer - Default Color Red"]
pub type DCRED_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Field `DCALPHA` reader - Default Color Alpha"]
pub type DCALPHA_R = crate::FieldReader;
#[doc = "Field `DCALPHA` writer - Default Color Alpha"]
pub type DCALPHA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Default Color Blue"]
    #[inline(always)]
    pub fn dcblue(&self) -> DCBLUE_R {
        DCBLUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Default Color Green"]
    #[inline(always)]
    pub fn dcgreen(&self) -> DCGREEN_R {
        DCGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Default Color Red"]
    #[inline(always)]
    pub fn dcred(&self) -> DCRED_R {
        DCRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Default Color Alpha"]
    #[inline(always)]
    pub fn dcalpha(&self) -> DCALPHA_R {
        DCALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Default Color Blue"]
    #[inline(always)]
    #[must_use]
    pub fn dcblue(&mut self) -> DCBLUE_W<DCCRrs> {
        DCBLUE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Default Color Green"]
    #[inline(always)]
    #[must_use]
    pub fn dcgreen(&mut self) -> DCGREEN_W<DCCRrs> {
        DCGREEN_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Default Color Red"]
    #[inline(always)]
    #[must_use]
    pub fn dcred(&mut self) -> DCRED_W<DCCRrs> {
        DCRED_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Default Color Alpha"]
    #[inline(always)]
    #[must_use]
    pub fn dcalpha(&mut self) -> DCALPHA_W<DCCRrs> {
        DCALPHA_W::new(self, 24)
    }
}
#[doc = "Layerx Default Color Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCCRrs;
impl crate::RegisterSpec for DCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dccr::R`](R) reader structure"]
impl crate::Readable for DCCRrs {}
#[doc = "`write(|w| ..)` method takes [`dccr::W`](W) writer structure"]
impl crate::Writable for DCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCCR to value 0"]
impl crate::Resettable for DCCRrs {
    const RESET_VALUE: u32 = 0;
}
