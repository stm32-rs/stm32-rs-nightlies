///Register `RCC_TZCR` reader
pub type R = crate::R<RCC_TZCRrs>;
///Register `RCC_TZCR` writer
pub type W = crate::W<RCC_TZCRrs>;
///Field `TZEN` reader - TZEN
pub type TZEN_R = crate::BitReader;
///Field `TZEN` writer - TZEN
pub type TZEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCKPROT` reader - MCKPROT
pub type MCKPROT_R = crate::BitReader;
///Field `MCKPROT` writer - MCKPROT
pub type MCKPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TZEN
    #[inline(always)]
    pub fn tzen(&self) -> TZEN_R {
        TZEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MCKPROT
    #[inline(always)]
    pub fn mckprot(&self) -> MCKPROT_R {
        MCKPROT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TZCR")
            .field("tzen", &self.tzen())
            .field("mckprot", &self.mckprot())
            .finish()
    }
}
impl W {
    ///Bit 0 - TZEN
    #[inline(always)]
    #[must_use]
    pub fn tzen(&mut self) -> TZEN_W<RCC_TZCRrs> {
        TZEN_W::new(self, 0)
    }
    ///Bit 1 - MCKPROT
    #[inline(always)]
    #[must_use]
    pub fn mckprot(&mut self) -> MCKPROT_W<RCC_TZCRrs> {
        MCKPROT_W::new(self, 1)
    }
}
/**This register is used to switch the RCC into secure mode. This register can only be accessed in secure mode.

You can [`read`](crate::Reg::read) this register and get [`rcc_tzcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_tzcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RCC_TZCR)*/
pub struct RCC_TZCRrs;
impl crate::RegisterSpec for RCC_TZCRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_tzcr::R`](R) reader structure
impl crate::Readable for RCC_TZCRrs {}
///`write(|w| ..)` method takes [`rcc_tzcr::W`](W) writer structure
impl crate::Writable for RCC_TZCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_TZCR to value 0x03
impl crate::Resettable for RCC_TZCRrs {
    const RESET_VALUE: u32 = 0x03;
}
