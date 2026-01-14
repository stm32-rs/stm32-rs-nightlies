///Register `DRAMTMG6` reader
pub type R = crate::R<DRAMTMG6rs>;
///Register `DRAMTMG6` writer
pub type W = crate::W<DRAMTMG6rs>;
///Field `T_CKCSX` reader - T_CKCSX
pub type T_CKCSX_R = crate::FieldReader;
///Field `T_CKCSX` writer - T_CKCSX
pub type T_CKCSX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `T_CKDPDX` reader - T_CKDPDX
pub type T_CKDPDX_R = crate::FieldReader;
///Field `T_CKDPDX` writer - T_CKDPDX
pub type T_CKDPDX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `T_CKDPDE` reader - T_CKDPDE
pub type T_CKDPDE_R = crate::FieldReader;
///Field `T_CKDPDE` writer - T_CKDPDE
pub type T_CKDPDE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - T_CKCSX
    #[inline(always)]
    pub fn t_ckcsx(&self) -> T_CKCSX_R {
        T_CKCSX_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 16:19 - T_CKDPDX
    #[inline(always)]
    pub fn t_ckdpdx(&self) -> T_CKDPDX_R {
        T_CKDPDX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:27 - T_CKDPDE
    #[inline(always)]
    pub fn t_ckdpde(&self) -> T_CKDPDE_R {
        T_CKDPDE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DRAMTMG6")
            .field("t_ckcsx", &self.t_ckcsx())
            .field("t_ckdpdx", &self.t_ckdpdx())
            .field("t_ckdpde", &self.t_ckdpde())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - T_CKCSX
    #[inline(always)]
    pub fn t_ckcsx(&mut self) -> T_CKCSX_W<'_, DRAMTMG6rs> {
        T_CKCSX_W::new(self, 0)
    }
    ///Bits 16:19 - T_CKDPDX
    #[inline(always)]
    pub fn t_ckdpdx(&mut self) -> T_CKDPDX_W<'_, DRAMTMG6rs> {
        T_CKDPDX_W::new(self, 16)
    }
    ///Bits 24:27 - T_CKDPDE
    #[inline(always)]
    pub fn t_ckdpde(&mut self) -> T_CKDPDE_W<'_, DRAMTMG6rs> {
        T_CKDPDE_W::new(self, 24)
    }
}
/**DDRCTRL SDRAM timing register 6

You can [`read`](crate::Reg::read) this register and get [`dramtmg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DRAMTMG6)*/
pub struct DRAMTMG6rs;
impl crate::RegisterSpec for DRAMTMG6rs {
    type Ux = u32;
}
///`read()` method returns [`dramtmg6::R`](R) reader structure
impl crate::Readable for DRAMTMG6rs {}
///`write(|w| ..)` method takes [`dramtmg6::W`](W) writer structure
impl crate::Writable for DRAMTMG6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DRAMTMG6 to value 0x0202_0005
impl crate::Resettable for DRAMTMG6rs {
    const RESET_VALUE: u32 = 0x0202_0005;
}
