#[doc = "Register `GICD_ICPENDR8` reader"]
pub type R = crate::R<GICD_ICPENDR8rs>;
#[doc = "Register `GICD_ICPENDR8` writer"]
pub type W = crate::W<GICD_ICPENDR8rs>;
#[doc = "Field `ICPENDR8` reader - ICPENDR8"]
pub type ICPENDR8_R = crate::FieldReader<u32>;
#[doc = "Field `ICPENDR8` writer - ICPENDR8"]
pub type ICPENDR8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ICPENDR8"]
    #[inline(always)]
    pub fn icpendr8(&self) -> ICPENDR8_R {
        ICPENDR8_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICPENDR8"]
    #[inline(always)]
    #[must_use]
    pub fn icpendr8(&mut self) -> ICPENDR8_W<GICD_ICPENDR8rs> {
        ICPENDR8_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ICPENDR8rs;
impl crate::RegisterSpec for GICD_ICPENDR8rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icpendr8::R`](R) reader structure"]
impl crate::Readable for GICD_ICPENDR8rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_icpendr8::W`](W) writer structure"]
impl crate::Writable for GICD_ICPENDR8rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICPENDR8 to value 0"]
impl crate::Resettable for GICD_ICPENDR8rs {
    const RESET_VALUE: u32 = 0;
}
