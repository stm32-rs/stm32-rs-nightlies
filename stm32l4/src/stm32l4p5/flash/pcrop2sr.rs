///Register `PCROP2SR` reader
pub type R = crate::R<PCROP2SRrs>;
///Register `PCROP2SR` writer
pub type W = crate::W<PCROP2SRrs>;
///Field `PCROP2_STRT` reader - Bank 2 PCROP area start offset
pub type PCROP2_STRT_R = crate::FieldReader<u32>;
///Field `PCROP2_STRT` writer - Bank 2 PCROP area start offset
pub type PCROP2_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32, crate::Safe>;
impl R {
    ///Bits 0:16 - Bank 2 PCROP area start offset
    #[inline(always)]
    pub fn pcrop2_strt(&self) -> PCROP2_STRT_R {
        PCROP2_STRT_R::new(self.bits & 0x0001_ffff)
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
    ///Bits 0:16 - Bank 2 PCROP area start offset
    #[inline(always)]
    pub fn pcrop2_strt(&mut self) -> PCROP2_STRT_W<PCROP2SRrs> {
        PCROP2_STRT_W::new(self, 0)
    }
}
/**Flash Bank 2 PCROP Start address register

You can [`read`](crate::Reg::read) this register and get [`pcrop2sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop2sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#FLASH:PCROP2SR)*/
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
