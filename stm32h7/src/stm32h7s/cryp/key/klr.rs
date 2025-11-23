///Register `KLR` writer
pub type W = crate::W<KLRrs>;
///Field `K` writer - Key bit x This write-only bitfield contains the bits \[255:224\] of the AES encryption or decryption key, depending on the operating mode. Write to CRYP_KxR/LR registers is ignored when CRYP is busy (BUSY bit set). When key is coming from the SAES peripheral (KMOD\[1:0\] = 0x2), write is also ignored. With KMOD\[1:0\] at 0x0, a special writing sequence is required. In this sequence, any valid write to CRYP_KxR/LR register clears the KEYVALID flag except for the sequence-completing write that sets it. Also refer to the description of the KEYVALID flag in the CRYP_SR register.
pub type K_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<KLRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Key bit x This write-only bitfield contains the bits \[255:224\] of the AES encryption or decryption key, depending on the operating mode. Write to CRYP_KxR/LR registers is ignored when CRYP is busy (BUSY bit set). When key is coming from the SAES peripheral (KMOD\[1:0\] = 0x2), write is also ignored. With KMOD\[1:0\] at 0x0, a special writing sequence is required. In this sequence, any valid write to CRYP_KxR/LR register clears the KEYVALID flag except for the sequence-completing write that sets it. Also refer to the description of the KEYVALID flag in the CRYP_SR register.
    #[inline(always)]
    pub fn k(&mut self) -> K_W<'_, KLRrs> {
        K_W::new(self, 0)
    }
}
/**CRYP key register 0L

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`klr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct KLRrs;
impl crate::RegisterSpec for KLRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`klr::W`](W) writer structure
impl crate::Writable for KLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KLR to value 0
impl crate::Resettable for KLRrs {}
