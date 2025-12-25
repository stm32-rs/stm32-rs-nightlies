///Register `KRR` writer
pub type W = crate::W<KRRrs>;
///Field `K` writer - Key bit x This write-only bitfield contains the bits \[223:192\] of the AES encryption or decryption key, depending on the operating mode. Refer to the CRYP_K0LR register for information relative to writing CRYP_KxR/LR registers.
pub type K_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<KRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Key bit x This write-only bitfield contains the bits \[223:192\] of the AES encryption or decryption key, depending on the operating mode. Refer to the CRYP_K0LR register for information relative to writing CRYP_KxR/LR registers.
    #[inline(always)]
    pub fn k(&mut self) -> K_W<'_, KRRrs> {
        K_W::new(self, 0)
    }
}
/**CRYP key register 0R

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`krr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct KRRrs;
impl crate::RegisterSpec for KRRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`krr::W`](W) writer structure
impl crate::Writable for KRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KRR to value 0
impl crate::Resettable for KRRrs {}
