///Register `PCROP2BSR` reader
pub type R = crate::R<PCROP2BSRrs>;
///Register `PCROP2BSR` writer
pub type W = crate::W<PCROP2BSRrs>;
///Field `PCROP2B_STRT` reader - PCROP2B area start offset, Bank 2
pub type PCROP2B_STRT_R = crate::FieldReader<u16>;
///Field `PCROP2B_STRT` writer - PCROP2B area start offset, Bank 2
pub type PCROP2B_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - PCROP2B area start offset, Bank 2
    #[inline(always)]
    pub fn pcrop2b_strt(&self) -> PCROP2B_STRT_R {
        PCROP2B_STRT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCROP2BSR")
            .field("pcrop2b_strt", &self.pcrop2b_strt())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - PCROP2B area start offset, Bank 2
    #[inline(always)]
    pub fn pcrop2b_strt(&mut self) -> PCROP2B_STRT_W<'_, PCROP2BSRrs> {
        PCROP2B_STRT_W::new(self, 0)
    }
}
/**FLASH PCROP2 area B start address register

You can [`read`](crate::Reg::read) this register and get [`pcrop2bsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop2bsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#FLASH:PCROP2BSR)*/
pub struct PCROP2BSRrs;
impl crate::RegisterSpec for PCROP2BSRrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop2bsr::R`](R) reader structure
impl crate::Readable for PCROP2BSRrs {}
///`write(|w| ..)` method takes [`pcrop2bsr::W`](W) writer structure
impl crate::Writable for PCROP2BSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCROP2BSR to value 0
impl crate::Resettable for PCROP2BSRrs {}
