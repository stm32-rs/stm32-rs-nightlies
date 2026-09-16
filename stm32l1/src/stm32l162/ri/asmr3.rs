///Register `ASMR3` reader
pub type R = crate::R<ASMR3rs>;
///Register `ASMR3` writer
pub type W = crate::W<ASMR3rs>;
///Field `PC` reader - Port C analog switch mode selection
pub type PC_R = crate::FieldReader<u16>;
///Field `PC` writer - Port C analog switch mode selection
pub type PC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Port C analog switch mode selection
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASMR3").field("pc", &self.pc()).finish()
    }
}
impl W {
    ///Bits 0:15 - Port C analog switch mode selection
    #[inline(always)]
    pub fn pc(&mut self) -> PC_W<'_, ASMR3rs> {
        PC_W::new(self, 0)
    }
}
/**Analog switch mode register

You can [`read`](crate::Reg::read) this register and get [`asmr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asmr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#RI:ASMR3)*/
pub struct ASMR3rs;
impl crate::RegisterSpec for ASMR3rs {
    type Ux = u32;
}
///`read()` method returns [`asmr3::R`](R) reader structure
impl crate::Readable for ASMR3rs {}
///`write(|w| ..)` method takes [`asmr3::W`](W) writer structure
impl crate::Writable for ASMR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ASMR3 to value 0
impl crate::Resettable for ASMR3rs {}
