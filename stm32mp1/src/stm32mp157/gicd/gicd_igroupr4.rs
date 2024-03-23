#[doc = "Register `GICD_IGROUPR4` reader"]
pub type R = crate::R<GICD_IGROUPR4rs>;
#[doc = "Register `GICD_IGROUPR4` writer"]
pub type W = crate::W<GICD_IGROUPR4rs>;
#[doc = "Field `IGROUPR4` reader - IGROUPR4"]
pub type IGROUPR4_R = crate::FieldReader<u32>;
#[doc = "Field `IGROUPR4` writer - IGROUPR4"]
pub type IGROUPR4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IGROUPR4"]
    #[inline(always)]
    pub fn igroupr4(&self) -> IGROUPR4_R {
        IGROUPR4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IGROUPR4"]
    #[inline(always)]
    #[must_use]
    pub fn igroupr4(&mut self) -> IGROUPR4_W<GICD_IGROUPR4rs> {
        IGROUPR4_W::new(self, 0)
    }
}
#[doc = "For interrupts ID = x*32 to ID = x*32+31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IGROUPR4rs;
impl crate::RegisterSpec for GICD_IGROUPR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_igroupr4::R`](R) reader structure"]
impl crate::Readable for GICD_IGROUPR4rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_igroupr4::W`](W) writer structure"]
impl crate::Writable for GICD_IGROUPR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IGROUPR4 to value 0"]
impl crate::Resettable for GICD_IGROUPR4rs {
    const RESET_VALUE: u32 = 0;
}
