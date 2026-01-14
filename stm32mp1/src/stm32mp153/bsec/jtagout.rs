///Register `JTAGOUT` reader
pub type R = crate::R<JTAGOUTrs>;
///Register `JTAGOUT` writer
pub type W = crate::W<JTAGOUTrs>;
///Field `DATA` reader - DATA
pub type DATA_R = crate::FieldReader<u16>;
///Field `DATA` writer - DATA
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - DATA
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JTAGOUT")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - DATA
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<'_, JTAGOUTrs> {
        DATA_W::new(self, 0)
    }
}
/**BSEC JTAG output register

You can [`read`](crate::Reg::read) this register and get [`jtagout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtagout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:JTAGOUT)*/
pub struct JTAGOUTrs;
impl crate::RegisterSpec for JTAGOUTrs {
    type Ux = u32;
}
///`read()` method returns [`jtagout::R`](R) reader structure
impl crate::Readable for JTAGOUTrs {}
///`write(|w| ..)` method takes [`jtagout::W`](W) writer structure
impl crate::Writable for JTAGOUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JTAGOUT to value 0
impl crate::Resettable for JTAGOUTrs {}
