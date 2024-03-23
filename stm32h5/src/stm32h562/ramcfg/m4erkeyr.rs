#[doc = "Register `M4ERKEYR` writer"]
pub type W = crate::W<M4ERKEYRrs>;
#[doc = "Field `ERASEKEY` writer - Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the RAMCFG_MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
pub type ERASEKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the RAMCFG_MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
    #[inline(always)]
    #[must_use]
    pub fn erasekey(&mut self) -> ERASEKEY_W<M4ERKEYRrs> {
        ERASEKEY_W::new(self, 0)
    }
}
#[doc = "RAMCFG memory 4 erase key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m4erkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M4ERKEYRrs;
impl crate::RegisterSpec for M4ERKEYRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`m4erkeyr::W`](W) writer structure"]
impl crate::Writable for M4ERKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M4ERKEYR to value 0"]
impl crate::Resettable for M4ERKEYRrs {
    const RESET_VALUE: u32 = 0;
}
