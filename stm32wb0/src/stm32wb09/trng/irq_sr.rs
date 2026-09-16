///Register `IRQ_SR` reader
pub type R = crate::R<IRQ_SRrs>;
///Field `FF_FULL_IRQ` reader - Set to 1 when the output fifo is full of new random. Flag is cleared by writing a 1.
pub type FF_FULL_IRQ_R = crate::BitReader;
///Field `ERROR_IRQ` reader - Set to 1 when an error is reported by the health tests. Flag is cleared by writing a 1.
pub type ERROR_IRQ_R = crate::BitReader;
impl R {
    ///Bit 0 - Set to 1 when the output fifo is full of new random. Flag is cleared by writing a 1.
    #[inline(always)]
    pub fn ff_full_irq(&self) -> FF_FULL_IRQ_R {
        FF_FULL_IRQ_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Set to 1 when an error is reported by the health tests. Flag is cleared by writing a 1.
    #[inline(always)]
    pub fn error_irq(&self) -> ERROR_IRQ_R {
        ERROR_IRQ_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ_SR")
            .field("ff_full_irq", &self.ff_full_irq())
            .field("error_irq", &self.error_irq())
            .finish()
    }
}
/**TRNG_IRQ_SR register

You can [`read`](crate::Reg::read) this register and get [`irq_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:IRQ_SR)*/
pub struct IRQ_SRrs;
impl crate::RegisterSpec for IRQ_SRrs {
    type Ux = u32;
}
///`read()` method returns [`irq_sr::R`](R) reader structure
impl crate::Readable for IRQ_SRrs {}
///`reset()` method sets IRQ_SR to value 0
impl crate::Resettable for IRQ_SRrs {}
