#[doc = "Register `R4NONCER1` reader"]
pub type R = crate::R<R4NONCER1rs>;
#[doc = "Register `R4NONCER1` writer"]
pub type W = crate::W<R4NONCER1rs>;
#[doc = "Field `REGx_NONCE` reader - Region nonce, bits \\[63:32\\]
Refer to the OTFDEC_RxNONCER0 register for description of the NONCE\\[63:0\\]
bitfield."]
pub type REGX_NONCE_R = crate::FieldReader<u32>;
#[doc = "Field `REGx_NONCE` writer - Region nonce, bits \\[63:32\\]
Refer to the OTFDEC_RxNONCER0 register for description of the NONCE\\[63:0\\]
bitfield."]
pub type REGX_NONCE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region nonce, bits \\[63:32\\]
Refer to the OTFDEC_RxNONCER0 register for description of the NONCE\\[63:0\\]
bitfield."]
    #[inline(always)]
    pub fn regx_nonce(&self) -> REGX_NONCE_R {
        REGX_NONCE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region nonce, bits \\[63:32\\]
Refer to the OTFDEC_RxNONCER0 register for description of the NONCE\\[63:0\\]
bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn regx_nonce(&mut self) -> REGX_NONCE_W<R4NONCER1rs> {
        REGX_NONCE_W::new(self, 0)
    }
}
#[doc = "OTFDEC region 4 nonce register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r4noncer1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r4noncer1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R4NONCER1rs;
impl crate::RegisterSpec for R4NONCER1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r4noncer1::R`](R) reader structure"]
impl crate::Readable for R4NONCER1rs {}
#[doc = "`write(|w| ..)` method takes [`r4noncer1::W`](W) writer structure"]
impl crate::Writable for R4NONCER1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R4NONCER1 to value 0"]
impl crate::Resettable for R4NONCER1rs {
    const RESET_VALUE: u32 = 0;
}
