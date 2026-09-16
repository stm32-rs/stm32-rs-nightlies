///Register `EWCR` reader
pub type R = crate::R<EWCRrs>;
///Register `EWCR` writer
pub type W = crate::W<EWCRrs>;
///Field `EWIT` reader - EWIT
pub type EWIT_R = crate::FieldReader<u16>;
///Field `EWIT` writer - EWIT
pub type EWIT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `EWIC` reader - EWIC
pub type EWIC_R = crate::BitReader;
///Field `EWIC` writer - EWIC
pub type EWIC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWIE` reader - EWIE
pub type EWIE_R = crate::BitReader;
///Field `EWIE` writer - EWIE
pub type EWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11 - EWIT
    #[inline(always)]
    pub fn ewit(&self) -> EWIT_R {
        EWIT_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 14 - EWIC
    #[inline(always)]
    pub fn ewic(&self) -> EWIC_R {
        EWIC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - EWIE
    #[inline(always)]
    pub fn ewie(&self) -> EWIE_R {
        EWIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EWCR")
            .field("ewit", &self.ewit())
            .field("ewic", &self.ewic())
            .field("ewie", &self.ewie())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - EWIT
    #[inline(always)]
    pub fn ewit(&mut self) -> EWIT_W<'_, EWCRrs> {
        EWIT_W::new(self, 0)
    }
    ///Bit 14 - EWIC
    #[inline(always)]
    pub fn ewic(&mut self) -> EWIC_W<'_, EWCRrs> {
        EWIC_W::new(self, 14)
    }
    ///Bit 15 - EWIE
    #[inline(always)]
    pub fn ewie(&mut self) -> EWIE_W<'_, EWCRrs> {
        EWIE_W::new(self, 15)
    }
}
/**IWDG early wake-up interrupt register

You can [`read`](crate::Reg::read) this register and get [`ewcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IWDG1:EWCR)*/
pub struct EWCRrs;
impl crate::RegisterSpec for EWCRrs {
    type Ux = u32;
}
///`read()` method returns [`ewcr::R`](R) reader structure
impl crate::Readable for EWCRrs {}
///`write(|w| ..)` method takes [`ewcr::W`](W) writer structure
impl crate::Writable for EWCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EWCR to value 0
impl crate::Resettable for EWCRrs {}
