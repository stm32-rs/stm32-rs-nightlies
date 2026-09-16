///Register `PCROP2SR` reader
pub type R = crate::R<PCROP2SRrs>;
///Register `PCROP2SR` writer
pub type W = crate::W<PCROP2SRrs>;
///Field `PCROP2_STRT` reader - PCROP area start offset
pub type PCROP2_STRT_R = crate::FieldReader<u16>;
///Field `PCROP2_STRT` writer - PCROP area start offset
pub type PCROP2_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:14 - PCROP area start offset
    #[inline(always)]
    pub fn pcrop2_strt(&self) -> PCROP2_STRT_R {
        PCROP2_STRT_R::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCROP2SR")
            .field("pcrop2_strt", &self.pcrop2_strt())
            .finish()
    }
}
impl W {
    ///Bits 0:14 - PCROP area start offset
    #[inline(always)]
    pub fn pcrop2_strt(&mut self) -> PCROP2_STRT_W<'_, PCROP2SRrs> {
        PCROP2_STRT_W::new(self, 0)
    }
}
/**Flash PCROP2 Start address register

You can [`read`](crate::Reg::read) this register and get [`pcrop2sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop2sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#FLASH:PCROP2SR)*/
pub struct PCROP2SRrs;
impl crate::RegisterSpec for PCROP2SRrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop2sr::R`](R) reader structure
impl crate::Readable for PCROP2SRrs {}
///`write(|w| ..)` method takes [`pcrop2sr::W`](W) writer structure
impl crate::Writable for PCROP2SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCROP2SR to value 0xffff_0000
impl crate::Resettable for PCROP2SRrs {
    const RESET_VALUE: u32 = 0xffff_0000;
}
