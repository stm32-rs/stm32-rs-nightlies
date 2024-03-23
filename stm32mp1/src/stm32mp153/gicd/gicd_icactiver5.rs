#[doc = "Register `GICD_ICACTIVER5` reader"]
pub type R = crate::R<GICD_ICACTIVER5rs>;
#[doc = "Register `GICD_ICACTIVER5` writer"]
pub type W = crate::W<GICD_ICACTIVER5rs>;
#[doc = "Field `ICACTIVER5` reader - ICACTIVER5"]
pub type ICACTIVER5_R = crate::FieldReader<u32>;
#[doc = "Field `ICACTIVER5` writer - ICACTIVER5"]
pub type ICACTIVER5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ICACTIVER5"]
    #[inline(always)]
    pub fn icactiver5(&self) -> ICACTIVER5_R {
        ICACTIVER5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICACTIVER5"]
    #[inline(always)]
    #[must_use]
    pub fn icactiver5(&mut self) -> ICACTIVER5_W<GICD_ICACTIVER5rs> {
        ICACTIVER5_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ICACTIVER5rs;
impl crate::RegisterSpec for GICD_ICACTIVER5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icactiver5::R`](R) reader structure"]
impl crate::Readable for GICD_ICACTIVER5rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_icactiver5::W`](W) writer structure"]
impl crate::Writable for GICD_ICACTIVER5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICACTIVER5 to value 0"]
impl crate::Resettable for GICD_ICACTIVER5rs {
    const RESET_VALUE: u32 = 0;
}
