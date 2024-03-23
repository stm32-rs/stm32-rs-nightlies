#[doc = "Register `CSR89` reader"]
pub type R = crate::R<CSR89rs>;
#[doc = "Register `CSR89` writer"]
pub type W = crate::W<CSR89rs>;
#[doc = "Field `CSx` reader - Context swap 89 Refer to introduction."]
pub type CSX_R = crate::FieldReader<u32>;
#[doc = "Field `CSx` writer - Context swap 89 Refer to introduction."]
pub type CSX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap 89 Refer to introduction."]
    #[inline(always)]
    pub fn csx(&self) -> CSX_R {
        CSX_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap 89 Refer to introduction."]
    #[inline(always)]
    #[must_use]
    pub fn csx(&mut self) -> CSX_W<CSR89rs> {
        CSX_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 89\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr89::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr89::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR89rs;
impl crate::RegisterSpec for CSR89rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr89::R`](R) reader structure"]
impl crate::Readable for CSR89rs {}
#[doc = "`write(|w| ..)` method takes [`csr89::W`](W) writer structure"]
impl crate::Writable for CSR89rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR89 to value 0x0022_0002"]
impl crate::Resettable for CSR89rs {
    const RESET_VALUE: u32 = 0x0022_0002;
}
