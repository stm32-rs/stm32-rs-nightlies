///Register `DDRCTRL_DRAMTMG14` reader
pub type R = crate::R<DDRCTRL_DRAMTMG14rs>;
///Register `DDRCTRL_DRAMTMG14` writer
pub type W = crate::W<DDRCTRL_DRAMTMG14rs>;
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
        f.debug_struct("DDRCTRL_DRAMTMG14")
            .field("t_xsr", &self.t_xsr())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - T_XSR
    #[inline(always)]
    #[must_use]
    pub fn t_xsr(&mut self) -> T_XSR_W<DDRCTRL_DRAMTMG14rs> {
        T_XSR_W::new(self, 0)
    }
}
/**DDRCTRL SDRAM timing register 14

You can [`read`](crate::Reg::read) this register and get [`ddrctrl_dramtmg14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrctrl_dramtmg14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DDRCTRL_DRAMTMG14)*/
pub struct DDRCTRL_DRAMTMG14rs;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG14rs {
    type Ux = u32;
}
///`read()` method returns [`ddrctrl_dramtmg14::R`](R) reader structure
impl crate::Readable for DDRCTRL_DRAMTMG14rs {}
///`write(|w| ..)` method takes [`ddrctrl_dramtmg14::W`](W) writer structure
impl crate::Writable for DDRCTRL_DRAMTMG14rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRCTRL_DRAMTMG14 to value 0xa0
impl crate::Resettable for DDRCTRL_DRAMTMG14rs {
    const RESET_VALUE: u32 = 0xa0;
}
