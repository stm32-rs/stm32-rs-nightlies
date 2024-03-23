#[doc = "Register `FPUIMR` reader"]
pub type R = crate::R<FPUIMRrs>;
#[doc = "Register `FPUIMR` writer"]
pub type W = crate::W<FPUIMRrs>;
#[doc = "Field `FPU_IE` reader - Floating point unit interrupts enable bits"]
pub type FPU_IE_R = crate::FieldReader;
#[doc = "Field `FPU_IE` writer - Floating point unit interrupts enable bits"]
pub type FPU_IE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Floating point unit interrupts enable bits"]
    #[inline(always)]
    pub fn fpu_ie(&self) -> FPU_IE_R {
        FPU_IE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Floating point unit interrupts enable bits"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie(&mut self) -> FPU_IE_W<FPUIMRrs> {
        FPU_IE_W::new(self, 0)
    }
}
#[doc = "FPU interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpuimr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpuimr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPUIMRrs;
impl crate::RegisterSpec for FPUIMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpuimr::R`](R) reader structure"]
impl crate::Readable for FPUIMRrs {}
#[doc = "`write(|w| ..)` method takes [`fpuimr::W`](W) writer structure"]
impl crate::Writable for FPUIMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPUIMR to value 0x1f"]
impl crate::Resettable for FPUIMRrs {
    const RESET_VALUE: u32 = 0x1f;
}
