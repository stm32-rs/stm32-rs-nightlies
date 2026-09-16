///Register `SCSR` reader
pub type R = crate::R<SCSRrs>;
///Register `SCSR` writer
pub type W = crate::W<SCSRrs>;
///Field `SRAM2ER` reader - SRAM2 Erase
pub type SRAM2ER_R = crate::BitReader;
///Field `SRAM2ER` writer - SRAM2 Erase
pub type SRAM2ER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2BSY` reader - SRAM2 busy by erase operation
pub type SRAM2BSY_R = crate::BitReader;
impl R {
    ///Bit 0 - SRAM2 Erase
    #[inline(always)]
    pub fn sram2er(&self) -> SRAM2ER_R {
        SRAM2ER_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM2 busy by erase operation
    #[inline(always)]
    pub fn sram2bsy(&self) -> SRAM2BSY_R {
        SRAM2BSY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCSR")
            .field("sram2bsy", &self.sram2bsy())
            .field("sram2er", &self.sram2er())
            .finish()
    }
}
impl W {
    ///Bit 0 - SRAM2 Erase
    #[inline(always)]
    pub fn sram2er(&mut self) -> SRAM2ER_W<'_, SCSRrs> {
        SRAM2ER_W::new(self, 0)
    }
}
/**SCSR

You can [`read`](crate::Reg::read) this register and get [`scsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#SYSCFG:SCSR)*/
pub struct SCSRrs;
impl crate::RegisterSpec for SCSRrs {
    type Ux = u32;
}
///`read()` method returns [`scsr::R`](R) reader structure
impl crate::Readable for SCSRrs {}
///`write(|w| ..)` method takes [`scsr::W`](W) writer structure
impl crate::Writable for SCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCSR to value 0
impl crate::Resettable for SCSRrs {}
