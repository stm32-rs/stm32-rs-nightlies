///Register `SWIER2` reader
pub type R = crate::R<SWIER2rs>;
///Register `SWIER2` writer
pub type W = crate::W<SWIER2rs>;
///Field `SWI34` reader - Software rising edge event trigger on line 34 Setting of any bit by software triggers a rising edge event on the line 34, resulting in an interrupt, independently of EXTI_RTSR2 and EXTI_FTSR2 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI34_R = crate::BitReader;
///Field `SWI34` writer - Software rising edge event trigger on line 34 Setting of any bit by software triggers a rising edge event on the line 34, resulting in an interrupt, independently of EXTI_RTSR2 and EXTI_FTSR2 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
pub type SWI34_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Software rising edge event trigger on line 34 Setting of any bit by software triggers a rising edge event on the line 34, resulting in an interrupt, independently of EXTI_RTSR2 and EXTI_FTSR2 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi34(&self) -> SWI34_R {
        SWI34_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER2")
            .field("swi34", &self.swi34())
            .finish()
    }
}
impl W {
    ///Bit 2 - Software rising edge event trigger on line 34 Setting of any bit by software triggers a rising edge event on the line 34, resulting in an interrupt, independently of EXTI_RTSR2 and EXTI_FTSR2 settings. The bits are automatically cleared by HW. Reading of any bit always returns 0.
    #[inline(always)]
    pub fn swi34(&mut self) -> SWI34_W<SWIER2rs> {
        SWI34_W::new(self, 2)
    }
}
/**EXTI software interrupt event register 2

You can [`read`](crate::Reg::read) this register and get [`swier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#EXTI:SWIER2)*/
pub struct SWIER2rs;
impl crate::RegisterSpec for SWIER2rs {
    type Ux = u32;
}
///`read()` method returns [`swier2::R`](R) reader structure
impl crate::Readable for SWIER2rs {}
///`write(|w| ..)` method takes [`swier2::W`](W) writer structure
impl crate::Writable for SWIER2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SWIER2 to value 0
impl crate::Resettable for SWIER2rs {
    const RESET_VALUE: u32 = 0;
}
