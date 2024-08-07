///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `EOAIE` reader - End of acquisition interrupt enable
pub type EOAIE_R = crate::BitReader;
///Field `EOAIE` writer - End of acquisition interrupt enable
pub type EOAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCEIE` reader - Max count error interrupt enable
pub type MCEIE_R = crate::BitReader;
///Field `MCEIE` writer - Max count error interrupt enable
pub type MCEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - End of acquisition interrupt enable
    #[inline(always)]
    pub fn eoaie(&self) -> EOAIE_R {
        EOAIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Max count error interrupt enable
    #[inline(always)]
    pub fn mceie(&self) -> MCEIE_R {
        MCEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("mceie", &self.mceie())
            .field("eoaie", &self.eoaie())
            .finish()
    }
}
impl W {
    ///Bit 0 - End of acquisition interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn eoaie(&mut self) -> EOAIE_W<IERrs> {
        EOAIE_W::new(self, 0)
    }
    ///Bit 1 - Max count error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn mceie(&mut self) -> MCEIE_W<IERrs> {
        MCEIE_W::new(self, 1)
    }
}
/**interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#TSC:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
