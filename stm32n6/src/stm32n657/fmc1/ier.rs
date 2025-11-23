///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `IREE` reader - Interrupt rising edge detection enable bit
pub type IREE_R = crate::BitReader;
///Field `IREE` writer - Interrupt rising edge detection enable bit
pub type IREE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IHLE` reader - Interrupt high-level detection enable bit
pub type IHLE_R = crate::BitReader;
///Field `IHLE` writer - Interrupt high-level detection enable bit
pub type IHLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IFEE` reader - Interrupt falling edge detection enable bit
pub type IFEE_R = crate::BitReader;
///Field `IFEE` writer - Interrupt falling edge detection enable bit
pub type IFEE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Interrupt rising edge detection enable bit
    #[inline(always)]
    pub fn iree(&self) -> IREE_R {
        IREE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt high-level detection enable bit
    #[inline(always)]
    pub fn ihle(&self) -> IHLE_R {
        IHLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt falling edge detection enable bit
    #[inline(always)]
    pub fn ifee(&self) -> IFEE_R {
        IFEE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("iree", &self.iree())
            .field("ihle", &self.ihle())
            .field("ifee", &self.ifee())
            .finish()
    }
}
impl W {
    ///Bit 0 - Interrupt rising edge detection enable bit
    #[inline(always)]
    pub fn iree(&mut self) -> IREE_W<'_, IERrs> {
        IREE_W::new(self, 0)
    }
    ///Bit 1 - Interrupt high-level detection enable bit
    #[inline(always)]
    pub fn ihle(&mut self) -> IHLE_W<'_, IERrs> {
        IHLE_W::new(self, 1)
    }
    ///Bit 2 - Interrupt falling edge detection enable bit
    #[inline(always)]
    pub fn ifee(&mut self) -> IFEE_W<'_, IERrs> {
        IFEE_W::new(self, 2)
    }
}
/**FMC NAND interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
