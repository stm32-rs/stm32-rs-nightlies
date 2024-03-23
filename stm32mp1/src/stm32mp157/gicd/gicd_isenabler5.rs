#[doc = "Register `GICD_ISENABLER5` reader"]
pub type R = crate::R<GICD_ISENABLER5rs>;
#[doc = "Register `GICD_ISENABLER5` writer"]
pub type W = crate::W<GICD_ISENABLER5rs>;
#[doc = "Field `ISENABLER5` reader - ISENABLER5"]
pub type ISENABLER5_R = crate::FieldReader<u32>;
#[doc = "Field `ISENABLER5` writer - ISENABLER5"]
pub type ISENABLER5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ISENABLER5"]
    #[inline(always)]
    pub fn isenabler5(&self) -> ISENABLER5_R {
        ISENABLER5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISENABLER5"]
    #[inline(always)]
    #[must_use]
    pub fn isenabler5(&mut self) -> ISENABLER5_W<GICD_ISENABLER5rs> {
        ISENABLER5_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isenabler5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isenabler5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ISENABLER5rs;
impl crate::RegisterSpec for GICD_ISENABLER5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_isenabler5::R`](R) reader structure"]
impl crate::Readable for GICD_ISENABLER5rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_isenabler5::W`](W) writer structure"]
impl crate::Writable for GICD_ISENABLER5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ISENABLER5 to value 0"]
impl crate::Resettable for GICD_ISENABLER5rs {
    const RESET_VALUE: u32 = 0;
}
