#[doc = "Register `L1CFBAR` reader"]
pub type R = crate::R<L1CFBARrs>;
#[doc = "Register `L1CFBAR` writer"]
pub type W = crate::W<L1CFBARrs>;
#[doc = "Field `CFBADD` reader - Color Frame Buffer Start Address"]
pub type CFBADD_R = crate::FieldReader<u32>;
#[doc = "Field `CFBADD` writer - Color Frame Buffer Start Address"]
pub type CFBADD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Color Frame Buffer Start Address"]
    #[inline(always)]
    pub fn cfbadd(&self) -> CFBADD_R {
        CFBADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Color Frame Buffer Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn cfbadd(&mut self) -> CFBADD_W<L1CFBARrs> {
        CFBADD_W::new(self, 0)
    }
}
#[doc = "LTDC Layer Color Frame Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1cfbar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1cfbar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CFBARrs;
impl crate::RegisterSpec for L1CFBARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1cfbar::R`](R) reader structure"]
impl crate::Readable for L1CFBARrs {}
#[doc = "`write(|w| ..)` method takes [`l1cfbar::W`](W) writer structure"]
impl crate::Writable for L1CFBARrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1CFBAR to value 0"]
impl crate::Resettable for L1CFBARrs {
    const RESET_VALUE: u32 = 0;
}
