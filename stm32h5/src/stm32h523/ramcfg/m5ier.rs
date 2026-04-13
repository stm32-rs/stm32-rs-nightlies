///Register `M5IER` reader
pub type R = crate::R<M5IERrs>;
///Register `M5IER` writer
pub type W = crate::W<M5IERrs>;
///Field `SEIE` reader - ECC single error interrupt enable
pub type SEIE_R = crate::BitReader;
///Field `SEIE` writer - ECC single error interrupt enable
pub type SEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEIE` reader - ECC double error interrupt enable
pub type DEIE_R = crate::BitReader;
///Field `DEIE` writer - ECC double error interrupt enable
pub type DEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCNMI` reader - Double error NMI
pub type ECCNMI_R = crate::BitReader;
///Field `ECCNMI` writer - Double error NMI
pub type ECCNMI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ECC single error interrupt enable
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ECC double error interrupt enable
    #[inline(always)]
    pub fn deie(&self) -> DEIE_R {
        DEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Double error NMI
    #[inline(always)]
    pub fn eccnmi(&self) -> ECCNMI_R {
        ECCNMI_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M5IER")
            .field("seie", &self.seie())
            .field("deie", &self.deie())
            .field("eccnmi", &self.eccnmi())
            .finish()
    }
}
impl W {
    ///Bit 0 - ECC single error interrupt enable
    #[inline(always)]
    pub fn seie(&mut self) -> SEIE_W<'_, M5IERrs> {
        SEIE_W::new(self, 0)
    }
    ///Bit 1 - ECC double error interrupt enable
    #[inline(always)]
    pub fn deie(&mut self) -> DEIE_W<'_, M5IERrs> {
        DEIE_W::new(self, 1)
    }
    ///Bit 3 - Double error NMI
    #[inline(always)]
    pub fn eccnmi(&mut self) -> ECCNMI_W<'_, M5IERrs> {
        ECCNMI_W::new(self, 3)
    }
}
/**RAMCFG memory 5 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`m5ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#RAMCFG:M5IER)*/
pub struct M5IERrs;
impl crate::RegisterSpec for M5IERrs {
    type Ux = u32;
}
///`read()` method returns [`m5ier::R`](R) reader structure
impl crate::Readable for M5IERrs {}
///`write(|w| ..)` method takes [`m5ier::W`](W) writer structure
impl crate::Writable for M5IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M5IER to value 0
impl crate::Resettable for M5IERrs {}
