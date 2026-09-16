///Register `EPOCH_SELR` reader
pub type R = crate::R<EPOCH_SELRrs>;
///Register `EPOCH_SELR` writer
pub type W = crate::W<EPOCH_SELRrs>;
///Field `EPSEL` reader - Epoch selection. This value is wired out to the SAES peripheral.
pub type EPSEL_R = crate::BitReader;
///Field `EPSEL` writer - Epoch selection. This value is wired out to the SAES peripheral.
pub type EPSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Epoch selection. This value is wired out to the SAES peripheral.
    #[inline(always)]
    pub fn epsel(&self) -> EPSEL_R {
        EPSEL_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPOCH_SELR")
            .field("epsel", &self.epsel())
            .finish()
    }
}
impl W {
    ///Bit 0 - Epoch selection. This value is wired out to the SAES peripheral.
    #[inline(always)]
    pub fn epsel(&mut self) -> EPSEL_W<'_, EPOCH_SELRrs> {
        EPSEL_W::new(self, 0)
    }
}
/**BSEC epoch select register

You can [`read`](crate::Reg::read) this register and get [`epoch_selr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epoch_selr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:EPOCH_SELR)*/
pub struct EPOCH_SELRrs;
impl crate::RegisterSpec for EPOCH_SELRrs {
    type Ux = u32;
}
///`read()` method returns [`epoch_selr::R`](R) reader structure
impl crate::Readable for EPOCH_SELRrs {}
///`write(|w| ..)` method takes [`epoch_selr::W`](W) writer structure
impl crate::Writable for EPOCH_SELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EPOCH_SELR to value 0
impl crate::Resettable for EPOCH_SELRrs {}
