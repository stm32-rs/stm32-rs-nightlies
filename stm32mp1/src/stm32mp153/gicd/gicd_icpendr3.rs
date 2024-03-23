#[doc = "Register `GICD_ICPENDR3` reader"]
pub type R = crate::R<GICD_ICPENDR3rs>;
#[doc = "Register `GICD_ICPENDR3` writer"]
pub type W = crate::W<GICD_ICPENDR3rs>;
#[doc = "Field `ICPENDR3` reader - ICPENDR3"]
pub type ICPENDR3_R = crate::FieldReader<u32>;
#[doc = "Field `ICPENDR3` writer - ICPENDR3"]
pub type ICPENDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ICPENDR3"]
    #[inline(always)]
    pub fn icpendr3(&self) -> ICPENDR3_R {
        ICPENDR3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICPENDR3"]
    #[inline(always)]
    #[must_use]
    pub fn icpendr3(&mut self) -> ICPENDR3_W<GICD_ICPENDR3rs> {
        ICPENDR3_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ICPENDR3rs;
impl crate::RegisterSpec for GICD_ICPENDR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_icpendr3::R`](R) reader structure"]
impl crate::Readable for GICD_ICPENDR3rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_icpendr3::W`](W) writer structure"]
impl crate::Writable for GICD_ICPENDR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ICPENDR3 to value 0"]
impl crate::Resettable for GICD_ICPENDR3rs {
    const RESET_VALUE: u32 = 0;
}
