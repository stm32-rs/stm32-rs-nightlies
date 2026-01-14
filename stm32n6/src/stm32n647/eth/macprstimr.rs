///Register `MACPRSTIMR` reader
pub type R = crate::R<MACPRSTIMRrs>;
///Register `MACPRSTIMR` writer
pub type W = crate::W<MACPRSTIMRrs>;
///Field `MPTN` reader - MAC 1722 Presentation Time in ns
pub type MPTN_R = crate::FieldReader<u32>;
///Field `MPTN` writer - MAC 1722 Presentation Time in ns
pub type MPTN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - MAC 1722 Presentation Time in ns
    #[inline(always)]
    pub fn mptn(&self) -> MPTN_R {
        MPTN_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPRSTIMR")
            .field("mptn", &self.mptn())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - MAC 1722 Presentation Time in ns
    #[inline(always)]
    pub fn mptn(&mut self) -> MPTN_W<'_, MACPRSTIMRrs> {
        MPTN_W::new(self, 0)
    }
}
/**MAC presentation time register

You can [`read`](crate::Reg::read) this register and get [`macprstimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macprstimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ETH:MACPRSTIMR)*/
pub struct MACPRSTIMRrs;
impl crate::RegisterSpec for MACPRSTIMRrs {
    type Ux = u32;
}
///`read()` method returns [`macprstimr::R`](R) reader structure
impl crate::Readable for MACPRSTIMRrs {}
///`write(|w| ..)` method takes [`macprstimr::W`](W) writer structure
impl crate::Writable for MACPRSTIMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPRSTIMR to value 0
impl crate::Resettable for MACPRSTIMRrs {}
