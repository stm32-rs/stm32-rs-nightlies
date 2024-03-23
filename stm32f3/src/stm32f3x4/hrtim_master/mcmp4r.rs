#[doc = "Register `MCMP4R` reader"]
pub type R = crate::R<MCMP4Rrs>;
#[doc = "Register `MCMP4R` writer"]
pub type W = crate::W<MCMP4Rrs>;
#[doc = "Field `MCMP4` reader - Master Timer Compare 4 value"]
pub type MCMP4_R = crate::FieldReader<u16>;
#[doc = "Field `MCMP4` writer - Master Timer Compare 4 value"]
pub type MCMP4_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Master Timer Compare 4 value"]
    #[inline(always)]
    pub fn mcmp4(&self) -> MCMP4_R {
        MCMP4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Compare 4 value"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp4(&mut self) -> MCMP4_W<MCMP4Rrs> {
        MCMP4_W::new(self, 0)
    }
}
#[doc = "Master Timer Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcmp4r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcmp4r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCMP4Rrs;
impl crate::RegisterSpec for MCMP4Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcmp4r::R`](R) reader structure"]
impl crate::Readable for MCMP4Rrs {}
#[doc = "`write(|w| ..)` method takes [`mcmp4r::W`](W) writer structure"]
impl crate::Writable for MCMP4Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCMP4R to value 0"]
impl crate::Resettable for MCMP4Rrs {
    const RESET_VALUE: u32 = 0;
}
