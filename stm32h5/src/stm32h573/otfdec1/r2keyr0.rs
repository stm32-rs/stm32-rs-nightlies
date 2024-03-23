#[doc = "Register `R2KEYR0` writer"]
pub type W = crate::W<R2KEYR0rs>;
#[doc = "Field `REGx_KEY` writer - Region key, bits \\[31:0\\]
This register must be written before the region corresponding REG_EN bit in OTFDEC_RxCFGR is set. Reading this register returns a zero value. Writing to this register is discarded if performed while the region CONFIGLOCK or KEYLOCK bit is set in the OTFDEC_RxCFGR. Note: When application successfully changes MODE bits in OTFDEC_RxCFGR and OTFDEC_RxKEYR, and associated KEYCRC are erased."]
pub type REGX_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Region key, bits \\[31:0\\]
This register must be written before the region corresponding REG_EN bit in OTFDEC_RxCFGR is set. Reading this register returns a zero value. Writing to this register is discarded if performed while the region CONFIGLOCK or KEYLOCK bit is set in the OTFDEC_RxCFGR. Note: When application successfully changes MODE bits in OTFDEC_RxCFGR and OTFDEC_RxKEYR, and associated KEYCRC are erased."]
    #[inline(always)]
    #[must_use]
    pub fn regx_key(&mut self) -> REGX_KEY_W<R2KEYR0rs> {
        REGX_KEY_W::new(self, 0)
    }
}
#[doc = "OTFDEC region 2 key register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r2keyr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R2KEYR0rs;
impl crate::RegisterSpec for R2KEYR0rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`r2keyr0::W`](W) writer structure"]
impl crate::Writable for R2KEYR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R2KEYR0 to value 0"]
impl crate::Resettable for R2KEYR0rs {
    const RESET_VALUE: u32 = 0;
}
