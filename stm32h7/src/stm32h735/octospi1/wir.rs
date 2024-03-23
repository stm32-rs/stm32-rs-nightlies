#[doc = "Register `WIR` reader"]
pub type R = crate::R<WIRrs>;
#[doc = "Register `WIR` writer"]
pub type W = crate::W<WIRrs>;
#[doc = "Field `INSTRUCTION` reader - INSTRUCTION"]
pub type INSTRUCTION_R = crate::FieldReader<u32>;
#[doc = "Field `INSTRUCTION` writer - INSTRUCTION"]
pub type INSTRUCTION_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - INSTRUCTION"]
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - INSTRUCTION"]
    #[inline(always)]
    #[must_use]
    pub fn instruction(&mut self) -> INSTRUCTION_W<WIRrs> {
        INSTRUCTION_W::new(self, 0)
    }
}
#[doc = "instruction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIRrs;
impl crate::RegisterSpec for WIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wir::R`](R) reader structure"]
impl crate::Readable for WIRrs {}
#[doc = "`write(|w| ..)` method takes [`wir::W`](W) writer structure"]
impl crate::Writable for WIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIR to value 0"]
impl crate::Resettable for WIRrs {
    const RESET_VALUE: u32 = 0;
}
