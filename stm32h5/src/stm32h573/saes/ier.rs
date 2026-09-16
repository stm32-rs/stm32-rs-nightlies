///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `CCFIE` reader - Computation complete flag interrupt enable This bit enables or disables (masks) the SAES interrupt generation when CCF (computation complete flag) is set.
pub type CCFIE_R = crate::BitReader;
///Field `CCFIE` writer - Computation complete flag interrupt enable This bit enables or disables (masks) the SAES interrupt generation when CCF (computation complete flag) is set.
pub type CCFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWEIE` reader - Read or write error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RWEIF (read and/or write error flag) is set.
pub type RWEIE_R = crate::BitReader;
///Field `RWEIE` writer - Read or write error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RWEIF (read and/or write error flag) is set.
pub type RWEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEIE` reader - Key error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when KEIF (key error flag) is set.
pub type KEIE_R = crate::BitReader;
///Field `KEIE` writer - Key error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when KEIF (key error flag) is set.
pub type KEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGEIE` reader - RNG error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RNGEIF (RNG error flag) is set.
pub type RNGEIE_R = crate::BitReader;
///Field `RNGEIE` writer - RNG error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RNGEIF (RNG error flag) is set.
pub type RNGEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Computation complete flag interrupt enable This bit enables or disables (masks) the SAES interrupt generation when CCF (computation complete flag) is set.
    #[inline(always)]
    pub fn ccfie(&self) -> CCFIE_R {
        CCFIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Read or write error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RWEIF (read and/or write error flag) is set.
    #[inline(always)]
    pub fn rweie(&self) -> RWEIE_R {
        RWEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Key error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when KEIF (key error flag) is set.
    #[inline(always)]
    pub fn keie(&self) -> KEIE_R {
        KEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RNG error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RNGEIF (RNG error flag) is set.
    #[inline(always)]
    pub fn rngeie(&self) -> RNGEIE_R {
        RNGEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("ccfie", &self.ccfie())
            .field("rweie", &self.rweie())
            .field("keie", &self.keie())
            .field("rngeie", &self.rngeie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Computation complete flag interrupt enable This bit enables or disables (masks) the SAES interrupt generation when CCF (computation complete flag) is set.
    #[inline(always)]
    pub fn ccfie(&mut self) -> CCFIE_W<'_, IERrs> {
        CCFIE_W::new(self, 0)
    }
    ///Bit 1 - Read or write error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RWEIF (read and/or write error flag) is set.
    #[inline(always)]
    pub fn rweie(&mut self) -> RWEIE_W<'_, IERrs> {
        RWEIE_W::new(self, 1)
    }
    ///Bit 2 - Key error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when KEIF (key error flag) is set.
    #[inline(always)]
    pub fn keie(&mut self) -> KEIE_W<'_, IERrs> {
        KEIE_W::new(self, 2)
    }
    ///Bit 3 - RNG error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RNGEIF (RNG error flag) is set.
    #[inline(always)]
    pub fn rngeie(&mut self) -> RNGEIE_W<'_, IERrs> {
        RNGEIE_W::new(self, 3)
    }
}
/**SAES interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SAES:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
