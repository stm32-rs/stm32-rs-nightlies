///Register `LPTIM_CFGR2` reader
pub type R = crate::R<LPTIM_CFGR2rs>;
///Register `LPTIM_CFGR2` writer
pub type W = crate::W<LPTIM_CFGR2rs>;
///Field `IN1SEL` reader - LPTIM Input 1 selection
pub type IN1SEL_R = crate::FieldReader;
///Field `IN1SEL` writer - LPTIM Input 1 selection
pub type IN1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - LPTIM Input 1 selection
    #[inline(always)]
    pub fn in1sel(&self) -> IN1SEL_R {
        IN1SEL_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_CFGR2")
            .field("in1sel", &self.in1sel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - LPTIM Input 1 selection
    #[inline(always)]
    pub fn in1sel(&mut self) -> IN1SEL_W<LPTIM_CFGR2rs> {
        IN1SEL_W::new(self, 0)
    }
}
/**LPTIM configuration register 2

You can [`read`](crate::Reg::read) this register and get [`lptim_cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim_cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#LPTIM3:LPTIM_CFGR2)*/
pub struct LPTIM_CFGR2rs;
impl crate::RegisterSpec for LPTIM_CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`lptim_cfgr2::R`](R) reader structure
impl crate::Readable for LPTIM_CFGR2rs {}
///`write(|w| ..)` method takes [`lptim_cfgr2::W`](W) writer structure
impl crate::Writable for LPTIM_CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPTIM_CFGR2 to value 0
impl crate::Resettable for LPTIM_CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
