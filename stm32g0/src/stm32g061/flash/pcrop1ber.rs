///Register `PCROP1BER` reader
pub type R = crate::R<PCROP1BERrs>;
///Register `PCROP1BER` writer
pub type W = crate::W<PCROP1BERrs>;
///Field `PCROP1B_END` reader - PCROP1B area end offset
pub type PCROP1B_END_R = crate::FieldReader<u16>;
///Field `PCROP1B_END` writer - PCROP1B area end offset
pub type PCROP1B_END_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - PCROP1B area end offset
    #[inline(always)]
    pub fn pcrop1b_end(&self) -> PCROP1B_END_R {
        PCROP1B_END_R::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCROP1BER")
            .field("pcrop1b_end", &self.pcrop1b_end())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - PCROP1B area end offset
    #[inline(always)]
    pub fn pcrop1b_end(&mut self) -> PCROP1B_END_W<PCROP1BERrs> {
        PCROP1B_END_W::new(self, 0)
    }
}
/**Flash PCROP area B End address register

You can [`read`](crate::Reg::read) this register and get [`pcrop1ber::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrop1ber::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#FLASH:PCROP1BER)*/
pub struct PCROP1BERrs;
impl crate::RegisterSpec for PCROP1BERrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop1ber::R`](R) reader structure
impl crate::Readable for PCROP1BERrs {}
///`write(|w| ..)` method takes [`pcrop1ber::W`](W) writer structure
impl crate::Writable for PCROP1BERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PCROP1BER to value 0xf000_0000
impl crate::Resettable for PCROP1BERrs {
    const RESET_VALUE: u32 = 0xf000_0000;
}