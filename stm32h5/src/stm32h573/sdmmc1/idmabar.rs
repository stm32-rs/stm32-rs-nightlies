#[doc = "Register `IDMABAR` reader"]
pub type R = crate::R<IDMABARrs>;
#[doc = "Register `IDMABAR` writer"]
pub type W = crate::W<IDMABARrs>;
#[doc = "Field `IDMABA` reader - Word aligned Linked list memory base address Linked list memory base pointer. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type IDMABA_R = crate::FieldReader<u32>;
#[doc = "Field `IDMABA` writer - Word aligned Linked list memory base address Linked list memory base pointer. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type IDMABA_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Word aligned Linked list memory base address Linked list memory base pointer. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn idmaba(&self) -> IDMABA_R {
        IDMABA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Word aligned Linked list memory base address Linked list memory base pointer. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn idmaba(&mut self) -> IDMABA_W<IDMABARrs> {
        IDMABA_W::new(self, 2)
    }
}
#[doc = "SDMMC_IDMABAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idmabar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idmabar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDMABARrs;
impl crate::RegisterSpec for IDMABARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idmabar::R`](R) reader structure"]
impl crate::Readable for IDMABARrs {}
#[doc = "`write(|w| ..)` method takes [`idmabar::W`](W) writer structure"]
impl crate::Writable for IDMABARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDMABAR to value 0"]
impl crate::Resettable for IDMABARrs {
    const RESET_VALUE: u32 = 0;
}
