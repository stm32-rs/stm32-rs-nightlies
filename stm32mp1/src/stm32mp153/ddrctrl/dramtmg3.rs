///Register `DRAMTMG3` reader
pub type R = crate::R<DRAMTMG3rs>;
///Register `DRAMTMG3` writer
pub type W = crate::W<DRAMTMG3rs>;
///Field `T_MOD` reader - T_MOD
pub type T_MOD_R = crate::FieldReader<u16>;
///Field `T_MOD` writer - T_MOD
pub type T_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `T_MRD` reader - T_MRD
pub type T_MRD_R = crate::FieldReader;
///Field `T_MRD` writer - T_MRD
pub type T_MRD_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `T_MRW` reader - T_MRW
pub type T_MRW_R = crate::FieldReader<u16>;
///Field `T_MRW` writer - T_MRW
pub type T_MRW_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - T_MOD
    #[inline(always)]
    pub fn t_mod(&self) -> T_MOD_R {
        T_MOD_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 12:17 - T_MRD
    #[inline(always)]
    pub fn t_mrd(&self) -> T_MRD_R {
        T_MRD_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    ///Bits 20:29 - T_MRW
    #[inline(always)]
    pub fn t_mrw(&self) -> T_MRW_R {
        T_MRW_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DRAMTMG3")
            .field("t_mod", &self.t_mod())
            .field("t_mrd", &self.t_mrd())
            .field("t_mrw", &self.t_mrw())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - T_MOD
    #[inline(always)]
    pub fn t_mod(&mut self) -> T_MOD_W<'_, DRAMTMG3rs> {
        T_MOD_W::new(self, 0)
    }
    ///Bits 12:17 - T_MRD
    #[inline(always)]
    pub fn t_mrd(&mut self) -> T_MRD_W<'_, DRAMTMG3rs> {
        T_MRD_W::new(self, 12)
    }
    ///Bits 20:29 - T_MRW
    #[inline(always)]
    pub fn t_mrw(&mut self) -> T_MRW_W<'_, DRAMTMG3rs> {
        T_MRW_W::new(self, 20)
    }
}
/**DDRCTRL SDRAM timing register 3

You can [`read`](crate::Reg::read) this register and get [`dramtmg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DRAMTMG3)*/
pub struct DRAMTMG3rs;
impl crate::RegisterSpec for DRAMTMG3rs {
    type Ux = u32;
}
///`read()` method returns [`dramtmg3::R`](R) reader structure
impl crate::Readable for DRAMTMG3rs {}
///`write(|w| ..)` method takes [`dramtmg3::W`](W) writer structure
impl crate::Writable for DRAMTMG3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DRAMTMG3 to value 0x0050_400c
impl crate::Resettable for DRAMTMG3rs {
    const RESET_VALUE: u32 = 0x0050_400c;
}
