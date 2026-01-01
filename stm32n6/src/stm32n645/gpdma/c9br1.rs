///Register `C9BR1` reader
pub type R = crate::R<C9BR1rs>;
///Register `C9BR1` writer
pub type W = crate::W<C9BR1rs>;
///Field `BNDT` reader - block number of data bytes to transfer from the source
pub type BNDT_R = crate::FieldReader<u16>;
///Field `BNDT` writer - block number of data bytes to transfer from the source
pub type BNDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - block number of data bytes to transfer from the source
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C9BR1").field("bndt", &self.bndt()).finish()
    }
}
impl W {
    ///Bits 0:15 - block number of data bytes to transfer from the source
    #[inline(always)]
    pub fn bndt(&mut self) -> BNDT_W<'_, C9BR1rs> {
        BNDT_W::new(self, 0)
    }
}
/**GPDMA channel 9 block register 1

You can [`read`](crate::Reg::read) this register and get [`c9br1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9br1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPDMA:C9BR1)*/
pub struct C9BR1rs;
impl crate::RegisterSpec for C9BR1rs {
    type Ux = u32;
}
///`read()` method returns [`c9br1::R`](R) reader structure
impl crate::Readable for C9BR1rs {}
///`write(|w| ..)` method takes [`c9br1::W`](W) writer structure
impl crate::Writable for C9BR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C9BR1 to value 0
impl crate::Resettable for C9BR1rs {}
