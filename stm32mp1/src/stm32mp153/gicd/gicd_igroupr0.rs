#[doc = "Register `GICD_IGROUPR0` reader"]
pub type R = crate::R<GICD_IGROUPR0rs>;
#[doc = "Register `GICD_IGROUPR0` writer"]
pub type W = crate::W<GICD_IGROUPR0rs>;
#[doc = "Field `IGROUPR0` reader - IGROUPR0"]
pub type IGROUPR0_R = crate::FieldReader<u32>;
#[doc = "Field `IGROUPR0` writer - IGROUPR0"]
pub type IGROUPR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IGROUPR0"]
    #[inline(always)]
    pub fn igroupr0(&self) -> IGROUPR0_R {
        IGROUPR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IGROUPR0"]
    #[inline(always)]
    #[must_use]
    pub fn igroupr0(&mut self) -> IGROUPR0_W<GICD_IGROUPR0rs> {
        IGROUPR0_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IGROUPR0rs;
impl crate::RegisterSpec for GICD_IGROUPR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_igroupr0::R`](R) reader structure"]
impl crate::Readable for GICD_IGROUPR0rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_igroupr0::W`](W) writer structure"]
impl crate::Writable for GICD_IGROUPR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IGROUPR0 to value 0"]
impl crate::Resettable for GICD_IGROUPR0rs {
    const RESET_VALUE: u32 = 0;
}
