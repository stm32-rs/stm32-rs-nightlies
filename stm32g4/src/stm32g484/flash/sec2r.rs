///Register `SEC2R` reader
pub type R = crate::R<SEC2Rrs>;
///Register `SEC2R` writer
pub type W = crate::W<SEC2Rrs>;
///Field `SEC_SIZE2` reader - sets the number of pages used in the bank 2 Securable memory area.
pub type SEC_SIZE2_R = crate::FieldReader;
///Field `SEC_SIZE2` writer - sets the number of pages used in the bank 2 Securable memory area.
pub type SEC_SIZE2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - sets the number of pages used in the bank 2 Securable memory area.
    #[inline(always)]
    pub fn sec_size2(&self) -> SEC_SIZE2_R {
        SEC_SIZE2_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC2R")
            .field("sec_size2", &self.sec_size2())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - sets the number of pages used in the bank 2 Securable memory area.
    #[inline(always)]
    pub fn sec_size2(&mut self) -> SEC_SIZE2_W<'_, SEC2Rrs> {
        SEC_SIZE2_W::new(self, 0)
    }
}
/**Flash Securable area bank2 register

You can [`read`](crate::Reg::read) this register and get [`sec2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#FLASH:SEC2R)*/
pub struct SEC2Rrs;
impl crate::RegisterSpec for SEC2Rrs {
    type Ux = u32;
}
///`read()` method returns [`sec2r::R`](R) reader structure
impl crate::Readable for SEC2Rrs {}
///`write(|w| ..)` method takes [`sec2r::W`](W) writer structure
impl crate::Writable for SEC2Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SEC2R to value 0xffff_ff00
impl crate::Resettable for SEC2Rrs {
    const RESET_VALUE: u32 = 0xffff_ff00;
}
