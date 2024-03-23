#[doc = "Register `FTSR2` reader"]
pub type R = crate::R<FTSR2rs>;
#[doc = "Register `FTSR2` writer"]
pub type W = crate::W<FTSR2rs>;
#[doc = "Field `FT33` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT33_R = crate::BitReader;
#[doc = "Field `FT33` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT40_41` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT40_41_R = crate::FieldReader;
#[doc = "Field `FT40_41` writer - Falling trigger event configuration bit of Configurable Event input"]
pub type FT40_41_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft33(&self) -> FT33_R {
        FT33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft40_41(&self) -> FT40_41_R {
        FT40_41_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn ft33(&mut self) -> FT33_W<FTSR2rs> {
        FT33_W::new(self, 1)
    }
    #[doc = "Bits 8:9 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn ft40_41(&mut self) -> FT40_41_W<FTSR2rs> {
        FT40_41_W::new(self, 8)
    }
}
#[doc = "falling trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FTSR2rs;
impl crate::RegisterSpec for FTSR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftsr2::R`](R) reader structure"]
impl crate::Readable for FTSR2rs {}
#[doc = "`write(|w| ..)` method takes [`ftsr2::W`](W) writer structure"]
impl crate::Writable for FTSR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FTSR2 to value 0"]
impl crate::Resettable for FTSR2rs {
    const RESET_VALUE: u32 = 0;
}
