///Register `MCUCR` reader
pub type R = crate::R<MCUCRrs>;
///Register `MCUCR` writer
pub type W = crate::W<MCUCRrs>;
///Field `PDDS` reader - PDDS
pub type PDDS_R = crate::BitReader;
///Field `PDDS` writer - PDDS
pub type PDDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOPF` reader - STOPF
pub type STOPF_R = crate::BitReader;
///Field `SBF` reader - SBF
pub type SBF_R = crate::BitReader;
///Field `CSSF` reader - CSSF
pub type CSSF_R = crate::BitReader;
///Field `CSSF` writer - CSSF
pub type CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEEPSLEEP` reader - DEEPSLEEP
pub type DEEPSLEEP_R = crate::BitReader;
impl R {
    ///Bit 0 - PDDS
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new((self.bits & 1) != 0)
    }
    ///Bit 5 - STOPF
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SBF
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - CSSF
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 15 - DEEPSLEEP
    #[inline(always)]
    pub fn deepsleep(&self) -> DEEPSLEEP_R {
        DEEPSLEEP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCUCR")
            .field("pdds", &self.pdds())
            .field("stopf", &self.stopf())
            .field("sbf", &self.sbf())
            .field("cssf", &self.cssf())
            .field("deepsleep", &self.deepsleep())
            .finish()
    }
}
impl W {
    ///Bit 0 - PDDS
    #[inline(always)]
    pub fn pdds(&mut self) -> PDDS_W<'_, MCUCRrs> {
        PDDS_W::new(self, 0)
    }
    ///Bit 9 - CSSF
    #[inline(always)]
    pub fn cssf(&mut self) -> CSSF_W<'_, MCUCRrs> {
        CSSF_W::new(self, 9)
    }
}
/**See individual bits for reset condition. Access 6 wait states when writing this register. This register is always non-secure. When a system reset occurs during the register write cycle the written data is not guaranteed.

You can [`read`](crate::Reg::read) this register and get [`mcucr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#PWR:MCUCR)*/
pub struct MCUCRrs;
impl crate::RegisterSpec for MCUCRrs {
    type Ux = u32;
}
///`read()` method returns [`mcucr::R`](R) reader structure
impl crate::Readable for MCUCRrs {}
///`write(|w| ..)` method takes [`mcucr::W`](W) writer structure
impl crate::Writable for MCUCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MCUCR to value 0
impl crate::Resettable for MCUCRrs {}
