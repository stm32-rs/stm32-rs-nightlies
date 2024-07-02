///Register `DDRCTRL_DRAMTMG8` reader
pub type R = crate::R<DDRCTRL_DRAMTMG8rs>;
///Register `DDRCTRL_DRAMTMG8` writer
pub type W = crate::W<DDRCTRL_DRAMTMG8rs>;
///Field `T_XS_X32` reader - T_XS_X32
pub type T_XS_X32_R = crate::FieldReader;
///Field `T_XS_X32` writer - T_XS_X32
pub type T_XS_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `T_XS_DLL_X32` reader - T_XS_DLL_X32
pub type T_XS_DLL_X32_R = crate::FieldReader;
///Field `T_XS_DLL_X32` writer - T_XS_DLL_X32
pub type T_XS_DLL_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - T_XS_X32
    #[inline(always)]
    pub fn t_xs_x32(&self) -> T_XS_X32_R {
        T_XS_X32_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:14 - T_XS_DLL_X32
    #[inline(always)]
    pub fn t_xs_dll_x32(&self) -> T_XS_DLL_X32_R {
        T_XS_DLL_X32_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRCTRL_DRAMTMG8")
            .field("t_xs_x32", &self.t_xs_x32())
            .field("t_xs_dll_x32", &self.t_xs_dll_x32())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - T_XS_X32
    #[inline(always)]
    #[must_use]
    pub fn t_xs_x32(&mut self) -> T_XS_X32_W<DDRCTRL_DRAMTMG8rs> {
        T_XS_X32_W::new(self, 0)
    }
    ///Bits 8:14 - T_XS_DLL_X32
    #[inline(always)]
    #[must_use]
    pub fn t_xs_dll_x32(&mut self) -> T_XS_DLL_X32_W<DDRCTRL_DRAMTMG8rs> {
        T_XS_DLL_X32_W::new(self, 8)
    }
}
/**DDRCTRL SDRAM timing register 8

You can [`read`](crate::Reg::read) this register and get [`ddrctrl_dramtmg8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrctrl_dramtmg8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DDRCTRL_DRAMTMG8)*/
pub struct DDRCTRL_DRAMTMG8rs;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG8rs {
    type Ux = u32;
}
///`read()` method returns [`ddrctrl_dramtmg8::R`](R) reader structure
impl crate::Readable for DDRCTRL_DRAMTMG8rs {}
///`write(|w| ..)` method takes [`ddrctrl_dramtmg8::W`](W) writer structure
impl crate::Writable for DDRCTRL_DRAMTMG8rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRCTRL_DRAMTMG8 to value 0x4405
impl crate::Resettable for DDRCTRL_DRAMTMG8rs {
    const RESET_VALUE: u32 = 0x4405;
}
