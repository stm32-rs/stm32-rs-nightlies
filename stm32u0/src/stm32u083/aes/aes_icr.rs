///Register `AES_ICR` writer
pub type W = crate::W<AES_ICRrs>;
///Field `CCF` writer - Computation complete flag clear Setting this bit clears the CCF status bit of the AES_ISR register.
pub type CCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWEIF` writer - Read or write error interrupt flag clear Setting this bit clears the RWEIF status bit of the AES_ISR register, and clears both RDERRF and WRERRF flags in the AES_SR register.
pub type RWEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEIF` writer - Key error interrupt flag clear Setting this bit clears the KEIF status bit of the AES_ISR register.
pub type KEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AES_ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Computation complete flag clear Setting this bit clears the CCF status bit of the AES_ISR register.
    #[inline(always)]
    pub fn ccf(&mut self) -> CCF_W<AES_ICRrs> {
        CCF_W::new(self, 0)
    }
    ///Bit 1 - Read or write error interrupt flag clear Setting this bit clears the RWEIF status bit of the AES_ISR register, and clears both RDERRF and WRERRF flags in the AES_SR register.
    #[inline(always)]
    pub fn rweif(&mut self) -> RWEIF_W<AES_ICRrs> {
        RWEIF_W::new(self, 1)
    }
    ///Bit 2 - Key error interrupt flag clear Setting this bit clears the KEIF status bit of the AES_ISR register.
    #[inline(always)]
    pub fn keif(&mut self) -> KEIF_W<AES_ICRrs> {
        KEIF_W::new(self, 2)
    }
}
/**AES interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_ICR)*/
pub struct AES_ICRrs;
impl crate::RegisterSpec for AES_ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`aes_icr::W`](W) writer structure
impl crate::Writable for AES_ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AES_ICR to value 0
impl crate::Resettable for AES_ICRrs {
    const RESET_VALUE: u32 = 0;
}