#[doc = "Register `GICD_ISPENDR3` reader"]
pub type R = crate::R<GICD_ISPENDR3rs>;
#[doc = "Register `GICD_ISPENDR3` writer"]
pub type W = crate::W<GICD_ISPENDR3rs>;
#[doc = "Field `ISPENDR3` reader - ISPENDR3"]
pub type ISPENDR3_R = crate::FieldReader<u32>;
#[doc = "Field `ISPENDR3` writer - ISPENDR3"]
pub type ISPENDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ISPENDR3"]
    #[inline(always)]
    pub fn ispendr3(&self) -> ISPENDR3_R {
        ISPENDR3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISPENDR3"]
    #[inline(always)]
    #[must_use]
    pub fn ispendr3(&mut self) -> ISPENDR3_W<GICD_ISPENDR3rs> {
        ISPENDR3_W::new(self, 0)
    }
}
#[doc = "For interrupts ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ISPENDR3rs;
impl crate::RegisterSpec for GICD_ISPENDR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ispendr3::R`](R) reader structure"]
impl crate::Readable for GICD_ISPENDR3rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_ispendr3::W`](W) writer structure"]
impl crate::Writable for GICD_ISPENDR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ISPENDR3 to value 0"]
impl crate::Resettable for GICD_ISPENDR3rs {
    const RESET_VALUE: u32 = 0;
}
