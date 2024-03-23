#[doc = "Register `CHSELR0` reader"]
pub type R = crate::R<CHSELR0rs>;
#[doc = "Register `CHSELR0` writer"]
pub type W = crate::W<CHSELR0rs>;
#[doc = "Field `CHSEL` reader - CHSEL"]
pub type CHSEL_R = crate::FieldReader<u32>;
#[doc = "Field `CHSEL` writer - CHSEL"]
pub type CHSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - CHSEL"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - CHSEL"]
    #[inline(always)]
    #[must_use]
    pub fn chsel(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 0)
    }
}
#[doc = "channel selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chselr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chselr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHSELR0rs;
impl crate::RegisterSpec for CHSELR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chselr0::R`](R) reader structure"]
impl crate::Readable for CHSELR0rs {}
#[doc = "`write(|w| ..)` method takes [`chselr0::W`](W) writer structure"]
impl crate::Writable for CHSELR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHSELR0 to value 0"]
impl crate::Resettable for CHSELR0rs {
    const RESET_VALUE: u32 = 0;
}
