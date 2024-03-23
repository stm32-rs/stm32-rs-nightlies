#[doc = "Register `CRYP_K0LR` writer"]
pub type W = crate::W<CRYP_K0LRrs>;
#[doc = "Field `K` writer - K"]
pub type K_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - K"]
    #[inline(always)]
    #[must_use]
    pub fn k(&mut self) -> K_W<CRYP_K0LRrs> {
        K_W::new(self, 0)
    }
}
#[doc = "CRYP key registers contain the cryptographic keys. In DES/TDES mode, the keys are 64-bit binary values (number from left to right, that is the leftmost bit is bit 1) and named K1, K2 and K3 (K0 is not used). Each key consists of 56 information bits and 8 parity bits. In AES mode, the key is considered as a single 128, 192 or 256 bits long sequence K0K1K2...K127/191/255. The AES key is entered into the registers as follows: for AES-128: K0..K127 corresponds to b127..b0 (b255..b128 are not used), for AES-192: K0..K191 corresponds to b191..b0 (b255..b192 are not used), for AES-256: K0..K255 corresponds to b255..b0. In all cases key bit K0 is the leftmost bit in CRYP inner memory and register bit b0 is the rightmost bit in corresponding CRYP_KxLR key register. For more information refer to Section39.3.17: CRYP key registers. Write accesses to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_k0lr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYP_K0LRrs;
impl crate::RegisterSpec for CRYP_K0LRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cryp_k0lr::W`](W) writer structure"]
impl crate::Writable for CRYP_K0LRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYP_K0LR to value 0"]
impl crate::Resettable for CRYP_K0LRrs {
    const RESET_VALUE: u32 = 0;
}
