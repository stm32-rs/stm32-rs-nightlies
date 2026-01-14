///Register `IRQ_CR` reader
pub type R = crate::R<IRQ_CRrs>;
///Register `IRQ_CR` writer
pub type W = crate::W<IRQ_CRrs>;
///Field `EN_FF_FULL_IRQ` reader - Enable the interrupt when the output fifo is full of new random.
pub type EN_FF_FULL_IRQ_R = crate::BitReader;
///Field `EN_FF_FULL_IRQ` writer - Enable the interrupt when the output fifo is full of new random.
pub type EN_FF_FULL_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_ERROR_IRQ` reader - Enable the interrupt when an error is reported by the health tests.
pub type EN_ERROR_IRQ_R = crate::BitReader;
///Field `EN_ERROR_IRQ` writer - Enable the interrupt when an error is reported by the health tests.
pub type EN_ERROR_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable the interrupt when the output fifo is full of new random.
    #[inline(always)]
    pub fn en_ff_full_irq(&self) -> EN_FF_FULL_IRQ_R {
        EN_FF_FULL_IRQ_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Enable the interrupt when an error is reported by the health tests.
    #[inline(always)]
    pub fn en_error_irq(&self) -> EN_ERROR_IRQ_R {
        EN_ERROR_IRQ_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ_CR")
            .field("en_ff_full_irq", &self.en_ff_full_irq())
            .field("en_error_irq", &self.en_error_irq())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable the interrupt when the output fifo is full of new random.
    #[inline(always)]
    pub fn en_ff_full_irq(&mut self) -> EN_FF_FULL_IRQ_W<'_, IRQ_CRrs> {
        EN_FF_FULL_IRQ_W::new(self, 0)
    }
    ///Bit 8 - Enable the interrupt when an error is reported by the health tests.
    #[inline(always)]
    pub fn en_error_irq(&mut self) -> EN_ERROR_IRQ_W<'_, IRQ_CRrs> {
        EN_ERROR_IRQ_W::new(self, 8)
    }
}
/**TRNG_IRQ_CR register

You can [`read`](crate::Reg::read) this register and get [`irq_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:IRQ_CR)*/
pub struct IRQ_CRrs;
impl crate::RegisterSpec for IRQ_CRrs {
    type Ux = u32;
}
///`read()` method returns [`irq_cr::R`](R) reader structure
impl crate::Readable for IRQ_CRrs {}
///`write(|w| ..)` method takes [`irq_cr::W`](W) writer structure
impl crate::Writable for IRQ_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IRQ_CR to value 0
impl crate::Resettable for IRQ_CRrs {}
