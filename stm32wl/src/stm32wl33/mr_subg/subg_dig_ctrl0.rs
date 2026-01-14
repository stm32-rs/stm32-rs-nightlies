///Register `SUBG_DIG_CTRL0` reader
pub type R = crate::R<SUBG_DIG_CTRL0rs>;
///Register `SUBG_DIG_CTRL0` writer
pub type W = crate::W<SUBG_DIG_CTRL0rs>;
///Field `FORCE_GPIO_OUTPUT` reader - Option for the direct GPIO signal output
pub type FORCE_GPIO_OUTPUT_R = crate::BitReader;
///Field `FORCE_GPIO_OUTPUT` writer - Option for the direct GPIO signal output
pub type FORCE_GPIO_OUTPUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Option for the direct GPIO signal output
    #[inline(always)]
    pub fn force_gpio_output(&self) -> FORCE_GPIO_OUTPUT_R {
        FORCE_GPIO_OUTPUT_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUBG_DIG_CTRL0")
            .field("force_gpio_output", &self.force_gpio_output())
            .finish()
    }
}
impl W {
    ///Bit 0 - Option for the direct GPIO signal output
    #[inline(always)]
    pub fn force_gpio_output(&mut self) -> FORCE_GPIO_OUTPUT_W<'_, SUBG_DIG_CTRL0rs> {
        FORCE_GPIO_OUTPUT_W::new(self, 0)
    }
}
/**SUBG_DIG_CTRL0 register

You can [`read`](crate::Reg::read) this register and get [`subg_dig_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subg_dig_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:SUBG_DIG_CTRL0)*/
pub struct SUBG_DIG_CTRL0rs;
impl crate::RegisterSpec for SUBG_DIG_CTRL0rs {
    type Ux = u32;
}
///`read()` method returns [`subg_dig_ctrl0::R`](R) reader structure
impl crate::Readable for SUBG_DIG_CTRL0rs {}
///`write(|w| ..)` method takes [`subg_dig_ctrl0::W`](W) writer structure
impl crate::Writable for SUBG_DIG_CTRL0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SUBG_DIG_CTRL0 to value 0
impl crate::Resettable for SUBG_DIG_CTRL0rs {}
