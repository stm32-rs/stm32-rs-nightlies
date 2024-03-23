#[doc = "Register `R1NONCER0` reader"]
pub type R = crate::R<R1NONCER0rs>;
#[doc = "Register `R1NONCER0` writer"]
pub type W = crate::W<R1NONCER0rs>;
#[doc = "Field `REGx_NONCE` reader - Region nonce, bits \\[31:0\\]
This register must be written before the region corresponding REG_EN bit in OTFDEC_RxCFGR is set. Writing is discarded in this register if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR is set."]
pub type REGX_NONCE_R = crate::FieldReader<u32>;
#[doc = "Field `REGx_NONCE` writer - Region nonce, bits \\[31:0\\]
This register must be written before the region corresponding REG_EN bit in OTFDEC_RxCFGR is set. Writing is discarded in this register if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR is set."]
pub type REGX_NONCE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region nonce, bits \\[31:0\\]
This register must be written before the region corresponding REG_EN bit in OTFDEC_RxCFGR is set. Writing is discarded in this register if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR is set."]
    #[inline(always)]
    pub fn regx_nonce(&self) -> REGX_NONCE_R {
        REGX_NONCE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region nonce, bits \\[31:0\\]
This register must be written before the region corresponding REG_EN bit in OTFDEC_RxCFGR is set. Writing is discarded in this register if performed while the region CONFIGLOCK bit in the OTFDEC_RxCFGR is set."]
    #[inline(always)]
    #[must_use]
    pub fn regx_nonce(&mut self) -> REGX_NONCE_W<R1NONCER0rs> {
        REGX_NONCE_W::new(self, 0)
    }
}
#[doc = "OTFDEC region 1 nonce register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r1noncer0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r1noncer0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R1NONCER0rs;
impl crate::RegisterSpec for R1NONCER0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r1noncer0::R`](R) reader structure"]
impl crate::Readable for R1NONCER0rs {}
#[doc = "`write(|w| ..)` method takes [`r1noncer0::W`](W) writer structure"]
impl crate::Writable for R1NONCER0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R1NONCER0 to value 0"]
impl crate::Resettable for R1NONCER0rs {
    const RESET_VALUE: u32 = 0;
}
