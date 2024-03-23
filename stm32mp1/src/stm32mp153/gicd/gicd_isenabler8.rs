#[doc = "Register `GICD_ISENABLER8` reader"]
pub type R = crate::R<GICD_ISENABLER8rs>;
#[doc = "Register `GICD_ISENABLER8` writer"]
pub type W = crate::W<GICD_ISENABLER8rs>;
#[doc = "Field `ISENABLER8` reader - ISENABLER8"]
pub type ISENABLER8_R = crate::FieldReader<u32>;
#[doc = "Field `ISENABLER8` writer - ISENABLER8"]
pub type ISENABLER8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ISENABLER8"]
    #[inline(always)]
    pub fn isenabler8(&self) -> ISENABLER8_R {
        ISENABLER8_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISENABLER8"]
    #[inline(always)]
    #[must_use]
    pub fn isenabler8(&mut self) -> ISENABLER8_W<GICD_ISENABLER8rs> {
        ISENABLER8_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isenabler8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isenabler8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ISENABLER8rs;
impl crate::RegisterSpec for GICD_ISENABLER8rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_isenabler8::R`](R) reader structure"]
impl crate::Readable for GICD_ISENABLER8rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_isenabler8::W`](W) writer structure"]
impl crate::Writable for GICD_ISENABLER8rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ISENABLER8 to value 0"]
impl crate::Resettable for GICD_ISENABLER8rs {
    const RESET_VALUE: u32 = 0;
}
