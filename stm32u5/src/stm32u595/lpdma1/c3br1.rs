///Register `C3BR1` reader
pub type R = crate::R<C3BR1rs>;
///Register `C3BR1` writer
pub type W = crate::W<C3BR1rs>;
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
        f.debug_struct("C3BR1").field("bndt", &self.bndt()).finish()
    }
}
impl W {
    ///Bits 0:15 - block number of data bytes to transfer from the source
    #[inline(always)]
    pub fn bndt(&mut self) -> BNDT_W<C3BR1rs> {
        BNDT_W::new(self, 0)
    }
}
/**LPDMA channel x block register 1

You can [`read`](crate::Reg::read) this register and get [`c3br1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3br1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:C3BR1)*/
pub struct C3BR1rs;
impl crate::RegisterSpec for C3BR1rs {
    type Ux = u32;
}
///`read()` method returns [`c3br1::R`](R) reader structure
impl crate::Readable for C3BR1rs {}
///`write(|w| ..)` method takes [`c3br1::W`](W) writer structure
impl crate::Writable for C3BR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C3BR1 to value 0
impl crate::Resettable for C3BR1rs {
    const RESET_VALUE: u32 = 0;
}