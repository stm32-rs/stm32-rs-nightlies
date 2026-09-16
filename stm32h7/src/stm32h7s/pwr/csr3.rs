///Register `CSR3` reader
pub type R = crate::R<CSR3rs>;
///Register `CSR3` writer
pub type W = crate::W<CSR3rs>;
///Field `PDDS` reader - Power Down Deepsleep. This bit allows CPU to define the Deepsleep mode
pub type PDDS_R = crate::BitReader;
///Field `PDDS` writer - Power Down Deepsleep. This bit allows CPU to define the Deepsleep mode
pub type PDDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSSF` reader - Clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware.
pub type CSSF_R = crate::BitReader;
///Field `CSSF` writer - Clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware.
pub type CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOPF` reader - STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU CSSF bit.
pub type STOPF_R = crate::BitReader;
///Field `SBF` reader - System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU CSSF bit
pub type SBF_R = crate::BitReader;
impl R {
    ///Bit 0 - Power Down Deepsleep. This bit allows CPU to define the Deepsleep mode
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware.
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU CSSF bit.
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU CSSF bit
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR3")
            .field("pdds", &self.pdds())
            .field("cssf", &self.cssf())
            .field("stopf", &self.stopf())
            .field("sbf", &self.sbf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Power Down Deepsleep. This bit allows CPU to define the Deepsleep mode
    #[inline(always)]
    pub fn pdds(&mut self) -> PDDS_W<'_, CSR3rs> {
        PDDS_W::new(self, 0)
    }
    ///Bit 1 - Clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware.
    #[inline(always)]
    pub fn cssf(&mut self) -> CSSF_W<'_, CSR3rs> {
        CSSF_W::new(self, 1)
    }
}
/**PWR CPU control register 3

You can [`read`](crate::Reg::read) this register and get [`csr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#PWR:CSR3)*/
pub struct CSR3rs;
impl crate::RegisterSpec for CSR3rs {
    type Ux = u32;
}
///`read()` method returns [`csr3::R`](R) reader structure
impl crate::Readable for CSR3rs {}
///`write(|w| ..)` method takes [`csr3::W`](W) writer structure
impl crate::Writable for CSR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR3 to value 0
impl crate::Resettable for CSR3rs {}
