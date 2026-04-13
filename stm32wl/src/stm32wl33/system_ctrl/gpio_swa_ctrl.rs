///Register `GPIO_SWA_CTRL` reader
pub type R = crate::R<GPIO_SWA_CTRLrs>;
///Register `GPIO_SWA_CTRL` writer
pub type W = crate::W<GPIO_SWA_CTRLrs>;
///Field `ATB1_nPVD` reader - ATB1_nPVD: select the analog feature on PB14 between ATB1 and PVD when the PB14 I/O is programmed in analog mode (in the associated GPIO_MODER register): 0: PVD external voltage feature is selected (default). 1: ATB1 feature is selected
pub type ATB1_N_PVD_R = crate::BitReader;
///Field `ATB1_nPVD` writer - ATB1_nPVD: select the analog feature on PB14 between ATB1 and PVD when the PB14 I/O is programmed in analog mode (in the associated GPIO_MODER register): 0: PVD external voltage feature is selected (default). 1: ATB1 feature is selected
pub type ATB1_N_PVD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ATB1_nPVD: select the analog feature on PB14 between ATB1 and PVD when the PB14 I/O is programmed in analog mode (in the associated GPIO_MODER register): 0: PVD external voltage feature is selected (default). 1: ATB1 feature is selected
    #[inline(always)]
    pub fn atb1_n_pvd(&self) -> ATB1_N_PVD_R {
        ATB1_N_PVD_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SWA_CTRL")
            .field("atb1_n_pvd", &self.atb1_n_pvd())
            .finish()
    }
}
impl W {
    ///Bit 0 - ATB1_nPVD: select the analog feature on PB14 between ATB1 and PVD when the PB14 I/O is programmed in analog mode (in the associated GPIO_MODER register): 0: PVD external voltage feature is selected (default). 1: ATB1 feature is selected
    #[inline(always)]
    pub fn atb1_n_pvd(&mut self) -> ATB1_N_PVD_W<'_, GPIO_SWA_CTRLrs> {
        ATB1_N_PVD_W::new(self, 0)
    }
}
/**GPIO_SWA_CTRL register

You can [`read`](crate::Reg::read) this register and get [`gpio_swa_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_swa_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:GPIO_SWA_CTRL)*/
pub struct GPIO_SWA_CTRLrs;
impl crate::RegisterSpec for GPIO_SWA_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`gpio_swa_ctrl::R`](R) reader structure
impl crate::Readable for GPIO_SWA_CTRLrs {}
///`write(|w| ..)` method takes [`gpio_swa_ctrl::W`](W) writer structure
impl crate::Writable for GPIO_SWA_CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPIO_SWA_CTRL to value 0
impl crate::Resettable for GPIO_SWA_CTRLrs {}
