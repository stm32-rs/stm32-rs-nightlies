///Register `AES_IER` reader
pub type R = crate::R<AES_IERrs>;
///Register `AES_IER` writer
pub type W = crate::W<AES_IERrs>;
///Field `CCFIE` reader - Computation complete flag interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set.
pub type CCFIE_R = crate::BitReader;
///Field `CCFIE` writer - Computation complete flag interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set.
pub type CCFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWEIE` reader - Read or write error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RWEIF (read and/or write error flag) is set.
pub type RWEIE_R = crate::BitReader;
///Field `RWEIE` writer - Read or write error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RWEIF (read and/or write error flag) is set.
pub type RWEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEIE` reader - Key error interrupt enable This bit enables or disables (masks) the AES interrupt generation when KEIF (key error flag) is set.
pub type KEIE_R = crate::BitReader;
///Field `KEIE` writer - Key error interrupt enable This bit enables or disables (masks) the AES interrupt generation when KEIF (key error flag) is set.
pub type KEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Computation complete flag interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set.
    #[inline(always)]
    pub fn ccfie(&self) -> CCFIE_R {
        CCFIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Read or write error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RWEIF (read and/or write error flag) is set.
    #[inline(always)]
    pub fn rweie(&self) -> RWEIE_R {
        RWEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Key error interrupt enable This bit enables or disables (masks) the AES interrupt generation when KEIF (key error flag) is set.
    #[inline(always)]
    pub fn keie(&self) -> KEIE_R {
        KEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AES_IER")
            .field("ccfie", &self.ccfie())
            .field("rweie", &self.rweie())
            .field("keie", &self.keie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Computation complete flag interrupt enable This bit enables or disables (masks) the AES interrupt generation when CCF (computation complete flag) is set.
    #[inline(always)]
    pub fn ccfie(&mut self) -> CCFIE_W<AES_IERrs> {
        CCFIE_W::new(self, 0)
    }
    ///Bit 1 - Read or write error interrupt enable This bit enables or disables (masks) the AES interrupt generation when RWEIF (read and/or write error flag) is set.
    #[inline(always)]
    pub fn rweie(&mut self) -> RWEIE_W<AES_IERrs> {
        RWEIE_W::new(self, 1)
    }
    ///Bit 2 - Key error interrupt enable This bit enables or disables (masks) the AES interrupt generation when KEIF (key error flag) is set.
    #[inline(always)]
    pub fn keie(&mut self) -> KEIE_W<AES_IERrs> {
        KEIE_W::new(self, 2)
    }
}
/**AES interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`aes_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_IER)*/
pub struct AES_IERrs;
impl crate::RegisterSpec for AES_IERrs {
    type Ux = u32;
}
///`read()` method returns [`aes_ier::R`](R) reader structure
impl crate::Readable for AES_IERrs {}
///`write(|w| ..)` method takes [`aes_ier::W`](W) writer structure
impl crate::Writable for AES_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AES_IER to value 0
impl crate::Resettable for AES_IERrs {
    const RESET_VALUE: u32 = 0;
}