///Register `PCROP1SR` reader
pub type R = crate::R<PCROP1SRrs>;
///Register `PCROP1SR` writer
pub type W = crate::W<PCROP1SRrs>;
///Field `PCROP1_STRT` reader - PCROP area start offset
pub type PCROP1_STRT_R = crate::FieldReader<u16>;
///Field `PCROP1_STRT` writer - PCROP area start offset
pub type PCROP1_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:14 - PCROP area start offset
    #[inline(always)]
    pub fn pcrop1_strt(&self) -> PCROP1_STRT_R {
        PCROP1_STRT_R::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCROP1SR")
            .field("pcrop1_strt", &self.pcrop1_strt())
            .finish()
    }
}
impl W {
    ///Bits 0:14 - PCROP area start offset
    #[inline(always)]
    pub fn pcrop1_strt(&mut self) -> PCROP1_STRT_W<'_, PCROP1SRrs> {
        PCROP1_STRT_W::new(self, 0)
    }
}
/**Flash PCROP1 Start address register

You can [`read`](crate::Reg::read) this register and get [`pcrop1sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483.html#FLASH:PCROP1SR)*/
pub struct PCROP1SRrs;
impl crate::RegisterSpec for PCROP1SRrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop1sr::R`](R) reader structure
impl crate::Readable for PCROP1SRrs {}
///`write(|w| ..)` method takes [`pcrop1sr::W`](W) writer structure
impl crate::Writable for PCROP1SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCROP1SR to value 0xffff_0000
impl crate::Resettable for PCROP1SRrs {
    const RESET_VALUE: u32 = 0xffff_0000;
}
