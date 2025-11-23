///Register `PCROP2BER` reader
pub type R = crate::R<PCROP2BERrs>;
///Register `PCROP2BER` writer
pub type W = crate::W<PCROP2BERrs>;
///Field `PCROP2B_END` reader - PCROP2B area end offset, Bank 2
pub type PCROP2B_END_R = crate::FieldReader<u16>;
///Field `PCROP2B_END` writer - PCROP2B area end offset, Bank 2
pub type PCROP2B_END_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - PCROP2B area end offset, Bank 2
    #[inline(always)]
    pub fn pcrop2b_end(&self) -> PCROP2B_END_R {
        PCROP2B_END_R::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCROP2BER")
            .field("pcrop2b_end", &self.pcrop2b_end())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - PCROP2B area end offset, Bank 2
    #[inline(always)]
    pub fn pcrop2b_end(&mut self) -> PCROP2B_END_W<'_, PCROP2BERrs> {
        PCROP2B_END_W::new(self, 0)
    }
}
/**FLASH PCROP2 area B end address register

You can [`read`](crate::Reg::read) this register and get [`pcrop2ber::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop2ber::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#FLASH:PCROP2BER)*/
pub struct PCROP2BERrs;
impl crate::RegisterSpec for PCROP2BERrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop2ber::R`](R) reader structure
impl crate::Readable for PCROP2BERrs {}
///`write(|w| ..)` method takes [`pcrop2ber::W`](W) writer structure
impl crate::Writable for PCROP2BERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCROP2BER to value 0
impl crate::Resettable for PCROP2BERrs {}
