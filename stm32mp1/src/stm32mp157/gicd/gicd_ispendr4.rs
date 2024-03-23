#[doc = "Register `GICD_ISPENDR4` reader"]
pub type R = crate::R<GICD_ISPENDR4rs>;
#[doc = "Register `GICD_ISPENDR4` writer"]
pub type W = crate::W<GICD_ISPENDR4rs>;
#[doc = "Field `ISPENDR4` reader - ISPENDR4"]
pub type ISPENDR4_R = crate::FieldReader<u32>;
#[doc = "Field `ISPENDR4` writer - ISPENDR4"]
pub type ISPENDR4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ISPENDR4"]
    #[inline(always)]
    pub fn ispendr4(&self) -> ISPENDR4_R {
        ISPENDR4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISPENDR4"]
    #[inline(always)]
    #[must_use]
    pub fn ispendr4(&mut self) -> ISPENDR4_W<GICD_ISPENDR4rs> {
        ISPENDR4_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ISPENDR4rs;
impl crate::RegisterSpec for GICD_ISPENDR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ispendr4::R`](R) reader structure"]
impl crate::Readable for GICD_ISPENDR4rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_ispendr4::W`](W) writer structure"]
impl crate::Writable for GICD_ISPENDR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ISPENDR4 to value 0"]
impl crate::Resettable for GICD_ISPENDR4rs {
    const RESET_VALUE: u32 = 0;
}
