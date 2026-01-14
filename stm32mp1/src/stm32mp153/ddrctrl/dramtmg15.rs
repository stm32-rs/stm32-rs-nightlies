///Register `DRAMTMG15` reader
pub type R = crate::R<DRAMTMG15rs>;
///Register `DRAMTMG15` writer
pub type W = crate::W<DRAMTMG15rs>;
///Field `T_STAB_X32` reader - T_STAB_X32
pub type T_STAB_X32_R = crate::FieldReader;
///Field `T_STAB_X32` writer - T_STAB_X32
pub type T_STAB_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EN_DFI_LP_T_STAB` reader - EN_DFI_LP_T_STAB
pub type EN_DFI_LP_T_STAB_R = crate::BitReader;
///Field `EN_DFI_LP_T_STAB` writer - EN_DFI_LP_T_STAB
pub type EN_DFI_LP_T_STAB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - T_STAB_X32
    #[inline(always)]
    pub fn t_stab_x32(&self) -> T_STAB_X32_R {
        T_STAB_X32_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 31 - EN_DFI_LP_T_STAB
    #[inline(always)]
    pub fn en_dfi_lp_t_stab(&self) -> EN_DFI_LP_T_STAB_R {
        EN_DFI_LP_T_STAB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DRAMTMG15")
            .field("t_stab_x32", &self.t_stab_x32())
            .field("en_dfi_lp_t_stab", &self.en_dfi_lp_t_stab())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - T_STAB_X32
    #[inline(always)]
    pub fn t_stab_x32(&mut self) -> T_STAB_X32_W<'_, DRAMTMG15rs> {
        T_STAB_X32_W::new(self, 0)
    }
    ///Bit 31 - EN_DFI_LP_T_STAB
    #[inline(always)]
    pub fn en_dfi_lp_t_stab(&mut self) -> EN_DFI_LP_T_STAB_W<'_, DRAMTMG15rs> {
        EN_DFI_LP_T_STAB_W::new(self, 31)
    }
}
/**DDRCTRL SDRAM timing register 15

You can [`read`](crate::Reg::read) this register and get [`dramtmg15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DRAMTMG15)*/
pub struct DRAMTMG15rs;
impl crate::RegisterSpec for DRAMTMG15rs {
    type Ux = u32;
}
///`read()` method returns [`dramtmg15::R`](R) reader structure
impl crate::Readable for DRAMTMG15rs {}
///`write(|w| ..)` method takes [`dramtmg15::W`](W) writer structure
impl crate::Writable for DRAMTMG15rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DRAMTMG15 to value 0
impl crate::Resettable for DRAMTMG15rs {}
