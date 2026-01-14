///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `DMAUDR` reader - DMAUDR: DAC channel DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1). 0: No DMA underrun error condition occurred for DAC channel 1: DMA underrun error condition occurred for DAC channel (the currently selected trigger is driving DAC channel conversion at a frequency higher than the DMA service capability rate)
pub type DMAUDR_R = crate::BitReader;
///Field `DMAUDR` writer - DMAUDR: DAC channel DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1). 0: No DMA underrun error condition occurred for DAC channel 1: DMA underrun error condition occurred for DAC channel (the currently selected trigger is driving DAC channel conversion at a frequency higher than the DMA service capability rate)
pub type DMAUDR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 13 - DMAUDR: DAC channel DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1). 0: No DMA underrun error condition occurred for DAC channel 1: DMA underrun error condition occurred for DAC channel (the currently selected trigger is driving DAC channel conversion at a frequency higher than the DMA service capability rate)
    #[inline(always)]
    pub fn dmaudr(&self) -> DMAUDR_R {
        DMAUDR_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("dmaudr", &self.dmaudr())
            .finish()
    }
}
impl W {
    ///Bit 13 - DMAUDR: DAC channel DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1). 0: No DMA underrun error condition occurred for DAC channel 1: DMA underrun error condition occurred for DAC channel (the currently selected trigger is driving DAC channel conversion at a frequency higher than the DMA service capability rate)
    #[inline(always)]
    pub fn dmaudr(&mut self) -> DMAUDR_W<'_, SRrs> {
        DMAUDR_W::new(self, 13)
    }
}
/**SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DAC:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
