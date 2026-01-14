///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `CCF` writer - Computation complete flag clear Setting this bit clears the CCF status bit of the SAES_ISR register.
pub type CCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWEIF` writer - Read or write error interrupt flag clear Setting this bit clears the RWEIF status bit of the SAES_ISR register, and clears both RDERRF and WRERRF flags in the SAES_SR register.
pub type RWEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEIF` writer - Key error interrupt flag clear Setting this bit clears the KEIF status bit of the SAES_ISR register.
pub type KEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGEIF` writer - RNG error interrupt flag clear Application must set this bit to clear the RNGEIF status bit in SAES_ISR register.
pub type RNGEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Computation complete flag clear Setting this bit clears the CCF status bit of the SAES_ISR register.
    #[inline(always)]
    pub fn ccf(&mut self) -> CCF_W<'_, ICRrs> {
        CCF_W::new(self, 0)
    }
    ///Bit 1 - Read or write error interrupt flag clear Setting this bit clears the RWEIF status bit of the SAES_ISR register, and clears both RDERRF and WRERRF flags in the SAES_SR register.
    #[inline(always)]
    pub fn rweif(&mut self) -> RWEIF_W<'_, ICRrs> {
        RWEIF_W::new(self, 1)
    }
    ///Bit 2 - Key error interrupt flag clear Setting this bit clears the KEIF status bit of the SAES_ISR register.
    #[inline(always)]
    pub fn keif(&mut self) -> KEIF_W<'_, ICRrs> {
        KEIF_W::new(self, 2)
    }
    ///Bit 3 - RNG error interrupt flag clear Application must set this bit to clear the RNGEIF status bit in SAES_ISR register.
    #[inline(always)]
    pub fn rngeif(&mut self) -> RNGEIF_W<'_, ICRrs> {
        RNGEIF_W::new(self, 3)
    }
}
/**SAES interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SAES:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
