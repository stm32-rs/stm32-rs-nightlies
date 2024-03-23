#[doc = "Register `GICD_ISPENDR8` reader"]
pub type R = crate::R<GICD_ISPENDR8rs>;
#[doc = "Register `GICD_ISPENDR8` writer"]
pub type W = crate::W<GICD_ISPENDR8rs>;
#[doc = "Field `ISPENDR8` reader - ISPENDR8"]
pub type ISPENDR8_R = crate::FieldReader<u32>;
#[doc = "Field `ISPENDR8` writer - ISPENDR8"]
pub type ISPENDR8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ISPENDR8"]
    #[inline(always)]
    pub fn ispendr8(&self) -> ISPENDR8_R {
        ISPENDR8_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISPENDR8"]
    #[inline(always)]
    #[must_use]
    pub fn ispendr8(&mut self) -> ISPENDR8_W<GICD_ISPENDR8rs> {
        ISPENDR8_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ISPENDR8rs;
impl crate::RegisterSpec for GICD_ISPENDR8rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ispendr8::R`](R) reader structure"]
impl crate::Readable for GICD_ISPENDR8rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_ispendr8::W`](W) writer structure"]
impl crate::Writable for GICD_ISPENDR8rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ISPENDR8 to value 0"]
impl crate::Resettable for GICD_ISPENDR8rs {
    const RESET_VALUE: u32 = 0;
}
