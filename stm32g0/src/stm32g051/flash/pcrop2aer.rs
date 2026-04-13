///Register `PCROP2AER` reader
pub type R = crate::R<PCROP2AERrs>;
///Register `PCROP2AER` writer
pub type W = crate::W<PCROP2AERrs>;
///Field `PCROP2A_END` reader - PCROP2A area end offset, bank2
pub type PCROP2A_END_R = crate::FieldReader<u16>;
///Field `PCROP2A_END` writer - PCROP2A area end offset, bank2
pub type PCROP2A_END_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - PCROP2A area end offset, bank2
    #[inline(always)]
    pub fn pcrop2a_end(&self) -> PCROP2A_END_R {
        PCROP2A_END_R::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCROP2AER")
            .field("pcrop2a_end", &self.pcrop2a_end())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - PCROP2A area end offset, bank2
    #[inline(always)]
    pub fn pcrop2a_end(&mut self) -> PCROP2A_END_W<'_, PCROP2AERrs> {
        PCROP2A_END_W::new(self, 0)
    }
}
/**Flash PCROP2 area A end address register

You can [`read`](crate::Reg::read) this register and get [`pcrop2aer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop2aer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#FLASH:PCROP2AER)*/
pub struct PCROP2AERrs;
impl crate::RegisterSpec for PCROP2AERrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop2aer::R`](R) reader structure
impl crate::Readable for PCROP2AERrs {}
///`write(|w| ..)` method takes [`pcrop2aer::W`](W) writer structure
impl crate::Writable for PCROP2AERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCROP2AER to value 0
impl crate::Resettable for PCROP2AERrs {}
