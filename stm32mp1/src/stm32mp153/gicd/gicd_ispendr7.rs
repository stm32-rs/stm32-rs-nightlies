#[doc = "Register `GICD_ISPENDR7` reader"]
pub type R = crate::R<GICD_ISPENDR7rs>;
#[doc = "Register `GICD_ISPENDR7` writer"]
pub type W = crate::W<GICD_ISPENDR7rs>;
#[doc = "Field `ISPENDR7` reader - ISPENDR7"]
pub type ISPENDR7_R = crate::FieldReader<u32>;
#[doc = "Field `ISPENDR7` writer - ISPENDR7"]
pub type ISPENDR7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ISPENDR7"]
    #[inline(always)]
    pub fn ispendr7(&self) -> ISPENDR7_R {
        ISPENDR7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISPENDR7"]
    #[inline(always)]
    #[must_use]
    pub fn ispendr7(&mut self) -> ISPENDR7_W<GICD_ISPENDR7rs> {
        ISPENDR7_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ISPENDR7rs;
impl crate::RegisterSpec for GICD_ISPENDR7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ispendr7::R`](R) reader structure"]
impl crate::Readable for GICD_ISPENDR7rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_ispendr7::W`](W) writer structure"]
impl crate::Writable for GICD_ISPENDR7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ISPENDR7 to value 0"]
impl crate::Resettable for GICD_ISPENDR7rs {
    const RESET_VALUE: u32 = 0;
}
