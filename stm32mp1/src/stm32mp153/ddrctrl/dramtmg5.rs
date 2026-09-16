///Register `DRAMTMG5` reader
pub type R = crate::R<DRAMTMG5rs>;
///Register `DRAMTMG5` writer
pub type W = crate::W<DRAMTMG5rs>;
///Field `T_CKE` reader - T_CKE
pub type T_CKE_R = crate::FieldReader;
///Field `T_CKE` writer - T_CKE
pub type T_CKE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `T_CKESR` reader - T_CKESR
pub type T_CKESR_R = crate::FieldReader;
///Field `T_CKESR` writer - T_CKESR
pub type T_CKESR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `T_CKSRE` reader - T_CKSRE
pub type T_CKSRE_R = crate::FieldReader;
///Field `T_CKSRE` writer - T_CKSRE
pub type T_CKSRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `T_CKSRX` reader - T_CKSRX
pub type T_CKSRX_R = crate::FieldReader;
///Field `T_CKSRX` writer - T_CKSRX
pub type T_CKSRX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:4 - T_CKE
    #[inline(always)]
    pub fn t_cke(&self) -> T_CKE_R {
        T_CKE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:13 - T_CKESR
    #[inline(always)]
    pub fn t_ckesr(&self) -> T_CKESR_R {
        T_CKESR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:19 - T_CKSRE
    #[inline(always)]
    pub fn t_cksre(&self) -> T_CKSRE_R {
        T_CKSRE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:27 - T_CKSRX
    #[inline(always)]
    pub fn t_cksrx(&self) -> T_CKSRX_R {
        T_CKSRX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DRAMTMG5")
            .field("t_cke", &self.t_cke())
            .field("t_ckesr", &self.t_ckesr())
            .field("t_cksre", &self.t_cksre())
            .field("t_cksrx", &self.t_cksrx())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - T_CKE
    #[inline(always)]
    pub fn t_cke(&mut self) -> T_CKE_W<'_, DRAMTMG5rs> {
        T_CKE_W::new(self, 0)
    }
    ///Bits 8:13 - T_CKESR
    #[inline(always)]
    pub fn t_ckesr(&mut self) -> T_CKESR_W<'_, DRAMTMG5rs> {
        T_CKESR_W::new(self, 8)
    }
    ///Bits 16:19 - T_CKSRE
    #[inline(always)]
    pub fn t_cksre(&mut self) -> T_CKSRE_W<'_, DRAMTMG5rs> {
        T_CKSRE_W::new(self, 16)
    }
    ///Bits 24:27 - T_CKSRX
    #[inline(always)]
    pub fn t_cksrx(&mut self) -> T_CKSRX_W<'_, DRAMTMG5rs> {
        T_CKSRX_W::new(self, 24)
    }
}
/**DDRCTRL SDRAM timing register 5

You can [`read`](crate::Reg::read) this register and get [`dramtmg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dramtmg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DRAMTMG5)*/
pub struct DRAMTMG5rs;
impl crate::RegisterSpec for DRAMTMG5rs {
    type Ux = u32;
}
///`read()` method returns [`dramtmg5::R`](R) reader structure
impl crate::Readable for DRAMTMG5rs {}
///`write(|w| ..)` method takes [`dramtmg5::W`](W) writer structure
impl crate::Writable for DRAMTMG5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DRAMTMG5 to value 0x0505_0403
impl crate::Resettable for DRAMTMG5rs {
    const RESET_VALUE: u32 = 0x0505_0403;
}
