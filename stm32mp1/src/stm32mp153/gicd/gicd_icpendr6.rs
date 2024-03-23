#[doc = "Register `GICD_ICPENDR6` reader"]
pub type R = crate::R<GICD_ICPENDR6rs>;
#[doc = "Register `GICD_ICPENDR6` writer"]
pub type W = crate::W<GICD_ICPENDR6rs>;
#[doc = "Field `ICPENDR6` reader - ICPENDR6"]
pub type ICPENDR6_R = crate::FieldReader<u32>;
#[doc = "Field `ICPENDR6` writer - ICPENDR6"]
pub type ICPENDR6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ICPENDR6"]
    #[inline(always)]
    pub fn icpendr6(&self) -> ICPENDR6_R {
        ICPENDR6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICPENDR6"]
    #[inline(always)]
    #[must_use]
    pub fn icpendr6(&mut self) -> ICPENDR6_W<GICD_ICPENDR6rs> {
        ICPENDR6_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ICPENDR6rs;
impl crate::RegisterSpec for GICD_ICPENDR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icpendr6::R`](R) reader structure"]
impl crate::Readable for GICD_ICPENDR6rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_icpendr6::W`](W) writer structure"]
impl crate::Writable for GICD_ICPENDR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICPENDR6 to value 0"]
impl crate::Resettable for GICD_ICPENDR6rs {
    const RESET_VALUE: u32 = 0;
}
