///Register `RPR2` reader
pub type R = crate::R<RPR2rs>;
///Register `RPR2` writer
pub type W = crate::W<RPR2rs>;
///Field `RPIF34` reader - Rising edge event pending for configurable line 34 Each bit is set upon a rising edge event generated by hardware or by software (through the EXTI_SWIER2 register) on the line 34. Each bit is cleared by writing 1 into it.
pub type RPIF34_R = crate::BitReader;
///Field `RPIF34` writer - Rising edge event pending for configurable line 34 Each bit is set upon a rising edge event generated by hardware or by software (through the EXTI_SWIER2 register) on the line 34. Each bit is cleared by writing 1 into it.
pub type RPIF34_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Rising edge event pending for configurable line 34 Each bit is set upon a rising edge event generated by hardware or by software (through the EXTI_SWIER2 register) on the line 34. Each bit is cleared by writing 1 into it.
    #[inline(always)]
    pub fn rpif34(&self) -> RPIF34_R {
        RPIF34_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPR2")
            .field("rpif34", &self.rpif34())
            .finish()
    }
}
impl W {
    ///Bit 2 - Rising edge event pending for configurable line 34 Each bit is set upon a rising edge event generated by hardware or by software (through the EXTI_SWIER2 register) on the line 34. Each bit is cleared by writing 1 into it.
    #[inline(always)]
    pub fn rpif34(&mut self) -> RPIF34_W<RPR2rs> {
        RPIF34_W::new(self, 2)
    }
}
/**EXTI rising edge pending register 2

You can [`read`](crate::Reg::read) this register and get [`rpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#EXTI:RPR2)*/
pub struct RPR2rs;
impl crate::RegisterSpec for RPR2rs {
    type Ux = u32;
}
///`read()` method returns [`rpr2::R`](R) reader structure
impl crate::Readable for RPR2rs {}
///`write(|w| ..)` method takes [`rpr2::W`](W) writer structure
impl crate::Writable for RPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RPR2 to value 0
impl crate::Resettable for RPR2rs {
    const RESET_VALUE: u32 = 0;
}