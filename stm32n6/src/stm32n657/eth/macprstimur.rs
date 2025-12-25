///Register `MACPRSTIMUR` reader
pub type R = crate::R<MACPRSTIMURrs>;
///Register `MACPRSTIMUR` writer
pub type W = crate::W<MACPRSTIMURrs>;
///Field `MPTU` reader - MAC 1722 Presentation Time Update
pub type MPTU_R = crate::FieldReader<u32>;
///Field `MPTU` writer - MAC 1722 Presentation Time Update
pub type MPTU_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - MAC 1722 Presentation Time Update
    #[inline(always)]
    pub fn mptu(&self) -> MPTU_R {
        MPTU_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPRSTIMUR")
            .field("mptu", &self.mptu())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - MAC 1722 Presentation Time Update
    #[inline(always)]
    pub fn mptu(&mut self) -> MPTU_W<'_, MACPRSTIMURrs> {
        MPTU_W::new(self, 0)
    }
}
/**MAC presentation time update register

You can [`read`](crate::Reg::read) this register and get [`macprstimur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macprstimur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:MACPRSTIMUR)*/
pub struct MACPRSTIMURrs;
impl crate::RegisterSpec for MACPRSTIMURrs {
    type Ux = u32;
}
///`read()` method returns [`macprstimur::R`](R) reader structure
impl crate::Readable for MACPRSTIMURrs {}
///`write(|w| ..)` method takes [`macprstimur::W`](W) writer structure
impl crate::Writable for MACPRSTIMURrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPRSTIMUR to value 0
impl crate::Resettable for MACPRSTIMURrs {}
