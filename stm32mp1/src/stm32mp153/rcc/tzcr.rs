///Register `TZCR` reader
pub type R = crate::R<TZCRrs>;
///Register `TZCR` writer
pub type W = crate::W<TZCRrs>;
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
        f.debug_struct("TZCR")
            .field("tzen", &self.tzen())
            .field("mckprot", &self.mckprot())
            .finish()
    }
}
impl W {
    ///Bit 0 - TZEN
    #[inline(always)]
    pub fn tzen(&mut self) -> TZEN_W<'_, TZCRrs> {
        TZEN_W::new(self, 0)
    }
    ///Bit 1 - MCKPROT
    #[inline(always)]
    pub fn mckprot(&mut self) -> MCKPROT_W<'_, TZCRrs> {
        MCKPROT_W::new(self, 1)
    }
}
/**This register is used to switch the RCC into secure mode. This register can only be accessed in secure mode.

You can [`read`](crate::Reg::read) this register and get [`tzcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:TZCR)*/
pub struct TZCRrs;
impl crate::RegisterSpec for TZCRrs {
    type Ux = u32;
}
///`read()` method returns [`tzcr::R`](R) reader structure
impl crate::Readable for TZCRrs {}
///`write(|w| ..)` method takes [`tzcr::W`](W) writer structure
impl crate::Writable for TZCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TZCR to value 0x03
impl crate::Resettable for TZCRrs {
    const RESET_VALUE: u32 = 0x03;
}
