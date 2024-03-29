#[doc = "Register `R2NONCER1` reader"]
pub type R = crate::R<R2NONCER1rs>;
#[doc = "Register `R2NONCER1` writer"]
pub type W = crate::W<R2NONCER1rs>;
#[doc = "Field `REG2_NONCE` reader - Region nonce, bits \\[63:32\\]REGx_NONCE\\[63:32\\]"]
pub type REG2_NONCE_R = crate::FieldReader<u32>;
#[doc = "Field `REG2_NONCE` writer - Region nonce, bits \\[63:32\\]REGx_NONCE\\[63:32\\]"]
pub type REG2_NONCE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region nonce, bits \\[63:32\\]REGx_NONCE\\[63:32\\]"]
    #[inline(always)]
    pub fn reg2_nonce(&self) -> REG2_NONCE_R {
        REG2_NONCE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region nonce, bits \\[63:32\\]REGx_NONCE\\[63:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_nonce(&mut self) -> REG2_NONCE_W<R2NONCER1rs> {
        REG2_NONCE_W::new(self, 0)
    }
}
#[doc = "OTFDEC region x nonce register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r2noncer1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r2noncer1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R2NONCER1rs;
impl crate::RegisterSpec for R2NONCER1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r2noncer1::R`](R) reader structure"]
impl crate::Readable for R2NONCER1rs {}
#[doc = "`write(|w| ..)` method takes [`r2noncer1::W`](W) writer structure"]
impl crate::Writable for R2NONCER1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R2NONCER1 to value 0"]
impl crate::Resettable for R2NONCER1rs {
    const RESET_VALUE: u32 = 0;
}
