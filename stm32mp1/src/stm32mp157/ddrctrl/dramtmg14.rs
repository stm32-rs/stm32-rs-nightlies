///Register `DRAMTMG14` reader
pub type R = crate::R<DRAMTMG14rs>;
///Register `DRAMTMG14` writer
pub type W = crate::W<DRAMTMG14rs>;
///Field `T_XSR` reader - T_XSR
pub type T_XSR_R = crate::FieldReader<u16>;
///Field `T_XSR` writer - T_XSR
pub type T_XSR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - T_XSR
    #[inline(always)]
    pub fn t_xsr(&self) -> T_XSR_R {
        T_XSR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DRAMTMG14")
            .field("t_xsr", &self.t_xsr())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - T_XSR
    #[inline(always)]
    pub fn t_xsr(&mut self) -> T_XSR_W<'_, DRAMTMG14rs> {
        T_XSR_W::new(self, 0)
    }
}
/**DDRCTRL SDRAM timing register 14

You can [`read`](crate::Reg::read) this register and get [`dramtmg14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DRAMTMG14)*/
pub struct DRAMTMG14rs;
impl crate::RegisterSpec for DRAMTMG14rs {
    type Ux = u32;
}
///`read()` method returns [`dramtmg14::R`](R) reader structure
impl crate::Readable for DRAMTMG14rs {}
///`write(|w| ..)` method takes [`dramtmg14::W`](W) writer structure
impl crate::Writable for DRAMTMG14rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DRAMTMG14 to value 0xa0
impl crate::Resettable for DRAMTMG14rs {
    const RESET_VALUE: u32 = 0xa0;
}
