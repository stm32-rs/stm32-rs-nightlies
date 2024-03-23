#[doc = "Register `M2ECCKEYR` writer"]
pub type W = crate::W<M2ECCKEYRrs>;
#[doc = "Field `ECCKEY` writer - ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the RAMCFG_MxCR register. 1) Write 0xAE into ECCKEY\\[7:0\\]. 2) Write 0x75 into ECCKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
pub type ECCKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the RAMCFG_MxCR register. 1) Write 0xAE into ECCKEY\\[7:0\\]. 2) Write 0x75 into ECCKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
    #[inline(always)]
    #[must_use]
    pub fn ecckey(&mut self) -> ECCKEY_W<M2ECCKEYRrs> {
        ECCKEY_W::new(self, 0)
    }
}
#[doc = "RAMCFG memory 2 ECC key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2ecckeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2ECCKEYRrs;
impl crate::RegisterSpec for M2ECCKEYRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`m2ecckeyr::W`](W) writer structure"]
impl crate::Writable for M2ECCKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M2ECCKEYR to value 0"]
impl crate::Resettable for M2ECCKEYRrs {
    const RESET_VALUE: u32 = 0;
}
