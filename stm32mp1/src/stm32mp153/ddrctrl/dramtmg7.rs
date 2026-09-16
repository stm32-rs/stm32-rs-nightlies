///Register `DRAMTMG7` reader
pub type R = crate::R<DRAMTMG7rs>;
///Register `DRAMTMG7` writer
pub type W = crate::W<DRAMTMG7rs>;
///Field `T_CKPDX` reader - T_CKPDX
pub type T_CKPDX_R = crate::FieldReader;
///Field `T_CKPDX` writer - T_CKPDX
pub type T_CKPDX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `T_CKPDE` reader - T_CKPDE
pub type T_CKPDE_R = crate::FieldReader;
///Field `T_CKPDE` writer - T_CKPDE
pub type T_CKPDE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - T_CKPDX
    #[inline(always)]
    pub fn t_ckpdx(&self) -> T_CKPDX_R {
        T_CKPDX_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - T_CKPDE
    #[inline(always)]
    pub fn t_ckpde(&self) -> T_CKPDE_R {
        T_CKPDE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DRAMTMG7")
            .field("t_ckpdx", &self.t_ckpdx())
            .field("t_ckpde", &self.t_ckpde())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - T_CKPDX
    #[inline(always)]
    pub fn t_ckpdx(&mut self) -> T_CKPDX_W<'_, DRAMTMG7rs> {
        T_CKPDX_W::new(self, 0)
    }
    ///Bits 8:11 - T_CKPDE
    #[inline(always)]
    pub fn t_ckpde(&mut self) -> T_CKPDE_W<'_, DRAMTMG7rs> {
        T_CKPDE_W::new(self, 8)
    }
}
/**DDRCTRL SDRAM timing register 7

You can [`read`](crate::Reg::read) this register and get [`dramtmg7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DRAMTMG7)*/
pub struct DRAMTMG7rs;
impl crate::RegisterSpec for DRAMTMG7rs {
    type Ux = u32;
}
///`read()` method returns [`dramtmg7::R`](R) reader structure
impl crate::Readable for DRAMTMG7rs {}
///`write(|w| ..)` method takes [`dramtmg7::W`](W) writer structure
impl crate::Writable for DRAMTMG7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DRAMTMG7 to value 0x0202
impl crate::Resettable for DRAMTMG7rs {
    const RESET_VALUE: u32 = 0x0202;
}
